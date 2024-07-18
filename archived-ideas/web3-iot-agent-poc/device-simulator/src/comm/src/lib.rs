use anyhow::anyhow;
use helpers::AppResult;

use std::str::FromStr;
use std::thread;

// const MAX_LINE_LENGTH: usize = 1_024;
// const MAX_LINE_LENGTH: usize = 80;
const MAX_LINE_LENGTH: usize = 15;

pub fn run(port_path: &str, baud_rate: &str) -> AppResult<()> {
    use std::time::Duration;
    let rate = baud_rate
        .parse::<u32>()
        .map_err(|_| anyhow!("Invalid baud rate '{}' specified", baud_rate))?;

    let mut port = serialport::new(port_path, rate)
        .timeout(Duration::from_millis(10))
        .open()
        .map_err(|ref e| anyhow!("Port '{}' not available: {}", &port_path, e))?;

    println!("Connected to {} at {} baud", &port_path, &baud_rate);
    println!("Ctrl+C to stop.");
    let mut read_buffer = [0u8; MAX_LINE_LENGTH];
    loop {
        let bytes_waiting = port.bytes_to_read().expect("Error calling bytes_to_read");
        println!("Bytes available to read: {bytes_waiting}",);
        if bytes_waiting as usize > (MAX_LINE_LENGTH - 1) {
            port.read_exact(read_buffer.as_mut_slice())?;
            let input = String::from_utf8_lossy(&read_buffer);
            let maybe_first_line = input.lines().next();
            // Send RECEIVED
            if let Some(first_line) = maybe_first_line {
                if let Err(error) = do_task(first_line) {
                    eprintln!("ERROR: {error}");
                }
            }
            dbg!(maybe_first_line);
        }

        thread::sleep(Duration::from_millis(1000));
    }
}

fn do_task<T: AsRef<str>>(maybe_task: T) -> AppResult<()> {
    let task = maybe_task.as_ref().parse::<Task>()?;
    match task {
        Task::Event => do_event(),
        Task::Sensor => do_sensor(),
        Task::Nil => Ok(()),
    }
}

fn do_event() -> AppResult<()> {
    println!("Do event!");
    Ok(())
}

fn do_sensor() -> AppResult<()> {
    println!("Do sensor!");
    Ok(())
}

#[derive(Debug, Default)]
enum Task {
    #[default]
    Nil,
    Event,
    Sensor,
}

impl FromStr for Task {
    type Err = anyhow::Error;

    fn from_str(input_str: &str) -> Result<Self, Self::Err> {
        let task = input_str
            .split_whitespace()
            .next()
            .map(|task_str| match task_str {
                "EVENT" => Self::Event,
                "SENSOR" => Self::Sensor,
                _ => Self::Nil,
            })
            .ok_or(anyhow!("Invalid syntax: [{input_str:?}]"))?;
        Ok(task)
    }
}

/////////////
pub fn print_ports() {
    println!("print_ports():");
    let ports = serialport::available_ports().expect("No ports found!");
    if ports.len() > 0 {
        for p in ports {
            println!("Port found: [{}]", p.port_name);
        }
    } else {
        println!("No serial ports found!");
    }
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
