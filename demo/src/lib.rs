pub fn add(a: u32, b: u32) -> u32 { a + b }
pub fn mul(a: u32, b: u32) -> u32 { a * b }
pub fn div(a: u32, b: u32) -> u32 { if b == 0 { 0 } else { a / b } }

#[cfg(test)]
mod tests {
    #[test]
    fn inline_add_works() { assert_eq!(super::add(1, 2), 3); }
}
