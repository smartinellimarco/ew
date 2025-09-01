#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Edit {
    pub position: usize,
    pub deleted: String,
    pub inserted: String,
}

impl Edit {
    pub fn delete(position: usize, deleted: String) -> Self {
        Self {
            position,
            deleted,
            inserted: String::new(),
        }
    }

    pub fn insert(position: usize, inserted: String) -> Self {
        Self {
            position,
            deleted: String::new(),
            inserted,
        }
    }

    pub fn replace(position: usize, deleted: String, inserted: String) -> Self {
        Self {
            position,
            deleted,
            inserted,
        }
    }

    pub fn inverse(&self) -> Self {
        Self {
            position: self.position,
            deleted: self.inserted.clone(),
            inserted: self.deleted.clone(),
        }
    }

    pub fn is_noop(&self) -> bool {
        self.deleted.is_empty() && self.inserted.is_empty()
    }
}
