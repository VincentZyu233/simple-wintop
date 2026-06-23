mod collector;
mod data;
mod debug;
mod ui;

use std::io;
use std::time::{Duration, Instant};

use clap::Parser;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use crossterm::terminal::size;

use collector::Collector;
use data::{EmptyFill, Margins, SystemData};

#[derive(Parser)]
#[command(name = "simple-wintop", version = concat!("v", env!("CARGO_PKG_VERSION"), " (", env!("TARGET"), ")"))]
struct Args {
    #[arg(short = 't', long = "interval", default_value = "1000")]
    interval: u64,

    #[arg(long = "fill", value_enum, default_value = "space")]
    empty_fill: EmptyFill,

    #[arg(long = "combine", default_value = "1")]
    combine: usize,

    #[arg(long = "margin-top", default_value = "1")]
    margin_top: u16,

    #[arg(long = "margin-bottom", default_value = "0")]
    margin_bottom: u16,

    #[arg(long = "margin-left", default_value = "1")]
    margin_left: u16,

    #[arg(long = "margin-right", default_value = "1")]
    margin_right: u16,

    #[arg(long = "margin-center", default_value = "5")]
    margin_center: u16,

    #[arg(long)]
    debug: bool,
}

fn main() -> io::Result<()> {
    let args = Args::parse();
    let tick_rate = Duration::from_millis(args.interval);

    let terminal_size = size().ok();

    let mut terminal = ratatui::init();

    let mut collector = Collector::new(args.combine);
    let mut last_tick = Instant::now();
    let mut last_data: Option<SystemData> = None;

    let margins = Margins {
        top: args.margin_top,
        bottom: args.margin_bottom,
        left: args.margin_left,
        right: args.margin_right,
        center: args.margin_center,
    };

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
            last_data = Some(data);
            let data = last_data.as_ref().unwrap();
            terminal.draw(|frame| ui::draw(frame, data, &args.empty_fill, &margins))?;
            last_tick = Instant::now();
        }
    }

    ratatui::restore();

    if args.debug {
        debug::print_debug_info(terminal_size, last_data.as_ref(), &margins, args.interval, args.combine);
    }

    Ok(())
}
