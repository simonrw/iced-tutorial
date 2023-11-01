use iced::{
    executor,
    widget::{column, container, horizontal_space, row, text, text_editor},
    Application, Command, Element, Length, Settings, Theme,
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

impl Application for Editor {
    type Message = Message;
    type Executor = executor::Default;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Message>) {
        (
            Self {
                content: text_editor::Content::with(include_str!("main.rs")),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Iced Editor")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::EditText(action) => self.content.edit(action),
        }

        Command::none()
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
        Theme::Dark
    }
}
