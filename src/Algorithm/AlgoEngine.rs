//Param.getP = function(paramCode) {
//var tempData, dataVal, paraVal, isMulti, dataType;
//var i, len;
//tempData = this.getDatas();
//if ( ! ! tempData && tempData.hasOwnProperty(paramCode)) {
//dataVal = tempData[paramCode];
//paraVal = dataVal[this.DATAINDEX.manualValue];
//isMulti = dataVal[this.DATAINDEX.isMultiValue];
//dataType = dataVal[this.DATAINDEX.dataType];
//if ( ! isMulti) {
//var result = this.getPbyType(paraVal, dataType);
//if (String(result) == = "NaN") {
//return 0
//} else {
//return result
//}
//}
//if (typeof paraVal == = "string") {
//paraVal = paraVal.split(";")
//}
//for (i = 0,
//len = paraVal.length; i < len; i + + ) {
//paraVal[i] = this.getPbyType(paraVal[i], dataType)
//}
//return paraVal
//}
//return 0
//}
//


use crate::sys::Param;

