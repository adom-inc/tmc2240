use bitfield::bitfield;
use derive_more::{From, Into};

bitfield! {
    /// DIRECT_MODE
    #[derive(Clone, Copy, PartialEq, Eq, From, Into)]
    pub struct DirectModeRegister(u32);
    impl Debug;
    u16;

    pub direct_coil_b, _: 24, 16;
    pub direct_coil_a, _: 8, 0;
}
