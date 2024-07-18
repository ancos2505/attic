use std::fmt::Display;

use gtk::prelude::*;
use relm4::gtk::EntryBuffer;
use relm4::prelude::*;

#[derive(Debug)]
enum AppMsg {
    ChangeData(String),
}

#[derive(Debug)]
struct App {
    ui_input: AppInput,
    current_data: Morse,
}

#[derive(Debug, Default)]
enum AppInput {
    #[default]
    Normal,
    Error,
}
impl From<&String> for AppInput {
    fn from(value: &String) -> Self {
        use std::ops::Not;
        let mut outcome = AppInput::default();
        for c in value.chars() {
            if (c.is_ascii_alphanumeric() || c.is_ascii_whitespace()).not() {
                outcome = AppInput::Error;
            }
        }
        outcome
    }
}

impl AppInput {
    pub fn as_css(&self) -> &'static str {
        match self {
            AppInput::Normal => "",
            AppInput::Error => "color: red;",
        }
    }
    pub fn as_label_msg(&self) -> &'static str {
        match self {
            AppInput::Normal => "",
            AppInput::Error => "Sorry. Only ascii alphanumeric is allowed",
        }
    }
}

#[relm4::component]
impl SimpleComponent for App {
    type Init = ();
    type Input = AppMsg;
    type Output = ();

    view! {
        main_window = gtk::ApplicationWindow {
            set_width_request: 360,
            set_title: Some("Morse"),

            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,
                set_margin_all: 12,
                set_spacing: 6,

                gtk::Entry {
                    connect_text_notify[sender] => move |entry| {
                        let buffer = entry.buffer();
                        sender.input(AppMsg::ChangeData(buffer.text().to_uppercase()))
                    }
                },
                gtk::Label {
                    #[watch]
                    inline_css: model.ui_input.as_css(),
                    #[watch]
                    set_text: model.ui_input.as_label_msg(),
                },
                gtk::Entry {
                    #[watch]
                    set_buffer: &EntryBuffer::builder().text( &format!("{}", model.current_data)).build()

                },
            }
        }
    }

    fn update(&mut self, msg: AppMsg, _sender: ComponentSender<Self>) {
        match msg {
            AppMsg::ChangeData(data) => {
                println!("[DATA]: [{data}]");
                self.ui_input = AppInput::from(&data);
                self.current_data = Morse::from(data);
                println!("[MODEL]: {:?}", self);
            }
        }
    }

    fn init(
        _: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let current_data = Morse::default();
        let ui_input = AppInput::default();
        let model = App {
            ui_input,
            current_data,
        };

        let widgets = view_output!();

        ComponentParts { model, widgets }
    }
}

fn main() {
    let app = RelmApp::new("relm4.example.to_do");
    app.run::<App>(());
}

#[derive(Debug, Default)]
struct Morse(String);
impl Display for Morse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl From<String> for Morse {
    fn from(value: String) -> Self {
        let inner = value
            .split_ascii_whitespace()
            .map(|word| {
                word.chars()
                    .map(|c| MorseChar::from(c).to_string())
                    .collect::<Vec<_>>()
                    .join(" ")
            })
            .collect::<Vec<_>>()
            .join("   ");
        Self(inner)
    }
}

struct MorseChar(&'static str);
impl Display for MorseChar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<char> for MorseChar {
    fn from(value: char) -> Self {
        let inner = match value {
            'A' => ".-",
            'B' => "-...",
            'C' => "-.-.",
            'D' => "-..",
            'E' => ".",
            'F' => "..-.",
            'G' => "--.",
            'H' => "....",
            'I' => "..",
            'J' => ".---",
            'K' => "-.-",
            'L' => ".-..",
            'M' => "--",
            'N' => "-.",
            'O' => "---",
            'P' => ".--.",
            'Q' => "--.-",
            'R' => ".-.",
            'S' => "...",
            'T' => "-",
            'U' => "..-",
            'V' => "...-",
            'W' => ".--",
            'X' => "-..-",
            'Y' => "-.--",
            'Z' => "--..",
            '1' => ".----",
            '2' => "..---",
            '3' => "...--",
            '4' => "....-",
            '5' => ".....",
            '6' => "-....",
            '7' => "--...",
            '8' => "---..",
            '9' => "----.",
            '0' => "-----",
            _ => "",
        };
        Self(inner)
    }
}
