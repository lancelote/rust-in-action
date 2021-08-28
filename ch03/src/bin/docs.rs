//! Simulating files one step at a time.

use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(Debug, PartialEq)]
pub enum FileState {
    Open,
    Closed,
}

/// Represents a "file",
/// which probably lives on a file system.
#[derive(Debug)]
pub struct File {
    pub name: String,
    data: Vec<u8>,
    pub state: FileState,
}

impl File {
    /// New files are assumed to be empty, but a name is required.
    pub fn new(name: &str) -> File {
        File {
            name: String::from(name),
            data: Vec::new(),
            state: FileState::Closed,
        }
    }

    /// Returns the file's length in bytes.
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Checks if the file is empty.
    pub fn is_empty(&self) -> bool {
        self.data.len() == 0
    }

    /// Returns the file's name.
    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn read(self: &File, save_to: &mut Vec<u8>) -> Result<usize, String> {
        if self.state != FileState::Open {
            return Err(String::from("File must be open for reading"));
        }
        let mut tmp = self.data.clone();
        let read_length = tmp.len();
        save_to.reserve(read_length);
        save_to.append(&mut tmp);
        Ok(read_length)
    }
}

impl Display for FileState {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self {
            FileState::Open => write!(f, "OPEN"),
            FileState::Closed => write!(f, "CLOSED"),
        }
    }
}

impl Display for File {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "<{} ({})>", self.name, self.state)
    }
}

pub fn open(mut file: File) -> Result<File, String> {
    file.state = FileState::Open;
    Ok(file)
}

pub fn close(mut file: File) -> Result<File, String> {
    file.state = FileState::Closed;
    Ok(file)
}

fn main() {
    let file = File::new("5.txt");

    let file_name = file.name();
    let file_length = file.len();

    println!("{:?}", file);
    println!("{} is {} bytes long", file_name, file_length)
}
