use ratatui::style::{Color, Style};

pub const READ_COLOR: Color = Color::Rgb(180, 120, 255);  // purple
pub const WRITE_COLOR: Color = Color::Rgb(255, 150, 230); // pink
pub const QUEUE_LOW_COLOR: Color = Color::Rgb(180, 120, 255);
pub const QUEUE_MED_COLOR: Color = Color::Rgb(255, 150, 230);
pub const QUEUE_HIGH_COLOR: Color = Color::Rgb(255, 100, 200);
pub const HEADER_BG: Color = Color::DarkGray;
pub const SELECTED_TAB_COLOR: Color = Color::Cyan;
pub const LABEL_COLOR: Color = Color::Gray;
pub const BORDER_COLOR: Color = Color::DarkGray;
pub const HELP_BORDER_COLOR: Color = Color::Cyan;

pub fn read_style() -> Style {
    Style::default().fg(READ_COLOR)
}

pub fn write_style() -> Style {
    Style::default().fg(WRITE_COLOR)
}

pub fn border_style() -> Style {
    Style::default().fg(BORDER_COLOR)
}

pub fn label_style() -> Style {
    Style::default().fg(LABEL_COLOR)
}

/// Returns a color for utilization/queue depth gauges based on severity.
pub fn severity_color(pct: f64) -> Color {
    if pct < 50.0 {
        QUEUE_LOW_COLOR
    } else if pct < 80.0 {
        QUEUE_MED_COLOR
    } else {
        QUEUE_HIGH_COLOR
    }
}
