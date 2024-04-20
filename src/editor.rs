use std::io::{self, Read};
use termion::{
    event::Key,
    input::TermRead,
    raw::{RawTerminal, IntoRawMode}
};

const EXIT_CHARACTER: char = "q";

pub struct Editor {
    exit: bool,
    stdout: RawTerminal<Stdout>,
}

impl Editor {
    pub fn new() -> Result<Self, io::Error> {
        Ok(Editor {
            exit: false,
            stdout: io::stdout().into_raw_mode()?,
        })
    }

    /// Starts a loop where at each iteration we will draw 
    /// on the screen and blocks the loop until processes new keys
    pub fn run(&mut self) -> Result<(), io::Error> {
        while !self.exit {
            println!("Please input key!\r");
            self.process_key()?;
        }

        println!("goodbye!\r");
        self.stdout.flush();
    }

    /// When pressing ctrl+q changes the to true what end the programm
    /// When you click a symbol, we display it on the screen
    fn process_key(&mut self) -> Result<(), io::Error> {
        match self.next_key()? {
            Key::Ctrl(EXIT_CHARACTER) => { self.exit = true; },
            Key::Char(c) => { println!("Your input: {}\r", c); },
            _ => ()
        }
        Ok(())
    }

    fn next_key(&self) -> Result<Key, io::Error> {
        match io::stdin().keys().next() {
            Some(key) => key,
            None => Err(io::Error::New(
                io::ErrorKind::Other,
                "Invalid input"
            ))
        }
    }
}