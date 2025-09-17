use std::collections::HashMap;

use super::{
    finders::basic::BasicTextObjectFinder,
    textobject::{TextObject, TextObjectKind, TextRange},
    traits::{TextNavigator, TextObjectFinder},
};

// TODO: treesitter should have priority over basic
// define that property

// Registry that manages multiple text object finders
#[derive(Debug)]
pub struct TextObjectRegistry {
    finders: Vec<Box<dyn TextObjectFinder>>,
    capability_cache: HashMap<TextObjectKind, usize>, // Maps kind to finder index
}

impl TextObjectRegistry {
    pub fn new() -> Self {
        Self {
            finders: Vec::new(),
            capability_cache: HashMap::new(),
        }
    }

    /// Add a finder to the registry
    pub fn register_finder(&mut self, finder: Box<dyn TextObjectFinder>) {
        let finder_index = self.finders.len();

        // Update capability cache
        for kind in finder.supported_kinds() {
            self.capability_cache.insert(kind.clone(), finder_index);
        }

        self.finders.push(finder);
    }

    /// Find a capable finder for the given text object
    fn find_capable_finder(&self, text_obj: &TextObject) -> Option<&dyn TextObjectFinder> {
        // Try cache first
        if let Some(&index) = self.capability_cache.get(&text_obj.kind) {
            return self.finders.get(index).map(|f| f.as_ref());
        }

        // Fallback: search all finders
        self.finders
            .iter()
            .find(|finder| finder.can_handle(text_obj))
            .map(|f| f.as_ref())
    }

    pub fn find_at(
        &self,
        navigator: &dyn TextNavigator,
        pos: usize,
        text_obj: &TextObject,
    ) -> Option<TextRange> {
        self.find_capable_finder(text_obj)?
            .find_at(navigator, pos, text_obj)
    }

    pub fn find_next(
        &self,
        navigator: &dyn TextNavigator,
        pos: usize,
        text_obj: &TextObject,
    ) -> Option<TextRange> {
        self.find_capable_finder(text_obj)?
            .find_next(navigator, pos, text_obj)
    }

    pub fn find_prev(
        &self,
        navigator: &dyn TextNavigator,
        pos: usize,
        text_obj: &TextObject,
    ) -> Option<TextRange> {
        self.find_capable_finder(text_obj)?
            .find_prev(navigator, pos, text_obj)
    }

    /// Check if a text object type is supported
    pub fn supports(&self, kind: &TextObjectKind) -> bool {
        self.capability_cache.contains_key(kind)
            || self
                .finders
                .iter()
                .any(|f| f.supported_kinds().contains(kind))
    }

    /// Create a default registry with basic text object support
    pub fn with_defaults() -> Self {
        let mut registry = Self::new();
        registry.register_finder(Box::new(BasicTextObjectFinder::new()));
        registry
    }
}

impl Default for TextObjectRegistry {
    fn default() -> Self {
        Self::with_defaults()
    }
}
