use std::io::{Error, stdin, Stdout, stdout, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::{IntoRawMode, RawTerminal};
use termion::{cursor, terminal_size};

pub struct Size {
    pub width: u16,
    pub height: u16,
}
pub struct Terminal {
    size: Size,
    _stdout: RawTerminal<Stdout>,
}
impl Terminal {
    pub fn new() -> Result<Self, Error>{
        let (width,height)= terminal_size()?;
        Ok(Self{
            size: Size {
                width,
                height,
            },
            _stdout: stdout().into_raw_mode().unwrap(),
        })
    }
    pub fn size(&self) -> &Size {
        &self.size
    }

    pub fn clear_screen() {
        print!("{}", termion::clear::All);
    }

    pub fn clear_current_line() {
        print!("{}", termion::clear::CurrentLine);
    }

    pub fn move_cursor(x: u16, y: u16) {
        let x = x.saturating_add(1);
        let y = y.saturating_add(1);
        print!("{}", cursor::Goto(x,y));
    }

    pub fn read_key() -> Result<Key, Error> {
        loop {
            if let Some(key) = stdin().lock().keys().next() {
                return key;
            }
        }
    }
    pub fn flush() -> Result<(), Error> {
        stdout().flush()
    }

    pub fn hide_cursor() {
        print!("{}", cursor::Hide);
    }
    pub fn show_cursor() {
        print!("{}", cursor::Show);
    }
}