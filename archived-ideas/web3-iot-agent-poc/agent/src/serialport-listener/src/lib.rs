use anyhow::anyhow;
use crossbeam_channel::Receiver;
use helpers::{check_ctrlc, AppResult};
use serialport::{ClearBuffer, SerialPort};
use std::thread;

use crate::action::Action;
use crate::data::DataCategory;
use crate::event::Event;

mod action;
mod data;
mod event;

const RECEIVED_MSG_PREFIX: &str = "RECEIVED:";
const RECEIVED_MSG_SUFFIX: &str = "\n\r";
const MAX_LINE_LENGTH: usize = 1_024;
// const MAX_LINE_LENGTH: usize = 80;
// const MAX_LINE_LENGTH: usize = 15;

pub fn run(port_path: &String, baud_rate: u32, ctrl_c_events: Receiver<()>) -> AppResult<()> {
    use std::time::Duration;

    let mut rx: Box<dyn SerialPort> = serialport::new(port_path, baud_rate)
        .timeout(Duration::from_millis(10))
        .open()
        .map_err(|ref e| anyhow!("Port '{}' not available: {}", &port_path, e))?;

    // _ => return Err(anyhow::anyhow!("Invalid tuple: port_path and baud_rate")),

    let mut tx = rx.try_clone().expect("Failed to clone");
    println!("Connected to {} at {} baud", &port_path, &baud_rate);
    println!("Ctrl+C to stop.");
    let mut input_buffer: String = String::with_capacity(MAX_LINE_LENGTH);
    let mut read_buffer = [0u8; MAX_LINE_LENGTH];

    loop {
        if let Err(error) = check_ctrlc(&ctrl_c_events) {
            eprintln!("{error}");
            drop(tx);
            drop(rx);
            break;
        }

        let bytes_waiting = rx.bytes_to_read().expect("Error calling bytes_to_read");

        if bytes_waiting > 0 {
            println!("Bytes available to read: {bytes_waiting}");
            rx.read(read_buffer.as_mut_slice())?;
            rx.clear(ClearBuffer::Input)
                .expect("Failed to discard input buffer");
            dbg!(input_buffer.len());
            for c in read_buffer {
                if c != 0 {
                    input_buffer.push(c as char)
                }
            }
            dbg!(&input_buffer);

            if input_buffer.contains('\n') {
                if let Some(first_line) = input_buffer.lines().next() {
                    let task = match handle_event(first_line) {
                        Ok(task) => task,
                        Err(error) => {
                            eprintln!("ERROR: {error}");
                            Event::Nil
                        }
                    };
                    tx.clear(ClearBuffer::All)
                        .expect("Failed to discard output buffer");
                    tx.write_all(RECEIVED_MSG_PREFIX.as_bytes())?;
                    tx.write_all(task.to_string().as_bytes())?;
                    tx.write_all(RECEIVED_MSG_SUFFIX.as_bytes())?;

                    thread::sleep(Duration::from_millis(100));
                }
                input_buffer.clear();
                dbg!(&input_buffer);
            }
            tx.clear(ClearBuffer::All)
                .expect("Failed to discard output buffer");
            rx.clear(ClearBuffer::All)
                .expect("Failed to discard output buffer");
            dbg!(&input_buffer);
        }

        thread::sleep(Duration::from_millis(500));
    }
    Ok(())
}

fn handle_event<T: AsRef<str>>(event_str: T) -> AppResult<Event> {
    let columns: Vec<&str> = event_str.as_ref().split('|').collect();

    let event = match columns.first() {
        Some(s) => s.parse::<Event>()?,
        None => return Err(anyhow!("Invalid syntax: [{:?}]", event_str.as_ref())),
    };

    match event {
        Event::Action => do_action(columns)?,
        Event::Data => get_data(columns)?,
        Event::InternalError => handle_error(columns)?,
        _ => (),
    };
    Ok(event)
}

fn do_action(columns: Vec<&str>) -> AppResult<()> {
    if columns.len() != 3 {
        return Err(anyhow::anyhow!(
            "Invalid syntax on columns length: {columns:?}"
        ));
    }

    println!("{columns:?}");

    let action = match columns.iter().nth(2).and_then(|s| s.parse::<Action>().ok()) {
        Some(data) => data,
        None => {
            return Err(anyhow::anyhow!(
                "Invalid syntax on parsing Action: {columns:?}"
            ))
        }
    };
    println!("Do action!");
    println!("{action:?}");

    Ok(())
}

fn get_data(columns: Vec<&str>) -> AppResult<()> {
    if columns.len() != 5 {
        return Err(anyhow::anyhow!(
            "Invalid syntax on parsing Data: {columns:?}"
        ));
    }
    // TODO: Parse DataCategory
    let data_category = match columns.iter().nth(1).and_then(|s| s.parse::<DataCategory>().ok()) {
        Some(data) => data,
        None => {
            return Err(anyhow::anyhow!(
                "Invalid syntax on parsing Data: {columns:?}"
            ))
        }
    };

    

    println!("Get data!");
    println!("{columns:?}");

    println!("{data_category:?}");
    Ok(())
}

fn handle_error(columns: Vec<&str>) -> AppResult<()> {
    if columns.len() != 3 {
        return Err(anyhow::anyhow!(
            "Invalid syntax on parsing InternalError: {columns:?}"
        ));
    }
    println!("Handle error!");
    println!("{columns:?}");
    Ok(())
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
