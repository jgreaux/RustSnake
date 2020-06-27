use iced::{Column, Element, Sandbox, Settings};

pub fn display() {
    Scene::run(Settings::default())
}

#[derive(Default)]
struct Scene {
}

#[derive(Debug, Clone, Copy)]
enum Message {
}

impl Sandbox for Scene {
    type Message = Message;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        String::from("Snake !!!")
    }

    fn update(&mut self, message: Message) {
        match message {
        }
    }

    fn view(&mut self) -> Element<Message> {
        Column::new()
            .into()
    }
}
