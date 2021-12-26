#[allow(clippy::transmute_int_to_float)]
#[allow(clippy::transmute_float_to_int)]
fn main() {
    let a: f32 = 42.42;
    let frankentype: u32 = unsafe { std::mem::transmute(a) };

    println!("{}", frankentype);
    println!("{:032b}", frankentype);

    let b: f32 = unsafe { std::mem::transmute(frankentype) };

    println!("{}", b);
    assert_eq!(a, b);
}
