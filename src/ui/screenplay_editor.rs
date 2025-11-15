use crate::document::{Element, ElementType};
use iced::widget::canvas;
use iced::{mouse, Color, Point, Rectangle, Renderer, Theme};

pub struct ScreenplayEditor {
    elements: Vec<Element>,
}

impl ScreenplayEditor {
    pub fn new(elements: Vec<Element>) -> Self {
        Self { elements }
    }
}

#[derive(Debug, Clone)]
pub enum Message {}

impl<Message> canvas::Program<Message> for ScreenplayEditor {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<canvas::Geometry> {
        let mut frame = canvas::Frame::new(renderer, bounds.size());

        let mut y_position = 20.0;
        let line_height = 24.0;

        for element in &self.elements {
            let (text_content, x_position, font_size) = match element.element_type {
                ElementType::SceneHeading => {
                    (element.formatted_content(), 100.0, 12.0)
                }
                ElementType::Action => {
                    (element.content.clone(), 100.0, 12.0)
                }
                ElementType::Character => {
                    (element.formatted_content(), 250.0, 12.0)
                }
                ElementType::Dialogue => {
                    (element.content.clone(), 180.0, 12.0)
                }
                ElementType::Parenthetical => {
                    (element.content.clone(), 220.0, 12.0)
                }
                ElementType::Transition => {
                    (element.formatted_content(), 400.0, 12.0)
                }
            };

            frame.fill_text(canvas::Text {
                content: text_content,
                position: Point::new(x_position, y_position),
                color: Color::BLACK,
                size: font_size.into(),
                ..canvas::Text::default()
            });

            y_position += line_height;
        }

        vec![frame.into_geometry()]
    }
}