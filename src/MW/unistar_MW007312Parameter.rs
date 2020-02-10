use super::mw007312_def;
use super::rw000331_def;
use crate::sys;
use sys::pde;
use sys::getP;
use sys::EngineHelper::Relation;
use crate::sys::{MapVec, _solution};


pub fn callOneAlgo<T: Fn(&str) -> String>(paramCode: &str, paramObj: &unistar_MWParameter<T>) -> String {
    match paramCode {
        "var_productCode_Config" => paramObj.var_productCode_Config(),
        "P_Is_GEto10GE_Config" => paramObj.P_Is_GEto10GE_Config(),
        "PL_20_28P_SI_AC_Config" => paramObj.PL_20_28P_SI_AC_Config(),
        "PL_30_68C_HI_48S_Config" => paramObj.PL_30_68C_HI_48S_Config(),
        "PSFPP_SM_1550_80_Control" => paramObj.PSFPP_SM_1550_80_Control(),
        _ => "".to_string()
    }
}

pub struct unistar_MWParameter<T> where T: Fn(&str) -> String {
    pub getP: *mut T,
    pub _Solution: _solution,
}


impl<T> unistar_MWParameter<T> where T: Fn(&str) -> String {
    pub fn paramRelatedList(self) -> Vec<Relation> {
        vec![Relation{
            algoName: "P_5720EI_36C_AC".to_string(),
            algoType: "Config".to_string(),
            affections: vec!["var_productCode".to_string(),"P_Is_GEto10GE_Config".to_string()]
        }, Relation{
            algoName: "P_HI_56C_AC_5720".to_string(),
            algoType: "Config".to_string(),
            affections: vec!["PL_20_28P_SI_AC_Config".to_string(),"PL_30_68C_HI_48S_Config".to_string(),"PSFPP_SM_1550_80_Control".to_string()]
        }]
    }

    pub fn var_productCode_Config(&self) -> String {
        if sys::_objectEqual(pde::___PRODUCTCODE__(), mw007312_def::_Code_ECFG()) {
            mw007312_def::_Code_ECFG()
        } else {
            mw007312_def::_getEW_num(self._Para_to_IntValue(), mw007312_def::_Host_Init_EW_Num())
        }
    }

    pub fn _Para_to_IntValue(&self) -> MapVec {
        let mut result = MapVec::new();
        unsafe {
            //        下面这么写会有所有权的问题，self.getP会直接移出去了
            let getP = (*self).getP;
            result.push(sys::Map::new((*getP)("P_LI_10P_AC"), mw007312_def::_Int_LI_10P_AC()));
            result.push(sys::Map::new((*getP)("P_LI_28P_AC"), mw007312_def::_Int_LI_28P_AC()));
            result.push(sys::Map::new((*getP)("P_LI_28P_DC"), mw007312_def::_Int_LI_28P_DC()));
            result.push(sys::Map::new((*getP)("P_LI_28P_S_AC"), mw007312_def::_Int_LI_28P_S_AC()));
            result.push(sys::Map::new((*getP)("P_LI_52P_AC"), mw007312_def::_Int_LI_52P_AC()));
            result.push(sys::Map::new("1".to_string(), mw007312_def::_Int_LI_52P_DC()));
            result.push(sys::Map::new((*getP)("P_LI_52P_S_AC"), mw007312_def::_Int_LI_52P_S_AC()));
            result.push(sys::Map::new((*getP)("P_LI_10P_PWR_AC"), mw007312_def::_Int_LI_10P_PWR_AC()));
            result.push(sys::Map::new((*getP)("P_LI_28P_PWR_AC"), mw007312_def::_Int_LI_28P_PWR_AC()));
            result.push(sys::Map::new((*getP)("P_LI_52P_PWR_AC"), mw007312_def::_Int_LI_52P_PWR_AC()));
            result.push(sys::Map::new((*getP)("P_LI_28P_BAT"), mw007312_def::_Int_LI_28P_BAT()));
            result.push(sys::Map::new((*getP)("P_LI_28P_4AH"), mw007312_def::_Int_LI_28P_4AH()));
            result.push(sys::Map::new((*getP)("P_LI_28P_24S_BAT"), mw007312_def::_Int_LI_28P_24S_BAT()));
            result.push(sys::Map::new((*getP)("P_LI_28P_24S_4AH"), mw007312_def::_Int_LI_28P_24S_4AH()));
            result.push(sys::Map::new((*getP)("P_LI_28X_AC"), mw007312_def::_Int_LI_28X_AC()));
            result.push(sys::Map::new((*getP)("P_LI_28X_DC"), mw007312_def::_Int_LI_28X_DC()));
            result.push(sys::Map::new((*getP)("P_LI_28X_24S_AC"), mw007312_def::_Int_LI_28X_24S_AC()));
            result.push(sys::Map::new((*getP)("P_LI_28X_24S_DC"), mw007312_def::_Int_LI_28X_24S_DC()));
            result.push(sys::Map::new((*getP)("P_LI_28X_AC_5701"), mw007312_def::_Int_LI_28X_AC_5701()));
            result.push(sys::Map::new((*getP)("P_LI_28X_24S_AC_5701"), mw007312_def::_Int_LI_28X_24S_AC_01()));
            result.push(sys::Map::new((*getP)("P_LI_52X_AC"), mw007312_def::_Int_LI_52X_AC()));
            result.push(sys::Map::new((*getP)("P_LI_52X_DC"), mw007312_def::_Int_LI_52X_DC()));
            result.push(sys::Map::new((*getP)("P_LI_52X_48CS_AC"), mw007312_def::_Int_LI_52X_48CS_AC()));
            result.push(sys::Map::new((*getP)("P_LI_28X_PWR_AC"), mw007312_def::_Int_LI_28X_PWR_AC()));
            result.push(sys::Map::new((*getP)("P_LI_52X_PWR_AC"), mw007312_def::_Int_LI_52X_PWR_AC()));
            result.push(sys::Map::new((*getP)("P_SI_24TP_AC"), mw007312_def::_Int_SI_24TP_AC()));
            result.push(sys::Map::new((*getP)("P_SI_24TP_DC"), mw007312_def::_Int_SI_24TP_DC()));
            result.push(sys::Map::new((*getP)("P_SI_26X_12S_AC"), mw007312_def::_Int_SI_26X_12S_AC()));
            result.push(sys::Map::new((*getP)("P_SI_48TP_AC"), mw007312_def::_Int_SI_48TP_AC()));
            result.push(sys::Map::new((*getP)("P_SI_48TP_DC"), mw007312_def::_Int_SI_48TP_DC()));
            result.push(sys::Map::new((*getP)("P_SI_24TP_PWR"), mw007312_def::_Int_SI_24TP_PWR()));
            result.push(sys::Map::new((*getP)("P_SI_24TP_PWR_AC"), mw007312_def::_Int_SI_24TP_PWR_AC()));
            result.push(sys::Map::new((*getP)("P_SI_48TP_PWR"), mw007312_def::_Int_SI_48TP_PWR()));
            result.push(sys::Map::new((*getP)("P_SI_48TP_PWR_AC"), mw007312_def::_Int_SI_48TP_PWR_AC()));
            result.push(sys::Map::new((*getP)("P_SI_28C"), mw007312_def::_Int_SI_28C()));
            result.push(sys::Map::new((*getP)("P_SI_28C_AC"), mw007312_def::_Int_SI_28C_AC()));
            result.push(sys::Map::new((*getP)("P_SI_52C"), mw007312_def::_Int_SI_52C()));
            result.push(sys::Map::new((*getP)("P_SI_52C_AC"), mw007312_def::_Int_SI_52C_AC()));
            result.push(sys::Map::new((*getP)("P_SI_28C_PWR"), mw007312_def::_Int_SI_28C_PWR()));
            result.push(sys::Map::new((*getP)("P_SI_52C_PWR"), mw007312_def::_Int_SI_52C_PWR()));
            result.push(sys::Map::new((*getP)("P_EI_28C"), mw007312_def::_Int_EI_28C()));
            result.push(sys::Map::new((*getP)("P_EI_28C_AC"), mw007312_def::_Int_EI_28C_AC()));
            result.push(sys::Map::new((*getP)("P_EI_28C_24S"), mw007312_def::_Int_EI_28C_24S()));
            result.push(sys::Map::new((*getP)("P_EI_28C_24S_AC"), mw007312_def::_Int_EI_28C_24S_AC()));
            result.push(sys::Map::new((*getP)("P_EI_52C"), mw007312_def::_Int_EI_52C()));
            result.push(sys::Map::new((*getP)("P_EI_52C_AC"), mw007312_def::_Int_EI_52C_AC()));
            result.push(sys::Map::new((*getP)("P_EI_28C_PWR"), mw007312_def::_Int_EI_28C_PWR()));
            result.push(sys::Map::new((*getP)("P_EI_28C_PWR_AC"), mw007312_def::_Int_EI_28C_PWR_AC()));
            result.push(sys::Map::new((*getP)("P_EI_52C_PWR"), mw007312_def::_Int_EI_52C_PWR()));
            result.push(sys::Map::new((*getP)("P_EI_52C_PWR_AC"), mw007312_def::_Int_EI_52C_PWR_AC()));
            result.push(sys::Map::new((*getP)("P_EI_28C_5710"), mw007312_def::_Int_EI_28C_5710()));
            result.push(sys::Map::new((*getP)("P_EI_28C_AC_S5710"), mw007312_def::_Int_EI_28C_AC_S5710()));
            result.push(sys::Map::new((*getP)("P_EI_52C_5710"), mw007312_def::_Int_EI_52C_5710()));
            result.push(sys::Map::new((*getP)("P_EI_52C_AC_S5710"), mw007312_def::_Int_EI_52C_AC_S5710()));
            result.push(sys::Map::new((*getP)("P_EI_28C_PWR_AC_5710"), mw007312_def::_Int_EI_28C_PWR_AC_10()));
            result.push(sys::Map::new((*getP)("P_EI_52C_PWR_5710"), mw007312_def::_Int_EI_52C_PWR_5710()));
            result.push(sys::Map::new((*getP)("P_EI_52C_PWR_AC_5710"), mw007312_def::_Int_EI_52C_PWR_AC_10()));
            result.push(sys::Map::new((*getP)("P_HI_28C"), mw007312_def::_Int_HI_28C()));
            result.push(sys::Map::new((*getP)("P_HI_28C_AC"), mw007312_def::_Int_HI_28C_AC()));
            result.push(sys::Map::new((*getP)("P_HI_28C_24S"), mw007312_def::_Int_HI_28C_24S()));
            result.push(sys::Map::new((*getP)("P_HI_28C_24S_AC"), mw007312_def::_Int_HI_28C_24S_AC()));
            result.push(sys::Map::new((*getP)("P_HI_108C_PWR_5710"), mw007312_def::_Int_HI_108C_PWR_5710()));
            result.push(sys::Map::new((*getP)("P_HI_108C_PWR_10_B1"), mw007312_def::_Int_HI_108C_PWR_5710_B1()));
            result.push(sys::Map::new((*getP)("P_LI_28TP_AC"), mw007312_def::_Int_LI_28TP_AC()));
            result.push(sys::Map::new((*getP)("P_LI_28TP_PWR_AC"), mw007312_def::_Int_LI_28TP_PWR_AC()));
            result.push(sys::Map::new((*getP)("P_LI_28TP_PWR_AC_01"), mw007312_def::_Int_LI_28TP_PWR_AC_01()));
            result.push(sys::Map::new((*getP)("P_HI_32C_24S_AC_5720"), mw007312_def::_Int_HI_32C_24S_AC_5720()));
            result.push(sys::Map::new((*getP)("P_HI_56C_AC_5720"), mw007312_def::_Int_HI_56C_AC_5720()));
            result.push(sys::Map::new((*getP)("P_HI_56C_PWR_AC_5720"), mw007312_def::_Int_HI_56C_PWR_AC_5720()));
            result.push(sys::Map::new((*getP)("P_HI_56C_PWR_AC1_20"), mw007312_def::_int_HI_56C_PWR_AC1_20()));
            result.push(sys::Map::new((*getP)("P_5720EI_36C_28S_AC"), mw007312_def::_Int_5720EI_36C_28S_AC()));
            result.push(sys::Map::new((*getP)("P_5720EI_56C_48S_AC"), mw007312_def::_Int_5720EI_56C_48S_AC()));
            result.push(sys::Map::new((*getP)("P_5720EI_36C_AC"), mw007312_def::_Int_5720EI_36C_AC()));
            result.push(sys::Map::new((*getP)("P_5720EI_36PC_AC"), mw007312_def::_Int_5720EI_36PC_AC()));
            result.push(sys::Map::new((*getP)("P_5720EI_56C_AC"), mw007312_def::_Int_5720EI_56C_AC()));
            result.push(sys::Map::new((*getP)("P_5720EI_56PC_AC"), mw007312_def::_Int_5720EI_56PC_AC()));
            result.push(sys::Map::new((*getP)("P_5720EI_36C_PWR_AC"), mw007312_def::_Int_5720EI_36C_PWR_AC()));
            result.push(sys::Map::new((*getP)("P_5720EI_56C_PWR_AC"), mw007312_def::_Int_5720EI_56C_PWR_AC()));
            result.push(sys::Map::new((*getP)("P_5720EI_56C_PWR_AC1"), mw007312_def::_Int_5720EI_56C_PWR_AC1()));
            result.push(sys::Map::new((*getP)("P_5720EI_32X_24S_AC"), mw007312_def::_Int_5720EI_32X_24S_AC()));
            result.push(sys::Map::new((*getP)("P_5720EI_50X_46S_AC"), mw007312_def::_Int_5720EI_50X_46S_AC()));
            result.push(sys::Map::new((*getP)("P_5720EI_32X_AC"), mw007312_def::_Int_5720EI_32X_AC()));
            result.push(sys::Map::new((*getP)("P_5720EI_32P_AC"), mw007312_def::_Int_5720EI_32P_AC()));
            result.push(sys::Map::new((*getP)("P_5720EI_50X_AC"), mw007312_def::_Int_5720EI_50X_AC()));
            result.push(sys::Map::new((*getP)("P_5720EI_52X_AC"), mw007312_def::_Int_5720EI_52X_AC()));
            result.push(sys::Map::new((*getP)("P_5720EI_52P_AC"), mw007312_def::_Int_5720EI_52P_AC()));
            result.push(sys::Map::new((*getP)("P_00S_28X_LI_AC"), mw007312_def::_int_00S_28X_LI_AC()));
            result.push(sys::Map::new((*getP)("P_00S_52X_LI_AC"), mw007312_def::_int_00S_52X_LI_AC()));
            result.push(sys::Map::new((*getP)("P_00S_28P_PWR_LI_AC"), mw007312_def::_int_00S_28P_PWR_LI_AC()));
            result.push(sys::Map::new((*getP)("P_10_28X_LI_AC"), mw007312_def::_int_10_28X_LI_AC()));
            result.push(sys::Map::new((*getP)("P_10_52X_LI_AC"), mw007312_def::_int_10_52X_LI_AC()));
            result.push(sys::Map::new((*getP)("P_20S_28P_SI_AC"), mw007312_def::_int_20S_28P_SI_AC()));
            result.push(sys::Map::new((*getP)("P_20S_28X_SI_AC"), mw007312_def::_int_20S_28X_SI_AC()));
            result.push(sys::Map::new((*getP)("P_20S_52P_SI_AC"), mw007312_def::_int_20S_52P_SI_AC()));
            result.push(sys::Map::new((*getP)("P_20S_52X_SI_AC"), mw007312_def::_int_20S_52X_SI_AC()));
            result.push(sys::Map::new((*getP)("P_20_28P_SI_AC"), mw007312_def::_int_20_28P_SI_AC()));
            result.push(sys::Map::new((*getP)("P_20_28X_SI_AC"), mw007312_def::_int_20_28X_SI_AC()));
            result.push(sys::Map::new((*getP)("P_20_52P_SI_AC"), mw007312_def::_int_20_52P_SI_AC()));
            result.push(sys::Map::new((*getP)("P_20_52X_SI_AC"), mw007312_def::_int_20_52X_SI_AC()));
            result.push(sys::Map::new((*getP)("P_20_28X_PWR_SI_AC"), mw007312_def::_int_20_28X_PWR_SI_AC()));
            result.push(sys::Map::new((*getP)("P_20_52X_PWR_SI_AC"), mw007312_def::_int_20_52X_PWR_SI_AC()));
            result.push(sys::Map::new((*getP)("P_20_52X_PWR_SI_ACF"), mw007312_def::_int_20_52X_PWR_SI_ACF()));
            result.push(sys::Map::new((*getP)("P_20_28X_PWR_SI_DC"), mw007312_def::_int_20_28X_PWR_SI_DC()));
            result.push(sys::Map::new((*getP)("P_20_52X_PWR_SI_DC"), mw007312_def::_int_20_52X_PWR_SI_DC()));
            result.push(sys::Map::new((*getP)("P_20_28X_SI_DC"), mw007312_def::_int_20_28X_SI_DC()));
            result.push(sys::Map::new((*getP)("P_20_52X_SI_DC"), mw007312_def::_int_20_52X_SI_DC()));
            result.push(sys::Map::new((*getP)("P_20S_28X_SI_DC"), mw007312_def::_int_20S_28X_SI_DC()));
            result.push(sys::Map::new((*getP)("P_20S_52X_SI_DC"), mw007312_def::_int_20S_52X_SI_DC()));
            result.push(sys::Map::new((*getP)("P_5720EI_32X_DC"), mw007312_def::_int_5720EI_32X_DC()));
            result.push(sys::Map::new((*getP)("P_5720EI_50X_DC"), mw007312_def::_int_5720EI_50X_DC()));
            result.push(sys::Map::new((*getP)("P_5720EI_32X_24S_DC"), mw007312_def::_int_5720EI_32X_24S_DC()));
            result.push(sys::Map::new((*getP)("P_5720EI_50X_46S_DC"), mw007312_def::_int_5720EI_50X_46S_DC()));
            result.push(sys::Map::new((*getP)("P_5720EI_36C_DC"), mw007312_def::_int_5720EI_36C_DC()));
            result.push(sys::Map::new((*getP)("P_5720EI_56C_DC"), mw007312_def::_int_5720EI_56C_DC()));
            result.push(sys::Map::new((*getP)("P_5720EI_36C_PWR_DC"), mw007312_def::_int_5720EI_36C_PWR_DC()));
            result.push(sys::Map::new((*getP)("P_5720EI_56C_PWR_DC"), mw007312_def::_int_5720EI_56C_PWR_DC()));
            result.push(sys::Map::new((*getP)("P_5720EI_36C_28S_DC"), mw007312_def::_int_5720EI_36C_28S_DC()));
            result.push(sys::Map::new((*getP)("P_5720EI_56C_48S_DC"), mw007312_def::_int_5720EI_56C_48S_DC()));
            result.push(sys::Map::new((*getP)("P_00_8P_G"), mw007312_def::_int_5700_8P_G()));
            result.push(sys::Map::new((*getP)("P_20_14X_SI_AC"), mw007312_def::_int_S5720_14X_PWH_SI_AC()));
            result.push(sys::Map::new((*getP)("P_12TP_AC_LI20"), mw007312_def::_int_12TP_AC_LI20()));
            result.push(sys::Map::new((*getP)("P_12TP_AC_LI20S"), mw007312_def::_int_12TP_AC_LI20S()));
            result.push(sys::Map::new((*getP)("P_12TP_PWRAC_LI20"), mw007312_def::_int_12TP_PWRAC_LI20()));
            result.push(sys::Map::new((*getP)("P_12TP_PWRAC_LI20S"), mw007312_def::_int_12TP_PWRAC_LI20S()));
            result.push(sys::Map::new((*getP)("P_28P_AC_LI20"), mw007312_def::_int_28P_AC_LI20()));
            result.push(sys::Map::new((*getP)("P_28P_AC_LI20_SEA"), mw007312_def::_int_28P_AC_LI20()));
            result.push(sys::Map::new((*getP)("P_28P_AC_LI20S"), mw007312_def::_int_28P_AC_LI20S()));
            result.push(sys::Map::new((*getP)("P_28X_AC_LI20"), mw007312_def::_int_28X_AC_LI20()));
            result.push(sys::Map::new((*getP)("P_28X_AC_LI20S"), mw007312_def::_int_28X_AC_LI20S()));
            result.push(sys::Map::new((*getP)("P_28X_DC_LI20"), mw007312_def::_int_28X_DC_LI20()));
            result.push(sys::Map::new((*getP)("P_28P_PWRAC_LI20"), mw007312_def::_int_28P_PWRAC_LI20()));
            result.push(sys::Map::new((*getP)("P_28P_PWRAC_LI20_SEA"), mw007312_def::_int_28P_PWRAC_LI20()));
            result.push(sys::Map::new((*getP)("P_28P_PWRAC_LI20S"), mw007312_def::_int_28P_PWRAC_LI20S()));
            result.push(sys::Map::new((*getP)("P_28X_PWRAC_LI20"), mw007312_def::_int_28X_PWRAC_LI20()));
            result.push(sys::Map::new((*getP)("P_28X_PWRAC_LI20S"), mw007312_def::_int_28X_PWRAC_LI20S()));
            result.push(sys::Map::new((*getP)("P_52P_AC_LI20"), mw007312_def::_int_52P_AC_LI20()));
            result.push(sys::Map::new((*getP)("P_52P_AC_LI20_SEA"), mw007312_def::_int_52P_AC_LI20()));
            result.push(sys::Map::new((*getP)("P_52P_AC_LI20S"), mw007312_def::_int_52P_AC_LI20S()));
            result.push(sys::Map::new((*getP)("P_52X_AC_LI20"), mw007312_def::_int_52X_AC_LI20()));
            result.push(sys::Map::new((*getP)("P_52X_AC_LI20S"), mw007312_def::_int_52X_AC_LI20S()));
            result.push(sys::Map::new((*getP)("P_52X_DC_LI20"), mw007312_def::_int_52X_DC_LI20()));
            result.push(sys::Map::new((*getP)("P_52P_PWRAC_LI20"), mw007312_def::_int_52P_PWRAC_LI20()));
            result.push(sys::Map::new((*getP)("P_52P_PWRAC_LI20_SEA"), mw007312_def::_int_52P_PWRAC_LI20()));
            result.push(sys::Map::new((*getP)("P_52P_PWRAC_LI20S"), mw007312_def::_int_52P_PWRAC_LI20S()));
            result.push(sys::Map::new((*getP)("P_52X_PWRAC_LI20"), mw007312_def::_int_52X_PWRAC_LI20()));
            result.push(sys::Map::new((*getP)("P_52X_PWRAC_LI20S"), mw007312_def::_int_52X_PWRAC_LI20S()));
            result.push(sys::Map::new((*getP)("P_28X_24S_AC_LI20"), mw007312_def::_int_28X_24S_AC_LI20()));
            result.push(sys::Map::new((*getP)("P_28X_24S_AC_LI20S"), mw007312_def::_int_28X_24S_AC_LI20S()));
            result.push(sys::Map::new((*getP)("P_28X_24S_DC_LI20"), mw007312_def::_int_28X_24S_DC_LI20()));
            result.push(sys::Map::new((*getP)("P_28TP_PWRACL_LI20"), mw007312_def::_int_28TP_PWRACL_LI20()));
            result.push(sys::Map::new((*getP)("P_28TP_PWRACL_LI20S"), mw007312_def::_int_28TP_PWRACL_LI20S()));
            result.push(sys::Map::new((*getP)("P_28TP_PWRAC_LI20"), mw007312_def::_int_28TP_PWRAC_LI20()));
            result.push(sys::Map::new((*getP)("P_28TP_AC_LI20"), mw007312_def::_int_28TP_AC_LI20()));
            result.push(sys::Map::new((*getP)("P_26X_PWHAC_LI20"), mw007312_def::_int_26X_PWHAC_LI20()));
            result.push(sys::Map::new((*getP)("P_28X_PWHAC_LI20"), mw007312_def::_int_28X_PWHAC_LI20()));
            result.push(sys::Map::new((*getP)("P_28X_24S_AC_SI20"), mw007312_def::_int_28X_24S_AC_SI20()));
            result.push(sys::Map::new((*getP)("P_28X_24S_DC_SI20"), mw007312_def::_int_28X_24S_DC_SI20()));
            result.push(sys::Map::new((*getP)("P_28X24S_AC_SI21"), mw007312_def::_Int_28X24S_AC_SI21()));
            result.push(sys::Map::new((*getP)("P_52X_PWRACF_LI20"), mw007312_def::_Int_52X_PWRACF_LI20()));
            result.push(sys::Map::new((*getP)("P_48C_AC_SI30"), mw007312_def::_Int_48C_AC_SI30()));
            result.push(sys::Map::new((*getP)("P_48C_PWR_AC_SI30"), mw007312_def::_Int_48C_PWR_AC_SI30()));
            result.push(sys::Map::new((*getP)("P_68C_AC_SI30"), mw007312_def::_Int_68C_AC_SI30()));
            result.push(sys::Map::new((*getP)("P_68C_PWR_AC_SI30"), mw007312_def::_Int_68C_PWR_AC_SI30()));
            result.push(sys::Map::new((*getP)("P_68C_PWR_SI30"), mw007312_def::_Int_68C_PWR_SI30()));
            result.push(sys::Map::new((*getP)("P_48C_AC_EI30S"), mw007312_def::_Int_48C_AC_EI30S()));
            result.push(sys::Map::new((*getP)("P_48C_PWR_EI30S"), mw007312_def::_Int_48C_PWR_EI30S()));
            result.push(sys::Map::new((*getP)("P_68C_AC_EI30S"), mw007312_def::_Int_68C_AC_EI30S()));
            result.push(sys::Map::new((*getP)("P_68C_PWR_EI30S"), mw007312_def::_Int_68C_PWR_EI30S()));
            result.push(sys::Map::new((*getP)("P_20I_12X_SI_AC"), mw007312_def::_Int_68C_PWR_EI30S()));
            result.push(sys::Map::new((*getP)("P_20I_12X_PWH_SI_DC"), mw007312_def::_Int_20I_12X_PWH_SI_DC()));
            result.push(sys::Map::new((*getP)("P_20I_28X_SI_AC"), mw007312_def::_Int_20I_28X_SI_AC()));
            result.push(sys::Map::new((*getP)("P_20I_28X_PWH_SI_AC"), mw007312_def::_Int_20I_28X_PWH_SI_AC()));
            result.push(sys::Map::new((*getP)("P_30_36C_HI"), mw007312_def::_Int_30_36C_HI()));
            result.push(sys::Map::new((*getP)("P_30_44C_HI"), mw007312_def::_Int_30_44C_HI()));
            result.push(sys::Map::new((*getP)("P_30_36C_PWH_HI"), mw007312_def::_Int_30_36C_PWH_HI()));
            result.push(sys::Map::new((*getP)("P_30_44C_PWH_HI"), mw007312_def::_Int_30_44C_PWH_HI()));
            result.push(sys::Map::new((*getP)("P_30_60C_HI"), mw007312_def::_Int_30_60C_HI()));
            result.push(sys::Map::new((*getP)("P_30_68C_HI"), mw007312_def::_Int_30_68C_HI()));
            result.push(sys::Map::new((*getP)("P_30_60C_PWH_HI"), mw007312_def::_Int_30_60C_PWH_HI()));
            result.push(sys::Map::new((*getP)("P_30_68C_PWH_HI"), mw007312_def::_Int_30_68C_PWH_HI()));
            result.push(sys::Map::new((*getP)("P_20SV2_28P_LIAC"), mw007312_def::_Int_20SV2_28P_LIAC()));
            result.push(sys::Map::new((*getP)("P_20SV2_52P_LIAC"), mw007312_def::_Int_20SV2_52P_LIAC()));
            result.push(sys::Map::new((*getP)("P_20I_10XPWH_SI_AC"), mw007312_def::_Int_20I_10XPWH_SI_AC()));
            result.push(sys::Map::new((*getP)("P_20I_10X_SI_AC_SEA"), mw007312_def::_Int_20I_10XPWH_SI_AC()));
            result.push(sys::Map::new((*getP)("P_20I_6XPWH_SI_AC"), mw007312_def::_Int_20I_6XPWH_SI_AC()));
            result.push(sys::Map::new((*getP)("P_30_60C_HI_48S"), mw007312_def::_Int_30_60C_HI_48S()));
            result.push(sys::Map::new((*getP)("P_30_68C_HI_48S"), mw007312_def::_Int_30_68C_HI_48S()));
            result.push(sys::Map::new((*getP)("P_30_36C_HI_24S"), mw007312_def::_Int_30_36C_HI_24S()));
            result.push(sys::Map::new((*getP)("P_30_44C_HI_24S"), mw007312_def::_Int_30_44C_HI_24S()));
            result.push(sys::Map::new((*getP)("P_20_52X_LI_48S_AC"), mw007312_def::_Int_20_52X_LI_48S_AC()));
            result.push(sys::Map::new((*getP)("P_20_52X_SI_48S"), mw007312_def::_Int_20_52X_SI_48S()));
            result.push(sys::Map::new((*getP)("P_20_28XPWR_LI_ACF"), mw007312_def::_Int_20_28XPWR_LI_ACF()));
        }
        result
    }

    pub fn P_Is_GEto10GE_Config(&self) -> String {
        let tmparry = vec![mw007312_def::_Code_28P_PWRAC_LI20(), mw007312_def::_Code_28P_AC_LI20(), mw007312_def::_Code_52P_AC_LI20()];
        if tmparry.contains(&self._productCode()) && self._Solution.country.eq(&"1790".to_string()) {
            "1".to_string()
        } else {
            "0".to_string()
        }
    }

    pub fn _productCode(&self) -> String {
        unsafe {
            let GetP = self.getP;
            (*GetP)("var_productCode")
        }
    }

    pub fn _Init_Is_Japan(&self) -> bool {
        if self._Solution.country.eq("1250") {
            true
        } else {
            false
        }
    }

    pub fn PL_20_28P_SI_AC_Config(&self) -> String {
        unsafe {
            let GetP = (*self).getP;
            if !(*self)._Init_Is_Japan() {
                (*self)._getHostNum((*GetP)("P_20_28P_SI_AC"), mw007312_def::_code_20_28P_SI_AC(), self._productCode(), false, 0, 0)
            } else {
                "0".to_string()
            }
        }
    }

    pub fn PL_30_60C_HI_48S_Config(&self) -> String {
        unsafe {
            let GetP = (*self).getP;
            self._getHostNum((*GetP)("P_30_60C_HI_48S"), mw007312_def::_Code_30_60C_HI_48S(), self._productCode(), false, 0, 0)
        }
    }

    pub fn PL_30_68C_HI_48S_Config(&self) -> String {
        unsafe {
            let GetP = (*self).getP;
            if !self._Init_Is_Japan() {
                self._getHostNum((*GetP)("P_30_68C_HI_48S"), mw007312_def::_Code_30_68C_HI_48S(), self._productCode(), false, 0, 0)
            } else {
                "0".to_string()
            }
        }
    }

    pub fn _getHostNum(&self, _hostcode: String, _procode: String, _syscode: String, _tp: bool, _sType: i32, _hostType: i32) -> String {
        _hostcode
    }

    pub fn PSFPP_SM_1550_80_Control(&self) -> String {
        let tmpList = self._PL10GEOpticals();
        if self._Init_Support10GOpt() && !rw000331_def::_hasNotChoosed(rw000331_def::_SFPP_SM_1550_80(), tmpList) {
            rw000331_def::_Visible()
        } else {
            rw000331_def::_InVisible()
        }
    }

    pub fn _PL10GEOpticals(&self) -> Vec<String> {
        self._Init_ListSFPPOpt()
    }

    pub fn _Init_ListSFPPOpt(&self) -> Vec<String> {
        let mut tmp = self._Optical_10GSFPP();
        tmp.append(&mut self._Opt_ForStack());
        tmp
    }

    pub fn _Opt_ForStack(&self) -> Vec<String> {
        unsafe {
            let GetP = (*self).getP;
            if mw007312_def::_Only10G_Stack().contains(&(*GetP)("var_productCode")) && (*GetP)("PL_StackMode_Sel").eq(&"2".to_string()) {
                vec![rw000331_def::_SFPP_MM_850_D3(), rw000331_def::_SFPP_SM_1310_10(), rw000331_def::_SFPP_MM_850_D1(), rw000331_def::_SFPP_SM_1310_14()]
            } else {
                vec![]
            }
        }

    }

    pub fn _Optical_10GSFPP(&self) -> Vec<String> {
        unsafe {
            let GetP = (*self).getP;
            let mut _temp = vec![];
            if self._is_eCFG_IP() {
                _temp = self._getOpticalList(Vec::new(), rw000331_def::_SFPPOpt_NoJFE());
            } else if self._Is_SCT() {
                if self._ISVisible_10GE() {
                    _temp = rw000331_def::_SFPPOpt_NoJFE();
                    _temp.append(&mut vec![rw000331_def::_SFPP_SM_1310_14()]);
                } else {
                    _temp = vec![];
                }
            } else {
                _temp = vec![];
            }
            let _temp1;
            if vec![mw007312_def::_Code_16X_PWHAC_LI20(), mw007312_def::_Code_20I_12X_SI_AC(), mw007312_def::_Code_20I_12X_PWH_SI_DC(), mw007312_def::_Code_20I_28X_SI_AC(), mw007312_def::_Code_20I_28X_PWH_SI_AC(), mw007312_def::_Code_20I_10XPWH_SI_AC(), mw007312_def::_Code_20I_6XPWH_SI_AC()].contains(&(*GetP)("var_productCode")) {
                _temp1 = vec![rw000331_def::_SFPP_SM_1310_14()]
            } else {
                _temp1 = vec![];
            }
            let mut _temp2;
            if (*GetP)("PL_fun_GEto10GE").parse::<i32>().unwrap() >0 && vec![mw007312_def::_Code_28TP_AC_LI20()].contains(&(*GetP)("var_productCode"))  {
                _temp2 = vec![rw000331_def::_SFPP_MM_850_D1(), rw000331_def::_SFPP_MM_850_D3(), rw000331_def::_SFPP_SM_1310_10(),rw000331_def::_SFPP_SM_1310_14()];
            } else if (*GetP)("PL_fun_GEto10GE").parse::<i32>().unwrap() > 0 {
                _temp2 = rw000331_def::_SFPPOpt_NoJFE();
                _temp2.append(&mut vec![rw000331_def::_SFPP_SM_1310_14()]);
                _temp2.remove_item(&rw000331_def::_SFPP_MM_1310_D22());
            } else {
                _temp2 = vec![];
            }

            if vec![mw007312_def::_Code_16X_PWHAC_LI20(), mw007312_def::_Code_20I_12X_SI_AC(), mw007312_def::_Code_20I_12X_PWH_SI_DC(), mw007312_def::_Code_20I_28X_SI_AC(), mw007312_def::_Code_20I_28X_PWH_SI_AC(), mw007312_def::_Code_20I_10XPWH_SI_AC(), mw007312_def::_Code_20I_6XPWH_SI_AC()].contains(&(*GetP)("var_productCode")) {
                _temp1
            } else if  (*GetP)("PL_fun_GEto10GE").parse::<i32>().unwrap() > 0 {
                _temp2
            } else {
                _temp
            }
        }
    }

    pub fn _ISVisible_10GE(&self) -> bool {
        self._SupportSFPPGE() || (self._SumSpt10GE().parse::<i32>().unwrap() + self._SumSpt10GEorGE().parse::<i32>().unwrap()) > 0
    }

    pub fn _SupportSFPPGE(&self) -> bool {
        mw007312_def::_Spt_10GEorGE().contains(&self._productCode())
    }

    pub fn _SumSpt10GE(&self) -> String {
        if self._Init_IsSptGEor10GE() {
            "0".to_string()
        } else {
            unsafe {
                let GetP = (*self).getP;
                let num1 = (*GetP)("PL_2_10GE").parse::<i32>().unwrap();
                let num2 = (*GetP)("PL_4_10GE").parse::<i32>().unwrap();
                let num3 = (*GetP)("PL_4_10GE_S5710HI").parse::<i32>().unwrap();
                let num4 = (*GetP)("PL_2_10GE_5720EI").parse::<i32>().unwrap();
                let num5 = (*GetP)("PL_4_10GE_S5720HI").parse::<i32>().unwrap();
                let sum = (2 * num1 + 4 * num2 + num3 + 2 * num4 + 4 * num5);
                sum.to_string()
            }
        }
    }

    pub fn _SumSpt10GEorGE(&self) -> String {
        unsafe {
            let GetP = (*self).getP;
            if self._Init_IsSptGEor10GE() {
                let num1 = (*GetP)("PL_2_10GE").parse::<i32>().unwrap();
                let num2 = (*GetP)("PL_2_GE_10GE_S5710EI").parse::<i32>().unwrap();
                let num3 = (*GetP)("PL_2_GEor10GE_HI").parse::<i32>().unwrap();
                let num4 = (*GetP)("PL_4_GEor10GE_HI").parse::<i32>().unwrap();
                let num5 = (*GetP)("PL_4_10GE").parse::<i32>().unwrap();
                (2 * num1 + num2 + num3 + 4 * num4 + num5).to_string()
            } else {
                let num1 = (*GetP)("PL_2_GE_10GE_S5710EI").parse::<i32>().unwrap();
                let num2 = (*GetP)("PL_2_GEor10GE_HI").parse::<i32>().unwrap();
                let num3 = (*GetP)("PL_4_GEor10GE_HI").parse::<i32>().unwrap();
                let num4 = (*GetP)("PL_8_GE_S5730HI").parse::<i32>().unwrap();
                (2 * num1 + num2 + 4 * num3 + 8 * num4).to_string()
            }
        }
    }

    pub fn _Init_Support10GOpt(&self) -> bool {
        self._PL10GEOpticals().is_empty()
    }



    pub fn _is_eCFG_IP(&self) -> bool {
        false
    }

    pub fn _Is_SCT(&self) -> bool {
        true
    }

    pub fn _Init_IsSptGEor10GE(&self) -> bool {
        self._Is_SCT()
    }

    pub fn _getOpticalList(&self, _tpList: Vec<String>, _all: Vec<String>) -> Vec<String> {
        if self._is_eCFG_IP() {
            if self._is_CHorEur() {
                _tpList
            } else {
                _all
            }
        } else {
            Vec::new()
        }
    }

    pub fn _is_CHorEur(&self) -> bool {
        rw000331_def::_IntToBool(self._EuropeTpVisble()) || rw000331_def::_IntToBool(self._ChinaTpVisble())
    }

    pub fn _EuropeTpVisble(&self) -> String {
        unsafe {
            let GetP = (*self).getP;
            if self._is_eCFG_IP() && (*GetP)("PL_Europe").eq(&rw000331_def::_Yes()) {
                rw000331_def::_Visible()
            } else {
                rw000331_def::_InVisible()
            }
        }
    }

    pub fn _ChinaTpVisble(&self) -> String {
        unsafe {
            let GetP = (*self).getP;
            if self._is_eCFG_IP() && (*GetP)("PL_China").eq(&rw000331_def::_Yes()) {
                rw000331_def::_Visible()
            } else {
                rw000331_def::_InVisible()
            }
        }
    }
}

