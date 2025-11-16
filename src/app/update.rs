use super::message::Message;
use crate::document::ElementType;
use iced::keyboard::Key;
use iced::keyboard::key::Named;
use iced::{Event, Task, keyboard};

pub fn handle_event(event: Event) -> Option<Task<Message>> {
    if let Event::Keyboard(keyboard::Event::KeyPressed { key, text, .. }) = event {
        match key {
            Key::Named(Named::Tab) => {
                return Some(Task::done(Message::TabPressed));
            }
            Key::Named(Named::Enter) => {
                return Some(Task::done(Message::EnterPressed));
            }
            Key::Named(Named::Backspace) => {
                return Some(Task::done(Message::BackspacePressed));
            }
            Key::Named(Named::Delete) => {
                return Some(Task::done(Message::DeletePressed));
            }
            Key::Named(Named::ArrowLeft) => {
                return Some(Task::done(Message::ArrowLeft));
            }
            Key::Named(Named::ArrowRight) => {
                return Some(Task::done(Message::ArrowRight));
            }
            Key::Named(Named::ArrowUp) => {
                return Some(Task::done(Message::ArrowUp));
            }
            Key::Named(Named::ArrowDown) => {
                return Some(Task::done(Message::ArrowDown));
            }
            Key::Named(Named::Space) => {
                return Some(Task::done(Message::CharacterTyped(' ')));
            }
            _ => {}
        }

        if let Some(text) = text {
            for ch in text.chars() {
                return Some(Task::done(Message::CharacterTyped(ch)));
            }
        }

        if let Key::Character(ref smol_str) = key {
            if let Some(c) = smol_str.chars().next() {
                return Some(Task::done(Message::CharacterTyped(c)));
            }
        }
    }
    None
}

pub fn next_element_type(current: ElementType) -> ElementType {
    match current {
        ElementType::Action => ElementType::SceneHeading,
        ElementType::SceneHeading => ElementType::Character,
        ElementType::Character => ElementType::Dialogue,
        ElementType::Dialogue => ElementType::Parenthetical,
        ElementType::Parenthetical => ElementType::Transition,
        ElementType::Transition => ElementType::Action,
    }
}

pub fn detect_next_element_type(current: ElementType, text: &str) -> ElementType {
    match current {
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
