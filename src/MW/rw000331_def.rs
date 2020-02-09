

pub fn _getNumOptical(_OpticalType: String, _list: Vec<String>, _number: String) -> String{
    if _list.contains(&_OpticalType) {
        _number
    } else {
        "0".to_string()
    }
}

pub fn _IntToBool( _fun: String) -> bool {
    if _fun.eq(&_Visible()) {
        true
    } else {
        false
    }
}

pub fn _Visible() -> String {
    "1".to_string()
}


pub fn _InVisible() -> String {
    "0".to_string()
}

pub fn _Yes() -> String {
    "1".to_string()
}

pub fn _SFPP_MM_850_D3() -> String {
    "1".to_string()
}

pub fn _SFPP_SM_1310_10() -> String {
    "2".to_string()
}
pub fn  _SFPP_SM_1550_40()  -> String {
    "3".to_string()
}


pub fn _SFPP_MM_1310_D22()  -> String {
    "4".to_string()
}pub fn _SFPP_MM_850_D1()  -> String {
    "6".to_string()
}pub fn _SFPP_SM_1550_80_SNN()  -> String {
    "7".to_string()
}pub fn _SFPP_SM_1310_40()  -> String {
    "10".to_string()
}
pub fn _SFPP_SM_1310_14() -> String {
    "9".to_string()
}

pub fn _SFPP_SM_1550_80() -> String {
    "5".to_string()
}



pub fn _SFPPOpt_NoJFE() -> Vec<String> {
    vec![_SFPP_MM_850_D3(), _SFPP_SM_1310_10(),_SFPP_SM_1550_40(), _SFPP_MM_850_D1(), _SFPP_SM_1550_80_SNN(),_SFPP_SM_1310_40() ]
}

pub fn _hasNotChoosed(_OpticalType:String, _list:Vec<String> )->bool {
    if _list.contains(&_OpticalType) {
        true
    } else {
        false
    }
}