#[derive(Copy, Clone, Debug)]
pub enum DllReason {
    DllProcessDetach,
    DllProcessAttach,
    DllThreadAttach,
    DllThreadDetach,
}

impl From<u32> for DllReason {
    fn from(value: u32) -> Self {
        match value {
            0 => DllReason::DllProcessDetach,
            1 => DllReason::DllProcessAttach,
            2 => DllReason::DllThreadAttach,
            3 => DllReason::DllThreadDetach,
            _ => panic!("Unable to cast {} to DllReason", value),
        }
    }
}

impl From<DllReason> for u32 {
    fn from(value: DllReason) -> Self {
        match value {
            DllReason::DllProcessDetach => 0,
            DllReason::DllProcessAttach => 1,
            DllReason::DllThreadAttach => 2,
            DllReason::DllThreadDetach => 3,
        }
    }
}

#[macro_export]
macro_rules! dll_main {
    ($($arms:tt)*) => {
        #[no_mangle]
        pub unsafe extern "C" fn DllMain(_module: *mut c_void, rfc: u32, _lp_res: *mut c_void) -> bool {
            let reason = DllReason::from(rfc);
            match reason $($arms)*
        }
    };
}
