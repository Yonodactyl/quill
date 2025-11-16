use crate::app::{App, Message};
use crate::ui::ScreenplayEditor;
use iced::widget::{canvas, column, container, text};
use iced::Element;

pub fn view(app: &App) -> Element<'_, Message> {
    let screenplay_display = canvas(ScreenplayEditor::new(
        app.screenplay.elements.clone(),
        app.cursor_position,
        app.cursor_offset,
        app.cursor_visible,
    ))
    .width(iced::Length::Fill)
    .height(iced::Length::Fill);

    let element_type_display = text(format!(
        "Current: {} (Press Tab to change)",
        app.current_element_type.as_str()
    ))
    .size(14);

    let info = text(format!(
        "{} - {} elements | Cursor at element {} offset {}",
        app.screenplay.title,
        app.screenplay.element_count(),
        app.cursor_position,
        app.cursor_offset
    ))
    .size(12);

    let content = column![
        text("Quill").size(32),
        info,
        element_type_display,
        screenplay_display,
    ]
    .spacing(10)
    .padding(20);

    container(content)
        .width(iced::Length::Fill)
        .height(iced::Length::Fill)
        .into()
}