use std::cmp::min;
use termion::color;

use unicode_segmentation::UnicodeSegmentation;

use crate::highlighting;

#[derive(Default, Debug)]
pub struct Row {
    text: String,
    len: usize,
    highlighting: Vec<highlighting::Class>,
}

impl From<&str> for Row {
    fn from(str: &str) -> Self {
        let mut row = Row {
            highlighting: Vec::new(),
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
        let mut current_hg = &highlighting::Class::default();
        for (index, grapheme) in self.text[..].graphemes(true).enumerate().skip(start).take(end - start) {
            if let Some(c) = grapheme.chars().next() {
                let hg_class = self
                    .highlighting
                    .get(index)
                    .unwrap_or(&highlighting::Class::None);
                if current_hg != hg_class {
                    let start_hg =
                        format!("{}", color::Fg(hg_class.color()));
                    result.push_str(&start_hg[..]);
                    current_hg = hg_class;
                }
                if c == '\t' {
                    result.push(' ')
                } else {
                    result.push(c)
                }
            }
        }
        let end_hg = format!("{}", color::Fg(color::Reset));
        result.push_str(&end_hg[..]);
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

    pub fn as_bytes(&self) -> &[u8] {
        self.text.as_bytes()
    }

    pub fn highlight(&mut self) {
        let mut hg = Vec::new();
        for c in self.text.chars() {
            if c.is_ascii_digit() {
                hg.push(highlighting::Class::Number);
            } else {
                hg.push(highlighting::Class::None);
            }
        }
        self.highlighting = hg;
    }
}
