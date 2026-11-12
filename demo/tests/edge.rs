#[test]
fn div_by_zero_is_saturating() { assert_eq!(demo::div(1, 0), 0); assert_eq!(demo::div(2, 0), 0); }
