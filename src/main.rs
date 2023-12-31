use std::{
    io,
    path::{Path, PathBuf},
    sync::Arc,
};

use iced::{
    executor,
    highlighter::{self, Highlighter},
    theme,
    widget::{button, column, container, horizontal_space, row, text, text_editor, tooltip},
    Application, Command, Element, Font, Length, Settings, Theme,
};

fn main() -> iced::Result {
    Editor::run(Settings {
        default_font: Font::MONOSPACE,
        ..Default::default()
    })
}

struct Editor {
    path: Option<PathBuf>,
    content: text_editor::Content,
    error: Option<Error>,
    is_dirty: bool,
}

#[derive(Debug, Clone)]
enum Message {
    EditText(text_editor::Action),
    FileOpened(Result<(PathBuf, Arc<String>), Error>),
    Open,
    New,
    Save,
    FileSaved(Result<PathBuf, Error>),
}

impl Application for Editor {
    type Message = Message;
    type Executor = executor::Default;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Message>) {
        (
            Self {
                path: None,
                content: text_editor::Content::new(),
                error: None,
                is_dirty: true,
            },
            Command::perform(load_file(default_file()), Message::FileOpened),
        )
    }

    fn title(&self) -> String {
        String::from("Iced Editor")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::EditText(action) => {
                self.is_dirty = self.is_dirty || action.is_edit();
                self.content.edit(action);
                Command::none()
            }
            Message::FileOpened(Ok((path, content))) => {
                self.is_dirty = false;
                self.path = Some(path);
                self.content = text_editor::Content::with(&content);
                self.error = None;
                Command::none()
            }
            Message::FileOpened(Err(e)) => {
                self.error = Some(e);
                Command::none()
            }
            Message::Open => Command::perform(pick_file(), Message::FileOpened),
            Message::New => {
                self.path = None;
                self.is_dirty = true;
                self.content = text_editor::Content::new();
                Command::none()
            }
            Message::Save => Command::perform(
                save_file(self.content.text(), self.path.clone()),
                Message::FileSaved,
            ),
            Message::FileSaved(Ok(path)) => {
                self.path = Some(path);
                self.is_dirty = false;
                Command::none()
            }
            Message::FileSaved(Err(e)) => {
                self.error = Some(e);
                Command::none()
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let input = text_editor(&self.content)
            .on_edit(Message::EditText)
            .highlight::<Highlighter>(
                highlighter::Settings {
                    theme: highlighter::Theme::SolarizedDark,
                    extension: self
                        .path
                        .as_ref()
                        .and_then(|path| path.extension()?.to_str())
                        .unwrap_or("rs")
                        .to_string(),
                },
                |highlight, _theme| highlight.to_format(),
            );

        let status_bar = {
            let status = if let Some(Error::IO(error)) = self.error.as_ref() {
                text(error.to_string())
            } else {
                match self.path.as_deref().and_then(Path::to_str) {
                    Some(path) => text(path).size(14),
                    None => text("New file"),
                }
            };

            let position = {
                let (line, column) = self.content.cursor_position();
                text(format!("{}:{}", line + 1, column + 1))
            };

            row![status, horizontal_space(Length::Fill), position]
        };

        let controls = row![
            action("New", "New file", Some(Message::New)),
            action("Open", "Open a file", Some(Message::Open)),
            action(
                "Save",
                "Save the file",
                self.is_dirty.then_some(Message::Save)
            ),
        ]
        .spacing(10);

        container(column![controls, input, status_bar].spacing(10))
            .padding(10)
            .into()
    }

    fn theme(&self) -> iced::Theme {
        Theme::Dark
    }
}

fn action<'a>(
    content: impl Into<Element<'a, Message>>,
    label: &str,
    on_press: Option<Message>,
) -> Element<'a, Message> {
    let is_disabled = on_press.is_none();
    tooltip(
        button(container(content.into()).width(30).center_x())
            .on_press_maybe(on_press)
            .padding([5, 10])
            .style(if is_disabled {
                theme::Button::Secondary
            } else {
                theme::Button::Primary
            }),
        label,
        tooltip::Position::FollowCursor,
    )
    .style(theme::Container::Box)
    .into()
}

async fn pick_file() -> Result<(PathBuf, Arc<String>), Error> {
    let handle = rfd::AsyncFileDialog::new()
        .set_title("Choose a file")
        .pick_file()
        .await
        .ok_or(Error::DialogClosed)?;
    load_file(handle.path().to_path_buf()).await
}

async fn load_file(path: PathBuf) -> Result<(PathBuf, Arc<String>), Error> {
    let contents = tokio::fs::read_to_string(&path)
        .await
        .map(Arc::new)
        .map_err(|err| err.kind())
        .map_err(Error::IO)?;
    Ok((path, contents))
}

async fn save_file(contents: String, path: Option<PathBuf>) -> Result<PathBuf, Error> {
    let path = if let Some(path) = path {
        path
    } else {
        rfd::AsyncFileDialog::new()
            .set_title("Choose a file name")
            .save_file()
            .await
            .ok_or(Error::DialogClosed)
            .map(|path| path.path().to_path_buf())?
    };
    tokio::fs::write(&path, contents.into_bytes())
        .await
        .map_err(|err| err.kind())
        .map_err(Error::IO)?;
    Ok(path)
}

#[derive(Debug, Clone)]
enum Error {
    DialogClosed,
    IO(io::ErrorKind),
}

fn default_file() -> PathBuf {
    PathBuf::from(format!("{}/src/main.rs", env!("CARGO_MANIFEST_DIR")))
}
