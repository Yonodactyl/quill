use super::Element;
use crate::error::Result;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Screenplay {
    pub title: String,
    pub author: Option<String>,
    pub contact: Option<String>,
    pub elements: Vec<Element>,

    #[serde(skip)]
    pub file_path: Option<PathBuf>,

    #[serde(skip)]
    pub modified: bool,
}

impl Screenplay {
    pub fn new(title: String) -> Self {
        Self {
            title,
            author: None,
            contact: None,
            elements: Vec::new(),
            file_path: None,
            modified: false,
        }
    }

    pub fn empty() -> Self {
        Self::new("Untitled".to_string())
    }

    pub fn element_count(&self) -> usize {
        self.elements.len()
    }

    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }

    pub fn add_element(&mut self, element: Element) {
        self.elements.push(element);
        self.mark_modified();
    }

    pub fn insert_element(&mut self, index: usize, element: Element) {
        if index <= self.elements.len() {
            self.elements.insert(index, element);
            self.mark_modified();
        }
    }

    pub fn remove_element(&mut self, index: usize) -> Option<Element> {
        if index < self.elements.len() {
            self.mark_modified();
            Some(self.elements.remove(index))
        } else {
            None
        }
    }

    pub fn get_element(&self, index: usize) -> Option<&Element> {
        self.elements.get(index)
    }

    pub fn get_element_mut(&mut self, index: usize) -> Option<&mut Element> {
        if index < self.elements.len() {
            self.mark_modified();
        }
        self.elements.get_mut(index)
    }

    pub fn mark_modified(&mut self) {
        self.modified = true;
    }

    pub fn mark_saved(&mut self) {
        self.modified = false;
    }

    pub fn display_name(&self) -> String {
        self.file_path
            .as_ref()
            .and_then(|p| p.file_name())
            .and_then(|n| n.to_str())
            .map(|s| s.to_string())
            .unwrap_or_else(|| self.title.clone())
    }

    pub fn to_json(&self) -> Result<String> {
        Ok(serde_json::to_string_pretty(self)?)
    }

    pub fn from_json(json: &str) -> Result<Self> {
        Ok(serde_json::from_str(json)?)
    }
}

impl Default for Screenplay {
    fn default() -> Self {
        Self::empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_screenplay_creation() {
        let screenplay = Screenplay::new("Test".to_string());
        assert_eq!(screenplay.title, "Test");
        assert!(screenplay.is_empty());
    }
}