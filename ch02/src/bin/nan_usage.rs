#[allow(clippy::cmp_nan, clippy::eq_op)]
fn main() {
    assert_eq!(f32::NAN == f32::NAN, false);
}
