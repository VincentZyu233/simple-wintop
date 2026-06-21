use std::collections::VecDeque;
use std::time::Duration;

use sysinfo::{CpuRefreshKind, ProcessRefreshKind, ProcessesToUpdate, System};

use crate::data::*;

pub struct Collector {
    system: System,
    combine: usize,
    cpu_history: VecDeque<f64>,
}

const MAX_HISTORY: usize = 900;

impl Collector {
    pub fn new(combine: usize) -> Self {
        let mut system = System::new();
        system.refresh_cpu_list(CpuRefreshKind::everything());
        system.refresh_memory();
        let combine = if combine < 1 { 1 } else { combine };
        Self {
            system,
            combine,
            cpu_history: VecDeque::with_capacity(MAX_HISTORY),
        }
    }

    pub fn collect(&mut self) -> SystemData {
        self.system.refresh_cpu_usage();
        self.system.refresh_memory();
        self.system.refresh_processes_specifics(
            ProcessesToUpdate::All,
            true,
            ProcessRefreshKind::new(),
        );

        let raw: Vec<CpuData> = self
            .system
            .cpus()
            .iter()
            .enumerate()
            .map(|(i, cpu)| CpuData {
                name: i.to_string(),
                usage: cpu.cpu_usage() as f64,
            })
            .collect();

        let num_cores = raw.len() as f64;
        let cpus = group_cpus(&raw, self.combine);

        let overall_usage = self.system.global_cpu_usage() as f64;
        let load_value = if num_cores > 0.0 {
            overall_usage / 100.0 * num_cores
        } else {
            0.0
        };

        self.cpu_history.push_front(load_value);
        while self.cpu_history.len() > MAX_HISTORY {
            self.cpu_history.pop_back();
        }

        let total_proc = self.system.processes().len() as u32;
        let running_proc = self
            .system
            .processes()
            .values()
            .filter(|p| p.status() == sysinfo::ProcessStatus::Run)
            .count() as u32;

        SystemData {
            cpus,
            memory: MemoryData {
                total: self.system.total_memory(),
                used: self.system.used_memory(),
            },
            swap: SwapData {
                total: self.system.total_swap(),
                used: self.system.used_swap(),
            },
            tasks: TasksData {
                total: total_proc,
                running: running_proc,
            },
            load_avg: LoadAverageData {
                one: self.average_over(60),
                five: self.average_over(300),
                fifteen: self.average_over(900),
            },
            uptime: Duration::from_secs(System::uptime()),
        }
    }

    fn average_over(&self, seconds: usize) -> f64 {
        let n = self.cpu_history.len().min(seconds);
        if n == 0 {
            return 0.0;
        }
        let sum: f64 = self.cpu_history.iter().take(n).sum();
        sum / n as f64
    }
}

fn group_cpus(raw: &[CpuData], combine: usize) -> Vec<CpuData> {
    if combine <= 1 {
        return raw.to_vec();
    }
    raw.chunks(combine)
        .enumerate()
        .map(|(gi, chunk)| {
            let start = gi * combine;
            let end = start + chunk.len() - 1;
            let avg = chunk.iter().map(|c| c.usage).sum::<f64>() / chunk.len() as f64;
            CpuData {
                name: format!("{}-{}", start, end),
                usage: avg,
            }
        })
        .collect()
}
