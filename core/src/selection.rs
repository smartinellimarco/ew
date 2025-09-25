#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Selection {
    pub anchor: usize,
    pub head: usize,
}

impl Selection {
    pub fn new(anchor: usize, head: usize) -> Self {
        Self { anchor, head }
    }

    pub fn is_cursor(&self) -> bool {
        self.anchor == self.head
    }

    pub fn cursor_to(&mut self, position: usize) {
        self.anchor = position;
        self.head = position;
    }

    pub fn set_range(&mut self, anchor: usize, head: usize) {
        self.anchor = anchor;
        self.head = head;
    }

    pub fn range(&self) -> (usize, usize) {
        if self.anchor <= self.head {
            (self.anchor, self.head)
        } else {
            (self.head, self.anchor)
        }
    }
}
