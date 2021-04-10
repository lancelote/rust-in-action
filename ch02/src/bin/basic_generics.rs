use std::ops::Add;
use std::time::Duration;

fn main() {
    println!("{:?}", add_with_generics(1, 2));
    println!("{:?}", add_with_generics(1.1, 2.2));

    let duration1 = Duration::new(5, 0);
    let duration2 = Duration::new(10, 0);
    println!("{:?}", add_with_generics(duration1, duration2));
}

fn add_with_generics<T: Add<Output = T>>(i: T, j: T) -> T {
    i + j
}
