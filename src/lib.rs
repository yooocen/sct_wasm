#![feature(vec_remove_item)]

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
use crate::sys::Param::{ParamData, simulateData, getManualValue, OutParamData};
use crate::MW::callOneAlgo;
use crate::MW::unistar_MW007312Parameter::unistar_MWParameter;
use crate::sys::_solution;
use crate::Algorithm::AlgoEngine::_doBatchExecute;

pub mod sys;
mod MW;
mod Algorithm;


#[macro_use]
extern crate serde_derive;


#[wasm_bindgen]
impl ParamData {
    pub fn new() -> Self {
        simulateData()
    }

    pub fn test1(&self) -> JsValue {
        let mut getP = |paramCode: &str| -> String {
            getManualValue(self, paramCode)
        };

        let mwParameterObj = unistar_MWParameter {
            getP: &mut getP,
            _Solution: _solution::new(),
        };
        let res = _doBatchExecute("P_5720EI_36C_AC", &mwParameterObj);
        JsValue::from_serde(&res).unwrap()
    }

    pub fn setResult(&mut self, updata: &JsValue) -> bool {
        let updata2Self : HashMap<String, String> = updata.into_serde().unwrap();
        let mut isWork = false;
        for updataItem in updata2Self {
            isWork = self.setParaDatasArray("", updataItem.0 , updataItem.1);
        }
        isWork
    }

    pub fn getData(&self) -> JsValue {
        JsValue::from_serde(self).unwrap()
    }

}

//impl ParamData {
//    pub fn setResult(&mut self, updata: HashMap<String, String>) -> bool {
//        let mut isWork = false;
//        for updataItem in updata {
//            isWork = self.setParaDatasArray("", updataItem.0 , updataItem.1);
//        }
//        isWork
//    }
//}


#[cfg(test)]
pub mod test {
    use super::simulateData;
    use crate::sys::Param::ParamData;
    use wasm_bindgen::__rt::std::collections::HashMap;

    #[test]
    fn test1() {
        let mut param = ParamData::new();
        param.test1();
//        let mut t: HashMap<String, String> = HashMap::new();
//        t.insert("P_Is_GEto10GE".to_string(), "0".to_string());
//        param.setResult(t);

    }

    #[test]
    fn test2() {
        let mut param = vec![1, 2, 3];
        param[0] = 2;
        assert_eq!(param[0], 2);
    }
}


