use crate::document::{Element, ElementType, Screenplay};
use iced::keyboard::Key;
use iced::keyboard::key::Named;
use iced::widget::text_editor;
use iced::{Event, Subscription, Task, keyboard, time};
use std::time::Duration;

pub struct App {
    pub screenplay: Screenplay,
    pub content: text_editor::Content,
    pub current_element_type: ElementType,
    pub cursor_position: usize,
    pub cursor_offset: usize,
    pub cursor_visible: bool,
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
        }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    EditorAction(text_editor::Action),
    TabPressed,
    EnterPressed,
    CharacterTyped(char),
    BackspacePressed,
    EventOccurred(Event),
    Tick,
}

impl App {
    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Tick => {
                self.cursor_visible = !self.cursor_visible;
            }
            Message::EditorAction(action) => {
                self.content.perform(action);
            }
            Message::TabPressed => {
                self.current_element_type = self.next_element_type();

                if let Some(element) = self.screenplay.elements.get_mut(self.cursor_position) {
                    element.element_type = self.current_element_type;
                }
            }
            Message::CharacterTyped(c) => {
                if self.cursor_position >= self.screenplay.elements.len() {
                    let element = Element::new(self.current_element_type, String::new());
                    self.screenplay.add_element(element);
                }

                if let Some(element) = self.screenplay.elements.get_mut(self.cursor_position) {
                    element.content.insert(self.cursor_offset, c);
                    self.cursor_offset += 1;
                }
            }
            Message::BackspacePressed => {
                if self.cursor_offset > 0 {
                    if let Some(element) = self.screenplay.elements.get_mut(self.cursor_position) {
                        self.cursor_offset -= 1;
                        element.content.remove(self.cursor_offset);
                    }
                }
            }
            Message::EnterPressed => {
                self.cursor_position += 1;
                self.cursor_offset = 0;

                if let Some(current_element) =
                    self.screenplay.elements.get(self.cursor_position - 1)
                {
                    self.current_element_type =
                        self.detect_next_element_type(&current_element.content);
                }

                let element = Element::new(self.current_element_type, String::new());
                self.screenplay.add_element(element);
            }
            Message::EventOccurred(event) => {
                if let Event::Keyboard(keyboard::Event::KeyPressed { key, text, .. }) = event {
                    match key {
                        Key::Named(Named::Tab) => {
                            return Task::done(Message::TabPressed);
                        }
                        Key::Named(Named::Enter) => {
                            return Task::done(Message::EnterPressed);
                        }
                        Key::Named(Named::Backspace) => {
                            return Task::done(Message::BackspacePressed);
                        }
                        Key::Named(Named::Space) => {
                            return Task::done(Message::CharacterTyped(' '));
                        }
                        _ => {}
                    }

                    if let Some(text) = text {
                        for ch in text.chars() {
                            return Task::done(Message::CharacterTyped(ch));
                        }
                    }

                    if let Key::Character(ref smol_str) = key {
                        if let Some(c) = smol_str.chars().next() {
                            return Task::done(Message::CharacterTyped(c));
                        }
                    }
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

    fn next_element_type(&self) -> ElementType {
        match self.current_element_type {
            ElementType::Action => ElementType::SceneHeading,
            ElementType::SceneHeading => ElementType::Character,
            ElementType::Character => ElementType::Dialogue,
            ElementType::Dialogue => ElementType::Parenthetical,
            ElementType::Parenthetical => ElementType::Transition,
            ElementType::Transition => ElementType::Action,
        }
    }

    fn detect_next_element_type(&self, text: &str) -> ElementType {
        match self.current_element_type {
            ElementType::SceneHeading => ElementType::Action,
            ElementType::Character => ElementType::Dialogue,
            ElementType::Dialogue => ElementType::Action,
            ElementType::Parenthetical => ElementType::Dialogue,
            ElementType::Action => {
                if text
                    .chars()
                    .filter(|c| c.is_alphabetic())
                    .all(|c| c.is_uppercase())
                    && !text.starts_with("INT.")
                    && !text.starts_with("EXT.")
                {
                    ElementType::Character
                } else {
                    ElementType::Action
                }
            }
            ElementType::Transition => ElementType::SceneHeading,
        }
    }
}
