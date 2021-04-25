#[derive(Debug)]
struct File {
    name: String,
    data: Vec<u8>,
}

fn main() {
    let file = File {
        name: String::from("file.txt"),
        data: Vec::new(),
    };

    let file_name = &file.name;
    let file_length = &file.data.len();

    println!("{:?}", file);
    println!("{} is {} bytes long", file_name, file_length);
}
