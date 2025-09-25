use crate::edit::Edit;

#[derive(Debug, Clone)]
pub struct HistoryEntry {
    pub edits: Vec<Edit>,
    pub deleted_texts: Vec<String>,
    pub timestamp: std::time::SystemTime,
}

#[derive(Debug, Clone)]
pub struct History {
    undo_stack: Vec<HistoryEntry>,
    redo_stack: Vec<HistoryEntry>,
}

impl History {
    pub fn new() -> Self {
        Self {
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
        }
    }

    pub fn record(&mut self, edits: Vec<Edit>, deleted_texts: Vec<String>) {
        if self.should_ignore_edits(&edits) {
            return;
        }

        let entry = HistoryEntry {
            edits,
            deleted_texts,
            timestamp: std::time::SystemTime::now(),
        };

        self.undo_stack.push(entry);
        self.redo_stack.clear(); // New action invalidates redo history
    }

    pub fn undo(&mut self) -> Option<Vec<Edit>> {
        let entry = self.undo_stack.pop()?;
        let inverse_edits = self.create_inverse_edits(&entry);

        self.redo_stack.push(entry);
        Some(inverse_edits)
    }

    pub fn redo(&mut self) -> Option<Vec<Edit>> {
        let entry = self.redo_stack.pop()?;
        let edits_to_replay = entry.edits.clone();

        self.undo_stack.push(entry);
        Some(edits_to_replay)
    }

    pub fn can_undo(&self) -> bool {
        !self.undo_stack.is_empty()
    }

    pub fn can_redo(&self) -> bool {
        !self.redo_stack.is_empty()
    }

    pub fn clear(&mut self) {
        self.undo_stack.clear();
        self.redo_stack.clear();
    }

    fn should_ignore_edits(&self, edits: &[Edit]) -> bool {
        edits.is_empty() || edits.iter().all(|edit| edit.is_noop())
    }

    fn create_inverse_edits(&self, entry: &HistoryEntry) -> Vec<Edit> {
        entry
            .edits
            .iter()
            .zip(entry.deleted_texts.iter())
            .rev() // Apply inverses in reverse order
            .map(|(edit, deleted_text)| self.invert_edit(edit, deleted_text))
            .collect()
    }

    fn invert_insertion(&self, edit: &Edit) -> Edit {
        Edit::delete(edit.start, edit.start + edit.text.len())
    }

    fn invert_deletion(&self, edit: &Edit, deleted_text: &str) -> Edit {
        Edit::insert(edit.start, deleted_text)
    }

    fn invert_replacement(&self, edit: &Edit, deleted_text: &str) -> Edit {
        Edit::replace(edit.start, edit.start + edit.text.len(), deleted_text)
    }

    fn invert_noop(&self, edit: &Edit) -> Edit {
        Edit::insert(edit.start, String::new())
    }

    fn invert_edit(&self, edit: &Edit, deleted_text: &str) -> Edit {
        match edit {
            _ if edit.is_insert() => self.invert_insertion(edit),
            _ if edit.is_delete() => self.invert_deletion(edit, deleted_text),
            _ if edit.is_replace() => self.invert_replacement(edit, deleted_text),
            _ if edit.is_noop() => self.invert_noop(edit),
            _ => unreachable!("Invalid edit type"),
        }
    }
}

impl Default for History {
    fn default() -> Self {
        Self::new()
    }
}
