//this.var_productCode_Config = function() {
//if (sys._objectEquals(pde.___PRODUCTCODE__(), mw007312_def._Code_ECFG())) {
//return mw007312_def._Code_ECFG();
//} else {
//return mw007312_def._getEW_num(mw007312parameterdef._Para_to_IntValue(), mw007312_def._Host_Init_EW_Num());
//}
//}

use super::mw007312_def;
use crate::sys;
use sys::pde;
use wasm_bindgen::__rt::std::collections::HashMap;
use sys::getP;

pub fn var_productCode_Config() ->String {
    if sys::_objectEqual(pde::___PRODUCTCODE__(), mw007312_def::_Code_ECFG()) {
        mw007312_def::_Code_ECFG()
    } else {
        mw007312_def::_getEW_num(getP(""))
    }
}

pub fn _Para_to_IntValue() ->Vec<sys::map>  {
    let mut result: Vec<sys::map> = Vec::new();
    result.push( sys::map::new(getP("P_LI_10P_AC"), mw007312_def::_Int_LI_10P_AC()) );
    result.push(sys::map::new(getP("P_LI_28P_AC"), mw007312_def::P_LI_28P_AC()));
    result.push(sys::map::new(getP("P_LI_28P_DC"), mw007312_def::_Int_LI_28P_DC()));
    result.push(sys::map::new(getP("P_LI_28P_S_AC"), mw007312_def::_Int_LI_28P_S_AC()));
    result.push(sys::map::new(getP("P_LI_52P_AC"), mw007312_def::_Int_LI_52P_AC() ));
    result.push(sys::map::new(getP("P_LI_52P_DC"), mw007312_def::_Int_LI_52P_DC()));

}

return [new HashMap(GetP("P_LI_52P_S_AC"),mw007312_def._Int_LI_52P_S_AC()), new HashMap(GetP("P_LI_10P_PWR_AC"),mw007312_def._Int_LI_10P_PWR_AC()), new HashMap(GetP("P_LI_28P_PWR_AC"),mw007312_def._Int_LI_28P_PWR_AC()), new HashMap(GetP("P_LI_52P_PWR_AC"),mw007312_def._Int_LI_52P_PWR_AC()), new HashMap(GetP("P_LI_28P_BAT"),mw007312_def._Int_LI_28P_BAT()), new HashMap(GetP("P_LI_28P_4AH"),mw007312_def._Int_LI_28P_4AH()), new HashMap(GetP("P_LI_28P_24S_BAT"),mw007312_def._Int_LI_28P_24S_BAT()), new HashMap(GetP("P_LI_28P_24S_4AH"),mw007312_def._Int_LI_28P_24S_4AH()), new HashMap(GetP("P_LI_28X_AC"),mw007312_def._Int_LI_28X_AC()), new HashMap(GetP("P_LI_28X_DC"),mw007312_def._Int_LI_28X_DC()), new HashMap(GetP("P_LI_28X_24S_AC"),mw007312_def._Int_LI_28X_24S_AC()), new HashMap(GetP("P_LI_28X_24S_DC"),mw007312_def._Int_LI_28X_24S_DC()), new HashMap(GetP("P_LI_28X_AC_5701"),mw007312_def._Int_LI_28X_AC_5701()), new HashMap(GetP("P_LI_28X_24S_AC_5701"),mw007312_def._Int_LI_28X_24S_AC_01()), new HashMap(GetP("P_LI_52X_AC"),mw007312_def._Int_LI_52X_AC()), new HashMap(GetP("P_LI_52X_DC"),mw007312_def._Int_LI_52X_DC()), new HashMap(GetP("P_LI_52X_48CS_AC"),mw007312_def._Int_LI_52X_48CS_AC()), new HashMap(GetP("P_LI_28X_PWR_AC"),mw007312_def._Int_LI_28X_PWR_AC()), new HashMap(GetP("P_LI_52X_PWR_AC"),mw007312_def._Int_LI_52X_PWR_AC()), new HashMap(GetP("P_SI_24TP_AC"),mw007312_def._Int_SI_24TP_AC()), new HashMap(GetP("P_SI_24TP_DC"),mw007312_def._Int_SI_24TP_DC()), new HashMap(GetP("P_SI_26X_12S_AC"),mw007312_def._Int_SI_26X_12S_AC()), new HashMap(GetP("P_SI_48TP_AC"),mw007312_def._Int_SI_48TP_AC()), new HashMap(GetP("P_SI_48TP_DC"),mw007312_def._Int_SI_48TP_DC()), new HashMap(GetP("P_SI_24TP_PWR"),mw007312_def._Int_SI_24TP_PWR()), new HashMap(GetP("P_SI_24TP_PWR_AC"),mw007312_def._Int_SI_24TP_PWR_AC()), new HashMap(GetP("P_SI_48TP_PWR"),mw007312_def._Int_SI_48TP_PWR()), new HashMap(GetP("P_SI_48TP_PWR_AC"),mw007312_def._Int_SI_48TP_PWR_AC()), new HashMap(GetP("P_SI_28C"),mw007312_def._Int_SI_28C()), new HashMap(GetP("P_SI_28C_AC"),mw007312_def._Int_SI_28C_AC()), new HashMap(GetP("P_SI_52C"),mw007312_def._Int_SI_52C()), new HashMap(GetP("P_SI_52C_AC"),mw007312_def._Int_SI_52C_AC()), new HashMap(GetP("P_SI_28C_PWR"),mw007312_def._Int_SI_28C_PWR()), new HashMap(GetP("P_SI_52C_PWR"),mw007312_def._Int_SI_52C_PWR()), new HashMap(GetP("P_EI_28C"),mw007312_def._Int_EI_28C()), new HashMap(GetP("P_EI_28C_AC"),mw007312_def._Int_EI_28C_AC()), new HashMap(GetP("P_EI_28C_24S"),mw007312_def._Int_EI_28C_24S()), new HashMap(GetP("P_EI_28C_24S_AC"),mw007312_def._Int_EI_28C_24S_AC()), new HashMap(GetP("P_EI_52C"),mw007312_def._Int_EI_52C()), new HashMap(GetP("P_EI_52C_AC"),mw007312_def._Int_EI_52C_AC()), new HashMap(GetP("P_EI_28C_PWR"),mw007312_def._Int_EI_28C_PWR()), new HashMap(GetP("P_EI_28C_PWR_AC"),mw007312_def._Int_EI_28C_PWR_AC()), new HashMap(GetP("P_EI_52C_PWR"),mw007312_def._Int_EI_52C_PWR()), new HashMap(GetP("P_EI_52C_PWR_AC"),mw007312_def._Int_EI_52C_PWR_AC()), new HashMap(GetP("P_EI_28C_5710"),mw007312_def._Int_EI_28C_5710()), new HashMap(GetP("P_EI_28C_AC_S5710"),mw007312_def._Int_EI_28C_AC_S5710()), new HashMap(GetP("P_EI_52C_5710"),mw007312_def._Int_EI_52C_5710()), new HashMap(GetP("P_EI_52C_AC_S5710"),mw007312_def._Int_EI_52C_AC_S5710()), new HashMap(GetP("P_EI_28C_PWR_AC_5710"),mw007312_def._Int_EI_28C_PWR_AC_10()), new HashMap(GetP("P_EI_52C_PWR_5710"),mw007312_def._Int_EI_52C_PWR_5710()), new HashMap(GetP("P_EI_52C_PWR_AC_5710"),mw007312_def._Int_EI_52C_PWR_AC_10()), new HashMap(GetP("P_HI_28C"),mw007312_def._Int_HI_28C()), new HashMap(GetP("P_HI_28C_AC"),mw007312_def._Int_HI_28C_AC()), new HashMap(GetP("P_HI_28C_24S"),mw007312_def._Int_HI_28C_24S()), new HashMap(GetP("P_HI_28C_24S_AC"),mw007312_def._Int_HI_28C_24S_AC()), new HashMap(GetP("P_HI_108C_PWR_5710"),mw007312_def._Int_HI_108C_PWR_5710()), new HashMap(GetP("P_HI_108C_PWR_10_B1"),mw007312_def._Int_HI_108C_PWR_5710_B1()), new HashMap(GetP("P_LI_28TP_AC"),mw007312_def._Int_LI_28TP_AC()), new HashMap(GetP("P_LI_28TP_PWR_AC"),mw007312_def._Int_LI_28TP_PWR_AC()), new HashMap(GetP("P_LI_28TP_PWR_AC_01"),mw007312_def._Int_LI_28TP_PWR_AC_01()), new HashMap(GetP("P_HI_32C_24S_AC_5720"),mw007312_def._Int_HI_32C_24S_AC_5720()), new HashMap(GetP("P_HI_56C_AC_5720"),mw007312_def._Int_HI_56C_AC_5720()), new HashMap(GetP("P_HI_56C_PWR_AC_5720"),mw007312_def._Int_HI_56C_PWR_AC_5720()), new HashMap(GetP("P_HI_56C_PWR_AC1_20"),mw007312_def._int_HI_56C_PWR_AC1_20()), new HashMap(GetP("P_5720EI_36C_28S_AC"),mw007312_def._Int_5720EI_36C_28S_AC()), new HashMap(GetP("P_5720EI_56C_48S_AC"),mw007312_def._Int_5720EI_56C_48S_AC()), new HashMap(GetP("P_5720EI_36C_AC"),mw007312_def._Int_5720EI_36C_AC()), new HashMap(GetP("P_5720EI_36PC_AC"),mw007312_def._Int_5720EI_36PC_AC()), new HashMap(GetP("P_5720EI_56C_AC"),mw007312_def._Int_5720EI_56C_AC()), new HashMap(GetP("P_5720EI_56PC_AC"),mw007312_def._Int_5720EI_56PC_AC()), new HashMap(GetP("P_5720EI_36C_PWR_AC"),mw007312_def._Int_5720EI_36C_PWR_AC()), new HashMap(GetP("P_5720EI_56C_PWR_AC"),mw007312_def._Int_5720EI_56C_PWR_AC()), new HashMap(GetP("P_5720EI_56C_PWR_AC1"),mw007312_def._Int_5720EI_56C_PWR_AC1()), new HashMap(GetP("P_5720EI_32X_24S_AC"),mw007312_def._Int_5720EI_32X_24S_AC()), new HashMap(GetP("P_5720EI_50X_46S_AC"),mw007312_def._Int_5720EI_50X_46S_AC()), new HashMap(GetP("P_5720EI_32X_AC"),mw007312_def._Int_5720EI_32X_AC()), new HashMap(GetP("P_5720EI_32P_AC"),mw007312_def._Int_5720EI_32P_AC()), new HashMap(GetP("P_5720EI_50X_AC"),mw007312_def._Int_5720EI_50X_AC()), new HashMap(GetP("P_5720EI_52X_AC"),mw007312_def._Int_5720EI_52X_AC()), new HashMap(GetP("P_5720EI_52P_AC"),mw007312_def._Int_5720EI_52P_AC()), new HashMap(GetP("P_00S_28X_LI_AC"),mw007312_def._int_00S_28X_LI_AC()), new HashMap(GetP("P_00S_52X_LI_AC"),mw007312_def._int_00S_52X_LI_AC()), new HashMap(GetP("P_00S_28P_PWR_LI_AC"),mw007312_def._int_00S_28P_PWR_LI_AC()), new HashMap(GetP("P_10_28X_LI_AC"),mw007312_def._int_10_28X_LI_AC()), new HashMap(GetP("P_10_52X_LI_AC"),mw007312_def._int_10_52X_LI_AC()), new HashMap(GetP("P_20S_28P_SI_AC"),mw007312_def._int_20S_28P_SI_AC()), new HashMap(GetP("P_20S_28X_SI_AC"),mw007312_def._int_20S_28X_SI_AC()), new HashMap(GetP("P_20S_52P_SI_AC"),mw007312_def._int_20S_52P_SI_AC()), new HashMap(GetP("P_20S_52X_SI_AC"),mw007312_def._int_20S_52X_SI_AC()), new HashMap(GetP("P_20_28P_SI_AC"),mw007312_def._int_20_28P_SI_AC()), new HashMap(GetP("P_20_28X_SI_AC"),mw007312_def._int_20_28X_SI_AC()), new HashMap(GetP("P_20_52P_SI_AC"),mw007312_def._int_20_52P_SI_AC()), new HashMap(GetP("P_20_52X_SI_AC"),mw007312_def._int_20_52X_SI_AC()), new HashMap(GetP("P_20_28X_PWR_SI_AC"),mw007312_def._int_20_28X_PWR_SI_AC()), new HashMap(GetP("P_20_52X_PWR_SI_AC"),mw007312_def._int_20_52X_PWR_SI_AC()), new HashMap(GetP("P_20_52X_PWR_SI_ACF"),mw007312_def._int_20_52X_PWR_SI_ACF()), new HashMap(GetP("P_20_28X_PWR_SI_DC"),mw007312_def._int_20_28X_PWR_SI_DC()), new HashMap(GetP("P_20_52X_PWR_SI_DC"),mw007312_def._int_20_52X_PWR_SI_DC()), new HashMap(GetP("P_20_28X_SI_DC"),mw007312_def._int_20_28X_SI_DC()), new HashMap(GetP("P_20_52X_SI_DC"),mw007312_def._int_20_52X_SI_DC()), new HashMap(GetP("P_20S_28X_SI_DC"),mw007312_def._int_20S_28X_SI_DC()), new HashMap(GetP("P_20S_52X_SI_DC"),mw007312_def._int_20S_52X_SI_DC()), new HashMap(GetP("P_5720EI_32X_DC"),mw007312_def._int_5720EI_32X_DC()), new HashMap(GetP("P_5720EI_50X_DC"),mw007312_def._int_5720EI_50X_DC()), new HashMap(GetP("P_5720EI_32X_24S_DC"),mw007312_def._int_5720EI_32X_24S_DC()), new HashMap(GetP("P_5720EI_50X_46S_DC"),mw007312_def._int_5720EI_50X_46S_DC()), new HashMap(GetP("P_5720EI_36C_DC"),mw007312_def._int_5720EI_36C_DC()), new HashMap(GetP("P_5720EI_56C_DC"),mw007312_def._int_5720EI_56C_DC()), new HashMap(GetP("P_5720EI_36C_PWR_DC"),mw007312_def._int_5720EI_36C_PWR_DC()), new HashMap(GetP("P_5720EI_56C_PWR_DC"),mw007312_def._int_5720EI_56C_PWR_DC()), new HashMap(GetP("P_5720EI_36C_28S_DC"),mw007312_def._int_5720EI_36C_28S_DC()), new HashMap(GetP("P_5720EI_56C_48S_DC"),mw007312_def._int_5720EI_56C_48S_DC()), new HashMap(GetP("P_00_8P_G"),mw007312_def._int_5700_8P_G()), new HashMap(GetP("P_20_14X_SI_AC"),mw007312_def._int_S5720_14X_PWH_SI_AC()), new HashMap(GetP("P_12TP_AC_LI20"),mw007312_def._int_12TP_AC_LI20()), new HashMap(GetP("P_12TP_AC_LI20S"),mw007312_def._int_12TP_AC_LI20S()), new HashMap(GetP("P_12TP_PWRAC_LI20"),mw007312_def._int_12TP_PWRAC_LI20()), new HashMap(GetP("P_12TP_PWRAC_LI20S"),mw007312_def._int_12TP_PWRAC_LI20S()), new HashMap(GetP("P_28P_AC_LI20"),mw007312_def._int_28P_AC_LI20()), new HashMap(GetP("P_28P_AC_LI20_SEA"),mw007312_def._int_28P_AC_LI20()), new HashMap(GetP("P_28P_AC_LI20S"),mw007312_def._int_28P_AC_LI20S()), new HashMap(GetP("P_28X_AC_LI20"),mw007312_def._int_28X_AC_LI20()), new HashMap(GetP("P_28X_AC_LI20S"),mw007312_def._int_28X_AC_LI20S()), new HashMap(GetP("P_28X_DC_LI20"),mw007312_def._int_28X_DC_LI20()), new HashMap(GetP("P_28P_PWRAC_LI20"),mw007312_def._int_28P_PWRAC_LI20()), new HashMap(GetP("P_28P_PWRAC_LI20_SEA"),mw007312_def._int_28P_PWRAC_LI20()), new HashMap(GetP("P_28P_PWRAC_LI20S"),mw007312_def._int_28P_PWRAC_LI20S()), new HashMap(GetP("P_28X_PWRAC_LI20"),mw007312_def._int_28X_PWRAC_LI20()), new HashMap(GetP("P_28X_PWRAC_LI20S"),mw007312_def._int_28X_PWRAC_LI20S()), new HashMap(GetP("P_52P_AC_LI20"),mw007312_def._int_52P_AC_LI20()), new HashMap(GetP("P_52P_AC_LI20_SEA"),mw007312_def._int_52P_AC_LI20()), new HashMap(GetP("P_52P_AC_LI20S"),mw007312_def._int_52P_AC_LI20S()), new HashMap(GetP("P_52X_AC_LI20"),mw007312_def._int_52X_AC_LI20()), new HashMap(GetP("P_52X_AC_LI20S"),mw007312_def._int_52X_AC_LI20S()), new HashMap(GetP("P_52X_DC_LI20"),mw007312_def._int_52X_DC_LI20()), new HashMap(GetP("P_52P_PWRAC_LI20"),mw007312_def._int_52P_PWRAC_LI20()), new HashMap(GetP("P_52P_PWRAC_LI20_SEA"),mw007312_def._int_52P_PWRAC_LI20()), new HashMap(GetP("P_52P_PWRAC_LI20S"),mw007312_def._int_52P_PWRAC_LI20S()), new HashMap(GetP("P_52X_PWRAC_LI20"),mw007312_def._int_52X_PWRAC_LI20()), new HashMap(GetP("P_52X_PWRAC_LI20S"),mw007312_def._int_52X_PWRAC_LI20S()), new HashMap(GetP("P_28X_24S_AC_LI20"),mw007312_def._int_28X_24S_AC_LI20()), new HashMap(GetP("P_28X_24S_AC_LI20S"),mw007312_def._int_28X_24S_AC_LI20S()), new HashMap(GetP("P_28X_24S_DC_LI20"),mw007312_def._int_28X_24S_DC_LI20()), new HashMap(GetP("P_28TP_PWRACL_LI20"),mw007312_def._int_28TP_PWRACL_LI20()), new HashMap(GetP("P_28TP_PWRACL_LI20S"),mw007312_def._int_28TP_PWRACL_LI20S()), new HashMap(GetP("P_28TP_PWRAC_LI20"),mw007312_def._int_28TP_PWRAC_LI20()), new HashMap(GetP("P_28TP_AC_LI20"),mw007312_def._int_28TP_AC_LI20()), new HashMap(GetP("P_26X_PWHAC_LI20"),mw007312_def._int_26X_PWHAC_LI20()), new HashMap(GetP("P_28X_PWHAC_LI20"),mw007312_def._int_28X_PWHAC_LI20()), new HashMap(GetP("P_28X_24S_AC_SI20"),mw007312_def._int_28X_24S_AC_SI20()), new HashMap(GetP("P_28X_24S_DC_SI20"),mw007312_def._int_28X_24S_DC_SI20()), new HashMap(GetP("P_28X24S_AC_SI21"),mw007312_def._Int_28X24S_AC_SI21()), new HashMap(GetP("P_52X_PWRACF_LI20"),mw007312_def._Int_52X_PWRACF_LI20()), new HashMap(GetP("P_48C_AC_SI30"),mw007312_def._Int_48C_AC_SI30()), new HashMap(GetP("P_48C_PWR_AC_SI30"),mw007312_def._Int_48C_PWR_AC_SI30()), new HashMap(GetP("P_68C_AC_SI30"),mw007312_def._Int_68C_AC_SI30()), new HashMap(GetP("P_68C_PWR_AC_SI30"),mw007312_def._Int_68C_PWR_AC_SI30()), new HashMap(GetP("P_68C_PWR_SI30"),mw007312_def._Int_68C_PWR_SI30()), new HashMap(GetP("P_48C_AC_EI30S"),mw007312_def._Int_48C_AC_EI30S()), new HashMap(GetP("P_48C_PWR_EI30S"),mw007312_def._Int_48C_PWR_EI30S()), new HashMap(GetP("P_68C_AC_EI30S"),mw007312_def._Int_68C_AC_EI30S()), new HashMap(GetP("P_68C_PWR_EI30S"),mw007312_def._Int_68C_PWR_EI30S()), new HashMap(GetP("P_20I_12X_SI_AC"),mw007312_def._Int_20I_12X_SI_AC()), new HashMap(GetP("P_20I_12X_PWH_SI_DC"),mw007312_def._Int_20I_12X_PWH_SI_DC()), new HashMap(GetP("P_20I_28X_SI_AC"),mw007312_def._Int_20I_28X_SI_AC()), new HashMap(GetP("P_20I_28X_PWH_SI_AC"),mw007312_def._Int_20I_28X_PWH_SI_AC()), new HashMap(GetP("P_30_36C_HI"),mw007312_def._Int_30_36C_HI()), new HashMap(GetP("P_30_44C_HI"),mw007312_def._Int_30_44C_HI()), new HashMap(GetP("P_30_36C_PWH_HI"),mw007312_def._Int_30_36C_PWH_HI()), new HashMap(GetP("P_30_44C_PWH_HI"),mw007312_def._Int_30_44C_PWH_HI()), new HashMap(GetP("P_30_60C_HI"),mw007312_def._Int_30_60C_HI()), new HashMap(GetP("P_30_68C_HI"),mw007312_def._Int_30_68C_HI()), new HashMap(GetP("P_30_60C_PWH_HI"),mw007312_def._Int_30_60C_PWH_HI()), new HashMap(GetP("P_30_68C_PWH_HI"),mw007312_def._Int_30_68C_PWH_HI()), new HashMap(GetP("P_20SV2_28P_LIAC"),mw007312_def._Int_20SV2_28P_LIAC()), new HashMap(GetP("P_20SV2_52P_LIAC"),mw007312_def._Int_20SV2_52P_LIAC()), new HashMap(GetP("P_20I_10XPWH_SI_AC"),mw007312_def._Int_20I_10XPWH_SI_AC()), new HashMap(GetP("P_20I_10X_SI_AC_SEA"),mw007312_def._Int_20I_10XPWH_SI_AC()), new HashMap(GetP("P_20I_6XPWH_SI_AC"),mw007312_def._Int_20I_6XPWH_SI_AC()), new HashMap(GetP("P_30_60C_HI_48S"),mw007312_def._Int_30_60C_HI_48S()), new HashMap(GetP("P_30_68C_HI_48S"),mw007312_def._Int_30_68C_HI_48S()), new HashMap(GetP("P_30_36C_HI_24S"),mw007312_def._Int_30_36C_HI_24S()), new HashMap(GetP("P_30_44C_HI_24S"),mw007312_def._Int_30_44C_HI_24S()), new HashMap(GetP("P_20_52X_LI_48S_AC"),mw007312_def._Int_20_52X_LI_48S_AC()), new HashMap(GetP("P_20_52X_SI_48S"),mw007312_def._Int_20_52X_SI_48S()), new HashMap(GetP("P_20_28XPWR_LI_ACF"),mw007312_def._Int_20_28XPWR_LI_ACF())];
