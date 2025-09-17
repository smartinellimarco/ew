use crate::edit::Edit;

#[derive(Debug, Clone, Default)]
pub struct History {
    undo_stack: Vec<Vec<EditWithContext>>,
    redo_stack: Vec<Vec<Edit>>,
}

#[derive(Debug, Clone)]
struct EditWithContext {
    edit: Edit,
    deleted_text: String,
}

impl History {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn record(&mut self, edits: Vec<Edit>) {
        if !edits.is_empty() {
            // Convert edits to edits with context for proper undo
            let edits_with_context: Vec<EditWithContext> = edits
                .into_iter()
                .map(|edit| {
                    EditWithContext {
                        edit,
                        deleted_text: String::new(), // This should be filled by the caller
                    }
                })
                .collect();

            self.undo_stack.push(edits_with_context);
            self.redo_stack.clear();
        }
    }

    /// Record edits with the text that was deleted/replaced for proper undo
    pub fn record_with_context(&mut self, edits: Vec<(Edit, String)>) {
        if !edits.is_empty() {
            let edits_with_context: Vec<EditWithContext> = edits
                .into_iter()
                .map(|(edit, deleted_text)| EditWithContext { edit, deleted_text })
                .collect();

            self.undo_stack.push(edits_with_context);
            self.redo_stack.clear();
        }
    }

    pub fn undo(&mut self) -> Option<Vec<Edit>> {
        self.undo_stack.pop().map(|transaction| {
            let inverse = Self::invert_transaction(&transaction);

            // Convert back to regular edits for redo stack
            let original_edits: Vec<Edit> = transaction.into_iter().map(|ewc| ewc.edit).collect();
            self.redo_stack.push(original_edits);

            inverse
        })
    }

    pub fn redo(&mut self) -> Option<Vec<Edit>> {
        self.redo_stack.pop().map(|transaction| {
            // Convert to edits with context (we lose the original deleted text here)
            let edits_with_context: Vec<EditWithContext> = transaction
                .iter()
                .map(|edit| {
                    EditWithContext {
                        edit: edit.clone(),
                        deleted_text: String::new(), // This is a limitation - we lose the original context
                    }
                })
                .collect();

            self.undo_stack.push(edits_with_context);
            transaction
        })
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

    fn invert_transaction(transaction: &[EditWithContext]) -> Vec<Edit> {
        transaction
            .iter()
            .rev()
            .map(|ewc| Self::invert_edit_with_context(ewc))
            .collect()
    }

    fn invert_edit_with_context(edit_with_context: &EditWithContext) -> Edit {
        let edit = &edit_with_context.edit;

        if edit.start == edit.end && !edit.text.is_empty() {
            // This was an insertion, so the inverse is a deletion
            Edit::delete(edit.start, edit.start + edit.text.len())
        } else if edit.start != edit.end && edit.text.is_empty() {
            // This was a deletion, so the inverse is an insertion with the deleted text
            Edit::insert(edit.start, edit_with_context.deleted_text.clone())
        } else if edit.start != edit.end && !edit.text.is_empty() {
            // This was a replacement, so the inverse is a replacement with the original text
            Edit::replace(
                edit.start,
                edit.start + edit.text.len(),
                edit_with_context.deleted_text.clone(),
            )
        } else {
            // No-op edit
            Edit::insert(edit.start, String::new())
        }
    }
}

// Extension trait to make working with the history easier
pub trait HistoryExt {
    fn record_simple(&mut self, edits: Vec<Edit>);
}

impl HistoryExt for History {
    fn record_simple(&mut self, edits: Vec<Edit>) {
        self.record(edits);
    }
}
