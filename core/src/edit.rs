#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Edit {
    pub start: usize,
    pub end: usize,
    pub text: String,
}

impl Edit {
    pub fn insert(position: usize, text: impl Into<String>) -> Self {
        Self {
            start: position,
            end: position,
            text: text.into(),
        }
    }

    pub fn delete(start: usize, end: usize) -> Self {
        Self {
            start,
            end,
            text: String::new(),
        }
    }

    pub fn replace(start: usize, end: usize, text: impl Into<String>) -> Self {
        Self {
            start,
            end,
            text: text.into(),
        }
    }

    pub fn is_insert(&self) -> bool {
        self.start == self.end && !self.text.is_empty()
    }

    pub fn is_delete(&self) -> bool {
        self.start != self.end && self.text.is_empty()
    }

    pub fn is_replace(&self) -> bool {
        self.start != self.end && !self.text.is_empty()
    }

    pub fn is_noop(&self) -> bool {
        self.start == self.end && self.text.is_empty()
    }
}
