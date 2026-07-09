// Deep dark palette - extra blacks with high contrast accents
pub const BASE: &str = "#0a0a0f";
pub const MANTLE: &str = "#08080d";
pub const CRUST: &str = "#050508";
pub const SURFACE0: &str = "#16161e";
pub const SURFACE1: &str = "#1e1e2a";
pub const SURFACE2: &str = "#282838";
pub const OVERLAY0: &str = "#44445a";
pub const OVERLAY1: &str = "#5a5a74";
pub const TEXT: &str = "#c8c8d8";
pub const SUBTEXT0: &str = "#8888a0";
pub const SUBTEXT1: &str = "#a0a0b8";
pub const LAVENDER: &str = "#b4befe";
pub const BLUE: &str = "#7ba4f7";
pub const SAPPHIRE: &str = "#6ac0e8";
pub const SKY: &str = "#7ad8e8";
pub const TEAL: &str = "#80e0c8";
pub const GREEN: &str = "#90e890";
pub const YELLOW: &str = "#e8d890";
pub const PEACH: &str = "#e8a870";
pub const MAROON: &str = "#e09090";
pub const RED: &str = "#f07090";
pub const MAUVE: &str = "#c0a0f0";
pub const PINK: &str = "#e8b0d8";
pub const FLAMINGO: &str = "#e0c8c8";
pub const ROSEWATER: &str = "#e8d0c8";

pub fn hex_to_rgb(hex: &str) -> (u8, u8, u8) {
    let hex = hex.trim_start_matches('#');
    let r = u8::from_str_radix(&hex[0..2], 16).unwrap_or(0);
    let g = u8::from_str_radix(&hex[2..4], 16).unwrap_or(0);
    let b = u8::from_str_radix(&hex[4..6], 16).unwrap_or(0);
    (r, g, b)
}

pub fn color(hex: &str) -> ratatui::style::Color {
    let (r, g, b) = hex_to_rgb(hex);
    ratatui::style::Color::Rgb(r, g, b)
}

pub fn style(hex: &str) -> ratatui::style::Style {
    ratatui::style::Style::default().fg(color(hex))
}

pub fn bold(hex: &str) -> ratatui::style::Style {
    ratatui::style::Style::default()
        .fg(color(hex))
        .add_modifier(ratatui::style::Modifier::BOLD)
}

pub fn dim(hex: &str) -> ratatui::style::Style {
    ratatui::style::Style::default()
        .fg(color(hex))
        .add_modifier(ratatui::style::Modifier::DIM)
}
