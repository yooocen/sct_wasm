use crate::MW::unistar_MW007312Parameter::unistar_MWParameter;
use crate::sys::EngineHelper::Relation;
use crate::sys::Param::{ParamData, OutParamData};
use crate::MW::callOneAlgo;
use wasm_bindgen::__rt::std::collections::HashMap;

pub struct algo {
    algoType: String,
    itemId: String,
}


pub fn getParamRelatedList<T: Fn(&str) -> String>(paramObj: &unistar_MWParameter<T>) -> Vec<Relation> {
    paramObj.paramRelatedList()
}


pub fn getParamRelation<T: Fn(&str) -> String>(paramObj: &unistar_MWParameter<T>, paramCode: &String) -> Vec<Relation> {
    let mut paramItem = vec![];
    let paramRelatedList = getParamRelatedList(paramObj);
    for item in paramRelatedList {
        if item.algoName.eq(paramCode) {
            paramItem.push(item);
        }
    }
    paramItem
}

// 对外入口
pub fn _doBatchExecute<T: Fn(&str) -> String>(paramCode: &str,  paramObj: &unistar_MWParameter<T>) -> HashMap<String, String> {
    let relations = getParamRelation(paramObj, &paramCode.to_string());
    let flatArray = getFlatParamArray(relations);
    let mut output = HashMap::new();
    for algo1 in flatArray {
        let result = executeOneAlg(&algo1.itemId, &algo1.algoType, &paramCode.to_string(), paramObj);
        println!("{}",algo1.itemId);
        println!("{}",result);
        output.insert(algo1.itemId, result);
    }
    output

}

pub fn executeOneAlg<T: Fn(&str) -> String> (algoId: &String, algoType: &String, srcParamCode: &String, paramObj: &unistar_MWParameter<T>) -> String {
    let id = algoId.clone();
    let algoTypeTmp = algoType.clone();
    let algoName = id + "_" + algoTypeTmp.as_str();
    callOneAlgo(algoName.as_str(), &paramObj)
}

pub fn getFlatParamArray(relations: Vec<Relation>) -> Vec<algo> {
    let mut newArray = vec![];
    for relation in relations {
        let items = relation.affections;
        for item in items {
            newArray.push(algo {
                algoType: relation.algoType.clone(),
                itemId: item.clone(),
            })
        }
    }
    newArray
}
