use math::round;
use wasm_bindgen::__rt::std::collections::HashMap;

pub mod pde;
pub fn _objectEqual<T>(a: T, b: T) -> bool
    where T : PartialEq{
    if a.eq(&b) {
        true
    } else {
        false
    }
}
pub fn _toInt(str : &String) -> u32 {
    let num = str.parse().unwrap();
    round::floor(num, 0) as u32
}
pub fn _isEmpty<T>(list:&Vec<T>) -> bool {
    if list.len() == 0 {
        false
    } else {
        true
    }
}

pub fn _first<T>(nums : &Vec<T>) -> Option<&T> {
    if nums.len() > 0 {
        nums.get(0)
    } else {
        None
    }
}

pub fn _keys<T, U>(maps : Vec<HashMap<T, U>>) {
    for i in maps.iter()     {
        i.get_key_value()
    }
}

pub fn tt(a : &String)  {
    println!("{}", a)
}

#[cfg(test)]
pub mod test {
    use super::*;
    #[test]
    fn test1() {
        let mut ve1 = vec![1,2,3,4];
        tt(&"nihao".to_owned());
        assert_eq!(_first(&ve1),Some(&1))
    }
}