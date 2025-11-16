use crate::document::{Element, ElementType, Screenplay};

pub struct CursorState {
    pub position: usize,
    pub offset: usize,
    pub desired_offset: usize,
    pub element_type: ElementType,
}

impl CursorState {
    pub fn move_left(&mut self) {
        if self.offset > 0 {
            self.offset -= 1;
            self.desired_offset = self.offset;
        }
    }

    pub fn move_right(&mut self, screenplay: &Screenplay) {
        if let Some(element) = screenplay.elements.get(self.position) {
            if self.offset < element.content.len() {
                self.offset += 1;
                self.desired_offset = self.offset;
            } else if self.position < screenplay.elements.len() - 1 {
                self.position += 1;
                self.offset = 0;
                self.desired_offset = 0;
                if let Some(next_element) = screenplay.elements.get(self.position) {
                    self.element_type = next_element.element_type;
                }
            }
        }
    }

    pub fn move_up(&mut self, screenplay: &Screenplay) {
        if self.position > 0 {
            self.position -= 1;
            if let Some(element) = screenplay.elements.get(self.position) {
                self.offset = self.desired_offset.min(element.content.len());
                self.element_type = element.element_type;
            }
        }
    }

    pub fn move_down(&mut self, screenplay: &Screenplay) {
        if self.position < screenplay.elements.len() - 1 {
            self.position += 1;
            if let Some(element) = screenplay.elements.get(self.position) {
                self.offset = self.desired_offset.min(element.content.len());
                self.element_type = element.element_type;
            }
        }
    }

    pub fn insert_char(&mut self, screenplay: &mut Screenplay, c: char) {
        if self.position >= screenplay.elements.len() {
            let element = Element::new(self.element_type, String::new());
            screenplay.add_element(element);
        }

        if let Some(element) = screenplay.elements.get_mut(self.position) {
            element.content.insert(self.offset, c);
            self.offset += 1;
            self.desired_offset = self.offset;
        }
    }

    pub fn backspace(&mut self, screenplay: &mut Screenplay) {
        if self.offset > 0 {
            if let Some(element) = screenplay.elements.get_mut(self.position) {
                self.offset -= 1;
                element.content.remove(self.offset);
                self.desired_offset = self.offset;
            }
        } else if self.position > 0 {
            let current_is_empty = screenplay.elements
                .get(self.position)
                .map(|e| e.content.is_empty())
                .unwrap_or(false);

            if current_is_empty {
                screenplay.elements.remove(self.position);
                self.position -= 1;
                if let Some(element) = screenplay.elements.get(self.position) {
                    self.offset = element.content.len();
                    self.element_type = element.element_type;
                    self.desired_offset = self.offset;
                }
            } else {
                self.position -= 1;
                if let Some(element) = screenplay.elements.get(self.position) {
                    self.offset = element.content.len();
                    self.element_type = element.element_type;
                    self.desired_offset = self.offset;
                }
            }
        }
    }

    pub fn delete(&mut self, screenplay: &mut Screenplay) {
        if let Some(element) = screenplay.elements.get_mut(self.position) {
            if self.offset < element.content.len() {
                element.content.remove(self.offset);
            }
        }
    }
}
