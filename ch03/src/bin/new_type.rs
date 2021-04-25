#[derive(PartialEq)]
struct Hostname(String);

fn main() {
    let ordinary_string = String::from("localhost");

    let host1 = Hostname(ordinary_string.clone());
    let host2 = Hostname(ordinary_string);

    if host1 == host2 {
        println!("equal!");
    };
}
