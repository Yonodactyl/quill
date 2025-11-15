use crate::document::{Element, ElementType, Screenplay};
use iced::widget::text_editor;
use iced::{keyboard, time, Event, Subscription, Task};
use iced::keyboard::key::Named;
use iced::keyboard::Key;
use std::time::Duration;

pub struct App {
    pub screenplay: Screenplay,
    pub content: text_editor::Content,
    pub current_element_type: ElementType,
    pub cursor_position: usize,
    pub cursor_offset: usize,
}

impl Default for App {
    fn default() -> Self {
        Self {
            screenplay: Screenplay::new("Untitled".to_string()),
            content: text_editor::Content::new(),
            current_element_type: ElementType::Action,
            cursor_position: 0,
            cursor_offset: 0,
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
            Message::Tick => { }
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
                
                if let Some(current_element) = self.screenplay.elements.get(self.cursor_position - 1) {
                    self.current_element_type = self.detect_next_element_type(&current_element.content);
                }
                
                let element = Element::new(self.current_element_type, String::new());
                self.screenplay.add_element(element);
            }
            Message::EventOccurred(event) => {
                if let Event::Keyboard(keyboard::Event::KeyPressed {
                    key,
                    modifiers,
                    ..
                }) = event
                {
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
                        Key::Character(ref smol_str) => {
                            let chars: Vec<char> = smol_str.chars().collect();
                            if let Some(&base_char) = chars.first() {
                                let actual_char = Self::apply_us_qwerty_shift(base_char, modifiers.shift());
                                return Task::done(Message::CharacterTyped(actual_char));
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
        Task::none()
    }

    /// Apply shift modifier for US QWERTY keyboard layout
    /// 
    /// NOTE: This is a workaround for Iced 0.13's limitation where Key::Character
    /// returns the base (unshifted) character even when Shift is pressed.
    /// This function only works correctly for US QWERTY keyboards.
    /// 
    /// TODO: Find a better solution that works with international keyboards,
    /// possibly by using winit's ReceivedCharacter events directly if Iced exposes them.
    fn apply_us_qwerty_shift(base_char: char, shift_pressed: bool) -> char {
        if !shift_pressed {
            return base_char;
        }

        match base_char {
            '1' => '!',
            '2' => '@',
            '3' => '#',
            '4' => '$',
            '5' => '%',
            '6' => '^',
            '7' => '&',
            '8' => '*',
            '9' => '(',
            '0' => ')',

            '-' => '_',
            '=' => '+',
            '[' => '{',
            ']' => '}',
            '\\' => '|',
            ';' => ':',
            '\'' => '"',
            ',' => '<',
            '.' => '>',
            '/' => '?',
            '`' => '~',
            
            c if c.is_ascii_lowercase() => c.to_ascii_uppercase(),
            
            c => c,
        }
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