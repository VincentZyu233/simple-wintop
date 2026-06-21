mod collector;
mod data;
mod ui;

use std::io;
use std::time::{Duration, Instant};

use clap::Parser;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};

use collector::Collector;

#[derive(Parser)]
#[command(name = "htop-rust-for-win", version = concat!("v", env!("CARGO_PKG_VERSION"), " (", env!("TARGET"), ")"))]
struct Args {
    #[arg(short = 't', long = "interval", default_value = "1000")]
    interval: u64,
}

fn main() -> io::Result<()> {
    let args = Args::parse();
    let tick_rate = Duration::from_millis(args.interval);

    let mut terminal = ratatui::init();

    let mut collector = Collector::new();
    let mut last_tick = Instant::now();

    loop {
        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_default();

        if event::poll(timeout)? {
            match event::read()? {
                Event::Key(key) if key.kind == KeyEventKind::Press => match key.code {
                    KeyCode::Char('q') | KeyCode::Char('Q') | KeyCode::Esc => break,
                    _ => {}
                },
                _ => {}
            }
        }

        if last_tick.elapsed() >= tick_rate {
            let data = collector.collect();
            terminal.draw(|frame| ui::draw(frame, &data))?;
            last_tick = Instant::now();
        }
    }

    ratatui::restore();
    Ok(())
}
