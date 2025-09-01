use crate::edit::Edit;
use ropey::Rope;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct Buffer {
    content: Rope,
    path: Option<PathBuf>,
    modified: bool,
}

impl Buffer {
    pub fn new() -> Self {
        Self {
            content: Rope::new(),
            path: None,
            modified: false,
        }
    }

    pub fn apply(&mut self, edits: &[Edit]) {
        for edit in edits {
            if edit.is_noop() {
                continue;
            }

            let end = edit.position + edit.deleted.chars().count();

            self.content.remove(edit.position..end);
            self.content.insert(edit.position, &edit.inserted);
        }

        if !edits.is_empty() {
            self.modified = true;
        }
    }
}

impl Default for Buffer {
    fn default() -> Self {
        Self::new()
    }
}
