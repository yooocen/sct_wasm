use math::round;
pub mod pde;
pub fn _objectEqual<T>(a: T, b: T) -> bool
    where T : PartialEq{
    if a.eq(&b) {
        true
    } else {
        false
    }
}
pub fn _toInt(str : &str) -> u32 {
    let num = str.parse().unwrap();
    round::floor(num, 0) as u32
}

