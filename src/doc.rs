use std::fs;
use std::io::Error;

use crate::Row;

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
}
