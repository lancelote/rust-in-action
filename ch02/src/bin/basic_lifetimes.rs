fn main() {
    let a = 10;
    let b = 20;
    let res = add_with_lifetimes(&a, &b);
    println!("{}", res);
}

#[allow(clippy::needless_lifetimes)]
fn add_with_lifetimes<'a, 'b>(i: &'a i32, j: &'b i32) -> i32 {
    *i + *j
}
