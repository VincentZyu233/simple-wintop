use crate::data::{Margins, SystemData};

pub fn print_debug_info(
    terminal_size: Option<(u16, u16)>,
    data: Option<&SystemData>,
    margins: &Margins,
    interval: u64,
    combine: usize,
) {
    eprintln!();
    eprintln!("--- debug ---");
    if let Some((w, h)) = terminal_size {
        eprintln!("terminal:         {}x{}", w, h);
    }
    eprintln!("interval:         {} ms", interval);
    eprintln!("combine:          {}", combine);
    eprintln!(
        "margin:           top={} bottom={} left={} right={} center={}",
        margins.top, margins.bottom, margins.left, margins.right, margins.center
    );

    if let Some(d) = data {
        eprintln!("cpu groups:       {}", d.cpus.len());
        eprintln!(
            "processes:        total={} running={}",
            d.tasks.total, d.tasks.running
        );
        let secs = d.uptime.as_secs();
        eprintln!(
            "uptime:           {} days, {:02}:{:02}:{:02}",
            secs / 86400,
            (secs % 86400) / 3600,
            (secs % 3600) / 60,
            secs % 60
        );
    }
    eprintln!("---");
}
