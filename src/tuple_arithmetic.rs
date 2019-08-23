/// Treats `a` and `b` as points on a plane and finds a point on the segment
/// between them which divides this segment in ratio `ratio`:`1 - ratio`.
/// For example, the result is equal to `a` if `ratio` is `0.0` and
/// to `b` if `ratio` is `1.0`.
pub fn linear_interpolation(
    a: (impl Into<f64>, impl Into<f64>),
    b: (impl Into<f64>, impl Into<f64>),
    ratio: f32
) -> (f32, f32) {
    let ratio = f64::from(ratio);
    (
        (a.0.into() * (1.0 - ratio) + b.0.into() * ratio) as f32,
        (a.1.into() * (1.0 - ratio) + b.1.into() * ratio) as f32
    )
}

#[test]
fn test_linear_interpolation() {
    let a = (1u8, 1u16);
    let b = (2.0, 3i32);
    assert_eq!(linear_interpolation(a, b, 0.0), (1.0, 1.0));
    assert_eq!(linear_interpolation(a, b, 0.25), (1.25, 1.5));
    assert_eq!(linear_interpolation(a, b, 1.0), (2.0, 3.0));
}
