use std::io::{self, Read};
use termion::raw::{RawTerminal, IntoRawMode};

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

    pub fn run(&mut self) -> Result<(), io::Error> {
        while !self.exit {
            println!("Please input key!\r");
            self.process_key()?;
        }

        println!("goodbye!\r");
        self.stdout.flush();
    }

    fn process_key(&mut self) -> Result<(), io::Error> {}
}