use std::ffi::{CStr, c_char};

use crate::{define_obj_type, ns};

define_obj_type!(
    #[doc(alias = "nw_interface")]
    #[doc(alias = "nw_interface_t")]
    pub Iface(ns::Id)
);

#[doc(alias = "nw_interface_radio_type_t")]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(i32)]
pub enum IfaceType {
    /// A virtual or otherwise unknown interface type
    Other = 0,

    /// A Wi-Fi link
    Wifi = 1,

    /// A Cellular link
    Cellular = 2,

    ///  Wired Ethernet link
    Wired = 3,

    /// The loopback interface
    Loopback = 4,
}

impl Iface {
    #[doc(alias = "nw_interface_get_type")]
    #[inline]
    pub fn type_(&self) -> IfaceType {
        unsafe { nw_interface_get_type(self) }
    }

    #[doc(alias = "nw_interface_get_name")]
    #[inline]
    pub fn name(&self) -> Option<&CStr> {
        unsafe {
            let ptr = nw_interface_get_name(self);
            if ptr.is_null() {
                None
            } else {
                Some(CStr::from_ptr(ptr))
            }
        }
    }

    #[doc(alias = "nw_interface_get_index")]
    #[inline]
    pub fn idx(&self) -> u32 {
        unsafe { nw_interface_get_index(self) }
    }
}

#[doc(alias = "nw_interface_radio_type")]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(i32)]
pub enum IfaceRadioType {
    #[doc(alias = "nw_interface_radio_type_unknown")]
    Unknown = 0,

    #[doc(alias = "nw_interface_radio_type_wifi_b")]
    WifiB = 1,

    #[doc(alias = "nw_interface_radio_type_wifi_a")]
    WifiA = 2,

    #[doc(alias = "nw_interface_radio_type_wifi_g")]
    WifiG = 3,

    #[doc(alias = "nw_interface_radio_type_wifi_n")]
    WifiN = 4,

    #[doc(alias = "nw_interface_radio_type_wifi_ac")]
    WifiAc = 5,

    #[doc(alias = "nw_interface_radio_type_wifi_ax")]
    WifiAx = 6,

    /// 4G LTE
    #[doc(alias = "nw_interface_radio_type_cell_lte")]
    CellLte = 0x80,

    /// 5G Dual LTE & New Radio Sub6
    #[doc(alias = "nw_interface_radio_type_cell_endc_sub6")]
    CellEndcSub6 = 0x81,

    /// 5G Dual LTE & New Radio mmWave
    #[doc(alias = "nw_interface_radio_type_cell_endc_mmw")]
    CellEndcMmw = 0x82,

    /// 5G Stand Alone New Radio Sub6
    #[doc(alias = "nw_interface_radio_type_cell_nr_sa_sub6")]
    CellNrSaSub6 = 0x83,

    /// 5G Stand Alone New Radio mmWave
    #[doc(alias = "nw_interface_radio_type_cell_nr_sa_mmw")]
    CellNrSaMmw = 0x84,

    /// 3G WCDMA
    #[doc(alias = "nw_interface_radio_type_cell_wcdma")]
    CellWcdma = 0x85,

    /// 2G GSM
    #[doc(alias = "nw_interface_radio_type_cell_gsm")]
    CellGsm = 0x86,

    /// 1x data
    #[doc(alias = "nw_interface_radio_type_cell_cdma")]
    CellCdma = 0x87,

    /// HDR data
    #[doc(alias = "nw_interface_radio_type_cell_evdo")]
    CellEvdo = 0x88,
}

#[link(name = "Network", kind = "framework")]
unsafe extern "C-unwind" {
    fn nw_interface_get_type(iface: &Iface) -> IfaceType;
    fn nw_interface_get_name(iface: &Iface) -> *const c_char;
    fn nw_interface_get_index(iface: &Iface) -> u32;
}
