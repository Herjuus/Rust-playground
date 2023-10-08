use iced::{Element, Sandbox, Settings};
use iced::widget::{text, text_input};

fn main() -> iced::Result {
    Editor::run(Settings::default())
}

struct Editor;

#[derive(Debug, Clone)]
enum Message {
    TextInputChanged(String),
}

impl Sandbox for Editor {
    type Message = Message;

    fn new() -> Self {
        Self
    }

    fn title(&self) -> String {
        String::from("Herjussitor")
    }

    fn update(&mut self, message: Message) {
        match message { Message::TextInputChanged(_) => {} }
    }

    fn view(&self) -> Element<'_, Self::Message> {
        let value = "Test";

        text_input("Placeholder...", value).on_input(Message::TextInputChanged).into()
    }
}