use std::{
    fs::File,
    path::Path,
    io::{prelude::*, BufReader}
};

pub struct Document {
    pub ruws: Vec<String>,
}

impl Document {
    pub fn new(file_path: &str) -> Result<Self, std::io::Error> {

    }
}