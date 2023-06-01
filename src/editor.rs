use std::io::{stdout, Error};

use termion::event::Key;

use crate::terminal::Size;
use crate::{Doc, Row, Terminal};

#[derive(Default)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

pub struct Editor {
    terminal: Terminal,
    cursor_position: Position,
    doc: Doc
}
const VERSION: &str = env!("CARGO_PKG_VERSION");
impl Editor {
    pub fn default() -> Self {
        Self {
            terminal: Terminal::default().unwrap(),
            cursor_position: Position::default(),
            doc: Doc::default()
        }
    }
    pub fn init(&mut self) {
        loop {
            if let Err(err) = self.refresh_screen() {
                die(&err)
            }
            self.draw_lines();
            Terminal::position_cursor_at(&self.cursor_position);
            Terminal::flush().unwrap();
            if let Err(error) = self.on_key_press() {
                die(&error)
            }
        }
    }

    fn move_cursor(&mut self, key: Key) {
        let Position { mut y, mut x } = self.cursor_position;
        let Size { width, height } = self.terminal.size();
        let height = height.saturating_sub(1) as usize;
        let width = width.saturating_sub(1) as usize;
        match key {
            Key::Up => y = y.saturating_sub(1),
            Key::Down => {
                if y < height {
                    y = y.saturating_add(1)
                }
            },
            Key::Left => x = x.saturating_sub(1),
            Key::Right => {
                if x<width {
                    x = x.saturating_add(1)
                }
            },
            Key::PageUp => y = 0,
            Key::PageDown => y = height,
            Key::Home => x = 0,
            Key::End => x = width,
            _ => (),
        }
        self.cursor_position = Position { x, y }
    }

    fn on_key_press(&mut self) -> Result<(), Error> {
        let key = Terminal::read_key();
        match key {
            Ok(key) => match key {
                Key::Ctrl('q') => self.quit(),
                Key::Up
                | Key::Down
                | Key::Left
                | Key::Right
                | Key::PageUp
                | Key::PageDown
                | Key::End
                | Key::Home => self.move_cursor(key),
                _ => {}
            },
            Err(e) => die(&e),
        }
        Ok(())
    }
    fn quit(&self) {
        Terminal::clear_screen();
        std::process::exit(0)
    }
    fn refresh_screen(&self) -> Result<(), Error> {
        Terminal::hide_cursor();
        Terminal::clear_screen();
        Terminal::position_cursor_at(&Position::default());
        Terminal::show_cursor();
        Terminal::flush()
    }

    fn draw_welcome_message(&self) {
        let mut welcome_message = format!("Kilo-rs editor -- version {}", VERSION);
        let width = self.terminal.size().width as usize;
        let len = welcome_message.len();
        let padding = width.saturating_sub(len) / 2;
        let spaces = " ".repeat(padding.saturating_sub(1));
        welcome_message = format!("~{}{}", spaces, welcome_message);
        welcome_message.truncate(width);
        println!("{}\r", welcome_message);
    }

    fn draw_lines(&self) {
        let h = self.terminal.size().height;
        for i in 0..h {
            Terminal::clear_current_line();
            if h / 3 == i {
                self.draw_welcome_message()
            } else {
                print!("~\r\n")
            }
        }
    }
}

fn die(e: &Error) {
    panic!("{}", e);
}
