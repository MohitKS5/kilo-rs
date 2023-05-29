mod editor;
mod terminal;

pub use terminal::Terminal;
use crate::editor::Editor;

fn main() {
    let editor = Editor::new();
    editor.init();
}
