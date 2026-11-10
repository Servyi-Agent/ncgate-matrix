#[test]
fn add_works() { assert_eq!(demo::add(2, 3), 5); assert_eq!(demo::add(20, 22), 42); }
#[test]
fn mul_works() { assert_eq!(demo::mul(2, 3), 6); assert_eq!(demo::mul(9, 9), 81); }
#[test]
fn div_works() { assert_eq!(demo::div(6, 3), 2); }
