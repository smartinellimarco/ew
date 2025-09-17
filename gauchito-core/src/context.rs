use crate::buffer::Buffer;
use crate::edit::Edit;
use crate::history::History;
use crate::selection::Selection;

#[derive(Debug)]
pub struct Context {
    buffer: Buffer,
    selection: Selection,
    history: History,
    ast: 
}

impl Context {
    pub fn new() -> Self {
        Self {
            buffer: Buffer::new(),
            selection: Selection::new(0, 0),
            history: History::new(),
        }
    }

    pub fn with_content(content: &str) -> Self {
        Self {
            buffer: Buffer::from_str(content),
            selection: Selection::new(0, 0),
            history: History::new(),
        }
    }

    pub fn buffer(&self) -> &Buffer {
        &self.buffer
    }

    pub fn buffer_mut(&mut self) -> &mut Buffer {
        &mut self.buffer
    }

    pub fn selection(&self) -> &Selection {
        &self.selection
    }

    pub fn selection_mut(&mut self) -> &mut Selection {
        &mut self.selection
    }

    pub fn history_mut(&mut self) -> &mut History {
        &mut self.history
    }

    /// Apply edits with proper history tracking and cursor positioning
    pub fn apply_edits(&mut self, edits: Vec<Edit>) {
        if edits.is_empty() {
            return;
        }

        // Collect the text that will be deleted/replaced for proper undo
        let edits_with_context: Vec<(Edit, String)> = edits
            .into_iter()
            .map(|edit| {
                let deleted_text = if edit.start != edit.end {
                    self.buffer
                        .content()
                        .slice(edit.start..edit.end)
                        .to_string()
                } else {
                    String::new()
                };
                (edit, deleted_text)
            })
            .collect();

        // Extract just the edits for application
        let just_edits: Vec<Edit> = edits_with_context
            .iter()
            .map(|(edit, _)| edit.clone())
            .collect();

        // Determine the new cursor position after the edits
        let new_cursor_pos = self.calculate_cursor_position_after_edits(&just_edits);

        // Apply the edits to the buffer
        self.buffer.apply(&just_edits);

        // Record the edits with context for proper undo
        self.history.record_with_context(edits_with_context);

        // Update cursor position
        self.selection.cursor_to(new_cursor_pos);
    }

    /// Apply edits without history tracking (used internally by undo/redo)
    pub fn apply_edits_no_history(&mut self, edits: &[Edit]) {
        if !edits.is_empty() {
            self.buffer.apply(edits);

            // Update cursor position
            let new_cursor_pos = self.calculate_cursor_position_after_edits(edits);
            self.selection.cursor_to(new_cursor_pos);
        }
    }

    /// Calculate where the cursor should be positioned after applying edits
    fn calculate_cursor_position_after_edits(&self, edits: &[Edit]) -> usize {
        if edits.is_empty() {
            return self.selection.head;
        }

        // For now, position cursor at the end of the last edit
        // This could be made more sophisticated based on the type of operation
        let last_edit = edits.last().unwrap();

        if last_edit.text.is_empty() {
            // Deletion - cursor goes to start position
            last_edit.start
        } else {
            // Insertion or replacement - cursor goes to end of inserted text
            last_edit.start + last_edit.text.len()
        }
    }

    /// Get the current line number (1-based)
    pub fn current_line(&self) -> usize {
        self.buffer.content().char_to_line(self.selection.head) + 1
    }

    /// Get the current column number (1-based)
    pub fn current_column(&self) -> usize {
        let line_start = self
            .buffer
            .content()
            .line_to_char(self.buffer.content().char_to_line(self.selection.head));
        self.selection.head - line_start + 1
    }

    /// Get statistics about the buffer
    pub fn buffer_stats(&self) -> BufferStats {
        let content = self.buffer.content();
        let total_chars = content.len_chars();
        let total_lines = content.len_lines();
        let (sel_start, sel_end) = self.selection.range();
        let selected_chars = if sel_start != sel_end {
            sel_end - sel_start
        } else {
            0
        };

        BufferStats {
            total_chars,
            total_lines,
            selected_chars,
            current_line: self.current_line(),
            current_column: self.current_column(),
            is_modified: self.buffer.is_modified(),
        }
    }
}

impl Default for Context {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub struct BufferStats {
    pub total_chars: usize,
    pub total_lines: usize,
    pub selected_chars: usize,
    pub current_line: usize,
    pub current_column: usize,
    pub is_modified: bool,
}
