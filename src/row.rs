use std::cmp::min;

#[derive(Default, Debug)]
pub struct Row {
    text: String,
}

impl From<&str> for Row {
    fn from(str: &str) -> Self {
        return Row {
            text: String::from(str),
        };
    }
}

impl Row {
    pub fn render(&self, start: usize, end: usize) -> String {
        let end = min(end, self.text.len());
        let start = min(end, start);
        return self.text.get(start..end).unwrap_or_default().to_string();
    }

    pub fn len(&self) -> usize {
        self.text.len()
    }
    pub fn is_empty(&self) -> bool {
        self.text.is_empty()
    }
}
