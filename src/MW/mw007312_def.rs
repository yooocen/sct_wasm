// 两个入参都是数组，item是hashmap
pub fn _getEW_num(_cfg: MapVec, _ew_nums: MapVec) -> String {
    let mut _temp = Vec::new();
    let _158039561 = sys::_keys(&_cfg);
    for map in _158039561.iter() {
        if map.parse::<i32>().unwrap() > 0 {
            _temp.push(_cfg.getValue(map));
        }
    }
    if !sys::_isEmpty(&_temp) {
        _ew_nums.getValue(&sys::_toInt(sys::_first(&_temp).unwrap()).to_string()).clone()
    } else {
        "".to_string()
    }
}

use wasm_bindgen::__rt::std::collections::HashMap;
use crate::sys;
use crate::sys::MapVec;
use serde::de::Unexpected::Map;

pub fn _Code_ECFG() -> String {
    return String::from("EW012042");
}

pub fn _Host_Init_EW_Num() -> MapVec {
    let mut result = MapVec::new();
    result.push(sys::Map::new(_Int_LI_10P_AC(), _Code_LI_10P_AC()));
    result.push(sys::Map::new(_Int_LI_28P_S_AC(), _Code_LI_28P_AC_S()));
    result.push(sys::Map::new(_Int_LI_52P_S_AC(), _Code_LI_52P_AC_S()));
    result.push(sys::Map::new(_Int_LI_28P_AC(), _Code_LI_28P()));
    result.push(sys::Map::new(_Int_LI_28P_DC(), _Code_LI_28P()));
    result.push(sys::Map::new(_Int_LI_52P_AC(), _Code_LI_52P()));
    result.push(sys::Map::new(_Int_LI_52P_DC(), _Code_LI_52P()));
    result.push(sys::Map::new(_Int_LI_10P_PWR_AC(), _Code_LI_10P_PWR_AC()));
    result.push(sys::Map::new(_Int_LI_28P_PWR_AC(), _Code_LI_28P_PWR_AC()));
    result.push(sys::Map::new(_Int_LI_52P_PWR_AC(), _Code_LI_52P_PWR_AC()));
    result.push(sys::Map::new(_Int_LI_28X_AC(), _Code_LI_28X_AC()));
    result.push(sys::Map::new(_Int_LI_28X_DC(), _Code_LI_28X_DC()));
    result.push(sys::Map::new(_Int_LI_28X_24S_AC(), _Code_LI_28X_24S_AC()));
    result.push(sys::Map::new(_Int_LI_28X_24S_DC(), _Code_LI_28X_24S_DC()));
    result.push(sys::Map::new(_Int_LI_52X_AC(), _Code_LI_52X_AC()));
    result.push(sys::Map::new(_Int_LI_52X_DC(), _Code_LI_52X_DC()));
    result.push(sys::Map::new(_Int_LI_28X_PWR_AC(), _Code_LI_28X_PWR_AC()));
    result.push(sys::Map::new(_Int_LI_52X_PWR_AC(), _Code_LI_52X_PWR_AC()));
    result.push(sys::Map::new(_Int_SI_24TP_AC(), _Code_SI_24TP()));
    result.push(sys::Map::new(_Int_SI_24TP_DC(), _Code_SI_24TP()));
    result.push(sys::Map::new(_Int_SI_26X_12S_AC(), _Code_SI_26X_12S_AC()));
    result.push(sys::Map::new(_Int_SI_48TP_AC(), _Code_SI_48TP()));
    result.push(sys::Map::new(_Int_SI_48TP_DC(), _Code_SI_48TP()));
    result.push(sys::Map::new(_Int_SI_24TP_PWR(), _Code_SI_24TP_PWR()));
    result.push(sys::Map::new(_Int_SI_48TP_PWR(), _Code_SI_48TP_PWR()));
    result.push(sys::Map::new(_Int_SI_28C(), _Code_SI_28C()));
    result.push(sys::Map::new(_Int_SI_52C(), _Code_SI_52C()));
    result.push(sys::Map::new(_Int_SI_28C_PWR(), _Code_SI_28C_PWR()));
    result.push(sys::Map::new(_Int_SI_52C_PWR(), _Code_SI_52C_PWR()));
    result.push(sys::Map::new(_Int_EI_28C(), _Code_EI_28C()));
    result.push(sys::Map::new(_Int_EI_28C_24S(), _Code_EI_28C_24S()));
    result.push(sys::Map::new(_Int_EI_52C(), _Code_EI_52C()));
    result.push(sys::Map::new(_Int_EI_28C_PWR(), _Code_EI_28C_PWR()));
    result.push(sys::Map::new(_Int_EI_52C_PWR(), _Code_EI_52C_PWR()));
    result.push(sys::Map::new(_Int_EI_52C_5710(), _Code_EI_52C_10()));
    result.push(sys::Map::new(_Int_EI_28C_5710(), _Code_EI_28C_10()));
    result.push(sys::Map::new(_Int_EI_28C_PWR_AC_10(), _Code_EI_28C_PWR_AC_10()));
    result.push(sys::Map::new(_Int_EI_52C_PWR_5710(), _Code_EI_52C_PWR_10()));
    result.push(sys::Map::new(_Int_EI_52C_PWR_AC_10(), _Code_EI_52C_PWR_AC_10()));
    result.push(sys::Map::new(_Int_HI_28C(), _Code_HI_28C()));
    result.push(sys::Map::new(_Int_HI_28C_24S(), _Code_HI_28C_24S()));
    result.push(sys::Map::new(_Int_HI_108C_PWR_5710(), _Code_HI_108C_PWR()));
    result.push(sys::Map::new(_Int_HI_108C_PWR_5710_B1(), _Code_10HI_108C_PWR_B1()));
    result.push(sys::Map::new(_Int_LI_28P_BAT(), _Code_LI_28P_BAT()));
    result.push(sys::Map::new(_Int_LI_28P_4AH(), _Code_LI_28P_4AH()));
    result.push(sys::Map::new(_Int_LI_28P_24S_BAT(), _Code_LI_28P_24S_BAT()));
    result.push(sys::Map::new(_Int_LI_28P_24S_4AH(), _Code_LI_28P_24S_4AH()));
    result.push(sys::Map::new(_Int_LI_28X_AC_5701(), _Code_LI_28X_AC_01()));
    result.push(sys::Map::new(_Int_LI_28X_24S_AC_01(), _Code_LI_28X_24S_AC_01()));
    result.push(sys::Map::new(_Int_LI_52X_48CS_AC(), _Code_LI_52X_48CS_AC()));
    result.push(sys::Map::new(_Int_SI_24TP_PWR_AC(), _Code_SI_24TP_PWR_AC()));
    result.push(sys::Map::new(_Int_SI_48TP_PWR_AC(), _Code_SI_48TP_PWR_AC()));
    result.push(sys::Map::new(_Int_SI_28C_AC(), _Code_SI_28C_AC()));
    result.push(sys::Map::new(_Int_SI_52C_AC(), _Code_SI_52C_AC()));
    result.push(sys::Map::new(_Int_EI_28C_AC(), _Code_EI_28C_AC()));
    result.push(sys::Map::new(_Int_EI_28C_24S_AC(), _Code_EI_28C_24S_AC()));
    result.push(sys::Map::new(_Int_EI_52C_AC(), _Code_EI_52C_AC()));
    result.push(sys::Map::new(_Int_EI_28C_PWR_AC(), _Code_EI_28C_PWR_AC()));
    result.push(sys::Map::new(_Int_EI_52C_PWR_AC(), _Code_EI_52C_PWR_AC()));
    result.push(sys::Map::new(_Int_EI_52C_AC_S5710(), _Code_EI_52C_10_AC()));
    result.push(sys::Map::new(_Int_EI_28C_AC_S5710(), _Code_EI_28C_10_AC()));
    result.push(sys::Map::new(_Int_HI_28C_AC(), _Code_HI_28C_AC()));
    result.push(sys::Map::new(_Int_HI_28C_24S_AC(), _Code_HI_28C_24S_AC()));
    result.push(sys::Map::new(_Int_LI_28TP_AC(), _Code_LI_28TP_AC()));
    result.push(sys::Map::new(_Int_LI_28TP_PWR_AC(), _Code_LI_28TP_PWR_AC()));
    result.push(sys::Map::new(_Int_LI_28TP_PWR_AC_01(), _Code_LI_28TP_PWR_AC_01()));
    result.push(sys::Map::new(_Int_HI_32C_24S_AC_5720(), _Code_HI_32C_24S_AC_5720()));
    result.push(sys::Map::new(_Int_HI_56C_AC_5720(), _Code_HI_56C_AC_5720()));
    result.push(sys::Map::new(_Int_HI_56C_PWR_AC_5720(), _Code_HI_56C_PWR_AC_5720()));
    result.push(sys::Map::new(_Int_5720EI_36C_28S_AC(), _code_5720EI_36C_28S_AC()));
    result.push(sys::Map::new(_Int_5720EI_56C_48S_AC(), _code_5720EI_56C_48S_AC()));
    result.push(sys::Map::new(_Int_5720EI_36C_AC(), _code_5720EI_36C_AC()));
    result.push(sys::Map::new(_Int_5720EI_36PC_AC(), _code_5720EI_36PC_AC()));
    result.push(sys::Map::new(_Int_5720EI_56C_AC(), _code_5720EI_56C_AC()));
    result.push(sys::Map::new(_Int_5720EI_56PC_AC(), _code_5720EI_56PC_AC()));
    result.push(sys::Map::new(_Int_5720EI_36C_PWR_AC(), _code_5720EI_36C_PWR_AC()));
    result.push(sys::Map::new(_Int_5720EI_56C_PWR_AC(), _code_5720EI_56C_PWR_AC()));
    result.push(sys::Map::new(_Int_5720EI_56C_PWR_AC1(), _code_5720EI_56C_PWR_AC1()));
    result.push(sys::Map::new(_Int_5720EI_32X_24S_AC(), _code_5720EI_32X_24S_AC()));
    result.push(sys::Map::new(_Int_5720EI_50X_46S_AC(), _code_5720EI_50X_46S_AC()));
    result.push(sys::Map::new(_Int_5720EI_32X_AC(), _code_5720EI_32X_AC()));
    result.push(sys::Map::new(_Int_5720EI_32P_AC(), _code_5720EI_32P_AC()));
    result.push(sys::Map::new(_Int_5720EI_50X_AC(), _code_5720EI_50X_AC()));
    result.push(sys::Map::new(_Int_5720EI_52X_AC(), _code_5720EI_52X_AC()));
    result.push(sys::Map::new(_Int_5720EI_52P_AC(), _code_5720EI_52P_AC()));
    result.push(sys::Map::new(_int_00S_28X_LI_AC(), _code_00S_28X_LI_AC()));
    result.push(sys::Map::new(_int_00S_52X_LI_AC(), _code_00S_52X_LI_AC()));
    result.push(sys::Map::new(_int_00S_28P_PWR_LI_AC(), _code_00S_28P_PWR_LI_AC()));
    result.push(sys::Map::new(_int_10_28X_LI_AC(), _code_10_28X_LI_AC()));
    result.push(sys::Map::new(_int_10_52X_LI_AC(), _code_10_52X_LI_AC()));
    result.push(sys::Map::new(_int_20S_28P_SI_AC(), _code_20S_28P_SI_AC()));
    result.push(sys::Map::new(_int_20S_28X_SI_AC(), _code_20S_28X_SI_AC()));
    result.push(sys::Map::new(_int_20S_52P_SI_AC(), _code_20S_52P_SI_AC()));
    result.push(sys::Map::new(_int_20S_52X_SI_AC(), _code_20S_52X_SI_AC()));
    result.push(sys::Map::new(_int_20_28P_SI_AC(), _code_20_28P_SI_AC()));
    result.push(sys::Map::new(_int_20_28X_SI_AC(), _code_20_28X_SI_AC()));
    result.push(sys::Map::new(_int_20_52P_SI_AC(), _code_20_52P_SI_AC()));
    result.push(sys::Map::new(_int_20_52X_SI_AC(), _code_20_52X_SI_AC()));
    result.push(sys::Map::new(_int_20_28X_PWR_SI_AC(), _code_20_28X_PWR_SI_AC()));
    result.push(sys::Map::new(_int_20_52X_PWR_SI_AC(), _code_20_52X_PWR_SI_AC()));
    result.push(sys::Map::new(_int_20_52X_PWR_SI_ACF(), _code_20_52X_PWR_SI_ACF()));
    result.push(sys::Map::new(_int_20_28X_PWR_SI_DC(), _Code_20_28X_PWR_SI_DC()));
    result.push(sys::Map::new(_int_20_52X_PWR_SI_DC(), _Code_20_52X_PWR_SI_DC()));
    result.push(sys::Map::new(_int_20_28X_SI_DC(), _Code_20_28X_SI_DC()));
    result.push(sys::Map::new(_int_20_52X_SI_DC(), _Code_20_52X_SI_DC()));
    result.push(sys::Map::new(_int_20S_28X_SI_DC(), _Code_20S_28X_SI_DC()));
    result.push(sys::Map::new(_int_20S_52X_SI_DC(), _Code_20S_52X_SI_DC()));
    result.push(sys::Map::new(_int_5720EI_32X_DC(), _Code_5720EI_32X_DC()));
    result.push(sys::Map::new(_int_5720EI_50X_DC(), _Code_5720EI_50X_DC()));
    result.push(sys::Map::new(_int_5720EI_32X_24S_DC(), _Code_5720EI_32X_24S_DC()));
    result.push(sys::Map::new(_int_5720EI_50X_46S_DC(), _Code_5720EI_50X_46S_DC()));
    result.push(sys::Map::new(_int_5720EI_36C_DC(), _Code_5720EI_36C_DC()));
    result.push(sys::Map::new(_int_5720EI_56C_DC(), _Code_5720EI_56C_DC()));
    result.push(sys::Map::new(_int_5720EI_36C_PWR_DC(), _Code_5720EI_36C_PWR_DC()));
    result.push(sys::Map::new(_int_5720EI_56C_PWR_DC(), _Code_5720EI_56C_PWR_DC()));
    result.push(sys::Map::new(_int_5720EI_36C_28S_DC(), _Code_5720EI_36C_28S_DC()));
    result.push(sys::Map::new(_int_5720EI_56C_48S_DC(), _Code_5720EI_56C_48S_DC()));
    result.push(sys::Map::new(_int_HI_56C_PWR_AC1_20(), _Code_HI_56C_PWR_AC1_20()));
    result.push(sys::Map::new(_int_5700_8P_G(), _Code_5700_8P_G()));
    result.push(sys::Map::new(_int_S5720_14X_PWH_SI_AC(), _Code_S5720_14X_PWH_SI_AC()));
    result.push(sys::Map::new(_int_12TP_AC_LI20(), _Code_12TP_AC_LI20()));
    result.push(sys::Map::new(_int_12TP_AC_LI20S(), _Code_12TP_AC_LI20S()));
    result.push(sys::Map::new(_int_12TP_PWRAC_LI20(), _Code_12TP_PWRAC_LI20()));
    result.push(sys::Map::new(_int_12TP_PWRAC_LI20S(), _Code_12TP_PWRAC_LI20S()));
    result.push(sys::Map::new(_int_28P_AC_LI20(), _Code_28P_AC_LI20()));
    result.push(sys::Map::new(_int_28P_AC_LI20S(), _Code_28P_AC_LI20S()));
    result.push(sys::Map::new(_int_28X_AC_LI20(), _Code_28X_AC_LI20()));
    result.push(sys::Map::new(_int_28X_AC_LI20S(), _Code_28X_AC_LI20S()));
    result.push(sys::Map::new(_int_28X_DC_LI20(), _Code_28X_DC_LI20()));
    result.push(sys::Map::new(_int_28P_PWRAC_LI20(), _Code_28P_PWRAC_LI20()));
    result.push(sys::Map::new(_int_28P_PWRAC_LI20S(), _Code_28P_PWRAC_LI20S()));
    result.push(sys::Map::new(_int_28X_PWRAC_LI20(), _Code_28X_PWRAC_LI20()));
    result.push(sys::Map::new(_int_28X_PWRAC_LI20S(), _Code_28X_PWRAC_LI20S()));
    result.push(sys::Map::new(_int_52P_AC_LI20(), _Code_52P_AC_LI20()));
    result.push(sys::Map::new(_int_52P_AC_LI20S(), _Code_52P_AC_LI20S()));
    result.push(sys::Map::new(_int_52X_AC_LI20(), _Code_52X_AC_LI20()));
    result.push(sys::Map::new(_int_52X_AC_LI20S(), _Code_52X_AC_LI20S()));
    result.push(sys::Map::new(_int_52X_DC_LI20(), _Code_52X_DC_LI20()));
    result.push(sys::Map::new(_int_52P_PWRAC_LI20(), _Code_52P_PWRAC_LI20()));
    result.push(sys::Map::new(_int_52P_PWRAC_LI20S(), _Code_52P_PWRAC_LI20S()));
    result.push(sys::Map::new(_int_52X_PWRAC_LI20(), _Code_52X_PWRAC_LI20()));
    result.push(sys::Map::new(_int_52X_PWRAC_LI20S(), _Code_52X_PWRAC_LI20S()));
    result.push(sys::Map::new(_int_28X_24S_AC_LI20(), _Code_28X_24S_AC_LI20()));
    result.push(sys::Map::new(_int_28X_24S_AC_LI20S(), _Code_28X_24S_AC_LI20S()));
    result.push(sys::Map::new(_int_28X_24S_DC_LI20(), _Code_28X_24S_DC_LI20()));
    result.push(sys::Map::new(_int_28TP_PWRACL_LI20(), _Code_28TP_PWRACL_LI20()));
    result.push(sys::Map::new(_int_28TP_PWRACL_LI20S(), _Code_28TP_PWRACL_LI20S()));
    result.push(sys::Map::new(_int_28TP_PWRAC_LI20(), _Code_28TP_PWRAC_LI20()));
    result.push(sys::Map::new(_int_28TP_AC_LI20(), _Code_28TP_AC_LI20()));
    result.push(sys::Map::new(_int_26X_PWHAC_LI20(), _Code_16X_PWHAC_LI20()));
    result.push(sys::Map::new(_int_28X_PWHAC_LI20(), _Code_28X_PWHAC_LI20()));
    result.push(sys::Map::new(_int_28X_24S_AC_SI20(), _Code_28X_24S_AC_SI20()));
    result.push(sys::Map::new(_int_28X_24S_DC_SI20(), _Code_28X_24S_DC_SI20()));
    result.push(sys::Map::new(_Int_28X24S_AC_SI21(), _Code_28X24S_AC_SI21()));
    result.push(sys::Map::new(_Int_52X_PWRACF_LI20(), _Code_52X_PWRACF_LI20()));
    result.push(sys::Map::new(_Int_48C_AC_SI30(), _Code_48C_AC_SI30()));
    result.push(sys::Map::new(_Int_48C_PWR_AC_SI30(), _Code_48C_PWR_AC_SI30()));
    result.push(sys::Map::new(_Int_68C_AC_SI30(), _Code_68C_AC_SI30()));
    result.push(sys::Map::new(_Int_68C_PWR_AC_SI30(), _Code_68C_PWR_AC_SI30()));
    result.push(sys::Map::new(_Int_68C_PWR_SI30(), _Code_68C_PWR_SI30()));
    result.push(sys::Map::new(_Int_48C_AC_EI30S(), _Code_48C_AC_EI30S()));
    result.push(sys::Map::new(_Int_48C_PWR_EI30S(), _Code_48C_PWR_EI30S()));
    result.push(sys::Map::new(_Int_68C_AC_EI30S(), _Code_68C_AC_EI30S()));
    result.push(sys::Map::new(_Int_68C_PWR_EI30S(), _Code_68C_PWR_EI30S()));
    result.push(sys::Map::new(_Int_20I_12X_SI_AC(), _Code_20I_12X_SI_AC()));
    result.push(sys::Map::new(_Int_20I_12X_PWH_SI_DC(), _Code_20I_12X_PWH_SI_DC()));
    result.push(sys::Map::new(_Int_20I_28X_SI_AC(), _Code_20I_28X_SI_AC()));
    result.push(sys::Map::new(_Int_20I_28X_PWH_SI_AC(), _Code_20I_28X_PWH_SI_AC()));
    result.push(sys::Map::new(_Int_30_36C_HI(), _Code_30_36C_HI()));
    result.push(sys::Map::new(_Int_30_44C_HI(), _Code_30_44C_HI()));
    result.push(sys::Map::new(_Int_30_36C_PWH_HI(), _Code_30_36C_PWH_HI()));
    result.push(sys::Map::new(_Int_30_44C_PWH_HI(), _Code_30_44C_PWH_HI()));
    result.push(sys::Map::new(_Int_30_60C_HI(), _Code_30_60C_HI()));
    result.push(sys::Map::new(_Int_30_68C_HI(), _Code_30_68C_HI()));
    result.push(sys::Map::new(_Int_30_60C_PWH_HI(), _Code_30_60C_PWH_HI()));
    result.push(sys::Map::new(_Int_30_68C_PWH_HI(), _Code_30_68C_PWH_HI()));
    result.push(sys::Map::new(_Int_20SV2_28P_LIAC(), _Code_20SV2_28P_LIAC()));
    result.push(sys::Map::new(_Int_20SV2_52P_LIAC(), _Code_20SV2_52P_LIAC()));
    result.push(sys::Map::new(_Int_20I_10XPWH_SI_AC(), _Code_20I_10XPWH_SI_AC()));
    result.push(sys::Map::new(_Int_20I_6XPWH_SI_AC(), _Code_20I_6XPWH_SI_AC()));
    result.push(sys::Map::new(_Int_30_60C_HI_48S(), _Code_30_60C_HI_48S()));
    result.push(sys::Map::new(_Int_30_68C_HI_48S(), _Code_30_68C_HI_48S()));
    result.push(sys::Map::new(_Int_30_36C_HI_24S(), _Code_30_36C_HI_24S()));
    result.push(sys::Map::new(_Int_30_44C_HI_24S(), _Code_30_44C_HI_24S()));
    result.push(sys::Map::new(_Int_20_52X_LI_48S_AC(), _Code_20_52X_LI_48S_AC()));
    result.push(sys::Map::new(_Int_20_52X_SI_48S(), _Code_20_52X_SI_48S()));
    result.push(sys::Map::new(_Int_20_28XPWR_LI_ACF(), _Code_20_28XPWR_LI_ACF()));
    result
}


pub fn _Int_5720EI_36PC_AC() -> String { "73".to_string() }
pub fn _Int_LI_52X_48CS_AC() -> String { "49".to_string() }
pub fn _Int_5720EI_56C_48S_AC() -> String { "71".to_string() }
pub fn _Int_EI_52C_AC() -> String { "56".to_string() }
pub fn _int_12TP_PWRAC_LI20S() -> String { "125".to_string() }
pub fn _int_26X_PWHAC_LI20() -> String { "151".to_string() }
pub fn _Int_SI_28C_AC() -> String { "52".to_string() }
pub fn _int_20_52X_SI_DC() -> String { "105".to_string() }
pub fn _Int_EI_52C_PWR_5710() -> String { "38".to_string() }
pub fn _Int_68C_AC_SI30() -> String { "159".to_string() }
pub fn _Int_EI_28C_24S_AC() -> String { "55".to_string() }
pub fn _Int_68C_AC_EI30S() -> String { "164".to_string() }
pub fn _Int_68C_PWR_AC_SI30() -> String { "160".to_string() }
pub fn _Int_48C_PWR_AC_SI30() -> String { "158".to_string() }
pub fn _int_28TP_PWRACL_LI20() -> String { "147".to_string() }
pub fn _int_20S_28X_SI_AC() -> String { "92".to_string() }
pub fn _int_52X_AC_LI20() -> String { "137".to_string() }
pub fn _int_28P_AC_LI20S() -> String { "127".to_string() }
pub fn _Int_5720EI_36C_PWR_AC() -> String { "76".to_string() }
pub fn _Int_SI_48TP_PWR() -> String { "25".to_string() }
pub fn _int_20_52X_SI_AC() -> String { "98".to_string() }
pub fn _Int_SI_28C_PWR() -> String { "28".to_string() }
pub fn _Int_30_60C_PWH_HI() -> String { "176".to_string() }
pub fn _int_20S_28X_SI_DC() -> String { "106".to_string() }
pub fn _Int_5720EI_56C_PWR_AC() -> String { "77".to_string() }
pub fn _Int_68C_PWR_EI30S() -> String { "165".to_string() }
pub fn _Int_30_36C_HI_24S() -> String { "184".to_string() }
pub fn _int_20_52X_PWR_SI_ACF() -> String { "101".to_string() }
pub fn _Int_HI_28C() -> String { "40".to_string() }
pub fn _int_28TP_AC_LI20() -> String { "150".to_string() }
pub fn _Int_20I_28X_SI_AC() -> String { "168".to_string() }
pub fn _Int_EI_52C() -> String { "32".to_string() }
pub fn _int_28X_DC_LI20() -> String { "130".to_string() }
pub fn _Int_30_68C_PWH_HI() -> String { "177".to_string() }
pub fn _Int_LI_28X_24S_DC() -> String { "14".to_string() }
pub fn _Int_52X_PWRACF_LI20() -> String { "156".to_string() }
pub fn _Int_20SV2_52P_LIAC() -> String { "179".to_string() }
pub fn _Int_20SV2_28P_LIAC() -> String { "178".to_string() }
pub fn _int_5720EI_36C_28S_DC() -> String { "116".to_string() }
pub fn _Int_SI_24TP_AC() -> String { "19".to_string() }
pub fn _Int_HI_28C_24S_AC() -> String { "62".to_string() }
pub fn _Int_20I_12X_SI_AC() -> String { "166".to_string() }
pub fn _Int_EI_28C_5710() -> String { "36".to_string() }
pub fn _Int_HI_28C_24S() -> String { "41".to_string() }
pub fn _Int_LI_28P_24S_BAT() -> String { "45".to_string() }
pub fn _Int_5720EI_52P_AC() -> String { "85".to_string() }
pub fn _Int_SI_48TP_AC() -> String { "22".to_string() }
pub fn _int_28X_PWHAC_LI20() -> String { "154".to_string() }
pub fn _Int_LI_28P_DC() -> String { "5".to_string() }
pub fn _Int_LI_28P_4AH() -> String { "44".to_string() }
pub fn _int_HI_56C_PWR_AC1_20() -> String { "118".to_string() }
pub fn _Int_SI_24TP_DC() -> String { "20".to_string() }
pub fn _int_52X_PWRAC_LI20() -> String { "142".to_string() }
pub fn _Int_48C_AC_SI30() -> String { "157".to_string() }
pub fn _Int_20_52X_SI_48S() -> String { "187".to_string() }
pub fn _Int_30_36C_HI() -> String { "170".to_string() }
pub fn _Int_20_28XPWR_LI_ACF() -> String { "188".to_string() }
pub fn _int_28X_AC_LI20S() -> String { "129".to_string() }
pub fn _Int_HI_28C_AC() -> String { "61".to_string() }
pub fn _Int_48C_AC_EI30S() -> String { "162".to_string() }
pub fn _Int_30_60C_HI_48S() -> String { "182".to_string() }
pub fn _Int_LI_28P_BAT() -> String { "43".to_string() }
pub fn _Int_HI_32C_24S_AC_5720() -> String { "67".to_string() }
pub fn _int_52P_PWRAC_LI20() -> String { "140".to_string() }
pub fn _int_00S_52X_LI_AC() -> String { "87".to_string() }
pub fn _Int_5720EI_56C_PWR_AC1() -> String { "78".to_string() }
pub fn _Int_LI_52P_S_AC() -> String { "3".to_string() }
pub fn _int_S5720_14X_PWH_SI_AC() -> String { "121".to_string() }
pub fn _Int_30_68C_HI() -> String { "175".to_string() }
pub fn _int_10_28X_LI_AC() -> String { "89".to_string() }
pub fn _int_5720EI_50X_46S_DC() -> String { "111".to_string() }
pub fn _Int_LI_28TP_AC() -> String { "63".to_string() }
pub fn _Int_LI_52X_PWR_AC() -> String { "18".to_string() }
pub fn _Int_EI_28C_PWR() -> String { "33".to_string() }
pub fn _int_28X_24S_AC_SI20() -> String { "152".to_string() }
pub fn _Int_30_44C_HI_24S() -> String { "185".to_string() }
pub fn _Int_LI_10P_PWR_AC() -> String { "8".to_string() }
pub fn _Int_48C_PWR_EI30S() -> String { "163".to_string() }
pub fn _Int_HI_56C_PWR_AC_5720() -> String { "69".to_string() }
pub fn _Int_5720EI_50X_AC() -> String { "83".to_string() }
pub fn _Int_SI_48TP_PWR_AC() -> String { "51".to_string() }
pub fn _int_12TP_AC_LI20() -> String { "122".to_string() }
pub fn _int_52P_AC_LI20() -> String { "135".to_string() }
pub fn _int_5720EI_56C_PWR_DC() -> String { "115".to_string() }
pub fn _int_28P_AC_LI20() -> String { "126".to_string() }
pub fn _Int_LI_28TP_PWR_AC_01() -> String { "65".to_string() }
pub fn _Int_LI_28P_AC() -> String { "4".to_string() }
pub fn _Int_28X24S_AC_SI21() -> String { "155".to_string() }
pub fn _Int_20_52X_LI_48S_AC() -> String { "186".to_string() }
pub fn _int_20_28X_PWR_SI_DC() -> String { "102".to_string() }
pub fn _Int_SI_52C_AC() -> String { "53".to_string() }
pub fn _Int_5720EI_32P_AC() -> String { "82".to_string() }
pub fn _int_12TP_PWRAC_LI20() -> String { "124".to_string() }
pub fn _int_28X_AC_LI20() -> String { "128".to_string() }

pub fn _int_12TP_AC_LI20S() -> String { "123".to_string() }


pub fn _int_52X_AC_LI20S() -> String { "138".to_string() }


pub fn _int_00S_28X_LI_AC() -> String { "86".to_string() }

pub fn _int_28X_24S_DC_SI20() -> String { "153".to_string() }

pub fn _Int_LI_52P_AC() -> String { "6".to_string() }

pub fn _int_5720EI_56C_48S_DC() -> String { "117".to_string() }


pub fn _int_28X_PWRAC_LI20S() -> String { "134".to_string() }


pub fn _Int_5720EI_52X_AC() -> String { "84".to_string() }


pub fn _int_28P_PWRAC_LI20S() -> String { "132".to_string() }

pub fn _int_20_28X_SI_DC() -> String { "104".to_string() }

pub fn _Int_LI_52P_PWR_AC() -> String { "10".to_string() }

pub fn _int_20S_52P_SI_AC() -> String { "93".to_string() }


pub fn _int_20_28X_PWR_SI_AC() -> String { "99".to_string() }

pub fn _Int_20I_28X_PWH_SI_AC() -> String { "169".to_string() }

pub fn _int_28TP_PWRACL_LI20S() -> String { "148".to_string() }

pub fn _int_28X_24S_AC_LI20() -> String { "144".to_string() }

pub fn _Int_5720EI_50X_46S_AC() -> String { "80".to_string() }

pub fn _Int_SI_28C() -> String { "26".to_string() }

pub fn _int_5720EI_32X_DC() -> String { "108".to_string() }

pub fn _int_5720EI_32X_24S_DC() -> String { "110".to_string() }

pub fn _Int_SI_52C() -> String { "27".to_string() }

pub fn _int_20S_52X_SI_AC() -> String { "94".to_string() }

pub fn _Int_SI_52C_PWR() -> String { "29".to_string() }

pub fn _int_5720EI_36C_PWR_DC() -> String { "114".to_string() }


pub fn _Int_SI_48TP_DC() -> String { "23".to_string() }

pub fn _Int_LI_52P_DC() -> String { "7".to_string() }

pub fn _Int_5720EI_56C_AC() -> String { "74".to_string() }

pub fn _Int_EI_28C_PWR_AC_10() -> String { "37".to_string() }

pub fn _int_20_52P_SI_AC() -> String { "97".to_string() }

pub fn _Int_20I_6XPWH_SI_AC() -> String { "181".to_string() }


pub fn _Int_HI_56C_AC_5720() -> String { "68".to_string() }

pub fn _int_52P_AC_LI20S() -> String { "136".to_string() }

pub fn _int_5720EI_50X_DC() -> String { "109".to_string() }

pub fn _int_52P_PWRAC_LI20S() -> String { "141".to_string() }


pub fn _Int_SI_26X_12S_AC() -> String { "21".to_string() }

pub fn _Int_LI_28P_24S_4AH() -> String { "46".to_string() }


pub fn _Int_20I_12X_PWH_SI_DC() -> String { "167".to_string() }

pub fn _int_10_52X_LI_AC() -> String { "90".to_string() }


pub fn _Int_30_68C_HI_48S() -> String { "183".to_string() }


pub fn _int_00S_28P_PWR_LI_AC() -> String { "88".to_string() }

pub fn _Int_5720EI_36C_AC() -> String { "72".to_string() }


pub fn _Int_EI_28C_AC() -> String { "54".to_string() }

pub fn _int_20S_52X_SI_DC() -> String { "107".to_string() }


pub fn _int_20_52X_PWR_SI_AC() -> String { "100".to_string() }

pub fn _Int_EI_52C_PWR_AC_10() -> String { "39".to_string() }


pub fn _int_28X_24S_AC_LI20S() -> String { "145".to_string() }


pub fn _int_5700_8P_G() -> String { "120".to_string() }

pub fn _int_20S_28P_SI_AC() -> String { "91".to_string() }

pub fn _Int_HI_108C_PWR_5710_B1() -> String { "119".to_string() }


pub fn _Int_EI_52C_5710() -> String { "35".to_string() }


pub fn _Int_5720EI_36C_28S_AC() -> String { "70".to_string() }

pub fn _int_28TP_PWRAC_LI20() -> String { "149".to_string() }

pub fn _Int_5720EI_32X_24S_AC() -> String { "79".to_string() }

pub fn _Int_LI_28X_AC_5701() -> String { "47".to_string() }


pub fn _Int_EI_52C_AC_S5710() -> String { "59".to_string() }

pub fn _Int_LI_28P_S_AC() -> String { "2".to_string() }

pub fn _int_52X_PWRAC_LI20S() -> String { "143".to_string() }

pub fn _Int_EI_28C_AC_S5710() -> String { "60".to_string() }

pub fn _Int_30_44C_HI() -> String { "171".to_string() }

pub fn _int_52X_DC_LI20() -> String { "139".to_string() }


pub fn _Int_LI_28X_24S_AC() -> String { "13".to_string() }


pub fn _int_5720EI_36C_DC() -> String { "112".to_string() }

pub fn _Int_68C_PWR_SI30() -> String { "161".to_string() }

pub fn _Int_SI_24TP_PWR_AC() -> String { "50".to_string() }

pub fn _Int_SI_24TP_PWR() -> String { "24".to_string() }

pub fn _int_28X_24S_DC_LI20() -> String { "146".to_string() }

pub fn _Int_5720EI_32X_AC() -> String { "81".to_string() }

pub fn _Int_LI_28X_DC() -> String { "12".to_string() }


pub fn _Int_5720EI_56PC_AC() -> String { "75".to_string() }

pub fn _Int_LI_28X_PWR_AC() -> String { "17".to_string() }

pub fn _Int_LI_28P_PWR_AC() -> String { "9".to_string() }


pub fn _int_20_28P_SI_AC() -> String { "95".to_string() }

pub fn _int_5720EI_56C_DC() -> String { "113".to_string() }


pub fn _Int_LI_28X_AC() -> String { "11".to_string() }

pub fn _Int_30_60C_HI() -> String { "174".to_string() }

pub fn _Int_EI_52C_PWR() -> String { "34".to_string() }

pub fn _Int_LI_28X_24S_AC_01() -> String { "48".to_string() }


pub fn _Int_LI_52X_AC() -> String { "15".to_string() }


pub fn _Int_HI_108C_PWR_5710() -> String { "42".to_string() }


pub fn _Int_LI_28TP_PWR_AC() -> String { "64".to_string() }


pub fn _Int_EI_28C_PWR_AC() -> String { "57".to_string() }

pub fn _Int_LI_52X_DC() -> String { "16".to_string() }

pub fn _Int_LI_10P_AC() -> String { "1".to_string() }


pub fn _Int_EI_52C_PWR_AC() -> String { "58".to_string() }


pub fn _Int_20I_10XPWH_SI_AC() -> String { "180".to_string() }

pub fn _int_20_28X_SI_AC() -> String { "96".to_string() }

pub fn _int_20_52X_PWR_SI_DC() -> String { "103".to_string() }


pub fn _Int_30_36C_PWH_HI() -> String { "172".to_string() }


pub fn _Int_30_44C_PWH_HI() -> String { "173".to_string() }

pub fn _Int_EI_28C_24S() -> String { "31".to_string() }

pub fn _Int_EI_28C() -> String { "30".to_string() }

pub fn _int_28X_PWRAC_LI20() -> String { "133".to_string() }

pub fn _int_28P_PWRAC_LI20() -> String { "131".to_string() }


pub fn _Code_LI_10P_AC() -> String {
    "EW012484".to_string()
}

pub fn _Code_LI_28P_AC_S() -> String {
    "EW012486".to_string()
}

pub fn _Code_5720EI_32X_DC() -> String { "EW027254".to_string() }

pub fn _Code_EI_52C_10_AC() -> String { "EW022837".to_string() }

pub fn _Code_28X_PWRAC_LI20() -> String { "EW031763".to_string() }

pub fn _Code_SI_48TP() -> String { "EW012514".to_string() }

pub fn _Code_20I_12X_SI_AC() -> String { "EW047757".to_string() }

pub fn _Code_52P_PWRAC_LI20() -> String { "EW031777".to_string() }

pub fn _Code_28X_DC_LI20() -> String { "EW031757".to_string() }

pub fn _code_5720EI_36PC_AC() -> String { "EW022043".to_string() }

pub fn _code_20_52P_SI_AC() -> String { "EW025175".to_string() }

pub fn _Code_EI_28C_PWR_AC() -> String { "EW022839".to_string() }

pub fn _Code_EI_28C_10() -> String { "EW022011".to_string() }

pub fn _Code_52P_PWRAC_LI20S() -> String { "EW031779".to_string() }

pub fn _Code_52X_PWRAC_LI20() -> String { "EW031781".to_string() }

pub fn _Code_68C_AC_SI30() -> String { "EW039792".to_string() }

pub fn _Code_5720EI_36C_DC() -> String { "EW027262".to_string() }

pub fn _code_20_52X_SI_AC() -> String { "EW025177".to_string() }

pub fn _Code_LI_28P_24S_BAT() -> String { "EW017173".to_string() }

pub fn _Code_20_28X_SI_DC() -> String { "EW027246".to_string() }

pub fn _code_5720EI_56C_48S_AC() -> String { "EW022039".to_string() }

pub fn _Code_SI_28C_OLD() -> String { "EW012520".to_string() }

pub fn _Code_52X_PWRACF_LI20() -> String { "EW039786".to_string() }

pub fn _Code_EI_52C_10() -> String { "EW022021".to_string() }

pub fn _Code_48C_AC_EI30S() -> String { "EW039798".to_string() }

pub fn _Code_5720EI_36C_28S_DC() -> String { "EW027270".to_string() }

pub fn _Code_68C_PWR_EI30S() -> String { "EW039804".to_string() }

pub fn _Code_EI_28C_24S_OLD() -> String { "EW012530".to_string() }

pub fn _Code_HI_108C_PWR() -> String { "EW012552".to_string() }

pub fn _Code_SI_24TP_PWR() -> String { "EW021996".to_string() }

pub fn _Code_LI_28X_AC_OLD() -> String { "EW012500".to_string() }

pub fn _Code_20_52X_SI_48S() -> String { "EW051612".to_string() }

pub fn _code_20_28X_SI_AC() -> String { "EW025169".to_string() }

pub fn _Code_LI_28X_24S_AC() -> String { "EW022817".to_string() }

pub fn _Code_SI_28C_PWR() -> String { "EW012524".to_string() }

pub fn _Code_EI_28C_PWR_OLD() -> String { "EW012534".to_string() }


pub fn _Code_EI_28C_OLD() -> String { "EW012528".to_string() }


pub fn _Code_48C_PWR_AC_SI30() -> String { "EW039790".to_string() }

pub fn _Code_20I_6XPWH_SI_AC() -> String { "EW051600".to_string() }

pub fn _Code_LI_28X_24S_DC() -> String { "EW021953".to_string() }


pub fn _code_20_28X_PWR_SI_AC() -> String { "EW025181".to_string() }


pub fn _Code_EI_28C_24S_AC() -> String { "EW022831".to_string() }

pub fn _Code_LI_52P_PWR_AC() -> String { "EW012498".to_string() }

pub fn _code_5720EI_32P_AC() -> String { "EW022061".to_string() }

pub fn _Code_48C_PWR_EI30S() -> String { "EW039800".to_string() }

pub fn _Code_EI_52C_OLD() -> String { "EW012532".to_string() }

pub fn _Code_30_60C_HI() -> String { "EW047816".to_string() }

pub fn _Code_SI_26X_12S_AC() -> String { "EW012512".to_string() }

pub fn _Code_28X_24S_DC_SI20() -> String { "EW031803".to_string() }

pub fn _Code_20I_28X_SI_AC() -> String { "EW047768".to_string() }

pub fn _Code_S5720_14X_PWH_SI_AC() -> String { "EW029888".to_string() }

pub fn _Code_28P_AC_LI20S() -> String { "EW031751".to_string() }

pub fn _Code_30_60C_PWH_HI() -> String { "EW047820".to_string() }


pub fn _Code_28X_24S_AC_LI20() -> String { "EW031785".to_string() }

pub fn _Code_20_52X_PWR_SI_DC() -> String { "EW027244".to_string() }

pub fn _Code_52X_AC_LI20S() -> String { "EW031773".to_string() }

pub fn _Code_HI_28C_24S_OLD() -> String { "EW012550".to_string() }

pub fn _Code_LI_52X_PWR_AC() -> String { "EW012508".to_string() }


pub fn _Code_28P_PWRAC_LI20S() -> String { "EW031761".to_string() }


pub fn _Code_EI_52C_PWR_10() -> String { "EW012544".to_string() }

pub fn _Code_12TP_AC_LI20S() -> String { "EW031743".to_string() }

pub fn _Code_EI_28C_10_AC() -> String { "EW022833".to_string() }

pub fn _Code_20I_28X_PWH_SI_AC() -> String { "EW047771".to_string() }

pub fn _Code_LI_28P_4AH() -> String { "EW017171".to_string() }

pub fn _Code_52P_AC_LI20() -> String { "EW031767".to_string() }

pub fn _Code_LI_28P_BAT() -> String { "EW017169".to_string() }

pub fn _Code_30_68C_PWH_HI() -> String { "EW047822".to_string() }

pub fn _code_5720EI_32X_AC() -> String { "EW022059".to_string() }


pub fn _Code_30_36C_HI() -> String { "EW047774".to_string() }

pub fn _Code_EI_52C() -> String { "EW022013".to_string() }

pub fn _Code_68C_PWR_SI30() -> String { "EW039796".to_string() }

pub fn _Code_SI_28C_AC() -> String { "EW022821".to_string() }

pub fn _Code_28X_PWRAC_LI20S() -> String { "EW031765".to_string() }

pub fn _Code_20I_10XPWH_SI_AC() -> String { "EW051598".to_string() }

pub fn _Code_LI_28P_PWR_AC() -> String { "EW012496".to_string() }


pub fn _code_5720EI_56C_AC() -> String { "EW022045".to_string() }


pub fn _Code_HI_28C_AC() -> String { "EW022843".to_string() }

pub fn _Code_20_52X_SI_DC() -> String { "EW027248".to_string() }

pub fn _Code_28X_AC_LI20S() -> String { "EW031755".to_string() }

pub fn _Code_EI_28C_PWR() -> String { "EW022023".to_string() }


pub fn _Code_SI_52C() -> String { "EW021968".to_string() }

pub fn _Code_LI_28P() -> String { "EW012490".to_string() }

pub fn _Code_HI_28C_OLD() -> String { "EW012548".to_string() }

pub fn _Code_EI_28C_PWR_AC_10() -> String { "EW012542".to_string() }

pub fn _code_00S_52X_LI_AC() -> String { "EW025151".to_string() }

pub fn _Code_28X24S_AC_SI21() -> String { "EW039784".to_string() }


pub fn _Code_10HI_108C_PWR_B1() -> String { "EW028583".to_string() }

pub fn _Code_28TP_PWRACL_LI20() -> String { "EW031791".to_string() }


pub fn _Code_16X_PWHAC_LI20() -> String { "EW031799".to_string() }

pub fn _Code_SI_24TP_PWR_OLD() -> String { "EW012516".to_string() }


pub fn _Code_12TP_PWRAC_LI20S() -> String { "EW031747".to_string() }


pub fn _Code_28P_PWRAC_LI20() -> String { "EW031759".to_string() }

pub fn _Code_28X_24S_AC_LI20S() -> String { "EW031787".to_string() }

pub fn _Code_30_44C_HI_24S() -> String { "EW051608".to_string() }

pub fn _Code_20S_28X_SI_DC() -> String { "EW027250".to_string() }

pub fn _Code_EI_52C_PWR_AC_10() -> String { "EW012546".to_string() }

pub fn _Code_LI_28X_DC() -> String { "EW021955".to_string() }


pub fn _Code_EI_28C_10_OLD() -> String { "EW012540".to_string() }

pub fn _Code_28TP_PWRAC_LI20() -> String { "EW031795".to_string() }

pub fn _code_20_52X_PWR_SI_ACF() -> String { "EW025185".to_string() }


pub fn _Code_LI_52X_AC_OLD() -> String { "EW012504".to_string() }

pub fn _Code_28X_PWHAC_LI20() -> String { "EW036203".to_string() }

pub fn _code_5720EI_56C_PWR_AC() -> String { "EW022051".to_string() }
pub fn _Code_EI_52C_AC() -> String { "EW022835".to_string() }
pub fn _Code_EI_52C_PWR() -> String { "EW022025".to_string() }
pub fn _Code_HI_56C_PWR_AC1_20() -> String { "EW028238".to_string() }
pub fn _Code_5720EI_32X_24S_DC() -> String { "EW027258".to_string() }
pub fn _Code_SI_48TP_PWR() -> String { "EW021998".to_string() }
pub fn _Code_28X_AC_LI20() -> String { "EW031753".to_string() }
pub fn _Code_LI_28P_24S_4AH() -> String { "EW017175".to_string() }
pub fn _code_20_52X_PWR_SI_AC() -> String { "EW025183".to_string() }
pub fn _Code_SI_52C_OLD() -> String { "EW012522".to_string() }
pub fn _code_5720EI_56C_PWR_AC1() -> String { "EW022053".to_string() }
pub fn _Code_EI_28C() -> String { "EW022005".to_string() }
pub fn _Code_48C_AC_SI30() -> String { "EW039788".to_string() }
pub fn _Code_5720EI_36C_PWR_DC() -> String { "EW027266".to_string() }
pub fn _Code_LI_28TP_AC() -> String { "EW020442".to_string() }
pub fn _Code_20_28XPWR_LI_ACF() -> String { "EW051614".to_string() }
pub fn _Code_5720EI_56C_PWR_DC() -> String { "EW027268".to_string() }
pub fn _code_5720EI_56PC_AC() -> String { "EW022047".to_string() }
pub fn _Code_HI_28C_24S() -> String { "EW022029".to_string() }
pub fn _Code_LI_28X_AC_01() -> String { "EW017177".to_string() }
pub fn _code_5720EI_50X_46S_AC() -> String { "EW022057".to_string() }
pub fn _Code_HI_28C() -> String { "EW022027".to_string() }
pub fn _code_10_28X_LI_AC() -> String { "EW025155".to_string() }
pub fn _Code_HI_32C_24S_AC_5720() -> String { "EW020448".to_string() }
pub fn _Code_SI_52C_PWR() -> String { "EW012526".to_string() }
pub fn _Code_LI_10P_PWR_AC() -> String { "EW012494".to_string() }
pub fn _Code_EI_52C_PWR_OLD() -> String { "EW012536".to_string() }
pub fn _Code_5720EI_50X_46S_DC() -> String { "EW027260".to_string() }
pub fn _Code_SI_24TP_PWR_AC() -> String { "EW022825".to_string() }
pub fn _Code_EI_28C_AC() -> String { "EW022829".to_string() }
pub fn _Code_LI_52P() -> String { "EW012492".to_string() }
pub fn _code_5720EI_50X_AC() -> String { "EW022063".to_string() }
pub fn _Code_30_36C_HI_24S() -> String { "EW051606".to_string() }
pub fn _Code_28TP_AC_LI20() -> String { "EW031797".to_string() }
pub fn _code_5720EI_52X_AC() -> String { "EW022065".to_string() }
pub fn _code_20S_52X_SI_AC() -> String { "EW025165".to_string() }
pub fn _Code_20I_12X_PWH_SI_DC() -> String { "EW047764".to_string() }
pub fn _Code_HI_28C_24S_AC() -> String { "EW022845".to_string() }
pub fn _Code_LI_28X_AC() -> String { "EW022815".to_string() }
pub fn _Code_30_60C_HI_48S() -> String { "EW051602".to_string() }
pub fn _code_5720EI_36C_PWR_AC() -> String { "EW022049".to_string() }
pub fn _Code_LI_28X_PWR_AC() -> String { "EW012506".to_string() }
pub fn _Code_28X_24S_AC_SI20() -> String { "EW031801".to_string() }
pub fn _code_20S_52P_SI_AC() -> String { "EW025163".to_string() }
pub fn _code_20S_28X_SI_AC() -> String { "EW025161".to_string() }
pub fn _Code_LI_52X_48CS_AC() -> String { "EW017181".to_string() }
pub fn _Code_EI_52C_PWR_AC() -> String { "EW022841".to_string() }
pub fn _Code_28P_AC_LI20() -> String { "EW031749".to_string() }
pub fn _Code_LI_52X_DC() -> String { "EW021951".to_string() }
pub fn _Code_5700_8P_G() -> String { "EW029886".to_string() }
pub fn _Code_20_52X_LI_48S_AC() -> String { "EW051610".to_string() }
pub fn _Code_SI_28C() -> String { "EW021958".to_string() }
pub fn _Code_LI_28X_24S_AC_01() -> String { "EW017179".to_string() }
pub fn _Code_LI_28X_24S_AC_OLD() -> String { "EW012502".to_string() }
pub fn _Code_20S_52X_SI_DC() -> String { "EW027252".to_string() }
pub fn _Code_SI_24TP() -> String { "EW012510".to_string() }
pub fn _Code_20_28X_PWR_SI_DC() -> String { "EW027242".to_string() }
pub fn _Code_68C_AC_EI30S() -> String { "EW039802".to_string() }
pub fn _code_00S_28X_LI_AC() -> String { "EW025149".to_string() }
pub fn _Code_SI_48TP_PWR_AC() -> String { "EW022827".to_string() }
pub fn _Code_HI_56C_PWR_AC_5720() -> String { "EW020452".to_string() }
pub fn _Code_52P_AC_LI20S() -> String { "EW031769".to_string() }
pub fn _Code_SI_48TP_PWR_OLD() -> String { "EW012518".to_string() }
pub fn _Code_EI_28C_24S() -> String { "EW022009".to_string() }
pub fn _Code_HI_56C_AC_5720() -> String { "EW020450".to_string() }
pub fn _Code_LI_28TP_PWR_AC() -> String { "EW020444".to_string() }
pub fn _Code_52X_DC_LI20() -> String { "EW031775".to_string() }
pub fn _Code_5720EI_50X_DC() -> String { "EW027256".to_string() }
pub fn _Code_20SV2_52P_LIAC() -> String { "EW049555".to_string() }
pub fn _Code_30_68C_HI_48S() -> String { "EW051604".to_string() }
pub fn _code_10_52X_LI_AC() -> String { "EW025157".to_string() }
pub fn _code_5720EI_32X_24S_AC() -> String { "EW022055".to_string() }
pub fn _Code_5720EI_56C_DC() -> String { "EW027264".to_string() }
pub fn _Code_30_44C_HI() -> String { "EW047776".to_string() }
pub fn _Code_EI_52C_10_OLD() -> String { "EW012538".to_string() }
pub fn _Code_52X_PWRAC_LI20S() -> String { "EW031783".to_string() }
pub fn _Code_30_36C_PWH_HI() -> String { "EW047778".to_string() }
pub fn _Code_12TP_AC_LI20() -> String { "EW031741".to_string() }
pub fn _Code_28TP_PWRACL_LI20S() -> String { "EW031793".to_string() }
pub fn _code_00S_28P_PWR_LI_AC() -> String { "EW025153".to_string() }
pub fn _Code_5720EI_56C_48S_DC() -> String { "EW027272".to_string() }
pub fn _Code_LI_52X_AC() -> String { "EW022819".to_string() }
pub fn _Code_30_68C_HI() -> String { "EW047818".to_string() }
pub fn _Code_52X_AC_LI20() -> String { "EW031771".to_string() }
pub fn _Code_LI_28TP_PWR_AC_01() -> String { "EW020446".to_string() }
pub fn _code_20_28P_SI_AC() -> String { "EW025167".to_string() }
pub fn _Code_12TP_PWRAC_LI20() -> String { "EW031745".to_string() }
pub fn _Code_30_44C_PWH_HI() -> String { "EW047780".to_string() }
pub fn _code_5720EI_52P_AC() -> String { "EW022067".to_string() }
pub fn _code_5720EI_36C_AC() -> String { "EW022041".to_string() }
pub fn _Code_28X_24S_DC_LI20() -> String { "EW031789".to_string() }
pub fn _Code_LI_52P_AC_S() -> String { "EW012488".to_string() }
pub fn _Code_SI_52C_AC() -> String { "EW022823".to_string() }
pub fn _code_20S_28P_SI_AC() -> String { "EW025159".to_string() }
pub fn _Code_68C_PWR_AC_SI30() -> String { "EW039794".to_string() }
pub fn _code_5720EI_36C_28S_AC() -> String { "EW022037".to_string() }
pub fn _Code_20SV2_28P_LIAC() -> String { "EW049553".to_string() }

pub mod test {
    use super::*;

    #[test]
    fn test1() {}
}
