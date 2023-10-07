use std::fs;
use std::io::{Error, Write};
use crate::{Position, Row};

#[derive(Default)]
pub struct Doc {
    rows: Vec<Row>,
    pub file_name: Option<String>,
    pub modified: bool
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
            modified: false,
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

    fn insert_newline(&mut self, at: &Position) {
        if at.y > self.len(){
            return;
        }
        if at.y == self.len() {
            self.rows.push(Row::default());
            return;
        }
        let new_row = self.rows.get_mut(at.y).unwrap().split_at(at.x);
        self.rows.insert(at.y + 1, new_row)
    }
    pub fn insert(&mut self, at: &Position, c: char) {
        self.modified = true;
        if c == '\n' {
            self.insert_newline(at);
            return;
        }
        if at.y == self.len() {
            // if cursor at end of document
            let mut row = Row::default();
            row.insert(0, c);
            self.rows.push(row);
        } else {
            // if cursor within a document
            let row = self.rows.get_mut(at.y).unwrap();
            row.insert(at.x, c);
        }
    }

    pub fn delete(&mut self, at: &Position) {
        self.modified = true;
        let len = self.len();
        if at.x ==  self.rows.get(at.y).unwrap().len() && at.y < len - 1 {
            // cursor is at last character of current row but next row exists
            let next_row = self.rows.remove(at.y + 1); // remove row
            let row = self.rows.get_mut(at.y).unwrap();
            row.append_row(&next_row); // add contents to previous row
        } else {
            let row = self.rows.get_mut(at.y).unwrap();
            row.delete(at.x);
        }
    }
    pub fn save(&mut self) -> Result<(), Error> {
        if let Some(file_name) = &self.file_name {
            let mut file = fs::File::create(file_name)?;
            for row in &self.rows {
                file.write_all(row.as_bytes())?;
                file.write_all(b"\n")?;
            }
        }
        self.modified = false;
        Ok(())
    }
}
