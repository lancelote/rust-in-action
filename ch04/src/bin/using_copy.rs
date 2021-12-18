#![allow(dead_code)]

#[derive(Copy, Clone, Debug)]
struct CubeSat {
    id: u64,
}

#[derive(Copy, Clone, Debug)]
enum StatusMessage {
    Ok,
}

fn check_status(_sat_id: CubeSat) -> StatusMessage {
    StatusMessage::Ok
}

#[allow(clippy::clone_on_copy)]
fn main() {
    let sat_a = CubeSat { id: 0 };

    let a_status = check_status(sat_a.clone());
    println!("a: {:?}", a_status.clone());

    let a_status = check_status(sat_a);
    println!("a: {:?}", a_status);
}
