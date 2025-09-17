use crate::textobjects::textobject::{Selection, TextObject, TextObjectKind, TextRange};
use crate::textobjects::traits::{TextNavigator, TextObjectFinder};

/// Basic text object finder that handles standard text objects
pub struct BasicTextObjectFinder {
    supported: Vec<TextObjectKind>,
}

impl BasicTextObjectFinder {
    pub fn new() -> Self {
        Self {
            supported: vec![
                TextObjectKind::Word,
                TextObjectKind::BigWord,
                TextObjectKind::Line,
                TextObjectKind::Paragraph,
                TextObjectKind::Parentheses,
                TextObjectKind::Brackets,
                TextObjectKind::Braces,
            ],
        }
    }
}

impl TextObjectFinder for BasicTextObjectFinder {
    fn supported_kinds(&self) -> &[TextObjectKind] {
        &self.supported
    }

    fn find_at(
        &self,
        navigator: &dyn TextNavigator,
        pos: usize,
        text_obj: &TextObject,
    ) -> Option<TextRange> {
        if pos >= navigator.len_chars() {
            return None;
        }

        match text_obj.kind {
            TextObjectKind::Word => {
                let start = self.word_start(navigator, pos);
                let end = self.word_end(navigator, pos);
                if start < end {
                    Some(TextRange::new(start, end))
                } else {
                    None
                }
            }

            TextObjectKind::BigWord => {
                let start = self.big_word_start(navigator, pos);
                let end = self.big_word_end(navigator, pos);
                if start < end {
                    Some(TextRange::new(start, end))
                } else {
                    None
                }
            }

            TextObjectKind::Line => {
                let line_idx = navigator.char_to_line(pos);
                let start = navigator.line_to_char(line_idx);
                let end = if line_idx + 1 < navigator.len_lines() {
                    navigator.line_to_char(line_idx + 1)
                } else {
                    navigator.len_chars()
                };
                Some(TextRange::new(start, end))
            }

            TextObjectKind::Paragraph => {
                let start = self.paragraph_start(navigator, pos);
                let end = self.paragraph_end(navigator, pos);
                Some(TextRange::new(start, end))
            }

            TextObjectKind::Parentheses => {
                self.find_bracket_range(navigator, pos, '(', ')', text_obj.selection)
            }

            TextObjectKind::Brackets => {
                self.find_bracket_range(navigator, pos, '[', ']', text_obj.selection)
            }

            TextObjectKind::Braces => {
                self.find_bracket_range(navigator, pos, '{', '}', text_obj.selection)
            }

            _ => None, // Unsupported by this finder
        }
    }

    fn find_next(
        &self,
        navigator: &dyn TextNavigator,
        pos: usize,
        text_obj: &TextObject,
    ) -> Option<TextRange> {
        // Simple implementation - could be optimized per text object type
        for i in (pos + 1)..navigator.len_chars() {
            if let Some(range) = self.find_at(navigator, i, text_obj) {
                if range.start > pos {
                    return Some(range);
                }
            }
        }
        None
    }

    fn find_prev(
        &self,
        navigator: &dyn TextNavigator,
        pos: usize,
        text_obj: &TextObject,
    ) -> Option<TextRange> {
        // Simple implementation - could be optimized per text object type
        for i in (0..pos).rev() {
            if let Some(range) = self.find_at(navigator, i, text_obj) {
                if range.end <= pos {
                    return Some(range);
                }
            }
        }
        None
    }
}

impl BasicTextObjectFinder {
    fn prev_grapheme_boundary(&self, navigator: &dyn TextNavigator, pos: usize) -> usize {
        // TODO: unicodesegmentation?
        // Simple implementation - you'd integrate with your existing grapheme code
        // For now, just move back one char (you'd replace this with your ropey grapheme logic)
        if pos > 0 {
            pos - 1
        } else {
            0
        }
    }

    fn next_grapheme_boundary(&self, navigator: &dyn TextNavigator, pos: usize) -> usize {
        // TODO: unicodesegmentation? 
        // Simple implementation - you'd integrate with your existing grapheme code
        let len = navigator.len_chars();
        if pos < len {
            pos + 1
        } else {
            len
        }
    }

    fn word_start(&self, navigator: &dyn TextNavigator, pos: usize) -> usize {
        let mut current = pos;

        // Skip whitespace backwards
        while current > 0 {
            if let Some(ch) = navigator.char_at(current - 1) {
                if !ch.is_whitespace() {
                    break;
                }
            }
            current -= 1;
        }

        // Find word boundary
        while current > 0 {
            if let Some(ch) = navigator.char_at(current - 1) {
                if ch.is_whitespace() || !ch.is_alphanumeric() {
                    break;
                }
            }
            current -= 1;
        }

        current
    }

    fn word_end(&self, navigator: &dyn TextNavigator, pos: usize) -> usize {
        let len = navigator.len_chars();
        let mut current = pos;

        // Skip whitespace forwards
        while current < len {
            if let Some(ch) = navigator.char_at(current) {
                if !ch.is_whitespace() {
                    break;
                }
            }
            current += 1;
        }

        // Find word boundary
        while current < len {
            if let Some(ch) = navigator.char_at(current) {
                if ch.is_whitespace() || !ch.is_alphanumeric() {
                    break;
                }
            }
            current += 1;
        }

        current
    }

    fn big_word_start(&self, navigator: &dyn TextNavigator, pos: usize) -> usize {
        let mut current = pos;

        // Skip whitespace backwards
        while current > 0 {
            if let Some(ch) = navigator.char_at(current - 1) {
                if !ch.is_whitespace() {
                    break;
                }
            }
            current -= 1;
        }

        // Find whitespace boundary
        while current > 0 {
            if let Some(ch) = navigator.char_at(current - 1) {
                if ch.is_whitespace() {
                    break;
                }
            }
            current -= 1;
        }

        current
    }

    fn big_word_end(&self, navigator: &dyn TextNavigator, pos: usize) -> usize {
        let len = navigator.len_chars();
        let mut current = pos;

        // Skip whitespace forwards
        while current < len {
            if let Some(ch) = navigator.char_at(current) {
                if !ch.is_whitespace() {
                    break;
                }
            }
            current += 1;
        }

        // Find whitespace boundary
        while current < len {
            if let Some(ch) = navigator.char_at(current) {
                if ch.is_whitespace() {
                    break;
                }
            }
            current += 1;
        }

        current
    }

    fn paragraph_start(&self, navigator: &dyn TextNavigator, pos: usize) -> usize {
        let line_idx = navigator.char_to_line(pos);
        let mut current_line = line_idx;

        // Move up until we find an empty line or reach the beginning
        while current_line > 0 {
            let line_chars: Vec<char> = navigator.line_chars(current_line - 1).collect();
            if line_chars.iter().all(|c| c.is_whitespace()) {
                break;
            }
            current_line -= 1;
        }

        navigator.line_to_char(current_line)
    }

    fn paragraph_end(&self, navigator: &dyn TextNavigator, pos: usize) -> usize {
        let line_idx = navigator.char_to_line(pos);
        let mut current_line = line_idx;
        let max_line = navigator.len_lines();

        // Move down until we find an empty line or reach the end
        while current_line + 1 < max_line {
            let line_chars: Vec<char> = navigator.line_chars(current_line + 1).collect();
            if line_chars.iter().all(|c| c.is_whitespace()) {
                break;
            }
            current_line += 1;
        }

        if current_line + 1 < max_line {
            navigator.line_to_char(current_line + 1)
        } else {
            navigator.len_chars()
        }
    }

    fn find_bracket_range(
        &self,
        navigator: &dyn TextNavigator,
        pos: usize,
        open: char,
        close: char,
        selection: Selection,
    ) -> Option<TextRange> {
        // Find the opening bracket (search backwards from cursor or at cursor)
        let open_pos = self.find_bracket_backwards(navigator, pos, open, close)?;

        // Find the matching closing bracket
        let close_pos = self.find_matching_bracket_forward(navigator, open_pos, open, close)?;

        match selection {
            Selection::Inner => {
                if close_pos > open_pos + 1 {
                    Some(TextRange::new(open_pos + 1, close_pos))
                } else {
                    Some(TextRange::new(open_pos, open_pos)) // Empty range
                }
            }
            Selection::Around => Some(TextRange::new(open_pos, close_pos + 1)),
        }
    }

    fn find_bracket_backwards(
        &self,
        navigator: &dyn TextNavigator,
        pos: usize,
        open: char,
        close: char,
    ) -> Option<usize> {
        let mut count = 0;
        for i in (0..=pos).rev() {
            if let Some(ch) = navigator.char_at(i) {
                if ch == close {
                    count += 1;
                } else if ch == open {
                    if count == 0 {
                        return Some(i);
                    }
                    count -= 1;
                }
            }
        }
        None
    }

    fn find_matching_bracket_forward(
        &self,
        navigator: &dyn TextNavigator,
        start: usize,
        open: char,
        close: char,
    ) -> Option<usize> {
        let mut count = 1; // We start after the opening bracket
        let len = navigator.len_chars();

        for i in (start + 1)..len {
            if let Some(ch) = navigator.char_at(i) {
                if ch == open {
                    count += 1;
                } else if ch == close {
                    count -= 1;
                    if count == 0 {
                        return Some(i);
                    }
                }
            }
        }
        None
    }
}

impl Default for BasicTextObjectFinder {
    fn default() -> Self {
        Self::new()
    }
}
