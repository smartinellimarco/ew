use crate::buffer::Buffer;
use crate::editor::Editor;
use crate::history::History;
use crate::selection::Selection;

#[derive(Debug)]
pub struct Context {
    buffer: Buffer,
    selection: Selection,
    history: History,
}

impl Context {
    pub fn new() -> Self {
        Self {
            buffer: Buffer::new(),
            selection: Selection::new(0, 0),
            history: History::new(),
        }
    }
}
