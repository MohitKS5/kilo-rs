use std::cmp::min;

use unicode_segmentation::UnicodeSegmentation;

#[derive(Default, Debug)]
pub struct Row {
    text: String,
    len: usize,
}

impl From<&str> for Row {
    fn from(str: &str) -> Self {
        let mut row = Row {
            text: String::from(str),
            len: 0,
        };
        row.update_len();
        row
    }
}

impl Row {
    pub fn render(&self, start: usize, end: usize) -> String {
        let end = min(end, self.text.len());
        let start = min(end, start);
        let mut result = String::new();
        for mut grapheme in self.text[..].graphemes(true).skip(start).take(end - start) {
            if grapheme == "\t" {
                grapheme = ""
            }
            result.push_str(grapheme)
        }
        result
    }

    pub fn len(&self) -> usize {
        self.len
    }
    pub fn is_empty(&self) -> bool {
        self.text.is_empty()
    }

    fn update_len(&mut self) {
        self.len = self.text.graphemes(true).count()
    }

    pub fn insert(&mut self, at: usize, c: char) {
        if at >= self.len() {
            self.text.push(c)
        } else {
            let left: String = self.text.graphemes(true).take(at).collect();
            let right: String = self.text.graphemes(true).skip(at).collect();
            self.set_text(format!("{}{}{}", left, c, right))
        }
        self.update_len()
    }

    pub fn delete(&mut self, at: usize) {
        if at >= self.len {
            return;
        }
        let left: String = self.text.graphemes(true).take(at).collect();
        let right: String = self.text.graphemes(true).skip(at + 1).collect();
        self.set_text(left + right.as_str());
    }

    fn set_text(&mut self, text: String) {
        self.text = text;
        self.update_len();
    }

    pub fn append_row(&mut self, row: &Self) {
        self.text += row.text.as_str();
        self.update_len();
    }

    pub fn split_at(&mut self, at: usize) -> Self {
        let left: String = self.text.graphemes(true).take(at).collect();
        let right: String = self.text.graphemes(true).skip(at).collect();
        self.set_text(left);
        Row::from(right.as_str())
    }
}
