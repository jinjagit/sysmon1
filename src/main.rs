use iced::{
    button, executor, time, Align, Application, Button, Column, Command, Container, Element,
    HorizontalAlignment, Length, Row, Settings, Subscription, Text,
};
use std::time::Duration;
use sysinfo::{ProcessorExt, System, SystemExt};
use sysmon1::*;

pub fn main() -> iced::Result {
    SystemMonitor::run(Settings::default())
}

struct SystemMonitor {
    sysinfo: System,
    cpu_usage: f32,
    num_cores: u8,
    aves: Vec<f32>,
    cpu_usage_text: Text,
    state: State,
    toggle: button::State,
}

#[allow(dead_code)]
enum State {
    Idle,
    Ticking,
}

#[derive(Debug, Clone)]
enum Message {
    Toggle,
    Tick,
}

impl Application for SystemMonitor {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (SystemMonitor, Command<Message>) {
        (
            SystemMonitor {
                sysinfo: sysinfo::System::new_all(),
                cpu_usage: 0.0,
                num_cores: {
                    let system = sysinfo::System::new_all();
                    let mut num: u8 = 0;

                    // Count virtual cores.
                    for _processor in system.get_processors() {
                        num += 1;
                    }

                    num
                },
                aves: vec![0.0, 0.0, 0.0, 0.0, 0.0],
                cpu_usage_text: Text::new(format!("---")).size(40),
                state: State::Idle,
                toggle: button::State::new(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("CPU usage test")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::Toggle => match self.state {
                State::Idle => {
                    self.state = State::Ticking;
                }
                State::Ticking => {
                    self.state = State::Idle;
                }
            },
            Message::Tick => match &mut self.state {
                State::Ticking => {
                    self.sysinfo.refresh_all();

                    let mut total: f32 = 0.0;

                    for processor in self.sysinfo.get_processors() {
                        total += processor.get_cpu_usage();
                    }

                    let new_ave = total / self.num_cores as f32;

                    self.aves = add_to_queue(self.aves.clone(), new_ave);

                    self.cpu_usage = array_ave(self.aves.clone());

                    self.cpu_usage_text = Text::new(format!("{:.2}", self.cpu_usage)).size(40);
                }
                _ => {}
            },
        }

        Command::none()
    }

    fn subscription(&self) -> Subscription<Message> {
        const TICK: u64 = 500; // Tick time step, in miliseconds.
        match self.state {
            State::Idle => Subscription::none(),
            State::Ticking => {
                time::every(Duration::from_millis(TICK)).map(|_instant| Message::Tick)
            }
        }
    }

    fn view(&mut self) -> Element<Message> {
        let info_text = Text::new(format!("CPU % usage")).size(24);

        let cpu_usage_text = Text::new(format!("{:.2}", self.cpu_usage)).size(40);

        let button = |state, label, style| {
            Button::new(
                state,
                Text::new(label).horizontal_alignment(HorizontalAlignment::Center),
            )
            .min_width(80)
            .padding(10)
            .style(style)
        };

        let toggle_button = {
            let (label, color) = match self.state {
                State::Idle => ("Start", style::Button::Primary),
                State::Ticking { .. } => ("Stop", style::Button::Destructive),
            };

            button(&mut self.toggle, label, color).on_press(Message::Toggle)
        };

        let controls = Row::new().spacing(20).push(toggle_button);

        let content = Column::new()
            .align_items(Align::Center)
            .spacing(20)
            .push(info_text)
            .push(cpu_usage_text)
            .push(controls);

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .style(style::Container)
            .into()
    }
}

mod style {
    use iced::{button, container, Background, Color, Vector};

    pub enum Button {
        Primary,
        Destructive,
    }

    impl button::StyleSheet for Button {
        fn active(&self) -> button::Style {
            button::Style {
                background: Some(Background::Color(match self {
                    Button::Primary => Color::from_rgb(0.11, 0.42, 0.87),
                    Button::Destructive => Color::from_rgb(0.8, 0.2, 0.2),
                })),
                border_radius: 12.0,
                shadow_offset: Vector::new(1.0, 1.0),
                text_color: Color::WHITE,
                ..button::Style::default()
            }
        }
    }

    pub struct Container;

    impl container::StyleSheet for Container {
        fn style(&self) -> container::Style {
            container::Style {
                background: Some(Background::Color(Color::from_rgb8(0x36, 0x39, 0x3F))),
                text_color: Some(Color::WHITE),
                ..container::Style::default()
            }
        }
    }
}
