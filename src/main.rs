use iced::{
    widget::{column, container, horizontal_space, row, text, text_editor},
    Element, Length, Sandbox, Settings,
};

fn main() -> iced::Result {
    Editor::run(Settings::default())
}

struct Editor {
    content: text_editor::Content,
}

#[derive(Debug, Clone)]
enum Message {
    EditText(text_editor::Action),
}

impl Sandbox for Editor {
    type Message = Message;

    fn new() -> Self {
        Self {
            content: text_editor::Content::with(include_str!("main.rs")),
        }
    }

    fn title(&self) -> String {
        String::from("Iced Editor")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::EditText(action) => self.content.edit(action),
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let input = text_editor(&self.content).on_edit(Message::EditText);

        let position = {
            let (line, column) = self.content.cursor_position();
            text(format!("{}:{}", line + 1, column + 1))
        };

        let status_bar = row![horizontal_space(Length::Fill), position];

        container(column![input, status_bar].spacing(10))
            .padding(10)
            .into()
    }

    fn theme(&self) -> iced::Theme {
        iced::Theme::Dark
    }
}
