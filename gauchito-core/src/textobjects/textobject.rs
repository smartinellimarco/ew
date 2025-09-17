/// Represents a range in the text buffer
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TextRange {
    pub start: usize,
    pub end: usize,
}

impl TextRange {
    pub fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }

    pub fn is_empty(&self) -> bool {
        self.start == self.end
    }

    pub fn len(&self) -> usize {
        self.end.saturating_sub(self.start)
    }
}

/// Text object types that can be requested
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TextObjectKind {
    // Basic text objects
    Grapheme,
    Word,
    BigWord,
    Line,
    Paragraph,

    // Bracket-based
    Parentheses,
    Brackets,
    Braces,

    // Tree-sitter based
    Function,
    Class,
    Statement,
    Parameter,
    Comment,
    String,

    // Custom patterns
    Pattern(String),
}

/// Whether to select inner content or around (including delimiters)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Selection {
    Inner,
    Around,
}

/// A complete text object specification
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TextObject {
    pub kind: TextObjectKind,
    pub selection: Selection,
}

impl TextObject {
    pub fn grapheme() -> Self {
        Self {
            kind: TextObjectKind::Grapheme,
            selection: Selection::Around,
        }
    }

    pub fn word() -> Self {
        Self {
            kind: TextObjectKind::Word,
            selection: Selection::Around,
        }
    }

    pub fn inner_word() -> Self {
        Self {
            kind: TextObjectKind::Word,
            selection: Selection::Inner,
        }
    }

    pub fn big_word() -> Self {
        Self {
            kind: TextObjectKind::BigWord,
            selection: Selection::Around,
        }
    }

    pub fn line() -> Self {
        Self {
            kind: TextObjectKind::Line,
            selection: Selection::Around,
        }
    }

    pub fn paragraph() -> Self {
        Self {
            kind: TextObjectKind::Paragraph,
            selection: Selection::Around,
        }
    }

    pub fn inner_parens() -> Self {
        Self {
            kind: TextObjectKind::Parentheses,
            selection: Selection::Inner,
        }
    }

    pub fn around_parens() -> Self {
        Self {
            kind: TextObjectKind::Parentheses,
            selection: Selection::Around,
        }
    }

    pub fn inner_brackets() -> Self {
        Self {
            kind: TextObjectKind::Brackets,
            selection: Selection::Inner,
        }
    }

    pub fn around_brackets() -> Self {
        Self {
            kind: TextObjectKind::Brackets,
            selection: Selection::Around,
        }
    }

    pub fn inner_braces() -> Self {
        Self {
            kind: TextObjectKind::Braces,
            selection: Selection::Inner,
        }
    }

    pub fn around_braces() -> Self {
        Self {
            kind: TextObjectKind::Braces,
            selection: Selection::Around,
        }
    }

    pub fn function() -> Self {
        Self {
            kind: TextObjectKind::Function,
            selection: Selection::Around,
        }
    }

    pub fn inner_function() -> Self {
        Self {
            kind: TextObjectKind::Function,
            selection: Selection::Inner,
        }
    }

    pub fn class() -> Self {
        Self {
            kind: TextObjectKind::Class,
            selection: Selection::Around,
        }
    }

    pub fn inner_class() -> Self {
        Self {
            kind: TextObjectKind::Class,
            selection: Selection::Inner,
        }
    }
}
