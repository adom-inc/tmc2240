use bitfield::bitfield;
use derive_more::{From, Into};

bitfield! {
    /// IHOLD_IRUN
    #[derive(Clone, Copy, PartialEq, Eq, From, Into)]
    pub struct HoldRunCurrentRegister(u32);
    impl Debug;
    u8;

    pub irundelay, set_irundelay: 27, 24;
    pub iholddelay, set_iholddelay: 19, 16;
    pub irun, set_irun: 12, 8;
    pub ihold, set_ihold: 4, 0;
}

bitfield! {
    /// TPOWERDOWN
    #[derive(Clone, Copy, PartialEq, Eq, From, Into)]
    pub struct PowerDownTimeRegister(u32);
    impl Debug;
    u8;

    pub tpowerdown, set_tpowerdown: 7, 0;
}

bitfield! {
    /// TSTEP
    #[derive(Clone, Copy, PartialEq, Eq, From, Into)]
    pub struct MicrostepTimeRegister(u32);
    impl Debug;
    u32;

    pub tstep, _: 19, 0;
}

bitfield! {
    /// TPWMTHRS
    #[derive(Clone, Copy, PartialEq, Eq, From, Into)]
    pub struct StealthChop2ThresholdTimeRegister(u32);
    impl Debug;
    u32;

    pub tpwmthrs, set_tpwmthrs: 19, 0;
}

bitfield! {
    /// TCOOLTHRS
    #[derive(Clone, Copy, PartialEq, Eq, From, Into)]
    pub struct CoolStepThresholdTimeRegister(u32);
    impl Debug;
    u32;

    pub tcoolthrs, set_tcoolthrs: 19, 0;
}

bitfield! {
    /// THIGH
    #[derive(Clone, Copy, PartialEq, Eq, From, Into)]
    pub struct FullStepThresholdTimeRegister(u32);
    impl Debug;
    u32;

    pub thigh, set_thigh: 19, 0;
}
