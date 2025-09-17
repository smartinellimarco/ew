use ropey::{str_utils::byte_to_char_idx, Rope, RopeSlice};
use unicode_segmentation::{GraphemeCursor, GraphemeIncomplete};

// TODO: esto hace falta que tome ropes? no me gusta que operations lo use directamente
/// Finds the previous grapheme boundary before the given char position.
pub fn prev_grapheme_boundary(slice: &RopeSlice, char_idx: usize) -> usize {
    // Bounds check
    debug_assert!(char_idx <= slice.len_chars());

    // We work with bytes for this, so convert.
    let byte_idx = slice.char_to_byte(char_idx);

    // Get the chunk with our byte index in it.
    let (mut chunk, mut chunk_byte_idx, mut chunk_char_idx, _) = slice.chunk_at_byte(byte_idx);

    // Set up the grapheme cursor.
    let mut gc = GraphemeCursor::new(byte_idx, slice.len_bytes(), true);

    // Find the previous grapheme cluster boundary.
    loop {
        match gc.prev_boundary(chunk, chunk_byte_idx) {
            Ok(None) => return 0,
            Ok(Some(n)) => {
                let tmp = byte_to_char_idx(chunk, n - chunk_byte_idx);
                return chunk_char_idx + tmp;
            }
            Err(GraphemeIncomplete::PrevChunk) => {
                let (a, b, c, _) = slice.chunk_at_byte(chunk_byte_idx - 1);
                chunk = a;
                chunk_byte_idx = b;
                chunk_char_idx = c;
            }
            Err(GraphemeIncomplete::PreContext(n)) => {
                let ctx_chunk = slice.chunk_at_byte(n - 1).0;
                gc.provide_context(ctx_chunk, n - ctx_chunk.len());
            }
            _ => unreachable!(),
        }
    }
}

/// Finds the next grapheme boundary after the given char position.
pub fn next_grapheme_boundary(slice: &RopeSlice, char_idx: usize) -> usize {
    // Bounds check
    debug_assert!(char_idx <= slice.len_chars());

    // We work with bytes for this, so convert.
    let byte_idx = slice.char_to_byte(char_idx);

    // Get the chunk with our byte index in it.
    let (mut chunk, mut chunk_byte_idx, mut chunk_char_idx, _) = slice.chunk_at_byte(byte_idx);

    // Set up the grapheme cursor.
    let mut gc = GraphemeCursor::new(byte_idx, slice.len_bytes(), true);

    // Find the next grapheme cluster boundary.
    loop {
        match gc.next_boundary(chunk, chunk_byte_idx) {
            Ok(None) => return slice.len_chars(),
            Ok(Some(n)) => {
                let tmp = byte_to_char_idx(chunk, n - chunk_byte_idx);
                return chunk_char_idx + tmp;
            }
            Err(GraphemeIncomplete::NextChunk) => {
                chunk_byte_idx += chunk.len();
                let (a, _, c, _) = slice.chunk_at_byte(chunk_byte_idx);
                chunk = a;
                chunk_char_idx = c;
            }
            Err(GraphemeIncomplete::PreContext(n)) => {
                let ctx_chunk = slice.chunk_at_byte(n - 1).0;
                gc.provide_context(ctx_chunk, n - ctx_chunk.len());
            }
            _ => unreachable!(),
        }
    }
}

/// Returns whether the given char position is a grapheme boundary.
pub fn is_grapheme_boundary(slice: &RopeSlice, char_idx: usize) -> bool {
    // Bounds check
    debug_assert!(char_idx <= slice.len_chars());

    // We work with bytes for this, so convert.
    let byte_idx = slice.char_to_byte(char_idx);

    // Get the chunk with our byte index in it.
    let (chunk, chunk_byte_idx, _, _) = slice.chunk_at_byte(byte_idx);

    // Set up the grapheme cursor.
    let mut gc = GraphemeCursor::new(byte_idx, slice.len_bytes(), true);

    // Determine if the given position is a grapheme cluster boundary.
    loop {
        match gc.is_boundary(chunk, chunk_byte_idx) {
            Ok(n) => return n,
            Err(GraphemeIncomplete::PreContext(n)) => {
                let (ctx_chunk, ctx_byte_start, _, _) = slice.chunk_at_byte(n - 1);
                gc.provide_context(ctx_chunk, ctx_byte_start);
            }
            _ => unreachable!(),
        }
    }
}

/// Convenience functions for common operations
pub fn prev_grapheme_char_index(rope: &Rope, char_idx: usize) -> usize {
    prev_grapheme_boundary(&rope.slice(..), char_idx)
}

pub fn next_grapheme_char_index(rope: &Rope, char_idx: usize) -> usize {
    next_grapheme_boundary(&rope.slice(..), char_idx)
}

/// Find the start of the current word
pub fn word_start_index(slice: &RopeSlice, char_idx: usize) -> usize {
    if char_idx == 0 {
        return 0;
    }

    let mut pos = char_idx;

    // Skip whitespace backwards
    while pos > 0 {
        let ch = slice.char(pos - 1);
        if !ch.is_whitespace() {
            break;
        }
        pos -= 1;
    }

    // Find word boundary
    while pos > 0 {
        let ch = slice.char(pos - 1);
        if ch.is_whitespace() || !ch.is_alphanumeric() {
            break;
        }
        pos -= 1;
    }

    pos
}

/// Find the end of the current word
pub fn word_end_index(slice: &RopeSlice, char_idx: usize) -> usize {
    let len = slice.len_chars();
    if char_idx >= len {
        return len;
    }

    let mut pos = char_idx;

    // Skip whitespace forwards
    while pos < len {
        let ch = slice.char(pos);
        if !ch.is_whitespace() {
            break;
        }
        pos += 1;
    }

    // Find word boundary
    while pos < len {
        let ch = slice.char(pos);
        if ch.is_whitespace() || !ch.is_alphanumeric() {
            break;
        }
        pos += 1;
    }

    pos
}

/// Find the start of the current WORD (whitespace separated)
pub fn big_word_start_index(slice: &RopeSlice, char_idx: usize) -> usize {
    if char_idx == 0 {
        return 0;
    }

    let mut pos = char_idx;

    // Skip whitespace backwards
    while pos > 0 {
        let ch = slice.char(pos - 1);
        if !ch.is_whitespace() {
            break;
        }
        pos -= 1;
    }

    // Find whitespace boundary
    while pos > 0 {
        let ch = slice.char(pos - 1);
        if ch.is_whitespace() {
            break;
        }
        pos -= 1;
    }

    pos
}

/// Find the end of the current WORD (whitespace separated)
pub fn big_word_end_index(slice: &RopeSlice, char_idx: usize) -> usize {
    let len = slice.len_chars();
    if char_idx >= len {
        return len;
    }

    let mut pos = char_idx;

    // Skip whitespace forwards
    while pos < len {
        let ch = slice.char(pos);
        if !ch.is_whitespace() {
            break;
        }
        pos += 1;
    }

    // Find whitespace boundary
    while pos < len {
        let ch = slice.char(pos);
        if ch.is_whitespace() {
            break;
        }
        pos += 1;
    }

    pos
}

/// Find matching bracket/paren/brace
pub fn find_matching_bracket(slice: &RopeSlice, char_idx: usize) -> Option<usize> {
    if char_idx >= slice.len_chars() {
        return None;
    }

    let ch = slice.char(char_idx);
    let (open, close, direction) = match ch {
        '(' => ('(', ')', 1),
        ')' => ('(', ')', -1),
        '[' => ('[', ']', 1),
        ']' => ('[', ']', -1),
        '{' => ('{', '}', 1),
        '}' => ('{', '}', -1),
        _ => return None,
    };

    let mut count = 1;
    let mut pos = char_idx as isize + direction;

    while pos >= 0 && (pos as usize) < slice.len_chars() {
        let current_ch = slice.char(pos as usize);
        if current_ch == open {
            count += direction;
        } else if current_ch == close {
            count -= direction;
        }

        if count == 0 {
            return Some(pos as usize);
        }

        pos += direction;
    }

    None
}

/// Find the start of the current paragraph
pub fn paragraph_start_index(slice: &RopeSlice, char_idx: usize) -> usize {
    let line_idx = slice.char_to_line(char_idx);
    let mut current_line = line_idx;

    // Move up until we find an empty line or reach the beginning
    while current_line > 0 {
        let line = slice.line(current_line - 1);
        if line.chars().all(|c| c.is_whitespace()) {
            break;
        }
        current_line -= 1;
    }

    slice.line_to_char(current_line)
}

/// Find the end of the current paragraph
pub fn paragraph_end_index(slice: &RopeSlice, char_idx: usize) -> usize {
    let line_idx = slice.char_to_line(char_idx);
    let mut current_line = line_idx;
    let max_line = slice.len_lines();

    // Move down until we find an empty line or reach the end
    while current_line + 1 < max_line {
        let line = slice.line(current_line + 1);
        if line.chars().all(|c| c.is_whitespace()) {
            break;
        }
        current_line += 1;
    }

    if current_line + 1 < max_line {
        slice.line_to_char(current_line + 1)
    } else {
        slice.len_chars()
    }
}
