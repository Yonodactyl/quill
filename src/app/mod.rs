mod message;
mod navigation;
mod update;

pub use message::Message;

use crate::document::{Element, ElementType, Screenplay};
use iced::widget::text_editor;
use iced::{Subscription, Task, time};
use navigation::CursorState;
use std::time::Duration;

pub struct App {
    pub screenplay: Screenplay,
    pub content: text_editor::Content,
    pub current_element_type: ElementType,
    pub cursor_position: usize,
    pub cursor_offset: usize,
    pub cursor_visible: bool,
    pub desired_cursor_offset: usize,
}

impl Default for App {
    fn default() -> Self {
        let mut screenplay = Screenplay::new("Untitled".to_string());
        screenplay.add_element(Element::new(ElementType::Action, String::new()));

        Self {
            screenplay,
            content: text_editor::Content::new(),
            current_element_type: ElementType::Action,
            cursor_position: 0,
            cursor_offset: 0,
            cursor_visible: true,
            desired_cursor_offset: 0,
        }
    }
}

impl App {
    fn cursor_state(&self) -> CursorState {
        CursorState {
            position: self.cursor_position,
            offset: self.cursor_offset,
            desired_offset: self.desired_cursor_offset,
            element_type: self.current_element_type,
        }
    }

    fn apply_cursor_state(&mut self, cursor: CursorState) {
        self.cursor_position = cursor.position;
        self.cursor_offset = cursor.offset;
        self.desired_cursor_offset = cursor.desired_offset;
        self.current_element_type = cursor.element_type;
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Tick => {
                self.cursor_visible = !self.cursor_visible;
            }
            Message::EditorAction(action) => {
                self.content.perform(action);
            }
            Message::TabPressed => {
                self.current_element_type = update::next_element_type(self.current_element_type);
                if let Some(element) = self.screenplay.elements.get_mut(self.cursor_position) {
                    element.element_type = self.current_element_type;
                }
            }
            Message::CharacterTyped(c) => {
                let mut cursor = self.cursor_state();
                cursor.insert_char(&mut self.screenplay, c);
                self.apply_cursor_state(cursor);
            }
            Message::BackspacePressed => {
                let mut cursor = self.cursor_state();
                cursor.backspace(&mut self.screenplay);
                self.apply_cursor_state(cursor);
            }
            Message::DeletePressed => {
                let mut cursor = self.cursor_state();
                cursor.delete(&mut self.screenplay);
                self.apply_cursor_state(cursor);
            }
            Message::ArrowLeft => {
                let mut cursor = self.cursor_state();
                cursor.move_left();
                self.apply_cursor_state(cursor);
            }
            Message::ArrowRight => {
                let mut cursor = self.cursor_state();
                cursor.move_right(&self.screenplay);
                self.apply_cursor_state(cursor);
            }
            Message::ArrowUp => {
                let mut cursor = self.cursor_state();
                cursor.move_up(&self.screenplay);
                self.apply_cursor_state(cursor);
            }
            Message::ArrowDown => {
                let mut cursor = self.cursor_state();
                cursor.move_down(&self.screenplay);
                self.apply_cursor_state(cursor);
            }
            Message::EnterPressed => {
                if let Some(current_element) = self.screenplay.elements.get(self.cursor_position) {
                    self.current_element_type =
                        update::detect_next_element_type(self.current_element_type, &current_element.content);
                }

                let element = Element::new(self.current_element_type, String::new());
                self.screenplay.insert_element(self.cursor_position + 1, element);
                self.cursor_position += 1;
                self.cursor_offset = 0;
                self.desired_cursor_offset = 0;
            }
            Message::EventOccurred(event) => {
                if let Some(task) = update::handle_event(event) {
                    return task;
                }
            }
        }
        Task::none()
    }

    pub fn subscription(&self) -> Subscription<Message> {
        Subscription::batch(vec![
            iced::event::listen().map(Message::EventOccurred),
            time::every(Duration::from_millis(500)).map(|_| Message::Tick),
        ])
    }
}
