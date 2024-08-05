use std::io::{BufRead, BufReader};
use std::{collections::BTreeMap, fs::File};

use crate::AppResult;

#[derive(Debug)]
pub struct Cpu {
    user: String,
}

#[derive(Debug)]
pub struct Stats {
    input: String,
    cpus: Option<Vec<Cpu>>,
    output: String,
}

const MAX_BUFFER: usize = 256;

impl Stats {
    pub fn new() -> Self {
        use std::process::Command;

        let output = Command::new("cat")
            .arg("/proc/stat")
            .output()
            .expect("Failed to execute command");

        if !output.status.success() {
            eprintln!(
                "Command exited with error: {}",
                String::from_utf8_lossy(&output.stderr)
            );
        }

        Self {
            input: String::from_utf8_lossy(&output.stdout).to_string(),
            cpus: None,
            output: "".into(),
        }
    }
    pub fn render(mut self, prev_stats: &mut BTreeMap<String, (u64, u64)>) -> AppResult<String> {
        let mut msg = "".to_string();

        // {
        //     for line in self.input.lines() {
        //         if line.starts_with("cpu") {
        //             msg.push_str(line);
        //             msg.push('\n');
        //         }
        //     }
        // }
        {
            let cpu_stats = read_cpu_stats()?;
            let mem_stats = read_mem_stats()?;
            msg.push_str("\nCPU usage:");
            msg.push('\n');
            for (cpu, (idle, total)) in &cpu_stats {
                if let Some(&(prev_idle, prev_total)) = prev_stats.get(cpu) {
                    let idle_delta = idle - prev_idle;
                    let total_delta = total - prev_total;
                    let cpu_usage = 100.0 * (1.0 - idle_delta as f64 / total_delta as f64);

                    let line = format!(
                        "{} {} {:.2}%",
                        cpu,
                        render_progressbar(cpu_usage, 20),
                        cpu_usage
                    );

                    // let line = format!("{}: {:.2}%", cpu, cpu_usage);
                    msg.push_str(&line);
                    msg.push('\n');
                }
            }
            msg.push('\n');
            if let (Some(total), Some(free), Some(available)) = (
                mem_stats.get("MemTotal"),
                mem_stats.get("MemFree"),
                mem_stats.get("MemAvailable"),
            ) {
                let used = total - available;
                let usage_percent = (used as f64 / *total as f64) * 100.0;
                let total_mb = *total as f64 / 1024.0;
                let used_mb = used as f64 / 1024.0;
                let line = format!("Memory Usage:\n {} {:.2}%", render_progressbar(usage_percent, 20),usage_percent);
                msg.push_str(&line);
                msg.push('\n');
                let line = format!(
                    "({:.2} MB used / {:.2} MB total)",
                    used_mb, total_mb
                );
                msg.push_str(&line);
                msg.push('\n');
            }
            // Add a blank line between iterations
            msg.push('\n');

            *prev_stats = cpu_stats;
        }
        {
            for (n, line) in msg.lines().enumerate() {
                self.print(line, 1, n as u8);
            }
        }
        dbg!(&self.output);
        Ok(self.output)
    }
    fn print(&mut self, msg: impl AsRef<str>, column: u8, line: u8) {
        let outcome = format!("printString('{}', {column},{line});", msg.as_ref());
        self.output.push_str(&outcome);
    }
}

fn read_cpu_stats() -> AppResult<BTreeMap<String, (u64, u64)>> {
    let file = File::open("/proc/stat")?;
    let reader = BufReader::new(file);
    let mut cpu_stats = BTreeMap::new();

    for line in reader.lines() {
        let line = line?;
        if line.starts_with("cpu") {
            let mut parts = line.split_whitespace();
            let cpu_name = parts.next().unwrap_or("").to_string();

            if cpu_name != "cpu" {
                dbg!(&cpu_name);
                let cpu_fmt_name = cpu_name
                    .split("cpu")
                    .nth(1)
                    .and_then(|s| s.parse::<u8>().ok())
                    .map(|n| {
                        if n < 10 {
                            format!("0{n}")
                        } else {
                            format!("{n}")
                        }
                    })
                    .unwrap();
                let values: Vec<u64> = parts.map(|val| val.parse().unwrap_or(0)).collect();

                if values.len() >= 4 {
                    let idle = values[3];
                    let total: u64 = values.iter().sum();
                    cpu_stats.insert(cpu_fmt_name, (idle, total));
                }
            }
        }
    }

    Ok(cpu_stats)
}

fn read_mem_stats() -> AppResult<BTreeMap<String, u64>> {
    let file = File::open("/proc/meminfo")?;
    let reader = BufReader::new(file);
    let mut mem_stats = BTreeMap::new();

    for line in reader.lines() {
        let line = line?;
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 2 {
            let key = parts[0].trim_end_matches(':').to_string();
            if let Ok(value) = parts[1].parse::<u64>() {
                mem_stats.insert(key, value);
            }
        }
    }

    Ok(mem_stats)
}

fn render_progressbar(progress_percentage: f64, progress_length: usize) -> String {
    let progress = (progress_percentage / 100.0 * progress_length as f64)
        .round()
        .floor() as usize;
    dbg!(progress_percentage, progress_percentage, progress);
    format!(
        "{}{}{}{}",
        "[",
        "|".repeat(progress),
        " ".repeat(progress_length - progress),
        "]"
    )
}
