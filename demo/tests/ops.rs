#[test]
fn add_works() { assert_eq!(demo::add(2, 3), 5); }
#[test]
fn mul_works() { assert_eq!(demo::mul(2, 3), 6); assert_eq!(demo::mul(0, 9), 0); }
#[test]
fn div_works() { assert_eq!(demo::div(6, 3), 2); }
