use iced::widget::{column, container, text, text_editor};
use iced::{Element, Task};

fn main() -> iced::Result {
    iced::application("Quill", App::update, App::view).run()
}

struct App {
    content: text_editor::Content,
}

impl Default for App {
    fn default() -> Self {
        Self {
            content: text_editor::Content::new(),
        }
    }
}

#[derive(Debug, Clone)]
enum Message {
    EditorAction(text_editor::Action),
}

impl App {
    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::EditorAction(action) => {
                self.content.perform(action);
            }
        }
        Task::none()
    }

    fn view(&self) -> Element<'_, Message> {
        let editor = text_editor(&self.content)
            .on_action(Message::EditorAction)
            .placeholder("Start writing your screenplay...");

        let content = column![
            text("Quill").size(32),
            editor,
        ]
        .spacing(10)
        .padding(20);

        container(content)
            .width(iced::Length::Fill)
            .height(iced::Length::Fill)
            .into()
    }
}