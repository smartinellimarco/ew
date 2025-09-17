#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Edit {
    pub start: usize,
    pub end: usize,
    pub text: String,
}

impl Edit {
    /// Creates an insertion edit
    pub fn insert(position: usize, text: String) -> Self {
        Self {
            start: position,
            end: position,
            text,
        }
    }

    /// Creates a deletion edit
    pub fn delete(start: usize, end: usize) -> Self {
        Self {
            start,
            end,
            text: String::new(),
        }
    }

    /// Creates a replacement edit
    pub fn replace(start: usize, end: usize, text: String) -> Self {
        Self { start, end, text }
    }

    /// Returns the inverse of this edit (for undo functionality)
    pub fn inverse(&self) -> Self {
        if self.start == self.end {
            // This was an insertion, so the inverse is a deletion
            Self::delete(self.start, self.start + self.text.len())
        } else if self.text.is_empty() {
            // This was a deletion, so we need to get the deleted text
            // Note: This assumes we have the deleted text stored somewhere
            // In practice, this should be handled by the history system
            Self::insert(self.start, String::new()) // Placeholder
        } else {
            // This was a replacement
            Self::replace(self.start, self.start + self.text.len(), String::new())
        }
    }

    /// Returns the position after applying this edit
    pub fn position(&self) -> usize {
        self.start
    }

    /// Returns the inserted text
    pub fn inserted(&self) -> &str {
        &self.text
    }
}
