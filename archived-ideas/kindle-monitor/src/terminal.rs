use std::{fs::File, io::Read};

pub struct Terminal {
    columns: u8,
    // lines: u8,
    buffer: String,
    initial_position: usize,
}

const MAX_BUFFER: usize = 256;

impl Terminal {
    pub fn new() -> Self {
        let output_path = "/tmp/kindle-monitor.log";
        let mut output = File::open(output_path).unwrap();

        let mut buffer = String::new();

        let bytes_read = output.read_to_string(&mut buffer).unwrap();

        let chars = buffer.chars().count();
        let initial_position = if chars > MAX_BUFFER {
            chars - MAX_BUFFER
        } else {
            0
        };
        println!("Terminal: {bytes_read} read from {output_path}");

        Self {
            columns: 47,
            // lines: 22,
            buffer,
            initial_position,
        }
    }
    pub fn render(self) -> String {
        let mut outcome = "".to_string();
        let mut printed_chars = 0;
        dbg!(&self.buffer);
        for (idx, c) in self.buffer.chars().enumerate() {
            if idx < self.initial_position {
                continue;
            }
            if c == '\n' || printed_chars == self.columns {
                outcome.push(c);
                printed_chars = 0;
            } else {
                outcome.push(c);
                print!("{c}");
                printed_chars += 1;
            }
        }
        dbg!(&outcome);
        outcome.replace("\n", "<br/>").replace(" ", "&nbsp;")
    }
}
