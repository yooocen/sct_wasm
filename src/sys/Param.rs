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

pub fn simulateData() -> ParamData {
    let data = r#"{
        "parameterEnum": {"PL_1": ["0"]},
        "paraDatasArray": {
            "": {
                "P_Version_Default": ["211", "211", "211", "2", "23;25;28;29;210;211;212;213", "0", "false", "0", "false", "0",  "null",  "null",  "1", "0", "0", "false",  "1",  "null"],
                "P_NewExpand": ["1", "1", "1",  "1", "1;2", "0", "false", "0", "false", "0",  "null",  "null",  "1", "0", "0", "false",  "1",  "null"]
            }
        }
        }
    "#;
    let d = serde_json::from_str(data).unwrap();
    d
}

pub fn getManualValue(data: &ParamData, paramCode: &str)->String {
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
    }else {
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
        let aa :ParamData = simulateData();
        assert_eq!(aa.paraDatasArray.get("").unwrap().get("P_Version_Default").unwrap().get(0), Some(&"211".to_string()));
    }

}
