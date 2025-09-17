use crate::context::Context;
use crate::edit::Edit;
use crate::text_objects;

// grapheme as basic textobj
// moves always uses grapheme???
#[derive(Debug, Clone)]
pub enum OperationResult {
    Continue,
    SwitchMode(String),
    Exit,
}

pub trait Operation: std::fmt::Debug {
    fn execute(&self, ctx: &mut Context) -> OperationResult;
    fn name(&self) -> &'static str;
}

// ==== CURSOR MOVEMENT OPERATIONS ====

#[derive(Debug, Clone)]
pub struct MoveLeft;
impl Operation for MoveLeft {
    fn execute(&self, ctx: &mut Context) -> OperationResult {
        let new_head =
            text_objects::prev_grapheme_char_index(ctx.buffer().content(), ctx.selection().head);
        ctx.selection_mut().cursor_to(new_head);
        OperationResult::Continue
    }
    fn name(&self) -> &'static str {
        "move_left"
    }
}

#[derive(Debug, Clone)]
pub struct MoveRight;
impl Operation for MoveRight {
    fn execute(&self, ctx: &mut Context) -> OperationResult {
        let new_head =
            text_objects::next_grapheme_char_index(ctx.buffer().content(), ctx.selection().head);
        ctx.selection_mut().cursor_to(new_head);
        OperationResult::Continue
    }
    fn name(&self) -> &'static str {
        "move_right"
    }
}

#[derive(Debug, Clone)]
pub struct MoveUp;
impl Operation for MoveUp {
    fn execute(&self, ctx: &mut Context) -> OperationResult {
        let buffer = ctx.buffer();
        let head = ctx.selection().head;
        let line_idx = buffer.char_to_line(head);

        if line_idx > 0 {
            let line_start = buffer.line_to_char(line_idx);
            let col = head - line_start;
            let prev_line_start = buffer.line_to_char(line_idx - 1);
            let prev_line_len = buffer.line(line_idx - 1).len_chars().saturating_sub(1); // exclude newline
            let new_head = prev_line_start + std::cmp::min(col, prev_line_len);
            ctx.selection_mut().cursor_to(new_head);
        }
        OperationResult::Continue
    }
    fn name(&self) -> &'static str {
        "move_up"
    }
}

#[derive(Debug, Clone)]
pub struct MoveDown;
impl Operation for MoveDown {
    fn execute(&self, ctx: &mut Context) -> OperationResult {
        let buffer = ctx.buffer();
        let head = ctx.selection().head;
        let line_idx = buffer.char_to_line(head);

        if line_idx + 1 < buffer.len_lines() {
            let line_start = buffer.line_to_char(line_idx);
            let col = head - line_start;
            let next_line_start = buffer.line_to_char(line_idx + 1);
            let next_line_len = buffer.line(line_idx + 1).len_chars().saturating_sub(1); // exclude newline
            let new_head = next_line_start + std::cmp::min(col, next_line_len);
            ctx.selection_mut().cursor_to(new_head);
        }
        OperationResult::Continue
    }
    fn name(&self) -> &'static str {
        "move_down"
    }
}

#[derive(Debug, Clone)]
pub struct MoveLineStart;
impl Operation for MoveLineStart {
    fn execute(&self, ctx: &mut Context) -> OperationResult {
        let buffer = ctx.buffer();
        let head = ctx.selection().head;
        let line_idx = buffer.char_to_line(head);
        let line_start = buffer.line_to_char(line_idx);
        ctx.selection_mut().cursor_to(line_start);
        OperationResult::Continue
    }
    fn name(&self) -> &'static str {
        "move_line_start"
    }
}

#[derive(Debug, Clone)]
pub struct MoveLineEnd;
impl Operation for MoveLineEnd {
    fn execute(&self, ctx: &mut Context) -> OperationResult {
        let buffer = ctx.buffer();
        let head = ctx.selection().head;
        let line_idx = buffer.char_to_line(head);
        let line_start = buffer.line_to_char(line_idx);
        let line_len = buffer.line(line_idx).len_chars().saturating_sub(1); // exclude newline
        let line_end = line_start + line_len;
        ctx.selection_mut().cursor_to(line_end);
        OperationResult::Continue
    }
    fn name(&self) -> &'static str {
        "move_line_end"
    }
}

#[derive(Debug, Clone)]
pub struct MoveWordForward;
impl Operation for MoveWordForward {
    fn execute(&self, ctx: &mut Context) -> OperationResult {
        let buffer = ctx.buffer();
        let new_head = text_objects::word_end_index(&buffer.slice(..), ctx.selection().head);
        ctx.selection_mut().cursor_to(new_head);
        OperationResult::Continue
    }
    fn name(&self) -> &'static str {
        "move_word_forward"
    }
}

#[derive(Debug, Clone)]
pub struct MoveWordBackward;
impl Operation for MoveWordBackward {
    fn execute(&self, ctx: &mut Context) -> OperationResult {
        let buffer = ctx.buffer();
        let new_head = text_objects::word_start_index(&buffer.slice(..), ctx.selection().head);
        ctx.selection_mut().cursor_to(new_head);
        OperationResult::Continue
    }
    fn name(&self) -> &'static str {
        "move_word_backward"
    }
}

#[derive(Debug, Clone)]
pub struct MoveBigWordForward;
impl Operation for MoveBigWordForward {
    fn execute(&self, ctx: &mut Context) -> OperationResult {
        let buffer = ctx.buffer();
        let new_head = text_objects::big_word_end_index(&buffer.slice(..), ctx.selection().head);
        ctx.selection_mut().cursor_to(new_head);
        OperationResult::Continue
    }
    fn name(&self) -> &'static str {
        "move_big_word_forward"
    }
}

#[derive(Debug, Clone)]
pub struct MoveBigWordBackward;
impl Operation for MoveBigWordBackward {
    fn execute(&self, ctx: &mut Context) -> OperationResult {
        let buffer = ctx.buffer();
        let new_head = text_objects::big_word_start_index(&buffer.slice(..), ctx.selection().head);
        ctx.selection_mut().cursor_to(new_head);
        OperationResult::Continue
    }
    fn name(&self) -> &'static str {
        "move_big_word_backward"
    }
}

#[derive(Debug, Clone)]
pub struct MoveDocumentStart;
impl Operation for MoveDocumentStart {
    fn execute(&self, ctx: &mut Context) -> OperationResult {
        ctx.selection_mut().cursor_to(0);
        OperationResult::Continue
    }
    fn name(&self) -> &'static str {
        "move_document_start"
    }
}

#[derive(Debug, Clone)]
pub struct MoveDocumentEnd;
impl Operation for MoveDocumentEnd {
    fn execute(&self, ctx: &mut Context) -> OperationResult {
        let len = ctx.buffer().len_chars();
        ctx.selection_mut().cursor_to(len);
        OperationResult::Continue
    }
    fn name(&self) -> &'static str {
        "move_document_end"
    }
}

#[derive(Debug, Clone)]
pub struct MoveMatchingBracket;
impl Operation for MoveMatchingBracket {
    fn execute(&self, ctx: &mut Context) -> OperationResult {
        let buffer = ctx.buffer();
        if let Some(matching_pos) =
            text_objects::find_matching_bracket(&buffer.slice(..), ctx.selection().head)
        {
            ctx.selection_mut().cursor_to(matching_pos);
        }
        OperationResult::Continue
    }
    fn name(&self) -> &'static str {
        "move_matching_bracket"
    }
}

#[derive(Debug, Clone)]
pub struct MoveParagraphForward;
impl Operation for MoveParagraphForward {
    fn execute(&self, ctx: &mut Context) -> OperationResult {
        let buffer = ctx.buffer();
        let new_head = text_objects::paragraph_end_index(&buffer.slice(..), ctx.selection().head);
        ctx.selection_mut().cursor_to(new_head);
        OperationResult::Continue
    }
    fn name(&self) -> &'static str {
        "move_paragraph_forward"
    }
}

#[derive(Debug, Clone)]
pub struct MoveParagraphBackward;
impl Operation for MoveParagraphBackward {
    fn execute(&self, ctx: &mut Context) -> OperationResult {
        let buffer = ctx.buffer();
        let new_head = text_objects::paragraph_start_index(&buffer.slice(..), ctx.selection().head);
        ctx.selection_mut().cursor_to(new_head);
        OperationResult::Continue
    }
    fn name(&self) -> &'static str {
        "move_paragraph_backward"
    }
}

// ==== SELECTION OPERATIONS ====

#[derive(Debug, Clone)]
pub struct SelectLeft;
impl Operation for SelectLeft {
    fn execute(&self, ctx: &mut Context) -> OperationResult {
        let new_head =
            text_objects::prev_grapheme_char_index(ctx.buffer().content(), ctx.selection().head);
        let anchor = ctx.selection().anchor;
        ctx.selection_mut().set_range(anchor, new_head);
        OperationResult::Continue
    }
    fn name(&self) -> &'static str {
        "select_left"
    }
}

#[derive(Debug, Clone)]
pub struct SelectRight;
impl Operation for SelectRight {
    fn execute(&self, ctx: &mut Context) -> OperationResult {
        let new_head =
            text_objects::next_grapheme_char_index(ctx.buffer().content(), ctx.selection().head);
        let anchor = ctx.selection().anchor;
        ctx.selection_mut().set_range(anchor, new_head);
        OperationResult::Continue
    }
    fn name(&self) -> &'static str {
        "select_right"
    }
}

#[derive(Debug, Clone)]
pub struct SelectUp;
impl Operation for SelectUp {
    fn execute(&self, ctx: &mut Context) -> OperationResult {
        let buffer = ctx.buffer();
        let head = ctx.selection().head;
        let line_idx = buffer.char_to_line(head);

        if line_idx > 0 {
            let line_start = buffer.line_to_char(line_idx);
            let col = head - line_start;
            let prev_line_start = buffer.line_to_char(line_idx - 1);
            let prev_line_len = buffer.line(line_idx - 1).len_chars().saturating_sub(1);
            let new_head = prev_line_start + std::cmp::min(col, prev_line_len);
            let anchor = ctx.selection().anchor;
            ctx.selection_mut().set_range(anchor, new_head);
        }
        OperationResult::Continue
    }
    fn name(&self) -> &'static str {
        "select_up"
    }
}

#[derive(Debug, Clone)]
pub struct SelectDown;
impl Operation for SelectDown {
    fn execute(&self, ctx: &mut Context) -> OperationResult {
        let buffer = ctx.buffer();
        let head = ctx.selection().head;
        let line_idx = buffer.char_to_line(head);

        if line_idx + 1 < buffer.len_lines() {
            let line_start = buffer.line_to_char(line_idx);
            let col = head - line_start;
            let next_line_start = buffer.line_to_char(line_idx + 1);
            let next_line_len = buffer.line(line_idx + 1).len_chars().saturating_sub(1);
            let new_head = next_line_start + std::cmp::min(col, next_line_len);
            let anchor = ctx.selection().anchor;
            ctx.selection_mut().set_range(anchor, new_head);
        }
        OperationResult::Continue
    }
    fn name(&self) -> &'static str {
        "select_down"
    }
}

#[derive(Debug, Clone)]
pub struct SelectWord;
impl Operation for SelectWord {
    fn execute(&self, ctx: &mut Context) -> OperationResult {
        let buffer = ctx.buffer();
        let slice = buffer.slice(..);
        let start = text_objects::word_start_index(&slice, ctx.selection().head);
        let end = text_objects::word_end_index(&slice, ctx.selection().head);
        ctx.selection_mut().set_range(start, end);
        OperationResult::Continue
    }
    fn name(&self) -> &'static str {
        "select_word"
    }
}

#[derive(Debug, Clone)]
pub struct SelectLine;
impl Operation for SelectLine {
    fn execute(&self, ctx: &mut Context) -> OperationResult {
        let buffer = ctx.buffer();
        let head = ctx.selection().head;
        let line_idx = buffer.char_to_line(head);
        let line_start = buffer.line_to_char(line_idx);
        let line_end = if line_idx + 1 < buffer.len_lines() {
            buffer.line_to_char(line_idx + 1)
        } else {
            buffer.len_chars()
        };
        ctx.selection_mut().set_range(line_start, line_end);
        OperationResult::Continue
    }
    fn name(&self) -> &'static str {
        "select_line"
    }
}

#[derive(Debug, Clone)]
pub struct SelectAll;
impl Operation for SelectAll {
    fn execute(&self, ctx: &mut Context) -> OperationResult {
        let len = ctx.buffer().len_chars();
        ctx.selection_mut().set_range(0, len);
        OperationResult::Continue
    }
    fn name(&self) -> &'static str {
        "select_all"
    }
}

#[derive(Debug, Clone)]
pub struct SelectLineStart;
impl Operation for SelectLineStart {
    fn execute(&self, ctx: &mut Context) -> OperationResult {
        let buffer = ctx.buffer();
        let head = ctx.selection().head;
        let line_idx = buffer.char_to_line(head);
        let line_start = buffer.line_to_char(line_idx);
        let anchor = ctx.selection().anchor;
        ctx.selection_mut().set_range(anchor, line_start);
        OperationResult::Continue
    }
    fn name(&self) -> &'static str {
        "select_line_start"
    }
}

#[derive(Debug, Clone)]
pub struct SelectLineEnd;
impl Operation for SelectLineEnd {
    fn execute(&self, ctx: &mut Context) -> OperationResult {
        let buffer = ctx.buffer();
        let head = ctx.selection().head;
        let line_idx = buffer.char_to_line(head);
        let line_start = buffer.line_to_char(line_idx);
        let line_len = buffer.line(line_idx).len_chars().saturating_sub(1);
        let line_end = line_start + line_len;
        let anchor = ctx.selection().anchor;
        ctx.selection_mut().set_range(anchor, line_end);
        OperationResult::Continue
    }
    fn name(&self) -> &'static str {
        "select_line_end"
    }
}

#[derive(Debug, Clone)]
pub struct ClearSelection;
impl Operation for ClearSelection {
    fn execute(&self, ctx: &mut Context) -> OperationResult {
        let head = ctx.selection().head;
        ctx.selection_mut().cursor_to(head);
        OperationResult::Continue
    }
    fn name(&self) -> &'static str {
        "clear_selection"
    }
}

// ==== TEXT INSERTION AND MODIFICATION ====

#[derive(Debug, Clone)]
pub struct InsertChar {
    pub ch: char,
}
impl InsertChar {
    pub fn new(ch: char) -> Self {
        Self { ch }
    }
}
impl Operation for InsertChar {
    fn execute(&self, ctx: &mut Context) -> OperationResult {
        let (start, end) = ctx.selection().range();
        let edit = Edit::replace(start, end, self.ch.to_string());
        ctx.apply_edits(vec![edit]);
        OperationResult::Continue
    }
    fn name(&self) -> &'static str {
        "insert_char"
    }
}

#[derive(Debug, Clone)]
pub struct InsertString {
    pub text: String,
}
impl InsertString {
    pub fn new(text: String) -> Self {
        Self { text }
    }
}
impl Operation for InsertString {
    fn execute(&self, ctx: &mut Context) -> OperationResult {
        let (start, end) = ctx.selection().range();
        let edit = Edit::replace(start, end, self.text.clone());
        ctx.apply_edits(vec![edit]);
        OperationResult::Continue
    }
    fn name(&self) -> &'static str {
        "insert_string"
    }
}

// TODO: handle eol and tabs for each OS
// maybe make it a textobject
#[derive(Debug, Clone)]
pub struct InsertNewline;
impl Operation for InsertNewline {
    fn execute(&self, ctx: &mut Context) -> OperationResult {
        let (start, end) = ctx.selection().range();
        let edit = Edit::replace(start, end, "\n".to_string());
        ctx.apply_edits(vec![edit]);
        OperationResult::Continue
    }
    fn name(&self) -> &'static str {
        "insert_newline"
    }
}

#[derive(Debug, Clone)]
pub struct InsertTab;
impl Operation for InsertTab {
    fn execute(&self, ctx: &mut Context) -> OperationResult {
        let (start, end) = ctx.selection().range();
        let edit = Edit::replace(start, end, "\t".to_string());
        ctx.apply_edits(vec![edit]);
        OperationResult::Continue
    }
    fn name(&self) -> &'static str {
        "insert_tab"
    }
}

#[derive(Debug, Clone)]
pub struct InsertSpaces {
    pub count: usize,
}
impl InsertSpaces {
    pub fn new(count: usize) -> Self {
        Self { count }
    }
}
impl Operation for InsertSpaces {
    fn execute(&self, ctx: &mut Context) -> OperationResult {
        let (start, end) = ctx.selection().range();
        let spaces = " ".repeat(self.count);
        let edit = Edit::replace(start, end, spaces);
        ctx.apply_edits(vec![edit]);
        OperationResult::Continue
    }
    fn name(&self) -> &'static str {
        "insert_spaces"
    }
}

// ==== DELETION OPERATIONS ====

#[derive(Debug, Clone)]
pub struct DeleteChar;
impl Operation for DeleteChar {
    fn execute(&self, ctx: &mut Context) -> OperationResult {
        let (start, end) = ctx.selection().range();
        if start != end {
            // Delete selection
            let edit = Edit::delete(start, end);
            ctx.apply_edits(vec![edit]);
        } else if start < ctx.buffer().len_chars() {
            // Delete next character
            let grapheme_end =
                text_objects::next_grapheme_char_index(ctx.buffer().content(), start);
            let edit = Edit::delete(start, grapheme_end);
            ctx.apply_edits(vec![edit]);
        }
        OperationResult::Continue
    }
    fn name(&self) -> &'static str {
        "delete_char"
    }
}

#[derive(Debug, Clone)]
pub struct Backspace;
impl Operation for Backspace {
    fn execute(&self, ctx: &mut Context) -> OperationResult {
        let (start, end) = ctx.selection().range();
        if start != end {
            // Delete selection
            let edit = Edit::delete(start, end);
            ctx.apply_edits(vec![edit]);
        } else if start > 0 {
            // Delete previous character
            let grapheme_start =
                text_objects::prev_grapheme_boundary(&ctx.buffer().content().slice(..), start);
            let edit = Edit::delete(grapheme_start, start);
            ctx.apply_edits(vec![edit]);
        }
        OperationResult::Continue
    }
    fn name(&self) -> &'static str {
        "backspace"
    }
}

#[derive(Debug, Clone)]
pub struct DeleteWord;
impl Operation for DeleteWord {
    fn execute(&self, ctx: &mut Context) -> OperationResult {
        let buffer = ctx.buffer();
        let (start, end) = ctx.selection().range();
        if start != end {
            // Delete selection
            let edit = Edit::delete(start, end);
            ctx.apply_edits(vec![edit]);
        } else {
            // Delete word forward
            let word_end = text_objects::word_end_index(&buffer.slice(..), start);
            if start < word_end {
                let edit = Edit::delete(start, word_end);
                ctx.apply_edits(vec![edit]);
            }
        }
        OperationResult::Continue
    }
    fn name(&self) -> &'static str {
        "delete_word"
    }
}

#[derive(Debug, Clone)]
pub struct DeleteWordBackward;
impl Operation for DeleteWordBackward {
    fn execute(&self, ctx: &mut Context) -> OperationResult {
        let buffer = ctx.buffer();
        let (start, end) = ctx.selection().range();
        if start != end {
            // Delete selection
            let edit = Edit::delete(start, end);
            ctx.apply_edits(vec![edit]);
        } else {
            // Delete word backward
            let word_start = text_objects::word_start_index(&buffer.slice(..), start);
            if word_start < start {
                let edit = Edit::delete(word_start, start);
                ctx.apply_edits(vec![edit]);
            }
        }
        OperationResult::Continue
    }
    fn name(&self) -> &'static str {
        "delete_word_backward"
    }
}

#[derive(Debug, Clone)]
pub struct DeleteLine;
impl Operation for DeleteLine {
    fn execute(&self, ctx: &mut Context) -> OperationResult {
        let buffer = ctx.buffer();
        let head = ctx.selection().head;
        let line_idx = buffer.char_to_line(head);
        let start = buffer.line_to_char(line_idx);
        let end = if line_idx + 1 < buffer.len_lines() {
            buffer.line_to_char(line_idx + 1)
        } else {
            buffer.len_chars()
        };

        if start < end {
            let edit = Edit::delete(start, end);
            ctx.apply_edits(vec![edit]);
        }
        OperationResult::Continue
    }
    fn name(&self) -> &'static str {
        "delete_line"
    }
}

#[derive(Debug, Clone)]
pub struct DeleteToLineStart;
impl Operation for DeleteToLineStart {
    fn execute(&self, ctx: &mut Context) -> OperationResult {
        let buffer = ctx.buffer();
        let head = ctx.selection().head;
        let line_idx = buffer.char_to_line(head);
        let line_start = buffer.line_to_char(line_idx);

        if line_start < head {
            let edit = Edit::delete(line_start, head);
            ctx.apply_edits(vec![edit]);
        }
        OperationResult::Continue
    }
    fn name(&self) -> &'static str {
        "delete_to_line_start"
    }
}

#[derive(Debug, Clone)]
pub struct DeleteToLineEnd;
impl Operation for DeleteToLineEnd {
    fn execute(&self, ctx: &mut Context) -> OperationResult {
        let buffer = ctx.buffer();
        let head = ctx.selection().head;
        let line_idx = buffer.char_to_line(head);
        let line_start = buffer.line_to_char(line_idx);
        let line_len = buffer.line(line_idx).len_chars().saturating_sub(1);
        let line_end = line_start + line_len;

        if head < line_end {
            let edit = Edit::delete(head, line_end);
            ctx.apply_edits(vec![edit]);
        }
        OperationResult::Continue
    }
    fn name(&self) -> &'static str {
        "delete_to_line_end"
    }
}

// ==== CLIPBOARD OPERATIONS ====

#[derive(Debug, Clone)]
pub struct Copy;
impl Operation for Copy {
    fn execute(&self, ctx: &mut Context) -> OperationResult {
        let (start, end) = ctx.selection().range();
        if start != end {
            let text = ctx.buffer().content().slice(start..end).to_string();
            // TODO: Implement clipboard integration
            println!("Copied: {}", text);
        }
        OperationResult::Continue
    }
    fn name(&self) -> &'static str {
        "copy"
    }
}

#[derive(Debug, Clone)]
pub struct Cut;
impl Operation for Cut {
    fn execute(&self, ctx: &mut Context) -> OperationResult {
        let (start, end) = ctx.selection().range();
        if start != end {
            let text = ctx.buffer().content().slice(start..end).to_string();
            // TODO: Implement clipboard integration
            println!("Cut: {}", text);
            let edit = Edit::delete(start, end);
            ctx.apply_edits(vec![edit]);
        }
        OperationResult::Continue
    }
    fn name(&self) -> &'static str {
        "cut"
    }
}

#[derive(Debug, Clone)]
pub struct Paste {
    pub text: String,
}
impl Paste {
    pub fn new(text: String) -> Self {
        Self { text }
    }
}
impl Operation for Paste {
    fn execute(&self, ctx: &mut Context) -> OperationResult {
        let (start, end) = ctx.selection().range();
        let edit = Edit::replace(start, end, self.text.clone());
        ctx.apply_edits(vec![edit]);
        OperationResult::Continue
    }
    fn name(&self) -> &'static str {
        "paste"
    }
}

// ==== TEXT TRANSFORMATION OPERATIONS ====

#[derive(Debug, Clone)]
pub struct UppercaseSelection;
impl Operation for UppercaseSelection {
    fn execute(&self, ctx: &mut Context) -> OperationResult {
        let (start, end) = ctx.selection().range();
        if start != end {
            let text = ctx.buffer().slice(start..end).to_string();
            let uppercase_text = text.to_uppercase();
            let edit = Edit::replace(start, end, uppercase_text);
            ctx.apply_edits(vec![edit]);
        }
        OperationResult::Continue
    }
    fn name(&self) -> &'static str {
        "uppercase_selection"
    }
}

#[derive(Debug, Clone)]
pub struct LowercaseSelection;
impl Operation for LowercaseSelection {
    fn execute(&self, ctx: &mut Context) -> OperationResult {
        let (start, end) = ctx.selection().range();
        if start != end {
            let text = ctx.buffer().slice(start..end).to_string();
            let lowercase_text = text.to_lowercase();
            let edit = Edit::replace(start, end, lowercase_text);
            ctx.apply_edits(vec![edit]);
        }
        OperationResult::Continue
    }
    fn name(&self) -> &'static str {
        "lowercase_selection"
    }
}

#[derive(Debug, Clone)]
pub struct ToggleCaseSelection;
impl Operation for ToggleCaseSelection {
    fn execute(&self, ctx: &mut Context) -> OperationResult {
        let (start, end) = ctx.selection().range();
        if start != end {
            let text = ctx.buffer().slice(start..end).to_string();
            let toggled_text: String = text
                .chars()
                .map(|c| {
                    if c.is_uppercase() {
                        c.to_lowercase().collect::<String>()
                    } else {
                        c.to_uppercase().collect::<String>()
                    }
                })
                .collect();
            let edit = Edit::replace(start, end, toggled_text);
            ctx.apply_edits(vec![edit]);
        }
        OperationResult::Continue
    }
    fn name(&self) -> &'static str {
        "toggle_case_selection"
    }
}

#[derive(Debug, Clone)]
pub struct IndentSelection;
impl Operation for IndentSelection {
    fn execute(&self, ctx: &mut Context) -> OperationResult {
        let (start, end) = ctx.selection().range();
        let buffer = ctx.buffer();

        let start_line = buffer.char_to_line(start);
        let end_line = buffer.char_to_line(end);

        let mut edits = Vec::new();

        for line_idx in start_line..=end_line {
            let line_start = buffer.line_to_char(line_idx);
            edits.push(Edit::insert(line_start, "    ".to_string()));
        }

        ctx.apply_edits(edits);
        OperationResult::Continue
    }
    fn name(&self) -> &'static str {
        "indent_selection"
    }
}

// TODO: can treesitter do this?
#[derive(Debug, Clone)]
pub struct UnindentSelection;
impl Operation for UnindentSelection {
    fn execute(&self, ctx: &mut Context) -> OperationResult {
        let (start, end) = ctx.selection().range();
        let buffer = ctx.buffer();

        let start_line = buffer.char_to_line(start);
        let end_line = buffer.char_to_line(end);

        let mut edits = Vec::new();

        for line_idx in start_line..=end_line {
            let line_start = buffer.line_to_char(line_idx);
            let line = buffer.line(line_idx);

            // Remove up to 4 spaces or 1 tab from the beginning
            let mut chars_to_remove = 0;
            let mut chars = line.chars();

            if let Some(first_char) = chars.next() {
                if first_char == '\t' {
                    chars_to_remove = 1;
                } else if first_char == ' ' {
                    chars_to_remove = 1;
                    for _ in 0..3 {
                        if let Some(' ') = chars.next() {
                            chars_to_remove += 1;
                        } else {
                            break;
                        }
                    }
                }
            }

            if chars_to_remove > 0 {
                edits.push(Edit::delete(line_start, line_start + chars_to_remove));
            }
        }

        ctx.apply_edits(edits);
        OperationResult::Continue
    }
    fn name(&self) -> &'static str {
        "unindent_selection"
    }
}

// ==== LINE OPERATIONS ====

#[derive(Debug, Clone)]
pub struct DuplicateLine;
impl Operation for DuplicateLine {
    fn execute(&self, ctx: &mut Context) -> OperationResult {
        let buffer = ctx.buffer();
        let head = ctx.selection().head;
        let line_idx = buffer.char_to_line(head);
        let line_start = buffer.line_to_char(line_idx);
        let line = buffer.line(line_idx);
        let line_text = line.to_string();

        let edit = Edit::insert(line_start, line_text);
        ctx.apply_edits(vec![edit]);
        OperationResult::Continue
    }
    fn name(&self) -> &'static str {
        "duplicate_line"
    }
}

#[derive(Debug, Clone)]
pub struct MoveLineUp;
impl Operation for MoveLineUp {
    fn execute(&self, ctx: &mut Context) -> OperationResult {
        let buffer = ctx.buffer();
        let head = ctx.selection().head;
        let line_idx = buffer.char_to_line(head);

        if line_idx > 0 {
            let current_line = buffer.line(line_idx).to_string();
            let prev_line = buffer.line(line_idx - 1).to_string();

            let prev_line_start = buffer.line_to_char(line_idx - 1);
            let current_line_end = if line_idx + 1 < buffer.len_lines() {
                buffer.line_to_char(line_idx + 1)
            } else {
                buffer.len_chars()
            };

            let new_text = format!("{}{}", current_line, prev_line);
            let edit = Edit::replace(prev_line_start, current_line_end, new_text);
            ctx.apply_edits(vec![edit]);
        }
        OperationResult::Continue
    }
    fn name(&self) -> &'static str {
        "move_line_up"
    }
}

#[derive(Debug, Clone)]
pub struct MoveLineDown;
impl Operation for MoveLineDown {
    fn execute(&self, ctx: &mut Context) -> OperationResult {
        let buffer = ctx.buffer();
        let head = ctx.selection().head;
        let line_idx = buffer.char_to_line(head);

        if line_idx + 1 < buffer.len_lines() {
            let current_line = buffer.line(line_idx).to_string();
            let next_line = buffer.line(line_idx + 1).to_string();

            let current_line_start = buffer.line_to_char(line_idx);
            let next_line_end = if line_idx + 2 < buffer.len_lines() {
                buffer.line_to_char(line_idx + 2)
            } else {
                buffer.len_chars()
            };

            let new_text = format!("{}{}", next_line, current_line);
            let edit = Edit::replace(current_line_start, next_line_end, new_text);
            ctx.apply_edits(vec![edit]);
        }
        OperationResult::Continue
    }
    fn name(&self) -> &'static str {
        "move_line_down"
    }
}

#[derive(Debug, Clone)]
pub struct InsertLineAbove;
impl Operation for InsertLineAbove {
    fn execute(&self, ctx: &mut Context) -> OperationResult {
        let buffer = ctx.buffer();
        let head = ctx.selection().head;
        let line_idx = buffer.char_to_line(head);
        let line_start = buffer.line_to_char(line_idx);

        let edit = Edit::insert(line_start, "\n".to_string());
        ctx.apply_edits(vec![edit]);
        OperationResult::Continue
    }
    fn name(&self) -> &'static str {
        "insert_line_above"
    }
}

#[derive(Debug, Clone)]
pub struct InsertLineBelow;
impl Operation for InsertLineBelow {
    fn execute(&self, ctx: &mut Context) -> OperationResult {
        let buffer = ctx.buffer();
        let head = ctx.selection().head;
        let line_idx = buffer.char_to_line(head);
        let line_start = buffer.line_to_char(line_idx);
        let line_len = buffer.line(line_idx).len_chars();
        let line_end = line_start + line_len;

        let edit = Edit::insert(line_end, "\n".to_string());
        ctx.apply_edits(vec![edit]);
        OperationResult::Continue
    }
    fn name(&self) -> &'static str {
        "insert_line_below"
    }
}

// ==== SEARCH AND REPLACE OPERATIONS ====

#[derive(Debug, Clone)]
pub struct FindNext {
    pub pattern: String,
}
impl FindNext {
    pub fn new(pattern: String) -> Self {
        Self { pattern }
    }
}

// TODO: this should work on slices
impl Operation for FindNext {
    fn execute(&self, ctx: &mut Context) -> OperationResult {
        let content = ctx.buffer().content().to_string();
        let start = ctx.selection().head;

        if let Some(pos) = content[start..].find(&self.pattern) {
            let found_pos = start + pos;
            ctx.selection_mut()
                .set_range(found_pos, found_pos + self.pattern.len());
        }
        OperationResult::Continue
    }
    fn name(&self) -> &'static str {
        "find_next"
    }
}

#[derive(Debug, Clone)]
pub struct FindPrevious {
    pub pattern: String,
}
impl FindPrevious {
    pub fn new(pattern: String) -> Self {
        Self { pattern }
    }
}
impl Operation for FindPrevious {
    fn execute(&self, ctx: &mut Context) -> OperationResult {
        let content = ctx.buffer().content().to_string();
        let end = ctx.selection().head;

        if let Some(pos) = content[..end].rfind(&self.pattern) {
            ctx.selection_mut().set_range(pos, pos + self.pattern.len());
        }
        OperationResult::Continue
    }
    fn name(&self) -> &'static str {
        "find_previous"
    }
}

#[derive(Debug, Clone)]
pub struct Replace {
    pub pattern: String,
    pub replacement: String,
}
impl Replace {
    pub fn new(pattern: String, replacement: String) -> Self {
        Self {
            pattern,
            replacement,
        }
    }
}
impl Operation for Replace {
    fn execute(&self, ctx: &mut Context) -> OperationResult {
        let (start, end) = ctx.selection().range();
        let selected_text = ctx.buffer().content().slice(start..end).to_string();

        if selected_text == self.pattern {
            let edit = Edit::replace(start, end, self.replacement.clone());
            ctx.apply_edits(vec![edit]);
        }
        OperationResult::Continue
    }
    fn name(&self) -> &'static str {
        "replace"
    }
}

#[derive(Debug, Clone)]
pub struct ReplaceAll {
    pub pattern: String,
    pub replacement: String,
}
impl ReplaceAll {
    pub fn new(pattern: String, replacement: String) -> Self {
        Self {
            pattern,
            replacement,
        }
    }
}
impl Operation for ReplaceAll {
    fn execute(&self, ctx: &mut Context) -> OperationResult {
        let content = ctx.buffer().content().to_string();
        let new_content = content.replace(&self.pattern, &self.replacement);

        if content != new_content {
            let edit = Edit::replace(0, content.len(), new_content);
            ctx.apply_edits(vec![edit]);
        }
        OperationResult::Continue
    }
    fn name(&self) -> &'static str {
        "replace_all"
    }
}

// ==== HISTORY OPERATIONS ====

#[derive(Debug, Clone)]
pub struct Undo;
impl Operation for Undo {
    fn execute(&self, ctx: &mut Context) -> OperationResult {
        if let Some(inverse_edits) = ctx.history_mut().undo() {
            let new_cursor_pos = inverse_edits.first().map_or(0, |e| e.position());
            ctx.buffer_mut().apply(&inverse_edits);
            ctx.selection_mut().cursor_to(new_cursor_pos);
        }
        OperationResult::Continue
    }
    fn name(&self) -> &'static str {
        "undo"
    }
}

#[derive(Debug, Clone)]
pub struct Redo;
impl Operation for Redo {
    fn execute(&self, ctx: &mut Context) -> OperationResult {
        if let Some(edits_to_reapply) = ctx.history_mut().redo() {
            let new_cursor_pos = edits_to_reapply
                .last()
                .map_or(0, |e| e.position() + e.inserted().len());
            ctx.buffer_mut().apply(&edits_to_reapply);
            ctx.selection_mut().cursor_to(new_cursor_pos);
        }
        OperationResult::Continue
    }
    fn name(&self) -> &'static str {
        "redo"
    }
}

// ==== MODE AND SYSTEM OPERATIONS ====

#[derive(Debug, Clone)]
pub struct SwitchMode {
    pub target_mode: String,
}
impl SwitchMode {
    pub fn new(mode: String) -> Self {
        Self { target_mode: mode }
    }
}
impl Operation for SwitchMode {
    fn execute(&self, _ctx: &mut Context) -> OperationResult {
        OperationResult::SwitchMode(self.target_mode.clone())
    }
    fn name(&self) -> &'static str {
        "switch_mode"
    }
}

#[derive(Debug, Clone)]
pub struct Exit;
impl Operation for Exit {
    fn execute(&self, _ctx: &mut Context) -> OperationResult {
        OperationResult::Exit
    }
    fn name(&self) -> &'static str {
        "exit"
    }
}

#[derive(Debug, Clone)]
pub struct Save;
impl Operation for Save {
    fn execute(&self, ctx: &mut Context) -> OperationResult {
        // TODO: Implement file saving
        println!("Buffer saved (placeholder implementation)");
        ctx.buffer_mut().set_modified(false);
        OperationResult::Continue
    }
    fn name(&self) -> &'static str {
        "save"
    }
}

#[derive(Debug, Clone)]
pub struct SaveAs {
    pub path: std::path::PathBuf,
}
impl SaveAs {
    pub fn new(path: std::path::PathBuf) -> Self {
        Self { path }
    }
}
impl Operation for SaveAs {
    fn execute(&self, ctx: &mut Context) -> OperationResult {
        // TODO: Implement file saving
        println!(
            "Buffer saved as {:?} (placeholder implementation)",
            self.path
        );
        ctx.buffer_mut().set_path(Some(self.path.clone()));
        ctx.buffer_mut().set_modified(false);
        OperationResult::Continue
    }
    fn name(&self) -> &'static str {
        "save_as"
    }
}

// ==== JUMP OPERATIONS ====

#[derive(Debug, Clone)]
pub struct JumpToLine {
    pub line_number: usize,
}
impl JumpToLine {
    pub fn new(line_number: usize) -> Self {
        Self { line_number }
    }
}
impl Operation for JumpToLine {
    fn execute(&self, ctx: &mut Context) -> OperationResult {
        let content = ctx.buffer().content();
        let line_idx = self.line_number.saturating_sub(1); // Convert to 0-based
        let line_idx = std::cmp::min(line_idx, content.len_lines().saturating_sub(1));
        let pos = content.line_to_char(line_idx);
        ctx.selection_mut().cursor_to(pos);
        OperationResult::Continue
    }
    fn name(&self) -> &'static str {
        "jump_to_line"
    }
}

#[derive(Debug, Clone)]
pub struct JumpToCharacter {
    pub position: usize,
}
impl JumpToCharacter {
    pub fn new(position: usize) -> Self {
        Self { position }
    }
}
impl Operation for JumpToCharacter {
    fn execute(&self, ctx: &mut Context) -> OperationResult {
        let len = ctx.buffer().len_chars();
        let pos = std::cmp::min(self.position, len);
        ctx.selection_mut().cursor_to(pos);
        OperationResult::Continue
    }
    fn name(&self) -> &'static str {
        "jump_to_character"
    }
}
