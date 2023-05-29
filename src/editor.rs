use std::io::{stdout, Error};

use termion::event::Key;

use crate::Terminal;

pub struct Editor {
    terminal: Terminal,
}
const VERSION: &str = env!("CARGO_PKG_VERSION");
impl Editor {
    pub fn new() -> Self {
        Self {
            terminal: Terminal::new().unwrap(),
        }
    }
    pub fn init(&self) {
        loop {
            if let Err(err) = self.refresh_screen() {
                die(&err)
            }
            self.draw_lines();
            Terminal::move_cursor(0, 0);
            Terminal::flush().unwrap();
            if let Err(error) = self.on_key_press() {
                die(&error)
            }
        }
    }

    fn on_key_press(&self) -> Result<(), Error> {
        let key = Terminal::read_key();
        match key {
            Ok(key) => match key {
                Key::Ctrl('q') => self.quit(),
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
        Terminal::move_cursor(0, 0);
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
