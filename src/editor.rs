use std::cmp::min;
use std::env;
use std::io::Error;
use std::process::exit;
use std::time::{Duration, Instant};

use termion::color;
use termion::event::Key;

use crate::{Doc, Row, Terminal};

#[derive(Default)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

impl Position {
    fn sub(&self, offset: &Self) -> Self {
        Self {
            x: self.x - offset.x,
            y: self.y - offset.y,
        }
    }
}

const STATUS_BG_COLOR: color::Rgb = color::Rgb(239, 239, 239);
const STATUS_FG_COLOR: color::Rgb = color::Rgb(63, 63, 63);
const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct StatusMessage {
    pub text: String,
    pub time: Instant,
}

impl StatusMessage {
    fn from(msg: String) -> Self {
        Self {
            text: msg,
            time: Instant::now(),
        }
    }
}
pub struct Editor {
    terminal: Terminal,
    cursor_position: Position,
    doc: Doc,
    offset: Position,
    exit: bool,
    status_message: StatusMessage,
}

impl Editor {
    pub fn default() -> Self {
        let args: Vec<String> = env::args().collect();
        let mut initial_message = "HELP: Ctrl-Q = quit".to_string();
        let doc = if args.len() > 1 {
            let filename = &args[1];
            match Doc::open(filename) {
                Ok(doc) => doc,
                Err(_) => {
                    initial_message = format!("ERR: Could not open file: {}", filename);
                    Doc::default()
                }
            }
        } else {
            Doc::default()
        };
        Self {
            terminal: Terminal::default().unwrap(),
            cursor_position: Position::default(),
            doc,
            offset: Position::default(),
            exit: false,
            status_message: StatusMessage::from(initial_message),
        }
    }
    pub fn init(&mut self) {
        loop {
            if let Err(err) = self.refresh_screen() {
                die(&err);
            }
            if self.exit {
                break;
            }
            self.draw_rows();
            self.draw_status_bar();
            self.draw_message_bar();
            Terminal::position_cursor_at(&self.cursor_position.sub(&self.offset));
            Terminal::flush().unwrap();
            self.on_key_press();
        }
    }

    fn draw_status_bar(&self) {
        let mut filename = "[Untitled]".to_string();
        if let Some(name) = &self.doc.file_name {
            filename = name.clone();
            filename.truncate(20);
        }
        let mut status = format!("{} - {}", filename, self.doc.len());
        let width = self.terminal.size().width as usize;
        let cursor_indicator = format!(
            "{}:{}",
            self.cursor_position.y.saturating_add(1),
            self.cursor_position.x.saturating_add(1),
        );
        let len = status.len() + cursor_indicator.len();
        if width > len {
            status.push_str(&*" ".repeat(width - len));
            status.push_str(&cursor_indicator);
        } else {
            status.truncate(width);
        }
        Terminal::set_bg_color(STATUS_BG_COLOR);
        Terminal::set_fg_color(STATUS_FG_COLOR);
        println!("{}\r", status);
        Terminal::reset_bg_color();
        Terminal::reset_fg_color();
    }

    fn draw_message_bar(&self) {
        Terminal::clear_current_line();
        let message = &self.status_message;
        if message.time + Duration::new(5, 0) > Instant::now() {
            let mut msg = message.text.clone();
            msg.truncate(self.terminal.size().width as usize);
            print!("{}", msg);
        }
    }

    fn get_row_len(&self, y: usize) -> usize {
        if let Some(row) = self.doc.row(y) {
            row.len()
        } else {
            0
        }
    }

    fn move_cursor(&mut self, key: Key) {
        let Position { mut y, mut x } = self.cursor_position;
        let width = self.get_row_len(y);
        let height = self.doc.len();
        let terminal_height = self.terminal.size().height as usize;
        match key {
            Key::Up => y = y.saturating_sub(1),
            Key::Down => y = min(height, y + 1),
            Key::Left => {
                x = {
                    if x > 0 {
                        x - 1
                    } else {
                        y = y.saturating_sub(1);
                        self.get_row_len(y)
                    }
                }
            }
            Key::Right => {
                x = if x < width {
                    x.saturating_add(1)
                } else {
                    y = min(y + 1, height);
                    0
                }
            }
            Key::PageUp | Key::Ctrl('u') => y = y.saturating_sub(terminal_height),
            Key::PageDown | Key::Ctrl('d') => y = min(height, y + terminal_height),
            Key::Home => x = 0,
            Key::End => x = width,
            _ => (),
        }
        let width = self.get_row_len(y);
        if x > width {
            x = width
        }
        self.cursor_position = Position { x, y }
    }

    fn on_key_press(&mut self) {
        let key = Terminal::read_key();
        match key {
            Ok(key) => match key {
                Key::Ctrl('q') => self.quit(),
                Key::Char(c) => {
                    self.doc.insert(&self.cursor_position, c);
                    self.move_cursor(Key::Right);
                },
                Key::Delete => self.doc.delete(&self.cursor_position),
                Key::Backspace => {
                    self.move_cursor(Key::Left);
                    self.doc.delete(&self.cursor_position);
                },
                Key::Up
                | Key::Down
                | Key::Left
                | Key::Right
                | Key::PageUp
                | Key::Ctrl('u')
                | Key::PageDown
                | Key::Ctrl('d')
                | Key::End
                | Key::Home => self.move_cursor(key),
                _ => {}
            },
            Err(e) => die(&e),
        };
        self.scroll();
    }

    fn scroll(&mut self) {
        let Position { x, y } = self.cursor_position;
        let width = self.terminal.size().width as usize;
        let height = self.terminal.size().height as usize;
        let mut offset = &mut self.offset;
        if y < offset.y {
            offset.y = y;
        } else if y >= offset.y.saturating_add(height) {
            offset.y = y.saturating_sub(height).saturating_add(1);
        }
        if x < offset.x {
            offset.x = x;
        } else if x >= offset.x.saturating_add(width) {
            offset.x = x.saturating_sub(width).saturating_add(1);
        }
    }
    fn quit(&mut self) {
        Terminal::clear_screen();
        self.exit = true;
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

    pub fn draw_row(&self, row: &Row) {
        let start = 0 as usize;
        let end = self.terminal.size().width as usize;
        let str = row.render(self.offset.x + start, self.offset.y + end);
        println!("{}\r", str);
    }

    fn draw_rows(&self) {
        let h = self.terminal.size().height;
        for i in 0..h {
            Terminal::clear_current_line();
            if let Some(row) = self.doc.row(self.offset.y + i as usize) {
                self.draw_row(row)
            } else if self.doc.is_empty() && h / 3 == i {
                self.draw_welcome_message()
            } else {
                print!("~\r\n");
            }
        }
    }
}

fn die(e: &Error) {
    panic!("{}", e);
}
