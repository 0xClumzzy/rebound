use std::process::{Command, Stdio};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

use crate::tasks::Task;

#[derive(PartialEq, Clone)]
pub enum RunState {
    Idle,
    Running,
    Done,
}

pub struct Runner {
    pub state: Arc<Mutex<RunState>>,
    pub output: Arc<Mutex<Vec<String>>>,
    pub current: Arc<Mutex<usize>>,
    pub total: Arc<Mutex<usize>>,
    pub current_name: Arc<Mutex<String>>,
    pub success: Arc<Mutex<bool>>,
    pub tick: Arc<Mutex<u64>>,
    pub failed_tasks: Arc<Mutex<Vec<String>>>,
    pub last_progress: Arc<Mutex<Instant>>,
}

impl Runner {
    pub fn new() -> Self {
        Self {
            state: Arc::new(Mutex::new(RunState::Idle)),
            output: Arc::new(Mutex::new(Vec::new())),
            current: Arc::new(Mutex::new(0)),
            total: Arc::new(Mutex::new(0)),
            current_name: Arc::new(Mutex::new(String::new())),
            success: Arc::new(Mutex::new(true)),
            tick: Arc::new(Mutex::new(0)),
            failed_tasks: Arc::new(Mutex::new(Vec::new())),
            last_progress: Arc::new(Mutex::new(Instant::now())),
        }
    }

    pub fn run_all(&self, tasks: &[(&crate::tasks::Category, &Task)], home: &str) {
        *self.state.lock().unwrap() = RunState::Running;
        *self.tick.lock().unwrap() = 0;
        *self.output.lock().unwrap() = vec![
            "----".repeat(12),
            "  rebound".to_string(),
            "  Clumzzy's Arch Linux Environment".to_string(),
            "----".repeat(12),
            "".to_string(),
            format!("  {} tasks selected", tasks.len()),
            "".to_string(),
        ];
        *self.current.lock().unwrap() = 0;
        *self.success.lock().unwrap() = true;
        *self.failed_tasks.lock().unwrap() = Vec::new();

        let output = Arc::clone(&self.output);
        let current = Arc::clone(&self.current);
        let total = Arc::clone(&self.total);
        let current_name = Arc::clone(&self.current_name);
        let success = Arc::clone(&self.success);
        let state = Arc::clone(&self.state);
        let _tick = Arc::clone(&self.tick);
        let failed = Arc::clone(&self.failed_tasks);
        let last_progress = Arc::clone(&self.last_progress);
        let home = home.to_string();

        let tasks_owned: Vec<(String, Vec<String>, Vec<(String, String)>, bool, bool)> = tasks
            .iter()
            .map(|(cat, task)| {
                let has_deploy = !task.deploy_files.is_empty();
                (format!("{}  {}", cat.icon, task.name), task.commands.clone(), task.deploy_files.clone(), has_deploy, task.is_wallpapers)
            })
            .collect();

        // Count total commands for granular progress
        let mut total_cmds: usize = 0;
        for (_, commands, _, _, _) in &tasks_owned {
            total_cmds += commands.iter().filter(|c| !c.starts_with('#')).count();
        }
        // Add 1 per task for file deployments
        total_cmds += tasks_owned.len();
        *total.lock().unwrap() = total_cmds;

        thread::spawn(move || {
            let mut progress: usize = 0;

            for (i, (name, commands, deploy_files, has_deploy, is_wallpapers)) in tasks_owned.iter().enumerate() {
                current_name.lock().unwrap().clone_from(name);

                {
                    let mut out = output.lock().unwrap();
                    out.push(format!("  -- {} --", name));
                }

                // Handle wallpapers deployment
                if *is_wallpapers {
                    let results = crate::wallpapers::deploy_all(&home);
                    for result in results {
                        let mut out = output.lock().unwrap();
                        match result {
                            Ok(msg) => out.push(format!("  [ok] {}", msg)),
                            Err(msg) => {
                                out.push(format!("  [!!] {}", msg));
                                *success.lock().unwrap() = false;
                                failed.lock().unwrap().push(name.clone());
                            }
                        }
                    }
                    progress += 1;
                    *current.lock().unwrap() = progress;
                    *last_progress.lock().unwrap() = Instant::now();
                }

                // Deploy embedded files first
                if *has_deploy {
                    for (rel_path, content) in deploy_files {
                        let path = std::path::PathBuf::from(&home).join(rel_path);
                        if let Some(parent) = path.parent() {
                            let _ = std::fs::create_dir_all(parent);
                        }
                        match std::fs::write(&path, content) {
                            Ok(()) => {
                                let mut out = output.lock().unwrap();
                                out.push(format!("  [ok] deployed ~{}", rel_path));
                            }
                            Err(e) => {
                                let mut out = output.lock().unwrap();
                                out.push(format!("  [!!] failed ~{}: {}", rel_path, e));
                                *success.lock().unwrap() = false;
                                failed.lock().unwrap().push(name.clone());
                            }
                        }
                    }
                    progress += 1;
                    *current.lock().unwrap() = progress;
                    *last_progress.lock().unwrap() = Instant::now();
                }

                for cmd in commands {
                    if cmd.starts_with('#') {
                        continue;
                    }

                    // Detect sudo commands and warn about password
                    if cmd.contains("sudo") {
                        let mut out = output.lock().unwrap();
                        out.push("".to_string());
                        out.push("  >> SUDO - enter password when prompted <<".to_string());
                        out.push("".to_string());
                    }

                    {
                        let mut out = output.lock().unwrap();
                        out.push(format!("  \u{251c}\u{2500} $ {}", cmd));
                    }

                    let result = Command::new("sh")
                        .arg("-c")
                        .arg(cmd)
                        .stdout(Stdio::piped())
                        .stderr(Stdio::piped())
                        .output();

                    match result {
                        Ok(res) => {
                            let stdout = String::from_utf8_lossy(&res.stdout);
                            let stderr = String::from_utf8_lossy(&res.stderr);

                            for line in stdout.lines().take(3) {
                                if !line.trim().is_empty() {
                                    let mut out = output.lock().unwrap();
                                    out.push(format!("  |  {}", line));
                                }
                            }

                            if !res.status.success() && !stderr.trim().is_empty() {
                                let mut out = output.lock().unwrap();
                                for line in stderr.lines().take(3) {
                                    out.push(format!("  |  ! {}", line));
                                }
                            }
                        }
                        Err(e) => {
                            let mut out = output.lock().unwrap();
                            out.push(format!("  |  [ERR] {}", e));
                            *success.lock().unwrap() = false;
                            failed.lock().unwrap().push(name.clone());
                        }
                    }

                    // Update progress after each command
                    progress += 1;
                    *current.lock().unwrap() = progress;
                    *last_progress.lock().unwrap() = Instant::now();
                }

                {
                    let mut out = output.lock().unwrap();
                    out.push(format!("  `-- [ok] complete"));
                    out.push("".to_string());
                }
            }

            // Set to 100% when done
            let final_total = *total.lock().unwrap();
            *current.lock().unwrap() = final_total;
            *last_progress.lock().unwrap() = Instant::now();

            {
                let mut out = output.lock().unwrap();
                out.push("\u{2500}".repeat(50));
                if *success.lock().unwrap() {
                    out.push("  All tasks completed!".to_string());
                } else {
                    out.push("  ! Completed with errors".to_string());
                }
                out.push("\u{2500}".repeat(50));
            }

            *state.lock().unwrap() = RunState::Done;
        });

        // Animation ticker
        let tick_clone = Arc::clone(&self.tick);
        let state_clone = Arc::clone(&self.state);
        thread::spawn(move || loop {
            {
                let s = state_clone.lock().unwrap().clone();
                if s == RunState::Done {
                    break;
                }
            }
            *tick_clone.lock().unwrap() += 1;
            thread::sleep(Duration::from_millis(80));
        });
    }

    pub fn is_done(&self) -> bool {
        *self.state.lock().unwrap() == RunState::Done
    }

    pub fn progress(&self) -> (usize, usize, String) {
        let current = *self.current.lock().unwrap();
        let total = *self.total.lock().unwrap();
        let name = self.current_name.lock().unwrap().clone();
        (current, total, name)
    }

    pub fn output(&self) -> Vec<String> {
        self.output.lock().unwrap().clone()
    }

    pub fn all_success(&self) -> bool {
        *self.success.lock().unwrap()
    }

    pub fn total_completed(&self) -> usize {
        *self.current.lock().unwrap()
    }

    pub fn tick(&self) -> u64 {
        *self.tick.lock().unwrap()
    }

    pub fn elapsed_since_progress(&self) -> Duration {
        self.last_progress.lock().unwrap().elapsed()
    }
}
