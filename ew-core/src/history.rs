use crate::edit::Edit;

#[derive(Debug, Clone, Default)]
pub struct History {
    undo_stack: Vec<Vec<Edit>>,
    redo_stack: Vec<Vec<Edit>>,
}

impl History {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn record(&mut self, edits: Vec<Edit>) {
        if !edits.is_empty() {
            self.undo_stack.push(edits);
            self.redo_stack.clear();
        }
    }

    pub fn undo(&mut self) -> Option<Vec<Edit>> {
        self.undo_stack.pop().map(|transaction| {
            let inverse = Self::invert_transaction(&transaction);

            self.redo_stack.push(transaction);
            inverse
        })
    }

    pub fn redo(&mut self) -> Option<Vec<Edit>> {
        self.redo_stack.pop().map(|transaction| {
            self.undo_stack.push(transaction.clone());
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

    fn invert_transaction(transaction: &[Edit]) -> Vec<Edit> {
        transaction.iter().rev().map(Edit::inverse).collect()
    }
}
