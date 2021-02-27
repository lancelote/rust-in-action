fn main() {
    let penguin_data = "\
    common name,length (cm)
    Little penguin,33
    Yellow-eyed penguin,65
    Fiordland penguin,60
    Invalid,data
    ";

    let records = penguin_data.lines();

    for (i, record) in records.enumerate() {
        if i == 0 || record.trim().len() == 0 {
            continue;
        }

        let fields: Vec<&str> = record
            .split(",")
            .map(|field| field.trim())
            .collect();

        if cfg!(debug_assertions) {
            eprintln!("debug: {:?} -> {:?}", record, fields);
        }

        let name = fields[0];
        let maybe_length: Result<f32, _> = fields[1].parse();

        if maybe_length.is_err() {
            continue;
        }

        let length = maybe_length.unwrap();
        println!("{}, {}cm", name, length);
    }
}
