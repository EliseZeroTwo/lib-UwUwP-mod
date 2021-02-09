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
