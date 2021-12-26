use std::mem::transmute;

fn to_chunks_of_8_bit(num: i32) -> String {
    let bits = format!("{:b}", num);
    bits.chars()
        .collect::<Vec<char>>()
        .chunks(8)
        .map(|chunk| chunk.iter().collect::<String>())
        .collect::<Vec<String>>()
        .join(" ")
}

fn main() {
    let big_endian: [u8; 4] = [0xAA, 0xBB, 0xCC, 0xDD];
    let little_endian: [u8; 4] = [0xDD, 0xCC, 0xBB, 0xAA];

    let a: i32 = unsafe { transmute(big_endian) };
    let b: i32 = unsafe { transmute(little_endian) };

    println!("0xAA = {:b}", 0xAA);

    println!("a    = {}", to_chunks_of_8_bit(a));
    println!("b    = {}", to_chunks_of_8_bit(b));

    println!();
    println!("{} vs {}", a, b);
}
