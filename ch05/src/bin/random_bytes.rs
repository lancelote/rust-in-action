#[allow(clippy::unusual_byte_groupings)]
fn mock_rand(n: u8) -> f32 {
    // exponent -1 = 0b01111110 (126 in base 10)
    let base: u32 = 0b0_01111110_00000000000000000000000;
    //                  └───┬──┘ └────────┬────────────┘
    //                  exponent       mantissa
    println!("base    : {:032b}", base);

    println!("n_as_u32: {:032b}", n as u32);

    let large_n = (n as u32) << 15;
    println!("large_n : {:032b}", large_n);

    let f32_bits = base | large_n;
    println!("f32_bits: {:032b}", f32_bits);

    let m = f32::from_bits(f32_bits);
    println!("m       : {:032b}", m.to_bits());

    2.0 * (m - 0.5) // normalize the input
}

fn main() {
    println!("{}", mock_rand(123));
}
