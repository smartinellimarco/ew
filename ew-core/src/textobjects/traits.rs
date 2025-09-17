use super::textobject::{TextObject, TextObjectKind, TextRange};

/// Abstraction for text data that text object finders can work with
pub trait TextNavigator {
    fn len_chars(&self) -> usize;
    fn len_lines(&self) -> usize;
    fn char_at(&self, pos: usize) -> Option<char>;
    fn char_to_line(&self, pos: usize) -> usize;
    fn line_to_char(&self, line: usize) -> usize;
    fn slice_to_string(&self, start: usize, end: usize) -> String;

    /// Iterator over characters in a line (for paragraph detection, etc.)
    fn line_chars(&self, line: usize) -> Box<dyn Iterator<Item = char> + '_>;
}

/// Trait for text object finders - each implementation handles different types
pub trait TextObjectFinder: Send + Sync {
    /// Returns the text object kinds this finder can handle
    fn supported_kinds(&self) -> &[TextObjectKind];

    /// Find a text object at the given position
    fn find_at(
        &self,
        navigator: &dyn TextNavigator,
        pos: usize,
        text_obj: &TextObject,
    ) -> Option<TextRange>;

    /// Find the next occurrence
    fn find_next(
        &self,
        navigator: &dyn TextNavigator,
        pos: usize,
        text_obj: &TextObject,
    ) -> Option<TextRange>;

    /// Find the previous occurrence  
    fn find_prev(
        &self,
        navigator: &dyn TextNavigator,
        pos: usize,
        text_obj: &TextObject,
    ) -> Option<TextRange>;

    /// Check if this finder can handle the given text object
    fn can_handle(&self, text_obj: &TextObject) -> bool {
        self.supported_kinds().contains(&text_obj.kind)
    }
}
