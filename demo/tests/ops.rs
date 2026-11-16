#[test]
fn add_works() { assert_eq!(demo::add(2, 3), 5); }
#[test]
fn mul_works() { assert_eq!(demo::mul(2, 3), 6); }
#[test]
fn div_works() { assert_eq!(demo::div(6, 3), 2); }

#[test]
fn sat_add_saturates() { assert_eq!(demo::sat_add(u32::MAX, 1), u32::MAX); }
