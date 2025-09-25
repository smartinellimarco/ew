use crate::edit::Edit;

use ropey::{Rope, RopeSlice};
use std::path::PathBuf;

#[derive(Debug)]
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

    pub fn from_str(text: &str) -> Self {
        Self {
            content: Rope::from_str(text),
            path: None,
            modified: false,
        }
    }

    pub fn content(&self) -> &Rope {
        &self.content
    }

    pub fn len_chars(&self) -> usize {
        self.content.len_chars()
    }

    pub fn len_lines(&self) -> usize {
        self.content.len_lines()
    }

    pub fn path(&self) -> Option<&PathBuf> {
        self.path.as_ref()
    }

    pub fn set_path(&mut self, path: Option<PathBuf>) {
        self.path = path;
    }

    pub fn is_modified(&self) -> bool {
        self.modified
    }

    pub fn set_modified(&mut self, modified: bool) {
        self.modified = modified;
    }

    pub fn apply(&mut self, edits: &[Edit]) -> Vec<String> {
        let mut indexed_edits: Vec<_> = edits.iter().enumerate().collect();

        // Sort in reverse order to avoid offset issues
        indexed_edits.sort_by(|(_, a), (_, b)| b.start.cmp(&a.start));

        let mut deleted_texts = vec![String::new(); edits.len()];

        for (original_index, edit) in indexed_edits {
            deleted_texts[original_index] = self.apply_edit(edit);
        }

        deleted_texts
    }

    pub fn apply_edit(&mut self, edit: &Edit) -> String {
        let deleted_text = match edit {
            _ if edit.is_insert() => self.handle_insertion(edit),
            _ if edit.is_delete() => self.handle_deletion(edit),
            _ if edit.is_replace() => self.handle_replacement(edit),
            _ if edit.is_noop() => self.handle_noop(edit),
            _ => unreachable!("Invalid edit type"),
        };

        self.mark_as_modified();

        deleted_text
    }

    fn handle_insertion(&mut self, edit: &Edit) -> String {
        self.content.insert(edit.start, &edit.text);

        String::new() // Nothing was deleted
    }

    fn handle_deletion(&mut self, edit: &Edit) -> String {
        let deleted_text = self.content.slice(edit.start..edit.end).to_string();

        self.content.remove(edit.start..edit.end);

        deleted_text
    }

    fn handle_replacement(&mut self, edit: &Edit) -> String {
        let deleted_text = self.content.slice(edit.start..edit.end).to_string();

        self.content.remove(edit.start..edit.end);
        self.content.insert(edit.start, &edit.text);

        deleted_text
    }

    fn handle_noop(&mut self, _edit: &Edit) -> String {
        String::new()
    }

    fn mark_as_modified(&mut self) {
        self.modified = true;
    }

    pub fn line(&self, line_idx: usize) -> RopeSlice {
        self.content.line(line_idx)
    }

    pub fn char_to_line(&self, char_idx: usize) -> usize {
        self.content.char_to_line(char_idx)
    }

    pub fn line_to_char(&self, line_idx: usize) -> usize {
        self.content.line_to_char(line_idx)
    }

    pub fn char_at(&self, pos: usize) -> Option<char> {
        if pos < self.len_chars() {
            Some(self.content.char(pos))
        } else {
            None
        }
    }
}

impl Default for Buffer {
    fn default() -> Self {
        Self::new()
    }
}
