use wasm_bindgen::__rt::std::collections::HashMap;
use crate::sys;
pub fn _Code_ECFG() -> String {
    return String::from("EW012042");
}

// 两个入参都是数组，item是hashmap
pub fn _getEW_num(_cfg : HashMap<u32,String>, _ew_nums: HashMap<u32, String> )-> String  {
    let mut _temp = Vec::new();
    for i in _cfg.keys() {
        if i > &0 {
            _temp.push(i);
        }
    }
    if !sys::_isEmpty(&_temp) {
        _ew_nums.get(&sys::_toInt(&sys::_first(&_temp).unwrap().to_string())).unwrap().clone()
    } else {
        "".to_string()
    }

}

pub fn _Int_LI_10P_AC() -> String {
    "1".to_string()
}

pub fn _Int_LI_28P_AC() -> String {
    "4".to_string()
}

pub fn _Int_LI_28P_DC() ->String {
    "5".to_string()
}

pub fn _Int_LI_28P_S_AC() -> String {
    "2".to_string()
}

pub fn _Int_LI_52P_AC() ->String {
    "6".to_string()
}

pub fn _Int_LI_52P_DC() -> String {
    "7".to_string()
}

pub mod test {
    use super::*;
    #[test]
    fn test1() {
        let mut has1 = HashMap::new();
        has1.insert(1,"2".to_owned());
        let mut has2 = HashMap::new();
        has2.insert(1,"2".to_owned());
        let a = _getEW_num(has1,has2);
        assert_eq!(a, "2".to_string());
    }
}