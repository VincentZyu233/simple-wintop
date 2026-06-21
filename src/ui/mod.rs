pub mod bars;
pub mod info;

use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::Paragraph,
    Frame,
};

use crate::data::{EmptyFill, SystemData};

pub fn draw(frame: &mut Frame, data: &SystemData, empty_fill: &EmptyFill) {
    let area = frame.area();

    if area.height < 12 || area.width < 60 {
        draw_too_small(frame, area);
        return;
    }

    let outer = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(2),
            Constraint::Min(6),
            Constraint::Length(1),
        ])
        .split(area);

    let horz = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(2),
            Constraint::Min(20),
            Constraint::Length(6),
            Constraint::Min(20),
            Constraint::Length(2),
        ])
        .split(outer[1]);

    draw_left_column(frame, horz[1], data, empty_fill);
    draw_right_column(frame, horz[3], data, empty_fill);
}

fn draw_left_column(frame: &mut Frame, area: Rect, data: &SystemData, empty_fill: &EmptyFill) {
    let n_cpus = data.cpus.len();
    let n_left = (n_cpus + 1) / 2;

    let mut rows = Vec::new();
    for _ in 0..n_left {
        rows.push(Constraint::Length(1));
    }
    rows.push(Constraint::Length(1));
    rows.push(Constraint::Length(1));
    rows.push(Constraint::Min(0));

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(rows)
        .split(area);

    for (i, cpu) in data.cpus.iter().take(n_left).enumerate() {
        let spans = bars::render_cpu_bar(cpu, chunks[i].width as usize, empty_fill);
        frame.render_widget(Paragraph::new(Line::from(spans)), chunks[i]);
    }

    let off = n_left;
    frame.render_widget(
        Paragraph::new(Line::from(bars::render_mem_bar(&data.memory, chunks[off].width as usize, empty_fill))),
        chunks[off],
    );
    frame.render_widget(
        Paragraph::new(Line::from(bars::render_swap_bar(&data.swap, chunks[off + 1].width as usize, empty_fill))),
        chunks[off + 1],
    );
}

fn draw_right_column(frame: &mut Frame, area: Rect, data: &SystemData, empty_fill: &EmptyFill) {
    let n_cpus = data.cpus.len();
    let n_right = n_cpus / 2;
    let n_left = (n_cpus + 1) / 2;

    let mut rows = Vec::new();
    for _ in 0..n_right {
        rows.push(Constraint::Length(1));
    }
    rows.push(Constraint::Length(1));
    rows.push(Constraint::Length(1));
    rows.push(Constraint::Length(1));
    rows.push(Constraint::Min(0));

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(rows)
        .split(area);

    for (i, cpu) in data.cpus.iter().skip(n_left).enumerate() {
        let spans = bars::render_cpu_bar(cpu, chunks[i].width as usize, empty_fill);
        frame.render_widget(Paragraph::new(Line::from(spans)), chunks[i]);
    }

    let off = n_right;
    frame.render_widget(
        Paragraph::new(Line::from(info::render_tasks(&data.tasks, chunks[off].width as usize))),
        chunks[off],
    );
    frame.render_widget(
        Paragraph::new(Line::from(info::render_uptime(&data.uptime, chunks[off + 1].width as usize))),
        chunks[off + 1],
    );
}

fn draw_too_small(frame: &mut Frame, area: Rect) {
    let msg = "Terminal too small";
    let x = area.width.saturating_sub(msg.len() as u16) / 2;
    let y = area.height / 2;
    frame.render_widget(
        Paragraph::new(Line::from(Span::styled(
            msg,
            Style::default()
                .fg(Color::Red)
                .add_modifier(Modifier::BOLD),
        ))),
        Rect {
            x: area.x + x,
            y: area.y + y,
            width: msg.len() as u16,
            height: 1,
        },
    );
}
