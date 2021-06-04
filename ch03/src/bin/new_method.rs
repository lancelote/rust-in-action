#[derive(Debug)]
struct File {
    name: String,
    data: Vec<u8>,
}

impl File {
    fn new(name: &str) -> File {
        File {
            name: String::from(name),
            data: Vec::new(),
        }
    }
}

fn main() {
    let file = File::new("file.txt");

    let file_name = &file.name;
    let file_data_length = file.data.len();

    println!("{:?}", file);
    println!("{} is {} bytes long", file_name, file_data_length);
}
