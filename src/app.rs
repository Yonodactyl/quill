use crate::document::{Element, ElementType, Screenplay};
use iced::widget::text_editor;
use iced::{keyboard, Event, Subscription, Task};
use iced::keyboard::key::Named;

pub struct App {
    pub screenplay: Screenplay,
    pub content: text_editor::Content,
    pub current_element_type: ElementType,
    pub current_line: String,
}

impl Default for App {
    fn default() -> Self {
        Self {
            screenplay: Screenplay::new("Untitled".to_string()),
            content: text_editor::Content::new(),
            current_element_type: ElementType::Action,
            current_line: String::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    EditorAction(text_editor::Action),
    TabPressed,
    EnterPressed,
    EventOccurred(Event),
    TextChanged(String),
}

impl App {
    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::EditorAction(action) => {
                self.content.perform(action);
            }
            Message::TabPressed => {
                self.current_element_type = self.next_element_type();
            }
            Message::TextChanged(text) => {
                self.current_line = text;
            }
            Message::EnterPressed => {
                if !self.current_line.trim().is_empty() {
                    let element = Element::new(
                        self.current_element_type,
                        self.current_line.clone()
                    );
                    
                    self.screenplay.add_element(element);
                    self.current_element_type = self.detect_next_element_type(&self.current_line);
                    self.current_line.clear();
                }
            }
            Message::EventOccurred(event) => {
                match event {
                    Event::Keyboard(keyboard::Event::KeyPressed {
                        key: keyboard::Key::Named(Named::Tab),
                        modifiers: _,
                        ..
                    }) => {
                        return Task::done(Message::TabPressed);
                    }
                    _ => {}
                }
            }
        }
        Task::none()
    }

    pub fn subscription(&self) -> Subscription<Message> {
        iced::event::listen().map(Message::EventOccurred)
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
                if text.chars().filter(|c| c.is_alphabetic()).all(|c| c.is_uppercase()) 
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