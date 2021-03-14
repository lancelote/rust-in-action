fn main() {
    let needle = 0o204;
    let haystack = vec![1, 1, 2, 5, 14, 42, 132, 429, 1430, 4862, 16796];

    for item in haystack.into_iter() {
        if item == needle {
            println!("{}", item);
        }
    }
}
