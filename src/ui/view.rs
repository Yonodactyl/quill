use crate::app::{App, Message};
use crate::ui::ScreenplayEditor;
use iced::widget::{canvas, column, container, text, text_input};
use iced::Element;

pub fn view(app: &App) -> Element<'_, Message> {
    // Create the screenplay display using canvas function
    let screenplay_display = canvas(ScreenplayEditor::new(app.screenplay.elements.clone()))
        .width(iced::Length::Fill)
        .height(iced::Length::FillPortion(3));

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

    // Add a simple text input so we can type
    let input = text_input("Type here...", &app.current_line)
        .on_input(Message::TextChanged)
        .on_submit(Message::EnterPressed)
        .padding(10);

    let content = column![
        text("Quill").size(32),
        info,
        element_type_display,
        screenplay_display,
        text("Current line:").size(12),
        input,
    ]
    .spacing(10)
    .padding(20);

    container(content)
        .width(iced::Length::Fill)
        .height(iced::Length::Fill)
        .into()
}