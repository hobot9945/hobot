//! report

pub struct Report {
    pub(crate) text: String,
}

impl Report {
    pub fn new() -> Self {
        Self {
            text: "".to_string(),
        }
    }

    pub fn clear(&mut self) {
        self.text.clear();
    }
    
    pub fn is_empty(&self) -> bool {
        self.text.is_empty()
    }
}