use crate::app::{App, Message};
use iced::widget::{column, container, text, text_editor};
use iced::Element;

pub fn view(app: &App) -> Element<'_, Message> {
    let editor = text_editor(&app.content)
        .on_action(Message::EditorAction)
        .placeholder("Start writing your screenplay...");

    let element_type_display = text(format!(
        "Current: {} (Press Tab to change)",
        app.current_element_type.as_str()
    ))
    .size(14);

    let info = text(format!(
        "{} - {} elements",
        app.screenplay.title,
        app.screenplay.element_count()
    ))
    .size(12);

    let content = column![
        text("Quill").size(32),
        info,
        element_type_display,
        editor,
    ]
    .spacing(10)
    .padding(20);

    container(content)
        .width(iced::Length::Fill)
        .height(iced::Length::Fill)
        .into()
}