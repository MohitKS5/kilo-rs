use std::fs;
use std::io::Error;

use crate::{Position, Row};

#[derive(Default)]
pub struct Doc {
    rows: Vec<Row>,
    pub file_name: Option<String>,
}

impl Doc {
    pub fn open(filename: &str) -> Result<Self, Error> {
        let mut rows = vec![];
        let contents = fs::read_to_string(filename)?;

        for line in contents.lines() {
            rows.push(Row::from(line));
        }
        Ok(Self {
            rows,
            file_name: Some(filename.to_string()),
        })
    }

    pub fn row(&self, idx: usize) -> Option<&Row> {
        self.rows.get(idx)
    }

    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }
    pub fn len(&self) -> usize {
        self.rows.len()
    }
    pub fn insert(&mut self, at: &Position, c: char) {
        if at.y == self.len() {
            // if cursor at end of document
            let mut row = Row::default();
            row.insert(0, c);
            self.rows.push(row)
        } else {
            // if cursor within a document
            let mut row = self.rows.get_mut(at.y).unwrap();
            row.insert(at.x, c)
        }
    }

    pub fn delete(&mut self, at: &Position) {
        let len = self.len();
        if at.x ==  self.rows.get(at.y).unwrap().len() && at.y < len - 1 {
            // cursor is at last character of current row but next row exists
            let next_row = self.rows.remove(at.y + 1); // remove row
            let row = self.rows.get_mut(at.y).unwrap();
            row.append_row(&next_row) // add contents to previous row
        } else {
            let row = self.rows.get_mut(at.y).unwrap();
            row.delete(at.x)
        }
    }
}
