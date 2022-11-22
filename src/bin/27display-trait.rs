#![allow(dead_code)]
use colored::{self, Colorize};
use std::fmt::{self, Display};
#[derive(Debug, PartialEq)]
enum FileState {
    Open,
    Closed,
}
#[derive(Debug)]
struct File {
    name: String,
    data: Vec<u8>,
    state: FileState,
}
impl Display for FileState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FileState::Open => write!(f, "{}", "Open".green().bold()),
            FileState::Closed => write!(f, "{}", "Closed".red().bold()),
        }
    }
}
impl Display for File {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "File: {} is {}", self.name.cyan().italic(), self.state)
    }
}
impl File {
    fn new(name: &str) -> Self {
        File {
            name: name.to_string(),
            data: Vec::new(),
            state: FileState::Closed,
        }
    }
}
fn main() {
    let mut f = File::new("hello.txt");
    f.state = FileState::Open;
    println!("{}", f);
}
