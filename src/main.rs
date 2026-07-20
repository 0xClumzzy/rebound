use std::io::{self, Write};
use std::process::Command;

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
use ratatui::prelude::Stylize;

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
        println!("rebound - 0xClumzZy's Arch Linux Environment Setup");
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
) -> anyhow::Result<()>
where
    <B as ratatui::backend::Backend>::Error: Send + Sync + 'static,
{
    if app.auto_mode {
        app.start_auto();
    }

    loop {
        terminal.draw(|f| ui(f, app))?;

        let timeout = if app.state == AppState::Running {
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

    let check = Command::new("sudo").arg("-n").arg("true").output();
    if check.is_ok() && check.unwrap().status.success() {
        return Ok(());
    }

    eprint!("[sudo] password for user: ");
    io::stderr().flush()?;

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

    let verify = Command::new("sudo").arg("-n").arg("true").output();
    if verify.is_ok() && verify.unwrap().status.success() {
        Ok(())
    } else {
        eprintln!("[!] Incorrect password. Some tasks may require sudo.");
        Ok(())
    }
}

fn spinner(frame: u64) -> &'static str {
    match frame % 4 {
        0 => "\u{2571}",
        1 => "\u{2572}",
        2 => "\u{2571}",
        3 => "\u{2572}",
        _ => "\u{2571}",
    }
}

fn border_dim() -> Style {
    Style::default().fg(theme::color(theme::SURFACE2))
}

fn border_accent(hex: &str) -> Style {
    Style::default().fg(theme::color(hex))
}

// ---- Splash Screen ----

fn render_splash(f: &mut Frame, _app: &App) {
    let area = centered_area(f, 16, 66);

    let w = area.width as usize;
    let inner_w = w.saturating_sub(4);

    let mut lines = Vec::new();
    lines.push(Line::from(""));

    // Gradient top border
    let grad_top: Vec<Span> = (0..inner_w)
        .map(|i| {
            let t = i as f32 / inner_w as f32;
            Span::styled("\u{2500}", Style::default().fg(theme::blend(theme::MAUVE, theme::BLUE, t)))
        })
        .collect();
    let mut top_line: Vec<Span> = vec![Span::styled("  \u{256d}", Style::default().fg(theme::color(theme::MAUVE)))];
    top_line.extend(grad_top);
    top_line.push(Span::styled("\u{256e}", Style::default().fg(theme::color(theme::BLUE))));
    lines.push(Line::from(top_line));

    // Empty padding
    for _ in 0..2 {
        lines.push(Line::from(vec![
            Span::styled("  \u{2502}", Style::default().fg(theme::color(theme::SURFACE2))),
            Span::raw(" ".repeat(inner_w)),
            Span::styled("\u{2502}", Style::default().fg(theme::color(theme::SURFACE2))),
        ]));
    }

    // Logo line 1: ╔══╗
    let logo_top_inner = "\u{2554}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2557}";
    let padding_before_logo = (inner_w.saturating_sub(24)) / 2;
    let padding_after_logo = inner_w.saturating_sub(24).saturating_sub(padding_before_logo);
    lines.push(Line::from(vec![
        Span::styled("  \u{2502}", Style::default().fg(theme::color(theme::SURFACE2))),
        Span::raw(" ".repeat(padding_before_logo)),
        Span::styled(logo_top_inner, Style::default().fg(theme::color(theme::TEXT))),
        Span::raw(" ".repeat(padding_after_logo)),
        Span::styled("\u{2502}", Style::default().fg(theme::color(theme::SURFACE2))),
    ]));

    // Logo line 2: 0x██ REBOUND v1.0
    let _title = "0x\u{2588}\u{2588}";
    let label = " REBOUND ";
    let ver = "v1.0";
    let logo_content_len = 8 + 10 + 4;
    let pad_before = (inner_w.saturating_sub(logo_content_len)) / 2;
    let pad_after = inner_w.saturating_sub(logo_content_len).saturating_sub(pad_before);
    lines.push(Line::from(vec![
        Span::styled("  \u{2502}", Style::default().fg(theme::color(theme::SURFACE2))),
        Span::raw(" ".repeat(pad_before)),
        Span::styled("0", Style::default().fg(theme::color(theme::TEXT)).add_modifier(Modifier::BOLD)),
        Span::styled("x", Style::default().fg(theme::color(theme::OVERLAY0))),
        Span::styled("\u{2588}\u{2588}", Style::default().fg(theme::color(theme::TEXT)).add_modifier(Modifier::BOLD)),
        Span::styled(label, Style::default().fg(theme::color(theme::MAUVE)).add_modifier(Modifier::BOLD)),
        Span::styled(ver, Style::default().fg(theme::color(theme::OVERLAY1))),
        Span::raw(" ".repeat(pad_after)),
        Span::styled("\u{2502}", Style::default().fg(theme::color(theme::SURFACE2))),
    ]));

    // Logo line 3: ╚══╝
    let logo_bot_inner = "\u{255a}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{255d}";
    lines.push(Line::from(vec![
        Span::styled("  \u{2502}", Style::default().fg(theme::color(theme::SURFACE2))),
        Span::raw(" ".repeat(padding_before_logo)),
        Span::styled(logo_bot_inner, Style::default().fg(theme::color(theme::TEXT))),
        Span::raw(" ".repeat(padding_after_logo)),
        Span::styled("\u{2502}", Style::default().fg(theme::color(theme::SURFACE2))),
    ]));

    // Empty padding
    for _ in 0..2 {
        lines.push(Line::from(vec![
            Span::styled("  \u{2502}", Style::default().fg(theme::color(theme::SURFACE2))),
            Span::raw(" ".repeat(inner_w)),
            Span::styled("\u{2502}", Style::default().fg(theme::color(theme::SURFACE2))),
        ]));
    }

    // Author line
    let author_text = "Made by 0xClumzZy";
    let author_pad = (inner_w.saturating_sub(author_text.len())) / 2;
    let author_pad_r = inner_w.saturating_sub(author_text.len()).saturating_sub(author_pad);
    lines.push(Line::from(vec![
        Span::styled("  \u{2502}", Style::default().fg(theme::color(theme::SURFACE2))),
        Span::raw(" ".repeat(author_pad)),
        Span::styled("Made by ", Style::default().fg(theme::color(theme::SUBTEXT0))),
        Span::styled("0xClumzZy", Style::default().fg(theme::color(theme::MAUVE)).add_modifier(Modifier::BOLD)),
        Span::raw(" ".repeat(author_pad_r)),
        Span::styled("\u{2502}", Style::default().fg(theme::color(theme::SURFACE2))),
    ]));

    // GitHub line
    let gh_text = "@github.com/0xClumzzy";
    let gh_pad = (inner_w.saturating_sub(gh_text.len())) / 2;
    let gh_pad_r = inner_w.saturating_sub(gh_text.len()).saturating_sub(gh_pad);
    lines.push(Line::from(vec![
        Span::styled("  \u{2502}", Style::default().fg(theme::color(theme::SURFACE2))),
        Span::raw(" ".repeat(gh_pad)),
        Span::styled(gh_text, Style::default().fg(theme::color(theme::OVERLAY1))),
        Span::raw(" ".repeat(gh_pad_r)),
        Span::styled("\u{2502}", Style::default().fg(theme::color(theme::SURFACE2))),
    ]));

    // Empty padding
    for _ in 0..2 {
        lines.push(Line::from(vec![
            Span::styled("  \u{2502}", Style::default().fg(theme::color(theme::SURFACE2))),
            Span::raw(" ".repeat(inner_w)),
            Span::styled("\u{2502}", Style::default().fg(theme::color(theme::SURFACE2))),
        ]));
    }

    // Gradient bottom border
    let grad_bot: Vec<Span> = (0..inner_w)
        .map(|i| {
            let t = i as f32 / inner_w as f32;
            Span::styled("\u{2500}", Style::default().fg(theme::blend(theme::BLUE, theme::MAUVE, t)))
        })
        .collect();
    let mut bot_line: Vec<Span> = vec![Span::styled("  \u{2570}", Style::default().fg(theme::color(theme::BLUE)))];
    bot_line.extend(grad_bot);
    bot_line.push(Span::styled("\u{256f}", Style::default().fg(theme::color(theme::MAUVE))));
    lines.push(Line::from(bot_line));

    // Prompt
    lines.push(Line::from(""));
    lines.push(Line::from(vec![
        Span::styled("  press any key", Style::default().fg(theme::color(theme::OVERLAY0))).add_modifier(Modifier::DIM),
        Span::styled("  or wait", Style::default().fg(theme::color(theme::SURFACE2))).add_modifier(Modifier::DIM),
    ]));

    let widget = Paragraph::new(lines)
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::NONE)
                .style(Style::default().bg(theme::color(theme::CRUST)))
        );
    f.render_widget(widget, area);
}

// ---- Categories Screen ----

fn render_categories(f: &mut Frame, app: &App) {
    let area = f.area();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(5),
            Constraint::Min(8),
            Constraint::Length(3),
        ])
        .split(area);

    let total = app.total_selected();
    let total_tasks: usize = app.categories.iter().map(|c| c.tasks.len()).sum();

    // Header with title and keybinds
    let header_lines = vec![
        Line::from(vec![
            Span::styled("  rebound", theme::bold(theme::MAUVE)),
            Span::styled("  \u{2502}  ", theme::style(theme::SURFACE2)),
            Span::styled("Clumzzy's Arch Linux Environment", theme::style(theme::SUBTEXT0)),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("  ", Style::default()),
            Span::styled("\u{2191}\u{2193}", theme::bold(theme::GREEN)),
            Span::styled(" navigate ", theme::dim(theme::OVERLAY0)),
            Span::styled("  ", Style::default()),
            Span::styled("\u{23ce}", theme::bold(theme::BLUE)),
            Span::styled(" open ", theme::dim(theme::OVERLAY0)),
            Span::styled("  ", Style::default()),
            Span::styled("a", theme::bold(theme::MAUVE)),
            Span::styled(" auto ", theme::dim(theme::OVERLAY0)),
            Span::styled("  ", Style::default()),
            Span::styled("q", theme::bold(theme::RED)),
            Span::styled(" quit", theme::dim(theme::OVERLAY0)),
        ]),
    ];

    let header = Paragraph::new(header_lines)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(ratatui::widgets::BorderType::Rounded)
                .border_style(border_accent(theme::MAUVE))
                .style(Style::default().bg(theme::color(theme::CRUST)))
        );
    f.render_widget(header, chunks[0]);

    // Category list with icons
    let items: Vec<ListItem> = app
        .categories
        .iter()
        .enumerate()
        .map(|(i, cat)| {
            let sel = cat.tasks.iter().filter(|t| t.selected).count();
            let is_active = i == app.cat_index;
            let task_count = cat.tasks.len();

            let name_style = if is_active {
                theme::bold(theme::MAUVE)
            } else {
                theme::style(theme::TEXT)
            };

            let badge = if sel > 0 {
                Span::styled(
                    format!(" {}/{} ", sel, task_count),
                    theme::bold_bg(theme::CRUST, theme::GREEN),
                )
            } else {
                Span::styled(
                    format!(" {} ", task_count),
                    theme::style(theme::OVERLAY0),
                )
            };

            let indicator = if is_active {
                Span::styled(" \u{25b8} ", theme::bold(theme::MAUVE))
            } else {
                Span::styled("   ", Style::default())
            };

            let icon_span = Span::styled(format!("{} ", cat.icon), Style::default());

            ListItem::new(Line::from(vec![
                indicator,
                icon_span,
                Span::styled(&cat.name, name_style),
                Span::styled("  \u{2500}  ", theme::dim(theme::SURFACE2)),
                badge,
            ]))
        })
        .collect();

    let list = List::new(items)
        .block(
            Block::default()
                .title(Span::styled(
                    format!("  Categories  \u{2502}  {} total ", total_tasks),
                    theme::dim(theme::OVERLAY0),
                ))
                .borders(Borders::ALL)
                .border_type(ratatui::widgets::BorderType::Rounded)
                .border_style(border_dim())
                .style(Style::default().bg(theme::color(theme::CRUST)))
        )
        .highlight_style(
            Style::default()
                .bg(theme::color(theme::SURFACE0))
                .add_modifier(Modifier::BOLD)
        );

    let mut state = ListState::default();
    state.select(Some(app.cat_index));
    f.render_stateful_widget(list, chunks[1], &mut state);

    // Footer
    let footer_text = if total > 0 {
        vec![
            Span::styled("  ", Style::default()),
            Span::styled(format!("{}", total), theme::bold(theme::GREEN)),
            Span::styled(" selected  ", theme::dim(theme::SUBTEXT0)),
            Span::styled("  ", Style::default()),
            Span::styled("\u{23ce}", theme::bold(theme::BLUE)),
            Span::styled(" run", theme::dim(theme::OVERLAY0)),
        ]
    } else {
        vec![
            Span::styled("  ", Style::default()),
            Span::styled("press ", theme::dim(theme::OVERLAY0)),
            Span::styled("a", theme::bold(theme::MAUVE)),
            Span::styled(" auto install  ", theme::dim(theme::OVERLAY0)),
            Span::styled("  ", Style::default()),
            Span::styled("\u{23ce}", theme::bold(theme::BLUE)),
            Span::styled(" pick tasks", theme::dim(theme::OVERLAY0)),
        ]
    };

    let footer = Paragraph::new(Line::from(footer_text))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(ratatui::widgets::BorderType::Rounded)
                .border_style(border_dim())
                .style(Style::default().bg(theme::color(theme::CRUST)))
        );
    f.render_widget(footer, chunks[2]);
}

// ---- Tasks Screen ----

fn render_tasks(f: &mut Frame, app: &App) {
    let area = f.area();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(5),
            Constraint::Min(8),
            Constraint::Length(3),
        ])
        .split(area);

    let cat = &app.categories[app.cat_index];
    let sel = cat.tasks.iter().filter(|t| t.selected).count();

    let header_lines = vec![
        Line::from(vec![
            Span::styled(format!("  {} ", cat.icon), Style::default()),
            Span::styled(&cat.name, theme::bold(theme::MAUVE)),
            Span::styled(format!("  \u{2502}  {}/{} selected", sel, cat.tasks.len()), theme::dim(theme::SUBTEXT0)),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("  ", Style::default()),
            Span::styled("\u{2191}\u{2193}", theme::bold(theme::GREEN)),
            Span::styled(" nav ", theme::dim(theme::OVERLAY0)),
            Span::styled("  ", Style::default()),
            Span::styled("\u{2423}", theme::bold(theme::BLUE)),
            Span::styled(" toggle ", theme::dim(theme::OVERLAY0)),
            Span::styled("  ", Style::default()),
            Span::styled("a", theme::bold(theme::YELLOW)),
            Span::styled(" all ", theme::dim(theme::OVERLAY0)),
            Span::styled("  ", Style::default()),
            Span::styled("\u{232b}", theme::bold(theme::RED)),
            Span::styled(" clear ", theme::dim(theme::OVERLAY0)),
            Span::styled("  ", Style::default()),
            Span::styled("esc", theme::bold(theme::PEACH)),
            Span::styled(" back", theme::dim(theme::OVERLAY0)),
        ]),
    ];

    let header = Paragraph::new(header_lines)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(ratatui::widgets::BorderType::Rounded)
                .border_style(border_accent(theme::MAUVE))
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

            let indicator = if is_active {
                Span::styled(" \u{25b8} ", theme::bold(theme::MAUVE))
            } else {
                Span::styled("   ", Style::default())
            };

            ListItem::new(Line::from(vec![
                indicator,
                Span::styled(format!("{} ", check), check_style),
                Span::styled(&task.name, name_style),
                Span::styled("  \u{2500}  ", theme::dim(theme::SURFACE2)),
                Span::styled(&task.desc, theme::dim(theme::OVERLAY0)),
            ]))
        })
        .collect();

    let list = List::new(items)
        .block(
            Block::default()
                .title(Span::styled("  Tasks  ", theme::bold(theme::MAUVE)))
                .borders(Borders::ALL)
                .border_type(ratatui::widgets::BorderType::Rounded)
                .border_style(border_dim())
                .style(Style::default().bg(theme::color(theme::CRUST)))
        )
        .highlight_style(
            Style::default()
                .bg(theme::color(theme::SURFACE0))
                .add_modifier(Modifier::BOLD)
        );

    let mut state = ListState::default();
    state.select(Some(app.task_index));
    f.render_stateful_widget(list, chunks[1], &mut state);

    let total = app.total_selected();
    let footer_text = if total > 0 {
        vec![
            Span::styled("  ", Style::default()),
            Span::styled(format!("{}", total), theme::bold(theme::GREEN)),
            Span::styled(" queued  ", theme::dim(theme::SUBTEXT0)),
            Span::styled("  ", Style::default()),
            Span::styled("\u{23ce}", theme::bold(theme::BLUE)),
            Span::styled(" run  ", theme::dim(theme::OVERLAY0)),
            Span::styled("  ", Style::default()),
            Span::styled("\u{232b}", theme::bold(theme::RED)),
            Span::styled(" clear", theme::dim(theme::OVERLAY0)),
        ]
    } else {
        vec![
            Span::styled("  ", Style::default()),
            Span::styled("press ", theme::dim(theme::OVERLAY0)),
            Span::styled("\u{2423}", theme::bold(theme::BLUE)),
            Span::styled(" to select a task", theme::dim(theme::OVERLAY0)),
        ]
    };

    let footer = Paragraph::new(Line::from(footer_text))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(ratatui::widgets::BorderType::Rounded)
                .border_style(border_dim())
                .style(Style::default().bg(theme::color(theme::CRUST)))
        );
    f.render_widget(footer, chunks[2]);
}

// ---- Running Screen ----

fn render_running(f: &mut Frame, app: &App) {
    let area = f.area();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(6),
            Constraint::Min(8),
            Constraint::Length(2),
        ])
        .split(area);

    let (current, total, current_name) = app.runner.progress();
    let pct = if total > 0 { current * 100 / total } else { 0 };
    let tick = app.runner.tick();
    let elapsed = app.runner.elapsed_since_progress();

    // Gradient progress bar
    let bar_w = 50;
    let filled = (pct as usize * bar_w) / 100;

    let pulse_ms = elapsed.as_millis() as u64;
    let bar_chars = [".", "o", "O", "o"];
    let anim_idx = if pulse_ms < 500 {
        ((pulse_ms / 100) as usize) % bar_chars.len()
    } else {
        ((tick / 4) as usize) % bar_chars.len()
    };

    let mut bar_spans: Vec<Span> = Vec::new();
    for i in 0..bar_w {
        if i < filled {
            let t = i as f32 / bar_w as f32;
            bar_spans.push(Span::styled(
                "\u{2588}",
                Style::default().fg(theme::blend(theme::GREEN, theme::TEAL, t)),
            ));
        } else if i == filled && filled < bar_w {
            let t = filled as f32 / bar_w as f32;
            bar_spans.push(Span::styled(
                bar_chars[anim_idx],
                Style::default().fg(theme::blend(theme::GREEN, theme::TEAL, t)),
            ));
        } else {
            bar_spans.push(Span::styled(
                "\u{2592}",
                Style::default().fg(theme::color(theme::SURFACE1)),
            ));
        }
    }

    let spin = spinner(tick);

    let header_lines = vec![
        Line::from(vec![
            Span::styled(format!("  {} ", spin), theme::bold(theme::MAUVE)),
            Span::styled("Running", theme::bold(theme::MAUVE)),
            Span::styled(format!("  \u{2502}  {}/{} tasks", current, total), theme::dim(theme::SUBTEXT0)),
        ]),
        Line::from(""),
        Line::from({
            let mut v: Vec<Span> = vec![
                Span::styled("  ", Style::default()),
            ];
            v.extend(bar_spans);
            v.push(Span::styled(format!("  {}%", pct), theme::bold(theme::YELLOW)));
            v.push(Span::styled("  \u{2502}  ", theme::style(theme::SURFACE2)));
            v.push(Span::styled(&current_name, theme::style(theme::TEXT)));
            v
        }),
    ];

    let header = Paragraph::new(header_lines)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(ratatui::widgets::BorderType::Rounded)
                .border_style(border_accent(theme::MAUVE))
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

    let log_lines: Vec<Line> = output
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

    let output_widget = Paragraph::new(log_lines)
        .wrap(Wrap { trim: false })
        .block(
            Block::default()
                .title(Span::styled("  Output  ", theme::bold(theme::BLUE)))
                .borders(Borders::ALL)
                .border_type(ratatui::widgets::BorderType::Rounded)
                .border_style(border_accent(theme::BLUE))
                .style(Style::default().bg(theme::color(theme::CRUST)))
        );
    f.render_widget(output_widget, chunks[1]);

    let footer = Paragraph::new(Line::from(vec![
        Span::styled("  ", Style::default()),
        Span::styled("q", theme::bold(theme::RED)),
        Span::styled(" quit", theme::dim(theme::OVERLAY0)),
    ]))
    .block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(ratatui::widgets::BorderType::Rounded)
            .border_style(border_dim())
            .style(Style::default().bg(theme::color(theme::CRUST)))
    );
    f.render_widget(footer, chunks[2]);
}

// ---- Done Screen ----

fn render_done(f: &mut Frame, app: &App) {
    let area = f.area();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(5),
            Constraint::Min(8),
            Constraint::Length(2),
        ])
        .split(area);

    let success = app.runner.all_success();
    let completed = app.runner.total_completed();

    let (title, accent) = if success {
        ("\u{2714}  Setup Complete!", theme::GREEN)
    } else {
        ("\u{2716}  Finished with Errors", theme::YELLOW)
    };

    let header_lines = vec![
        Line::from(vec![
            Span::styled(format!("  {} ", title), theme::bold(accent)),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("  ", Style::default()),
            Span::styled(format!("{}", completed), theme::bold(theme::GREEN)),
            Span::styled(" task(s) completed successfully", theme::dim(theme::SUBTEXT0)),
        ]),
    ];

    let header = Paragraph::new(header_lines)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(ratatui::widgets::BorderType::Rounded)
                .border_style(border_accent(accent))
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
                .border_style(border_accent(theme::GREEN))
                .style(Style::default().bg(theme::color(theme::CRUST)))
        );
    f.render_widget(output_widget, chunks[1]);

    let footer = Paragraph::new(Line::from(vec![
        Span::styled("  ", Style::default()),
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
            .border_style(border_accent(theme::GREEN))
            .style(Style::default().bg(theme::color(theme::CRUST)))
    );
    f.render_widget(footer, chunks[2]);
}
