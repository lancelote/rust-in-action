const BIAS: i32 = 127;
const RADIX: f32 = 2.0;

fn main() {
    let n: f32 = 42.42;

    let sign_bit = get_sign_part(n);
    let exponent_bits = get_exponent_part(n);
    let fraction_bits = get_fraction_part(n);

    let sign = decode_sign(sign_bit);
    let exponent = decode_exponent(exponent_bits);
    let mantissa = decode_fraction(fraction_bits);

    let result = from_parts(sign, exponent, mantissa);

    let sign_bit_str = format!("{:01b}", sign_bit);
    let exponent_bits_str = format!("{:08b}", exponent_bits);
    let fraction_bits_str = format!("{:023b}", fraction_bits);

    println!("{} -> {}", n, result);
    println!("field    | {:23} | as real number", "as bits");
    println!("sign     | {:23} | {}", sign_bit_str, sign);
    println!("exponent | {:23} | {}", exponent_bits_str, exponent);
    println!("mantissa | {:23} | {}", fraction_bits_str, mantissa);
}

fn get_sign_part(n: f32) -> u32 {
    n.to_bits() >> 31 & 1
}

fn get_exponent_part(n: f32) -> u32 {
    n.to_bits() >> 23 & 0xff
}

fn get_fraction_part(n: f32) -> u32 {
    n.to_bits() & 0x7fffff
}

fn from_parts(sign: f32, exponent: f32, mantissa: f32) -> f32 {
    sign * exponent * mantissa
}

fn decode_sign(sign_bit: u32) -> f32 {
    (-1.0_f32).powf(sign_bit as f32)
}

fn decode_exponent(exponent_bits: u32) -> f32 {
    let exponent = (exponent_bits as i32) - BIAS;
    RADIX.powf(exponent as f32)
}

fn decode_fraction(fraction_bits: u32) -> f32 {
    let mut mantissa: f32 = 1.0;

    for i in 0..23 {
        let mask = 1 << i;
        let one_at_bit_i = fraction_bits & mask;
        if one_at_bit_i != 0 {
            let weight = 2_f32.powf(i as f32 - 23.0);
            mantissa += weight
        }
    }

    mantissa
}
