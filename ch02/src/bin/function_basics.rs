fn main() {
    let a = 10;
    let b: i32 = 20;
    let c = 30i32;
    let d = 30_i32;
    let e = add(add(a, b), add(c, d));
    println!("( a + b ) + ( c + d ) = {}", e);
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}
