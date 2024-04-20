use std::io::{self, Read};
use termion::raw::{RawTerminal, IntoRawMode};

pub struct Editor {
    exit: bool,
    stdout: RawTerminal<Stdout>,
}