// Midnight Violet - cohesive dark palette with purple accent
pub const BASE: &str = "#0c0c14";
pub const MANTLE: &str = "#0a0a12";
pub const CRUST: &str = "#06060c";
pub const SURFACE0: &str = "#12121c";
pub const SURFACE1: &str = "#1a1a26";
pub const SURFACE2: &str = "#242432";
pub const OVERLAY0: &str = "#3a3a50";
pub const OVERLAY1: &str = "#50506a";
pub const TEXT: &str = "#d8d8e8";
pub const SUBTEXT0: &str = "#7878a0";
pub const SUBTEXT1: &str = "#9898b8";
pub const LAVENDER: &str = "#b8b8f0";
pub const BLUE: &str = "#6888f0";
pub const SAPPHIRE: &str = "#58a0e8";
pub const SKY: &str = "#60c8e0";
pub const TEAL: &str = "#60d8b8";
pub const GREEN: &str = "#68e880";
pub const YELLOW: &str = "#e8d878";
pub const PEACH: &str = "#e8a060";
pub const MAROON: &str = "#e08080";
pub const RED: &str = "#f06080";
pub const MAUVE: &str = "#b090f0";
pub const PINK: &str = "#e0a0d0";
pub const FLAMINGO: &str = "#d8c8c8";
pub const ROSEWATER: &str = "#e0c8c0";

// Gradient helper - blends between two hex colors
pub fn blend(c1: &str, c2: &str, t: f32) -> ratatui::style::Color {
    let (r1, g1, b1) = hex_to_rgb(c1);
    let (r2, g2, b2) = hex_to_rgb(c2);
    let r = (r1 as f32 + (r2 as f32 - r1 as f32) * t) as u8;
    let g = (g1 as f32 + (g2 as f32 - g1 as f32) * t) as u8;
    let b = (b1 as f32 + (b2 as f32 - b1 as f32) * t) as u8;
    ratatui::style::Color::Rgb(r, g, b)
}

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

pub fn bold_bg(fg: &str, bg: &str) -> ratatui::style::Style {
    ratatui::style::Style::default()
        .fg(color(fg))
        .bg(color(bg))
        .add_modifier(ratatui::style::Modifier::BOLD)
}
