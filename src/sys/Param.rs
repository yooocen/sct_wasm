use std::collections::HashMap;
use crate::sys::_toInt;

extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Serialize, Deserialize)]
pub struct ParamData {
    parameterEnum: HashMap<String, Vec<String>>,
    paraDatasArray: HashMap<String, HashMap<String, Vec<String>>>,
}

#[wasm_bindgen]
#[derive(Serialize, Deserialize)]
pub struct OutParamData {
    updateData: HashMap<String, String>
}

#[wasm_bindgen]
impl OutParamData {
    pub fn new() -> Self {
        OutParamData {
            updateData: HashMap::new()
        }
    }

    pub fn setResult(&mut self, paramCode : String, result: String) {
        self.updateData.insert(paramCode, result);
    }
}

impl ParamData {
    pub fn setParaDatasArray(&mut self, modelId: &str, paramCode: String, result: String) -> bool {
        let tmp = self.paraDatasArray.get_mut(modelId).unwrap();
        let tmp2 = tmp.get_mut(paramCode.as_str());
        match tmp2 {
            Some(data) => {
                data[0] = result;
                true
            },
            None => {
                false
            }
        }
    }
}

pub fn simulateData() -> ParamData {
    let data = r#"{
        "parameterEnum": {"PL_1": ["0"]},
        "paraDatasArray": {
            "": {
                "P_Is_GEto10GE": ["211", "211", "211", "2", "23;25;28;29;210;211;212;213", "0", "false", "0", "false", "0",  "null",  "null",  "1", "0", "0", "false",  "1",  "null"],
                "var_productCode": ["1", "1", "1",  "1", "1;2", "0", "false", "0", "false", "0",  "null",  "null",  "1", "0", "0", "false",  "1",  "null"]

            }
        }
    }"#;
    let d = serde_json::from_str(data).unwrap();
    d
}

pub fn getManualValue(data: &ParamData, paramCode: &str) -> String {
    let tempData = data.paraDatasArray.get("").unwrap();
    if !tempData.is_empty() && tempData.contains_key(&paramCode.to_string()) {
        let dataVal = tempData.get(paramCode).unwrap();
        let paraVal = dataVal.get(0).unwrap();
        let isMulti = dataVal.get(0).unwrap();
        let dataType = dataVal.get(0).unwrap();
        if isMulti.eq("false") {
            getPbyType(paraVal, dataType)
        } else {
            "0".to_string()
        }
    } else {
        "0".to_string()
    }
}

pub fn getPbyType(paraVal: &String, dataType: &String) -> String {
    let res = paraVal.clone();
    if paraVal.contains(".") {
        _toInt(paraVal).to_string()
    } else {
        res
    }
}

#[cfg(test)]
pub mod test {
    use super::simulateData;
    use crate::sys::Param::ParamData;

    #[test]
    fn test1() {
        let aa: ParamData = simulateData();
        assert_eq!(aa.paraDatasArray.get("").unwrap().get("P_Version_Default").unwrap().get(0), Some(&"211".to_string()));
    }
}
