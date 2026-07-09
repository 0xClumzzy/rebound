use std::io::{self, Write};
use std::process::Command;
use std::time::Instant;

use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph, Wrap},
    Frame, Terminal,
};

mod tasks;
mod runner;
mod theme;
mod wallpapers;

use tasks::{Category, Task};
use runner::Runner;

#[derive(PartialEq)]
enum AppState {
    Splash,
    SelectCategory,
    SelectTasks,
    Running,
    Done,
}

struct App {
    state: AppState,
    categories: Vec<Category>,
    cat_index: usize,
    task_index: usize,
    runner: Runner,
    start_time: Instant,
    splash_frame: u64,
    auto_mode: bool,
}

impl App {
    fn new(auto: bool) -> Self {
        Self {
            state: AppState::Splash,
            categories: tasks::all_categories(),
            cat_index: 0,
            task_index: 0,
            runner: Runner::new(),
            start_time: Instant::now(),
            splash_frame: 0,
            auto_mode: auto,
        }
    }

    fn current_tasks(&self) -> &[Task] {
        &self.categories[self.cat_index].tasks
    }

    fn all_selected_tasks(&self) -> Vec<(&Category, &Task)> {
        let mut out = Vec::new();
        for cat in &self.categories {
            for task in &cat.tasks {
                if task.selected {
                    out.push((cat, task));
                }
            }
        }
        out
    }

    fn select_all(&mut self) {
        for cat in &mut self.categories {
            for task in &mut cat.tasks {
                task.selected = true;
            }
        }
    }

    fn total_selected(&self) -> usize {
        self.categories.iter().flat_map(|c| &c.tasks).filter(|t| t.selected).count()
    }

    fn toggle_task(&mut self) {
        let tasks = &mut self.categories[self.cat_index].tasks;
        if let Some(task) = tasks.get_mut(self.task_index) {
            task.selected = !task.selected;
        }
    }

    fn toggle_all(&mut self) {
        let all_sel = self.categories[self.cat_index].tasks.iter().all(|t| t.selected);
        for task in &mut self.categories[self.cat_index].tasks {
            task.selected = !all_sel;
        }
    }

    fn has_selection(&self) -> bool {
        self.categories.iter().any(|c| c.tasks.iter().any(|t| t.selected))
    }

    fn home(&self) -> String {
        std::env::var("HOME").unwrap_or_else(|_| "/root".to_string())
    }

    fn start_auto(&mut self) {
        self.select_all();
        self.state = AppState::Running;
        self.runner.run_all(&self.all_selected_tasks(), &self.home());
    }
}

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let auto = args.contains(&"--auto".to_string()) || args.contains(&"-a".to_string());

    if args.contains(&"--help".to_string()) || args.contains(&"-h".to_string()) {
        println!("rebound - Clumzzy's Arch Linux Environment Setup");
        println!();
        println!("USAGE:");
        println!("  rebound           Interactive TUI");
        println!("  rebound --auto    Auto-install everything (no TUI)");
        println!();
        println!("OPTIONS:");
        println!("  -a, --auto    Select all tasks and run immediately");
        println!("  -h, --help    Show this help message");
        return Ok(());
    }

    // Prompt for sudo password before TUI starts
    prompt_sudo_password()?;

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    let mut app = App::new(auto);
    let result = run_app(&mut terminal, &mut app);

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    result
}

fn run_app<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    app: &mut App,
) -> anyhow::Result<()> {
    // Auto mode: skip splash, select all, run immediately
    if app.auto_mode {
        app.start_auto();
    }

    loop {
        terminal.draw(|f| ui(f, app))?;

        // Splash auto-advance after 1.5s (skip in auto mode)
        if app.state == AppState::Splash {
            if app.start_time.elapsed().as_millis() > 1500 {
                if app.auto_mode {
                    app.start_auto();
                } else {
                    app.state = AppState::SelectCategory;
                }
            }
        }

        let timeout = if app.state == AppState::Running || app.state == AppState::Splash {
            std::time::Duration::from_millis(50)
        } else {
            std::time::Duration::from_millis(100)
        };

        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if key.kind != KeyEventKind::Press {
                    continue;
                }
                match app.state {
                    AppState::Splash => {
                        app.state = AppState::SelectCategory;
                    }
                    AppState::SelectCategory => match key.code {
                        KeyCode::Char('q') | KeyCode::Esc => return Ok(()),
                        KeyCode::Up | KeyCode::Char('k') => {
                            app.cat_index = app.cat_index.saturating_sub(1);
                        }
                        KeyCode::Down | KeyCode::Char('j') => {
                            if app.cat_index < app.categories.len() - 1 {
                                app.cat_index += 1;
                            }
                        }
                        KeyCode::Enter => {
                            app.state = AppState::SelectTasks;
                            app.task_index = 0;
                        }
                        KeyCode::Char('a') => {
                            app.start_auto();
                        }
                        _ => {}
                    },
                    AppState::SelectTasks => match key.code {
                        KeyCode::Char('q') | KeyCode::Esc => {
                            app.state = AppState::SelectCategory;
                        }
                        KeyCode::Up | KeyCode::Char('k') => {
                            app.task_index = app.task_index.saturating_sub(1);
                        }
                        KeyCode::Down | KeyCode::Char('j') => {
                            let len = app.current_tasks().len();
                            if len > 0 && app.task_index < len - 1 {
                                app.task_index += 1;
                            }
                        }
                        KeyCode::Char(' ') => {
                            app.toggle_task();
                            if app.task_index < app.current_tasks().len() - 1 {
                                app.task_index += 1;
                            }
                        }
                        KeyCode::Char('a') => app.toggle_all(),
                        KeyCode::Enter => {
                            if app.has_selection() {
                                app.state = AppState::Running;
                                app.runner.run_all(&app.all_selected_tasks(), &app.home());
                            }
                        }
                        KeyCode::Backspace => {
                            for task in &mut app.categories[app.cat_index].tasks {
                                task.selected = false;
                            }
                        }
                        _ => {}
                    },
                    AppState::Running => match key.code {
                        KeyCode::Char('q') | KeyCode::Esc => return Ok(()),
                        _ => {}
                    },
                    AppState::Done => match key.code {
                        KeyCode::Char('q') | KeyCode::Esc | KeyCode::Enter => return Ok(()),
                        _ => {}
                    },
                }
            }
        }

        if app.state == AppState::Running && app.runner.is_done() {
            app.state = AppState::Done;
        }
    }
}

// ---- Rendering ----

fn ui(f: &mut Frame, app: &App) {
    let bg = Block::default().style(Style::default().bg(theme::color(theme::BASE)));
    f.render_widget(bg, f.area());

    match app.state {
        AppState::Splash => render_splash(f, app),
        AppState::SelectCategory => render_categories(f, app),
        AppState::SelectTasks => render_tasks(f, app),
        AppState::Running => render_running(f, app),
        AppState::Done => render_done(f, app),
    }
}

fn centered_area(f: &Frame, h: u16, w: u16) -> Rect {
    let area = f.area();
    let x = (area.width.saturating_sub(w)) / 2;
    let y = (area.height.saturating_sub(h)) / 2;
    Rect::new(x, y, w.min(area.width), h.min(area.height))
}

fn prompt_sudo_password() -> anyhow::Result<()> {
    use crossterm::terminal;

    // Check if sudo is already cached
    let check = Command::new("sudo").arg("-n").arg("true").output();
    if check.is_ok() && check.unwrap().status.success() {
        return Ok(());
    }

    // Prompt for password
    eprint!("[sudo] password for user: ");
    io::stderr().flush()?;

    // Read password with stars
    let mut password = String::new();
    terminal::enable_raw_mode()?;
    loop {
        if crossterm::event::poll(std::time::Duration::from_millis(100))? {
            if let crossterm::event::Event::Key(key) = crossterm::event::read()? {
                match key.code {
                    crossterm::event::KeyCode::Enter => {
                        eprintln!();
                        break;
                    }
                    crossterm::event::KeyCode::Backspace => {
                        if !password.is_empty() {
                            password.pop();
                            eprint!("\x08 \x08");
                            io::stderr().flush()?;
                        }
                    }
                    crossterm::event::KeyCode::Char(c) => {
                        password.push(c);
                        eprint!("*");
                        io::stderr().flush()?;
                    }
                    _ => {}
                }
            }
        }
    }
    terminal::disable_raw_mode()?;

    // Cache the password
    let mut child = Command::new("sudo")
        .arg("-S")
        .arg("true")
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .spawn()?;

    if let Some(mut stdin) = child.stdin.take() {
        writeln!(stdin, "{}", password)?;
    }
    child.wait()?;

    // Verify it worked
    let verify = Command::new("sudo").arg("-n").arg("true").output();
    if verify.is_ok() && verify.unwrap().status.success() {
        Ok(())
    } else {
        eprintln!("[!] Incorrect password. Some tasks may require sudo.");
        Ok(()) // Don't block, just warn
    }
}

fn spinner(frame: u64) -> &'static str {
    match frame % 4 {
        0 => "|",
        1 => "/",
        2 => "-",
        3 => "\\",
        _ => "|",
    }
}

fn border_style() -> Style {
    Style::default().fg(theme::color(theme::SURFACE2))
}

fn border_glow(hex: &str) -> Style {
    Style::default().fg(theme::color(hex))
}

fn render_splash(f: &mut Frame, app: &App) {
    let area = centered_area(f, 14, 62);

    let dim = theme::color(theme::SURFACE2);
    let fg = theme::color(theme::TEXT);
    let accent = theme::color(theme::MAUVE);

    let lines = vec![
        Line::from(""),
        Line::from(vec![
            Span::styled("  ", Style::default()),
            Span::styled("\u{2554}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2557}",
                Style::default().fg(dim)),
        ]),
        Line::from(vec![
            Span::styled("  ", Style::default()),
            Span::styled("\u{2551}", Style::default().fg(dim)),
            Span::styled(" ", Style::default()),
            Span::styled("0", Style::default().fg(fg).add_modifier(Modifier::BOLD)),
            Span::styled("x", Style::default().fg(dim)),
            Span::styled("\u{2588}\u{2588}", Style::default().fg(fg).add_modifier(Modifier::BOLD)),
            Span::styled("\u{2557}", Style::default().fg(dim)),
            Span::styled("  REBOUND  ", Style::default().fg(accent).add_modifier(Modifier::BOLD)),
            Span::styled("v1.0", Style::default().fg(dim)),
            Span::styled("                   \u{2551}", Style::default().fg(dim)),
        ]),
        Line::from(vec![
            Span::styled("  ", Style::default()),
            Span::styled("\u{2551}", Style::default().fg(dim)),
            Span::styled("   ", Style::default()),
            Span::styled("\u{255d}", Style::default().fg(dim)),
            Span::styled("\u{2554}\u{2550}\u{2550}\u{2550}\u{255d}", Style::default().fg(fg)),
            Span::styled("                      \u{2551}", Style::default().fg(dim)),
        ]),
        Line::from(vec![
            Span::styled("  ", Style::default()),
            Span::styled("\u{2551}", Style::default().fg(dim)),
            Span::styled("   Made by ", Style::default().fg(fg)),
            Span::styled("0xClumzZy", Style::default().fg(accent).add_modifier(Modifier::BOLD)),
            Span::styled("                         \u{2551}", Style::default().fg(dim)),
        ]),
        Line::from(vec![
            Span::styled("  ", Style::default()),
            Span::styled("\u{2551}", Style::default().fg(dim)),
            Span::styled("   ", Style::default()),
            Span::styled("@github.com/0xClumzzy", Style::default().fg(dim)),
            Span::styled("               \u{2551}", Style::default().fg(dim)),
        ]),
        Line::from(vec![
            Span::styled("  ", Style::default()),
            Span::styled("\u{255a}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2559}",
                Style::default().fg(dim)),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("           --- ", theme::style(theme::SURFACE2)),
            Span::styled("press any key", theme::dim(theme::OVERLAY0)),
            Span::styled(" ---", theme::style(theme::SURFACE2)),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("           --- ", theme::style(theme::SURFACE2)),
            Span::styled("or wait", theme::dim(theme::OVERLAY0)),
            Span::styled(" ---", theme::style(theme::SURFACE2)),
        ]),
    ];

    let widget = Paragraph::new(lines)
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::NONE)
                .style(Style::default().bg(theme::color(theme::CRUST)))
        );
    f.render_widget(widget, area);
}

fn render_categories(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(4),
            Constraint::Min(8),
            Constraint::Length(3),
        ])
        .split(f.area());

    let total = app.total_selected();

    let header_lines = vec![
        Line::from(vec![
            Span::styled("  rebound", theme::bold(theme::MAUVE)),
            Span::styled("  \u{2502}  ", theme::style(theme::SURFACE2)),
            Span::styled("Clumzzy's Arch Linux", theme::style(theme::SUBTEXT0)),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("  \u{2191}\u{2193}", theme::bold(theme::GREEN)),
            Span::styled(" navigate  ", theme::dim(theme::OVERLAY0)),
            Span::styled("\u{23ce}", theme::bold(theme::BLUE)),
            Span::styled(" select  ", theme::dim(theme::OVERLAY0)),
            Span::styled("a", theme::bold(theme::MAUVE)),
            Span::styled(" ", theme::dim(theme::OVERLAY0)),
            Span::styled("auto install", theme::bold(theme::MAUVE)),
            Span::styled("  ", theme::dim(theme::OVERLAY0)),
            Span::styled("q", theme::bold(theme::RED)),
            Span::styled(" quit", theme::dim(theme::OVERLAY0)),
        ]),
    ];

    let header = Paragraph::new(header_lines)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(ratatui::widgets::BorderType::Rounded)
                .border_style(border_glow(theme::MAUVE))
                .style(Style::default().bg(theme::color(theme::CRUST)))
        );
    f.render_widget(header, chunks[0]);

    // Category list
    let items: Vec<ListItem> = app
        .categories
        .iter()
        .enumerate()
        .map(|(i, cat)| {
            let sel = cat.tasks.iter().filter(|t| t.selected).count();
            let is_active = i == app.cat_index;

            let style = if is_active {
                theme::bold(theme::MAUVE)
            } else {
                theme::style(theme::TEXT)
            };

            let badge = if sel > 0 {
                Span::styled(
                    format!(" {} ", sel),
                    Style::default().fg(theme::color(theme::CRUST)).bg(theme::color(theme::GREEN)).add_modifier(Modifier::BOLD),
                )
            } else {
                Span::raw("")
            };

            let arrow = if is_active {
                Span::styled(" \u{25b6} ", theme::bold(theme::MAUVE))
            } else {
                Span::raw("   ")
            };

            ListItem::new(Line::from(vec![
                arrow,
                Span::styled(&cat.name, style),
                Span::styled(format!("  \u{2014} {} tasks", cat.tasks.len()), theme::dim(theme::OVERLAY0)),
                badge,
            ]))
        })
        .collect();

    let list = List::new(items)
        .block(
            Block::default()
                .title(Span::styled("  Categories  ", theme::bold(theme::MAUVE)))
                .borders(Borders::ALL)
                .border_type(ratatui::widgets::BorderType::Rounded)
                .border_style(border_style())
                .style(Style::default().bg(theme::color(theme::CRUST)))
        )
        .highlight_style(Style::default().bg(theme::color(theme::SURFACE0)));

    let mut state = ListState::default();
    state.select(Some(app.cat_index));
    f.render_stateful_widget(list, chunks[1], &mut state);

    // Footer
    let footer_text = if total > 0 {
        vec![
            Span::styled("  ", theme::style(theme::SURFACE2)),
            Span::styled(format!("{}", total), theme::bold(theme::GREEN)),
            Span::styled(" selected  ", theme::dim(theme::SUBTEXT0)),
            Span::styled("\u{23ce}", theme::bold(theme::BLUE)),
            Span::styled(" run", theme::dim(theme::OVERLAY0)),
        ]
    } else {
        vec![
            Span::styled("  ", theme::style(theme::SURFACE2)),
            Span::styled("press ", theme::dim(theme::OVERLAY0)),
            Span::styled("a", theme::bold(theme::MAUVE)),
            Span::styled(" auto install all  ", theme::dim(theme::OVERLAY0)),
            Span::styled("\u{23ce}", theme::bold(theme::BLUE)),
            Span::styled(" pick tasks", theme::dim(theme::OVERLAY0)),
        ]
    };

    let footer = Paragraph::new(Line::from(footer_text))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(ratatui::widgets::BorderType::Rounded)
                .border_style(border_style())
                .style(Style::default().bg(theme::color(theme::CRUST)))
        );
    f.render_widget(footer, chunks[2]);
}

fn render_tasks(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(4),
            Constraint::Min(8),
            Constraint::Length(3),
        ])
        .split(f.area());

    let cat = &app.categories[app.cat_index];
    let sel = cat.tasks.iter().filter(|t| t.selected).count();

    let header_lines = vec![
        Line::from(vec![
            Span::styled(format!("  {} ", cat.icon), Style::default()),
            Span::styled(&cat.name, theme::bold(theme::MAUVE)),
            Span::styled(format!("  \u{2502}  {}/{}", sel, cat.tasks.len()), theme::dim(theme::SUBTEXT0)),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("  \u{2191}\u{2193}", theme::bold(theme::GREEN)),
            Span::styled(" nav  ", theme::dim(theme::OVERLAY0)),
            Span::styled("\u{2423}", theme::bold(theme::BLUE)),
            Span::styled(" toggle  ", theme::dim(theme::OVERLAY0)),
            Span::styled("a", theme::bold(theme::YELLOW)),
            Span::styled(" all  ", theme::dim(theme::OVERLAY0)),
            Span::styled("\u{232b}", theme::bold(theme::RED)),
            Span::styled(" clear  ", theme::dim(theme::OVERLAY0)),
            Span::styled("esc", theme::bold(theme::PEACH)),
            Span::styled(" back", theme::dim(theme::OVERLAY0)),
        ]),
    ];

    let header = Paragraph::new(header_lines)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(ratatui::widgets::BorderType::Rounded)
                .border_style(border_glow(theme::MAUVE))
                .style(Style::default().bg(theme::color(theme::CRUST)))
        );
    f.render_widget(header, chunks[0]);

    let items: Vec<ListItem> = cat
        .tasks
        .iter()
        .enumerate()
        .map(|(i, task)| {
            let is_active = i == app.task_index;

            let (check, check_style) = if task.selected {
                ("\u{2611}", Style::default().fg(theme::color(theme::GREEN)).add_modifier(Modifier::BOLD))
            } else {
                ("\u{2610}", Style::default().fg(theme::color(theme::SURFACE1)))
            };

            let name_style = if is_active && task.selected {
                theme::bold(theme::GREEN)
            } else if is_active {
                theme::bold(theme::TEXT)
            } else if task.selected {
                theme::style(theme::GREEN)
            } else {
                theme::style(theme::TEXT)
            };

            let arrow = if is_active {
                Span::styled(" \u{25b6} ", theme::bold(theme::MAUVE))
            } else {
                Span::raw("   ")
            };

            ListItem::new(Line::from(vec![
                arrow,
                Span::styled(format!("{} ", check), check_style),
                Span::styled(&task.name, name_style),
                Span::styled(format!("  \u{2014} {}", task.desc), theme::dim(theme::OVERLAY0)),
            ]))
        })
        .collect();

    let list = List::new(items)
        .block(
            Block::default()
                .title(Span::styled("  Tasks  ", theme::bold(theme::MAUVE)))
                .borders(Borders::ALL)
                .border_type(ratatui::widgets::BorderType::Rounded)
                .border_style(border_style())
                .style(Style::default().bg(theme::color(theme::CRUST)))
        )
        .highlight_style(Style::default().bg(theme::color(theme::SURFACE0)));

    let mut state = ListState::default();
    state.select(Some(app.task_index));
    f.render_stateful_widget(list, chunks[1], &mut state);

    let total = app.total_selected();
    let footer_text = if total > 0 {
        vec![
            Span::styled("  ", theme::style(theme::SURFACE2)),
            Span::styled(format!("{}", total), theme::bold(theme::GREEN)),
            Span::styled(" queued  ", theme::dim(theme::SUBTEXT0)),
            Span::styled("\u{23ce}", theme::bold(theme::BLUE)),
            Span::styled(" run  ", theme::dim(theme::OVERLAY0)),
            Span::styled("\u{232b}", theme::bold(theme::RED)),
            Span::styled(" clear", theme::dim(theme::OVERLAY0)),
        ]
    } else {
        vec![
            Span::styled("  ", theme::style(theme::SURFACE2)),
            Span::styled("press ", theme::dim(theme::OVERLAY0)),
            Span::styled("\u{2423}", theme::bold(theme::BLUE)),
            Span::styled(" to select", theme::dim(theme::OVERLAY0)),
        ]
    };

    let footer = Paragraph::new(Line::from(footer_text))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(ratatui::widgets::BorderType::Rounded)
                .border_style(border_style())
                .style(Style::default().bg(theme::color(theme::CRUST)))
        );
    f.render_widget(footer, chunks[2]);
}

fn render_running(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(5),
            Constraint::Min(8),
            Constraint::Length(2),
        ])
        .split(f.area());

    let (current, total, current_name) = app.runner.progress();
    let pct = if total > 0 { current * 100 / total } else { 0 };
    let tick = app.runner.tick();
    let elapsed = app.runner.elapsed_since_progress();

    let bar_w = 40;
    let filled = (pct as usize * bar_w) / 100;

    // Animate the leading edge of the bar in sync with progress
    let pulse_ms = elapsed.as_millis() as u64;
    let bar_chars = [".", "o", "O", "o"];
    let anim_idx = if pulse_ms < 500 {
        ((pulse_ms / 100) as usize) % bar_chars.len()
    } else {
        ((tick / 4) as usize) % bar_chars.len()
    };

    let mut bar = String::new();
    for _ in 0..filled {
        bar.push('#');
    }
    if filled < bar_w {
        bar.push_str(bar_chars[anim_idx]);
    }
    for _ in (filled + 1)..bar_w {
        bar.push('-');
    }

    let spin = spinner(tick);

    let header_lines = vec![
        Line::from(vec![
            Span::styled(format!("  {} ", spin), theme::bold(theme::MAUVE)),
            Span::styled("Running", theme::bold(theme::MAUVE)),
            Span::styled(format!("  \u{2502}  {}/{}", current, total), theme::dim(theme::SUBTEXT0)),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("  ", Style::default()),
            Span::styled(&bar, theme::bold(theme::GREEN)),
            Span::styled(format!("  {}%", pct), theme::bold(theme::YELLOW)),
            Span::styled("  \u{2502}  ", theme::style(theme::SURFACE2)),
            Span::styled(&current_name, theme::style(theme::TEXT)),
        ]),
    ];

    let header = Paragraph::new(header_lines)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(ratatui::widgets::BorderType::Rounded)
                .border_style(border_glow(theme::MAUVE))
                .style(Style::default().bg(theme::color(theme::CRUST)))
        );
    f.render_widget(header, chunks[0]);

    // Output log
    let output = app.runner.output();
    let visible_h = chunks[1].height as usize;
    let scroll = if output.len() > visible_h {
        output.len() - visible_h
    } else {
        0
    };

    let lines: Vec<Line> = output
        .iter()
        .skip(scroll)
        .map(|line| {
            if line.contains("[ok]") || line.contains("complete") {
                Line::from(Span::styled(line.as_str(), theme::style(theme::GREEN)))
            } else if line.contains("SUDO") {
                Line::from(Span::styled(line.as_str(), theme::bold(theme::YELLOW)))
            } else if line.starts_with("  \u{2502}") {
                Line::from(Span::styled(line.as_str(), theme::dim(theme::OVERLAY1)))
            } else if line.contains("\u{2500}\u{2500}") {
                Line::from(Span::styled(line.as_str(), theme::style(theme::SURFACE2)))
            } else if line.contains("rebound") {
                Line::from(Span::styled(line.as_str(), theme::bold(theme::MAUVE)))
            } else if line.contains("tasks") {
                Line::from(Span::styled(line.as_str(), theme::style(theme::BLUE)))
            } else if line.contains("[ERR]") || line.contains("Error") || line.contains("[!!]") {
                Line::from(Span::styled(line.as_str(), theme::style(theme::RED)))
            } else if line.contains("All tasks completed") {
                Line::from(Span::styled(line.as_str(), theme::bold(theme::GREEN)))
            } else {
                Line::from(Span::styled(line.as_str(), theme::dim(theme::SUBTEXT0)))
            }
        })
        .collect();

    let output_widget = Paragraph::new(lines)
        .wrap(Wrap { trim: false })
        .block(
            Block::default()
                .title(Span::styled("  Output  ", theme::bold(theme::BLUE)))
                .borders(Borders::ALL)
                .border_type(ratatui::widgets::BorderType::Rounded)
                .border_style(border_glow(theme::BLUE))
                .style(Style::default().bg(theme::color(theme::CRUST)))
        );
    f.render_widget(output_widget, chunks[1]);

    let footer = Paragraph::new(Line::from(vec![
        Span::styled("  ", theme::style(theme::SURFACE2)),
        Span::styled("q", theme::bold(theme::RED)),
        Span::styled(" quit", theme::dim(theme::OVERLAY0)),
    ]))
    .block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(ratatui::widgets::BorderType::Rounded)
            .border_style(border_style())
            .style(Style::default().bg(theme::color(theme::CRUST)))
    );
    f.render_widget(footer, chunks[2]);
}

fn render_done(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(5),
            Constraint::Min(8),
            Constraint::Length(2),
        ])
        .split(f.area());

    let success = app.runner.all_success();
    let completed = app.runner.total_completed();

    let (title, icon, accent) = if success {
        ("Setup Complete!", "[ok]", theme::GREEN)
    } else {
        ("Finished with Errors", "[!!]", theme::YELLOW)
    };

    let header_lines = vec![
        Line::from(vec![
            Span::styled(format!("  {} ", icon), theme::bold(accent)),
            Span::styled(title, theme::bold(accent)),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("  ", theme::style(theme::SURFACE2)),
            Span::styled(format!("{}", completed), theme::bold(theme::GREEN)),
            Span::styled(" task(s) completed", theme::dim(theme::SUBTEXT0)),
        ]),
    ];

    let header = Paragraph::new(header_lines)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(ratatui::widgets::BorderType::Rounded)
                .border_style(border_glow(accent))
                .style(Style::default().bg(theme::color(theme::CRUST)))
        );
    f.render_widget(header, chunks[0]);

    let output = app.runner.output();
    let lines: Vec<Line> = output
        .iter()
        .map(|line| {
            if line.contains("[ok]") || line.contains("complete") {
                Line::from(Span::styled(line.as_str(), theme::style(theme::GREEN)))
            } else if line.contains("[ERR]") || line.contains("Error") || line.contains("[!!]") {
                Line::from(Span::styled(line.as_str(), theme::style(theme::RED)))
            } else if line.contains("All tasks completed") {
                Line::from(Span::styled(line.as_str(), theme::bold(theme::GREEN)))
            } else {
                Line::from(Span::styled(line.as_str(), theme::dim(theme::SUBTEXT0)))
            }
        })
        .collect();

    let output_widget = Paragraph::new(lines)
        .wrap(Wrap { trim: false })
        .block(
            Block::default()
                .title(Span::styled("  Summary  ", theme::bold(theme::GREEN)))
                .borders(Borders::ALL)
                .border_type(ratatui::widgets::BorderType::Rounded)
                .border_style(border_glow(theme::GREEN))
                .style(Style::default().bg(theme::color(theme::CRUST)))
        );
    f.render_widget(output_widget, chunks[1]);

    let footer = Paragraph::new(Line::from(vec![
        Span::styled("  ", theme::style(theme::SURFACE2)),
        Span::styled("press ", theme::dim(theme::OVERLAY0)),
        Span::styled("enter", theme::bold(theme::BLUE)),
        Span::styled(" or ", theme::dim(theme::OVERLAY0)),
        Span::styled("q", theme::bold(theme::RED)),
        Span::styled(" to exit", theme::dim(theme::OVERLAY0)),
    ]))
    .block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(ratatui::widgets::BorderType::Rounded)
            .border_style(border_glow(theme::GREEN))
            .style(Style::default().bg(theme::color(theme::CRUST)))
    );
    f.render_widget(footer, chunks[2]);
}
