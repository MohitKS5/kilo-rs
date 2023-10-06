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

    pub fn update_len(&mut self) {
        self.len = self.text.graphemes(true).count()
    }
}
