use iced::widget::{button, column, text};
use iced::{Alignment, Element, Sandbox, Settings};

fn main() -> iced::Result{
    Counter::run(Settings::default())
}


//state
struct Counter {
    value: i32,
}
#[derive(Debug,Clone, Copy)]
enum Message {
    IncrementCounter,
    DecrementCounter,
}

impl Sandbox for Counter {
    type Message = Message;

    fn new() -> Self {
        Counter { value: 0 }
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::IncrementCounter => {
                self.value += 1;
            },
            Message::DecrementCounter => {
                self.value -= 1;
            }
        }
    }

    fn view(&self) -> Element<Message> {
        column![
            button("Increment").on_press(Message::IncrementCounter),
            text(self.value).size(50),
            button("Decrement").on_press(Message::DecrementCounter),
        ]
        .padding(20)
        .align_items(Alignment::Center)
        .into()
    }

    fn title(&self) -> String {
        String::from("Counter")
    }
}