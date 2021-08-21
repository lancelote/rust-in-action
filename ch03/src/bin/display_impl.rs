use std::fmt;
use std::fmt::{Display, Formatter};

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

impl File {
    fn new(name: &str) -> File {
        File {
            name: String::from(name),
            data: Vec::new(),
            state: FileState::Closed,
        }
    }

    fn read(self: &File, save_to: &mut Vec<u8>) -> Result<usize, String> {
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

fn open(mut file: File) -> Result<File, String> {
    file.state = FileState::Open;
    Ok(file)
}

fn close(mut file: File) -> Result<File, String> {
    file.state = FileState::Closed;
    Ok(file)
}

fn main() {
    let mut file = File::new("5.txt");
    let mut buffer: Vec<u8> = vec![];

    if file.read(&mut buffer).is_err() {
        println!("Error checking is working");
    }

    file = open(file).unwrap();
    let file_length = file.read(&mut buffer).unwrap();
    file = close(file).unwrap();

    let text = String::from_utf8_lossy(&buffer);

    println!("{:?}", file);
    println!("{}", file);
    println!("{} is {} bytes long", &file.name, file_length);
    println!("{}", text);
}
