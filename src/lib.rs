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
use crate::sys::Param::{ParamData, simulateData, getManualValue};
use crate::MW::callOneAlgo;
use crate::MW::unistar_MW007312Parameter::unistar_MWParameter;
use crate::sys::_solution;

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

    pub fn test1(&self) -> String {
        let mut getP = |paramCode: &str| -> String {
            getManualValue(self, paramCode)
        };
        let mwParameterObj = unistar_MWParameter {
            getP: &mut getP,
            _Solution: _solution::new()
        };
        callOneAlgo("PL_1", &mwParameterObj);
        callOneAlgo("PL_2", &mwParameterObj)
    }
}


#[cfg(test)]
pub mod test {
    use super::simulateData;
    use crate::sys::Param::ParamData;

    #[test]
    fn test1() {
        let param = ParamData::new();
        param.test1();
    }

}


