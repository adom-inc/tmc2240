use bitfield::bitfield;
use derive_more::{From, Into};

bitfield! {
    /// MSLUT_0
    #[derive(Clone, Copy, PartialEq, Eq, From, Into)]
    pub struct MicrostepLookupTable0Register(u32);
    impl Debug;
    u32;

    pub mslut_0, set_mslut_0: 31, 0;
}

bitfield! {
    /// MSLUT_1
    #[derive(Clone, Copy, PartialEq, Eq, From, Into)]
    pub struct MicrostepLookupTable1Register(u32);
    impl Debug;
    u32;

    pub mslut_1, set_mslut_1: 31, 0;
}

bitfield! {
    /// MSLUT_2
    #[derive(Clone, Copy, PartialEq, Eq, From, Into)]
    pub struct MicrostepLookupTable2Register(u32);
    impl Debug;
    u32;

    pub mslut_2, set_mslut_2: 31, 0;
}

bitfield! {
    /// MSLUT_3
    #[derive(Clone, Copy, PartialEq, Eq, From, Into)]
    pub struct MicrostepLookupTable3Register(u32);
    impl Debug;
    u32;

    pub mslut_3, set_mslut_3: 31, 0;
}

bitfield! {
    /// MSLUT_4
    #[derive(Clone, Copy, PartialEq, Eq, From, Into)]
    pub struct MicrostepLookupTable4Register(u32);
    impl Debug;
    u32;

    pub mslut_4, set_mslut_4: 31, 0;
}

bitfield! {
    /// MSLUT_5
    #[derive(Clone, Copy, PartialEq, Eq, From, Into)]
    pub struct MicrostepLookupTable5Register(u32);
    impl Debug;
    u32;

    pub mslut_5, set_mslut_5: 31, 0;
}

bitfield! {
    /// MSLUT_6
    #[derive(Clone, Copy, PartialEq, Eq, From, Into)]
    pub struct MicrostepLookupTable6Register(u32);
    impl Debug;
    u32;

    pub mslut_6, set_mslut_6: 31, 0;
}

bitfield! {
    /// MSLUT_7
    #[derive(Clone, Copy, PartialEq, Eq, From, Into)]
    pub struct MicrostepLookupTable7Register(u32);
    impl Debug;
    u32;

    pub mslut_7, set_mslut_7: 31, 0;
}

bitfield! {
    /// MSLUTSEL
    #[derive(Clone, Copy, PartialEq, Eq, From, Into)]
    pub struct MicrostepLookupTableSelectionRegister(u32);
    impl Debug;
    u8;

    pub x3, set_x3: 31, 24;
    pub x2, set_x2: 23, 16;
    pub x1, set_x1: 15, 8;
    pub w3, set_w3: 7, 6;
    pub w2, set_w2: 5, 4;
    pub w1, set_w1: 3, 2;
    pub w0, set_w0: 1, 0;
}

bitfield! {
    /// MSLUTSTART
    #[derive(Clone, Copy, PartialEq, Eq, From, Into)]
    pub struct MicrostepLookupTableStartRegister(u32);
    impl Debug;
    u8;

    pub offset_sin90, set_offset_sin90: 31, 24;
    pub start_sin90, set_start_sin90: 23, 16;
    pub start_sin, set_start_sin: 7, 0;
}

bitfield! {
    /// MSCNT
    #[derive(Clone, Copy, PartialEq, Eq, From, Into)]
    pub struct MicrostepCountRegister(u32);
    impl Debug;
    u16;

    pub mscnt, _: 9, 0;
}

bitfield! {
    /// MSCURACT
    #[derive(Clone, Copy, PartialEq, Eq, From, Into)]
    pub struct MicrostepCurrentRegister(u32);
    impl Debug;
    u16;

    pub cur_a, _: 24, 16;
    pub cur_b, _: 8, 0;
}

bitfield! {
    /// CHOPCONF
    #[derive(Clone, Copy, PartialEq, Eq, From, Into)]
    pub struct ChopConfigurationRegister(u32);
    impl Debug;
    u8;

    pub diss2vs, set_diss2vs: 31;
    pub diss2g, set_diss2g: 30;
    pub dedge, set_dedge: 29;
    pub intpol, set_intpol: 28;
    pub mres, set_mres: 27, 24;
    pub tpfd, set_tpfd: 23, 20;
    pub vhighchm, set_vhighchm: 19;
    pub vhighfs, set_vhighfs: 18;
    pub tbl, set_tbl: 16, 15;
    pub chm, set_chm: 14;
    pub disfdcc, set_disfdcc: 12;
    pub fd3, set_fd3: 11;
    pub hend_offset, set_hend_offset: 10, 7;
    pub hstrt_tfd210, set_hstrt_tfd210: 6, 4;
    pub toff, set_toff: 3, 0;
}

bitfield! {
    /// COOlCONF
    #[derive(Clone, Copy, PartialEq, Eq, From, Into)]
    pub struct CoolStepConfigurationRegister(u32);
    impl Debug;
    u8;

    pub sfilt, set_sfilt: 24;
    pub sgt, set_sgt: 22, 16;
    pub seimin, set_seimin: 15;
    pub sedn, set_sedn: 14, 13;
    pub semax, set_semax: 11, 8;
    pub seup, set_seup: 6, 5;
    pub semin, set_semin: 3, 0;
}

bitfield! {
    /// DRV_STATUS
    #[derive(Clone, Copy, PartialEq, Eq, From, Into)]
    pub struct DriverStatusRegister(u32);
    impl Debug;

    pub stst, _: 31;
    pub olb, _: 30;
    pub ola, _: 29;
    pub s2gb, _: 28;
    pub s2ga, _: 27;
    pub otpw, _: 26;
    pub ot, _: 25;
    pub stallguard, _: 24;

    u8;
    pub cs_actual, _: 20, 16;
    pub fsactive, _: 15;
    pub stealth, _: 14;
    pub s2vsb, _: 13;
    pub s2vsa, _: 12;

    u16;
    pub sg_result, _: 9, 0;
}

bitfield! {
    /// PWMCONF
    #[derive(Clone, Copy, PartialEq, Eq, From, Into)]
    pub struct PwmConfigurationRegister(u32);
    impl Debug;
    u8;

    pub pwm_lim, set_pwm_lim: 31, 28;
    pub pwm_reg, set_pwm_reg: 27, 24;
    pub pwm_dis_reg_stst, set_pwm_dis_reg_stst: 23;
    pub pwm_meas_sd_enable, set_pwm_meas_sd_enable: 22;
    pub freewheel, set_freewheel: 21, 20;
    pub pwm_autograd, set_pwm_autograd: 19;
    pub pwm_autoscale, set_pwm_autoscale: 18;
    pub pwm_freq, set_pwm_freq: 17, 16;
    pub pwm_grad, set_pwm_grad: 15, 8;
    pub pwm_ofs, set_pwm_ofs: 7, 0;
}

bitfield! {
    /// PWM_SCALE
    #[derive(Clone, Copy, PartialEq, Eq, From, Into)]
    pub struct PwmScaleRegister(u32);
    impl Debug;
    u16;

    pub pwm_scale_auto, _: 24, 16;
    pub pwm_scale_sum, _: 9, 0;
}

bitfield! {
    /// PWM_AUTO
    #[derive(Clone, Copy, PartialEq, Eq, From, Into)]
    pub struct PwmAutoRegister(u32);
    impl Debug;
    u8;

    pub pwm_grad_auto, _: 23, 16;
    pub pwm_ofs_auto, _: 7, 0;
}

bitfield! {
    /// SG4_THRS
    #[derive(Clone, Copy, PartialEq, Eq, From, Into)]
    pub struct StallGuard4ThresholdRegister(u32);
    impl Debug;
    u8;

    pub sg_angle_offset, set_sg_angle_offset: 9;
    pub sg4_filt_en, set_sg4_filt_en: 8;
    pub sg4_thrs, set_sg4_thrs: 7, 0;
}

bitfield! {
    /// SG4_RESULT
    #[derive(Clone, Copy, PartialEq, Eq, From, Into)]
    pub struct StallGuard4ResultRegister(u32);
    impl Debug;
    u16;

    pub sg4_result, _: 9, 0;
}

bitfield! {
    /// SG4_IND
    #[derive(Clone, Copy, PartialEq, Eq, From, Into)]
    pub struct StallGuard4IndividualValueRegister(u32);
    impl Debug;
    u8;

    pub sg4_ind_3, _: 31, 24;
    pub sg4_ind_2, _: 23, 16;
    pub sg4_ind_1, _: 15, 8;
    pub sg4_ind_0, _: 7, 0;
}

