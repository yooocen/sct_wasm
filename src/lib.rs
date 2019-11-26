extern crate cfg_if;
extern crate wasm_bindgen;
extern crate web_sys;
extern crate js_sys;
extern crate serde_wasm_bindgen;
mod utils;

use std::fmt;
use wasm_bindgen::prelude::*;
use web_sys::console;
use std::collections::{BTreeMap, HashMap};
use serde_json::{Result, Value};

#[macro_use]
extern crate serde_derive;

#[derive(Serialize, Deserialize)]
pub struct ParamData {
    pub parmEnumList: BTreeMap<String,Vec<i32>>,
}

#[wasm_bindgen]
pub struct Param {
    parmEnumList : BTreeMap<String,Vec<i32>>,
}

#[wasm_bindgen]
impl Param {
    pub fn new(paramEnumList: JsValue) -> Param {
        let parmEnumList = serde_wasm_bindgen::from_value(paramEnumList).unwrap();
        Param {
            parmEnumList
        }
    }
    pub fn GetP(&self, code: JsValue) -> JsValue {
        let paramCode : String = serde_wasm_bindgen::from_value(code).unwrap();
        let val : &Vec<i32> = self.parmEnumList.get(&paramCode).unwrap();
        let jsVal = serde_wasm_bindgen::to_value(&val).unwrap();
//        console::log_1(jsVal);
        jsVal
    }

    pub fn GetPTestPerf(&self) -> JsValue{
//        let paramCode : String = serde_wasm_bindgen::from_value(code).unwrap();
        for i in 0..1000000 {
              self.parmEnumList.get("PL_1");
        }
        serde_wasm_bindgen::to_value(self.parmEnumList.get("PL_1").unwrap()).unwrap()

    }

    pub fn fibonacci(&self, num: JsValue) -> JsValue{
        let mut a: u32;
        let mut b:u32 = 0;
//        let nn : u32= serde_wasm_bindgen::from_value(num).unwrap();
        for i in 0..1000000 {
            a = 0;
            b = 1;
            let n : u32 = 12;
            // Use a and b to store the previous two values in the sequence

            for i in 0..n {
                // As we iterate through, move b's value into a and the new computed
                // value into b.
                let c = a + b;
                a = b;
                b = c;
            }
        }
        serde_wasm_bindgen::to_value(&b).unwrap()

//        console::log_1(&b.to_string().into())
    }



}


#[wasm_bindgen]
pub struct Custom {
    OwnedValue : JsValue
}

#[wasm_bindgen]
impl Custom {

    pub fn new( target:  JsValue) -> Custom {
        let obj = Custom {
            OwnedValue : target
        };
        obj
    }


    pub fn getJsValue(&self, property_key: JsValue ) -> JsValue{
        let mut value: JsValue =  js_sys::Reflect::get(&self.OwnedValue, &property_key).unwrap();
        for i in 0..1000 {
            value = js_sys::Reflect::get(&self.OwnedValue, &property_key).unwrap();
        }
        value
    }

    pub fn getJsValuefromInput(&self, target : JsValue,property_key: JsValue ) -> JsValue{
        let mut value: JsValue =  js_sys::Reflect::get(&target, &property_key).unwrap();
        for i in 0..1000000 {
            value = js_sys::Reflect::get(&target, &property_key).unwrap();
        }
        value
    }
}


#[wasm_bindgen]
pub fn get_value_from_js(value: JsValue) -> JsValue {
    let value :ParamData = serde_wasm_bindgen::from_value(value).unwrap();
    serde_wasm_bindgen::to_value(&value).unwrap()
}

#[wasm_bindgen]
pub fn tryOut1() -> JsValue{
    serde_wasm_bindgen::to_value(&12345678).unwrap()
}


#[wasm_bindgen]
pub fn tryOut2() -> JsValue{
    let a:u64 =123456789123;
    serde_wasm_bindgen::to_value(&a).unwrap()
}

#[wasm_bindgen]
pub fn tryOut3() -> JsValue{
    serde_wasm_bindgen::to_value(&12.23).unwrap()
}

#[wasm_bindgen]
pub fn tryOut4() -> JsValue{
    serde_wasm_bindgen::to_value(&123456789).unwrap()
}

#[wasm_bindgen]
pub fn untyped_example() -> JsValue {
    // Some JSON input data as a &str. Maybe this comes from the user.
    let data = r#"
        {
            "name": "John Doe",
            "age": 43,
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ]
        }"#;

    // Parse the string of data into serde_json::Value.
    let v: Value = serde_json::from_str(data).unwrap();
    serde_wasm_bindgen::to_value(&v).unwrap()
}


#[wasm_bindgen]
pub struct Pref {
    parmEnumList : HashMap<String,Vec<i32>>,
}

#[wasm_bindgen]
impl Pref {
    pub fn new(paramEnumList: JsValue) -> Pref {
        let parmEnumList = serde_wasm_bindgen::from_value(paramEnumList).unwrap();
        Pref {
            parmEnumList
        }
    }

    pub fn GetPTestPerf(&self) -> JsValue{
//        let paramCode : String = serde_wasm_bindgen::from_value(code).unwrap();
        for i in 0..1000000 {
            self.parmEnumList.get("PL_1");
        }
        serde_wasm_bindgen::to_value(self.parmEnumList.get("PL_1").unwrap()).unwrap()
    }
}
