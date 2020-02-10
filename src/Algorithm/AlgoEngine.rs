use crate::MW::unistar_MW007312Parameter::unistar_MWParameter;
use crate::sys::EngineHelper::Relation;

pub fn getParamRelatedList<T>(paramObj: &unistar_MWParameter<T>) -> Vec<Relation> {
    paramObj.paramRelatedList()

}


pub fn getParamRelation<T>(paramObj: &unistar_MWParameter<T>, paramCode: String) -> Vec<Relation> {
    let mut paramItem =vec![];
    let paramRelatedList = getParamRelatedList(paramObj);
    for item in paramRelatedList {
        if item.algoName.eq(&paramCode  )  {
            paramItem.push(item);
        }
    }
    paramItem
}

