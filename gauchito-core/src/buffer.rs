use crate::textobjects::textobject::{TextObject, TextObjectKind, TextRange};
use crate::textobjects::traits::TextNavigator;
use crate::{edit::Edit, textobjects::registry::TextObjectRegistry};

use ropey::{Rope, RopeSlice};
use std::{ops::RangeBounds, path::PathBuf};

#[derive(Debug)]
pub struct Buffer {
    content: Rope,
    path: Option<PathBuf>,
    modified: bool,
    text_objects: TextObjectRegistry,
}

cache de los treesitter grammars con checksum (guardar alguna cache en /tmp y revisar si el file no cambio)
    
impl Buffer {
    pub fn new() -> Self {
        Self {
            content: Rope::new(),
            path: None,
            modified: false,
            text_objects: TextObjectRegistry::with_defaults(),
        }
    }

    pub fn from_str(text: &str) -> Self {
        Self {
            content: Rope::from_str(text),
            path: None,
            modified: false,
            text_objects: TextObjectRegistry::with_defaults(),
        }
    }

    /// Create a buffer with custom text object capabilities
    pub fn with_text_objects(mut self, registry: TextObjectRegistry) -> Self {
        self.text_objects = registry;
        self
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

        // Try to enable tree-sitter support based on file extension
        // TODO: uncomment
        // if let Some(path) = &path {
        //     if let Some(ext) = path.extension().and_then(|s| s.to_str()) {
        //         self.try_enable_treesitter_for_language(ext);
        //     }
        // }
    }

    pub fn is_modified(&self) -> bool {
        self.modified
    }

    pub fn set_modified(&mut self, modified: bool) {
        self.modified = modified;
    }

    pub fn apply(&mut self, edits: &[Edit]) {
        if edits.is_empty() {
            return;
        }
        let mut sorted_edits = edits.to_vec();
        sorted_edits.sort_by(|a, b| b.start.cmp(&a.start));
        for edit in sorted_edits {
            if edit.start != edit.end {
                self.content.remove(edit.start..edit.end);
            }
            if !edit.text.is_empty() {
                self.content.insert(edit.start, &edit.text);
            }
        }
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

    pub fn slice<R: RangeBounds<usize>>(&self, range: R) -> RopeSlice {
        self.content.slice(range)
    }

    // Text object capabilities

    /// Find a text object at the given position
    pub fn find_text_object_at(&self, pos: usize, text_obj: &TextObject) -> Option<TextRange> {
        self.text_objects.find_at(self, pos, text_obj)
    }

    /// Find the next occurrence of a text object
    pub fn find_text_object_next(&self, pos: usize, text_obj: &TextObject) -> Option<TextRange> {
        self.text_objects.find_next(self, pos, text_obj)
    }

    /// Find the previous occurrence of a text object
    pub fn find_text_object_prev(&self, pos: usize, text_obj: &TextObject) -> Option<TextRange> {
        self.text_objects.find_prev(self, pos, text_obj)
    }

    /// Get the text content of a range
    pub fn text_in_range(&self, range: &TextRange) -> String {
        if range.start >= range.end || range.end > self.len_chars() {
            return String::new();
        }
        self.content.slice(range.start..range.end).to_string()
    }

    /// Check if a text object type is supported
    pub fn supports_text_object(&self, kind: &TextObjectKind) -> bool {
        self.text_objects.supports(kind)
    }

    /// Try to enable tree-sitter support for a language
    // fn try_enable_treesitter_for_language(&mut self, language: &str) {
    //     if let Some(ts_finder) = TreeSitterTextObjectFinder::with_language(language) {
    //         {
    //             let this = &mut self.text_objects;
    //             let finder: Box<dyn TextObjectFinder> = Box::new(ts_finder);
    //             let finder_index = this.finders.len();
    //
    //             // Update capability cache
    //             for kind in finder.supported_kinds() {
    //                 this.capability_cache.insert(kind.clone(), finder_index);
    //             }
    //
    //             this.finders.push(finder);
    //         };
    //     }
    // }

    /// Add additional text object finding capability
    // pub fn add_text_object_finder(&mut self, finder: Box<dyn TextObjectFinder>) {
    //     {
    //         let this = &mut self.text_objects;
    //         let finder_index = this.finders.len();
    //
    //         // Update capability cache
    //         for kind in finder.supported_kinds() {
    //             this.capability_cache.insert(kind.clone(), finder_index);
    //         }
    //
    //         this.finders.push(finder);
    //     };
    // }

    /// Helper method to get character at position
    pub fn char_at(&self, pos: usize) -> Option<char> {
        if pos < self.len_chars() {
            Some(self.content.char(pos))
        } else {
            None
        }
    }
}

impl TextNavigator for Buffer {
    fn len_chars(&self) -> usize {
        self.content.len_chars()
    }

    fn len_lines(&self) -> usize {
        self.content.len_lines()
    }

    fn char_at(&self, pos: usize) -> Option<char> {
        if pos < self.content.len_chars() {
            Some(self.content.char(pos))
        } else {
            None
        }
    }

    fn char_to_line(&self, pos: usize) -> usize {
        self.content.char_to_line(pos)
    }

    fn line_to_char(&self, line: usize) -> usize {
        self.content.line_to_char(line)
    }

    fn slice_to_string(&self, start: usize, end: usize) -> String {
        if start >= end || end > self.len_chars() {
            return String::new();
        }
        self.content.slice(start..end).to_string()
    }

    fn line_chars(&self, line: usize) -> Box<dyn Iterator<Item = char> + '_> {
        if line < self.len_lines() {
            Box::new(self.content.line(line).chars())
        } else {
            Box::new(std::iter::empty())
        }
    }
}

impl Default for Buffer {
    fn default() -> Self {
        Self::new()
    }
}
