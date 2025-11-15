use crate::document::{Element, ElementType};
use iced::widget::canvas;
use iced::{mouse, Color, Font, Point, Rectangle, Renderer, Theme};
use std::time::Instant;

pub struct ScreenplayEditor {
    elements: Vec<Element>,
    cursor_position: usize,
    cursor_offset: usize,
    last_blink: Instant,
}

impl ScreenplayEditor {
    pub fn new(elements: Vec<Element>, cursor_position: usize, cursor_offset: usize) -> Self {
        Self {
            elements,
            cursor_position,
            cursor_offset,
            last_blink: Instant::now(),
        }
    }
}

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
        let font_size = 12.0;
        
        let char_width = 7.2;

        let courier_font = Font {
            family: iced::font::Family::Name("Courier New"),
            weight: iced::font::Weight::Normal,
            stretch: iced::font::Stretch::Normal,
            style: iced::font::Style::Normal,
        };

        for (index, element) in self.elements.iter().enumerate() {
            let (display_text, x_position) = match element.element_type {
                ElementType::SceneHeading => {
                    (element.content.to_uppercase(), 100.0)
                }
                ElementType::Action => {
                    (element.content.clone(), 100.0)
                }
                ElementType::Character => {
                    (element.content.to_uppercase(), 250.0)
                }
                ElementType::Dialogue => {
                    (element.content.clone(), 180.0)
                }
                ElementType::Parenthetical => {
                    (format!("({})", element.content), 220.0)
                }
                ElementType::Transition => {
                    (element.content.to_uppercase(), 400.0)
                }
            };

            frame.fill_text(canvas::Text {
                content: display_text.clone(),
                position: Point::new(x_position, y_position),
                color: Color::BLACK,
                size: font_size.into(),
                font: courier_font,
                ..canvas::Text::default()
            });

            if index == self.cursor_position {
                let should_show = (self.last_blink.elapsed().as_millis() / 500) % 2 == 0;
                
                if should_show {
                    let cursor_x = x_position + (self.cursor_offset as f32 * char_width);
                    
                    frame.fill_rectangle(
                        Point::new(cursor_x, y_position - 2.0),
                        iced::Size::new(2.0, line_height - 2.0),
                        Color::from_rgb(0.0, 0.0, 0.0),
                    );
                }
            }

            y_position += line_height;
        }

        vec![frame.into_geometry()]
    }
}