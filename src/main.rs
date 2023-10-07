extern crate core;

mod editor;
mod terminal;
mod row;
mod doc;
mod highlighting;

pub use terminal::Terminal;
pub use editor::Position;
pub use row::Row;
pub use doc::Doc;
use crate::editor::Editor;

fn main() {
    Editor::default().init();
}
