use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ElementType {
    SceneHeading,
    Action,
    Character,
    Dialogue,
    Parenthetical,
    Transition,
}

impl ElementType {
    pub fn is_uppercase(&self) -> bool {
        matches!(
            self,
            ElementType::SceneHeading | ElementType::Character | ElementType::Transition
        )
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            ElementType::SceneHeading => "Scene Heading",
            ElementType::Action => "Action",
            ElementType::Character => "Character",
            ElementType::Dialogue => "Dialogue",
            ElementType::Parenthetical => "Parenthetical",
            ElementType::Transition => "Transition",
        }
    }
}

impl Default for ElementType {
    fn default() -> Self {
        ElementType::Action
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Element {
    pub element_type: ElementType,
    pub content: String,
}

impl Element {
    pub fn new(element_type: ElementType, content: String) -> Self {
        Self {
            element_type,
            content,
        }
    }

    pub fn empty(element_type: ElementType) -> Self {
        Self::new(element_type, String::new())
    }

    pub fn is_empty(&self) -> bool {
        self.content.trim().is_empty()
    }

    pub fn formatted_content(&self) -> String {
        if self.element_type.is_uppercase() {
            self.content.to_uppercase()
        } else {
            self.content.clone()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_element_creation() {
        let element = Element::new(ElementType::Action, "Test".to_string());
        assert_eq!(element.element_type, ElementType::Action);
        assert_eq!(element.content, "Test");
    }

    #[test]
    fn test_uppercase_detection() {
        assert!(ElementType::SceneHeading.is_uppercase());
        assert!(!ElementType::Action.is_uppercase());
    }
}