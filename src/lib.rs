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
pub mod sys;
mod MW;
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

#[derive(Serialize, Deserialize)]
struct Person {
    name: String,
    age: String,
    phones: Vec<String>,
}

#[wasm_bindgen]
pub fn simulate() -> JsValue {
    let data = r#"
        {
            "name": "John Doe",
            "age": "43",
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ]
        }"#;
    let v: Person = serde_json::from_str(data).unwrap();

    let mut str = String::from("");
    for i in 0..10000 {

//    let name = serde_wasm_bindgen::to_value(&"John Doe").unwrap();


        if v.name == "John Doe" {
            str = "John Doe".to_owned() + "123";
        } else {
            str ="John Doe".to_owned()+ "123";
        }

    }
    let returnVal : JsValue = serde_wasm_bindgen::to_value(&str).unwrap();
    returnVal
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

    #[wasm_bindgen]
    pub fn bianli(&self) -> i32{
        let mut count: i32 = 0;
        for (key, value) in self.parmEnumList.iter() {
            count+=value[0];
        }
        count
    }
}


#[wasm_bindgen]
pub fn fib(i: u32) -> u32 {
    match i {
        0 => 0,
        1 => 1,
        _ => fib(i-1) + fib(i-2)
    }
}


#[wasm_bindgen]
pub fn fibonacci(num: u32) -> u32{
    let mut a: u32;
    let mut b:u32 = 0;
//        let nn : u32= serde_wasm_bindgen::from_value(num).unwrap();
    let n : u32 = num;
    let mut  count = 0;
    for i in 0..10000000 {
        a = 0;
        b = 1;
        for i in 0..45 {

            let c = a + b;
            a = b;
            b = c;
            count+=1;
        }
    }
    count+b
}

#[wasm_bindgen]
pub fn testEqual() -> i32{
//    let mut a  = HashMap::new();
//    a.insert(1,[1,2,3]);
//    let aa = sys::HashMap(a);
//    let mut b  = HashMap::new();
//    b.insert(1,[1,2,4]);
//    let bb = sys::HashMap(b);
//    sys::_objectEqual([1,2,3],[1,2,3])
    sys::_toInt(&"12.3".to_owned())

}


#[wasm_bindgen]
pub fn algo1() {
//    for i in 0..10000 {

        MW::unistar_MW007312Parameter::var_productCode_Config();
//    }
}

