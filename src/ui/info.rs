use std::time::Duration;

use ratatui::style::{Color, Modifier, Style};
use ratatui::text::Span;

use crate::data::*;

fn format_uptime(duration: Duration) -> String {
    let total_secs = duration.as_secs();
    let days = total_secs / 86400;
    let hours = (total_secs % 86400) / 3600;
    let minutes = (total_secs % 3600) / 60;
    let seconds = total_secs % 60;
    if days > 0 {
        format!("{} days, {:02}:{:02}:{:02}", days, hours, minutes, seconds)
    } else {
        format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
    }
}

/// Tasks: 446; 414 running
///   ^label   ^num  ^status
pub fn render_tasks(tasks: &TasksData, _width: usize) -> Vec<Span<'static>> {
    let mut spans: Vec<Span<'static>> = Vec::new();

    spans.push(Span::styled(
        "Tasks: ",
        Style::default().fg(Color::Cyan),
    ));

    spans.push(Span::styled(
        format!("{}", tasks.total),
        Style::default()
            .fg(Color::Green)
            .add_modifier(Modifier::BOLD),
    ));

    spans.push(Span::styled("; ", Style::default().fg(Color::Cyan)));

    spans.push(Span::styled(
        format!("{}", tasks.running),
        Style::default()
            .fg(Color::Green)
            .add_modifier(Modifier::BOLD),
    ));

    spans.push(Span::styled(
        " running",
        Style::default().fg(Color::Cyan),
    ));

    spans
}

/// Uptime: 80 days, 12:50:48
///   ^label   ^value
pub fn render_uptime(uptime: &Duration, _width: usize) -> Vec<Span<'static>> {
    let mut spans: Vec<Span<'static>> = Vec::new();

    spans.push(Span::styled(
        "Uptime: ",
        Style::default().fg(Color::Cyan),
    ));

    spans.push(Span::styled(
        format_uptime(*uptime),
        Style::default()
            .fg(Color::LightCyan)
            .add_modifier(Modifier::BOLD),
    ));

    spans
}
