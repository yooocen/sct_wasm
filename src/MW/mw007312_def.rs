use wasm_bindgen::__rt::std::collections::HashMap;

pub fn _Code_ECFG() -> String {
    return String::from("EW012042");
}
pub fn _getEW_num<T, U>(_cfg : HashMap<T,U>)->String {
    let _temp;
    let mut for_799486120 = Vec::new();
    for i in _cfg.keys() {
        if i > 0 {
            for_799486120.push(i);
        }
    }
    "sadf".to_owned()
}

this._getEW_num = function(_cfg, _ew_nums) {
var _temp;
var for_799486120 = [];
var _158039561 = sys._keys(_cfg);
for (var _629335111 = 0; _629335111 < _158039561.length; _629335111++) {
var _element = _158039561[_629335111];
if (_element > 0) {
for_799486120.push(_cfg.get(_element));
}
}
_temp = for_799486120;
if (!(sys._isEmpty(_temp))) {
return _ew_nums.get(sys._toInt(sys._first(_temp)));
} else {
return "";
}
}