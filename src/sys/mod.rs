
use math::round;
pub mod Param;
pub mod pde;
pub mod EngineHelper;
pub fn _objectEqual<T>(a: T, b: T) -> bool
    where T : PartialEq{
    if a.eq(&b) {
        true
    } else {
        false
    }
}
pub fn _toInt(str : &String) -> i32 {
    let num = str.parse().unwrap();
    round::floor(num, 0) as i32
}
pub fn _isEmpty<T>(list:&Vec<T>) -> bool {
    if list.len() == 0 {
        true
    } else {
        false
    }
}

pub fn _first<T>(nums : &Vec<T>) -> Option<&T> {
    if nums.len() > 0 {
        nums.get(0)
    } else {
        None
    }
}

pub fn getP(paramCode : &str) -> String {
    "0".to_string()
}


pub fn _keys(mapList:&MapVec ) -> StdVec<String> {
    let mut res = StdVec::new();
    for map in mapList.iter() {
        res.push(map.key.clone());
    }
    res
}

#[derive(Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Map {
    pub(crate) key: String,
    value: String
}

impl Map {
    pub fn new( key: String, value: String ) -> Map {
        Map {
            key : key,
            value : value
        }
    }
}


use std::ops::*;
use std::slice::Iter;

pub type StdVec<T> = std::vec::Vec<T>;
#[derive(Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MapVec(pub StdVec<Map>);


impl MapVec {
    pub fn getValue(&self, key : &String) -> String {
        let mut res = "".to_string();
        for i in self.iter() {
            if i.key == *key {
                res = i.value.clone();
            }
        }
        res
    }
    pub fn len(&self)-> usize {
        self.0.len()
    }
    pub fn push(&mut self, item: Map) {
        self.0.push(item)
    }
    pub fn new() -> Self{
        MapVec(StdVec::new())
    }
    pub fn iter(&self) -> Iter<'_, Map> {
        self.0.iter()
    }

}

impl Deref for MapVec {
    type Target = StdVec<Map>;
    fn deref(&self) -> &StdVec<Map> {
        &self.0
    }
}

impl DerefMut for MapVec {
    fn deref_mut(&mut self) -> &mut StdVec<Map> {
        &mut self.0
    }
}

#[cfg(test)]
pub mod test {
    use super::*;
    #[test]
    fn test1() {
//        let mut ve1 = vec![1,2,3,4];
//        tt(&"nihao".to_owned());
//        assert_eq!(_first(&ve1),Some(&1))
    }
}

pub struct _solution{
    pub country : String,
    pub dataVersionCode: String,
    fdnId :String,
    productLine: String,
    office: String,
    region: String,

}

impl _solution {
    pub fn new()->Self {
        _solution {
            country: "1790".to_string(),
            dataVersionCode: "0".to_string(),
            fdnId :"0".to_string(),
            productLine:"0".to_string(),
            office: "0".to_string(),
            region:"0".to_string(),
        }

    }
}