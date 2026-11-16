#[test]
fn add_works() { assert_eq!(demo::add(2, 3), 5); }
#[test]
fn add_roundtrip_works() { assert_eq!(demo::add(20, 3) - 3, 20); assert_eq!(demo::add(0, 7), 7); }
#[test]
fn mul_works() { assert_eq!(demo::mul(2, 3), 6); }
#[test]
fn div_works() { assert_eq!(demo::div(6, 3), 2); }
