use iced::{executor, Application, Command, Theme};


fn main() {
    println!("Hello, world!");
}

struct AppConfig {

}

#[derive(Debug,Clone)]
enum Message {
    InformationReceived
}

impl Application for AppConfig {
    type Executor=executor::Default;

    type Message=Message;

    type Theme=Theme;

    type Flags=();

    fn new(_flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        (
            Self{

            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("System Info")
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        todo!()
    }

    fn view(&self) -> iced::Element<'_, Self::Message, Self::Theme, iced::Renderer> {
        todo!()
    }
}