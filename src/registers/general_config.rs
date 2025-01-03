use bitfield::bitfield;
use derive_more::derive::{From, Into};

bitfield! {
    /// GCONF
    #[derive(Clone, Copy, PartialEq, Eq, From, Into)]
    pub struct GeneralConfigurationRegister(u32);
    impl Debug;
    u8;
    pub direct_mode, set_direct_mode: 16;
    pub stop_enable, set_stop_enable: 15;
    pub small_hysteresis, set_small_hysteresis: 14;
    pub diag1_pushpull, set_diag1_pushpull: 13;
    pub diag0_pushpull, set_diag0_pushpull: 12;

    pub diag1_onstate, set_diag1_onstate: 10;
    pub diag1_index, set_diag1_index: 9;
    pub diag1_stall, set_diag1_stall: 8;
    pub diag0_stall, set_diag0_stall: 7;
    pub diag0_optw, set_diag0_optw: 6;
    pub diag0_err, set_diag0_err: 5;
    pub shaft, set_shaft: 4;
    pub multistep_filt, set_multistep_filt: 3;
    pub en_pwm_mode, set_en_pwm_mode: 2;
    pub fast_standstill, set_fast_standstill: 1;
}

bitfield! {
    /// GSTAT
    #[derive(Clone, Copy, PartialEq, Eq, From, Into)]
    pub struct GeneralStatusRegister(u32);
    impl Debug;
    u8;

    // TODO: These are Write 1 to clear

    pub vm_uvlo, set_vm_uvlo: 4;
    pub register_reset, set_register_reset: 3;
    pub uv_cp, set_uv_cp: 2;
    pub drv_error, set_drv_error: 1;
    pub reset, set_reset: 0;
}

bitfield! {
    /// IFCNT
    #[derive(Clone, Copy, PartialEq, Eq, From, Into)]
    pub struct InterfaceTransmissionCounterRegister(u32);
    impl Debug;
    u8;

    pub ifcnt, _: 7, 0;
}

bitfield! {
    /// NODECONF
    #[derive(Clone, Copy, PartialEq, Eq, From, Into)]
    pub struct NodeConfigurationRegister(u32);
    impl Debug;
    u8;

    pub senddelay, set_senddelay: 11, 8;
    pub nodeaddr, set_nodeaddr: 7, 0;
}

bitfield! {
    /// IOIN
    #[derive(Clone, Copy, PartialEq, Eq, From, Into)]
    pub struct IoInputRegister(u32);
    impl Debug;
    u8;

    pub version, _: 31, 24;
    pub silicon_rv, _: 18, 16;

    pub adc_err, _: 15;
    pub ext_clk, _: 14;
    pub ext_res_det, _: 13;
    pub output, set_output: 12;
    pub comp_b1_b2, _: 11;
    pub comp_a1_a2, _: 10;
    pub comp_b, _: 9;
    pub comp_a, _: 8;

    pub uart_en, _: 6;
    pub encn, _: 5;
    pub drv_enn, _: 4;
    pub enca, _: 3;
    pub encb, _: 2;
    pub dir, _: 1;
    pub step, _: 0;
}

bitfield! {
    /// DRV_CONF
    #[derive(Clone, Copy, PartialEq, Eq, From, Into)]
    pub struct DriverConfigurationRegister(u32);
    impl Debug;
    u8;

    pub slope_control, set_slope_control: 5, 4;
    pub current_range, set_current_range: 1, 0;
}

bitfield! {
    /// GLOBAL_SCALER
    #[derive(Clone, Copy, PartialEq, Eq, From, Into)]
    pub struct GlobalScalerRegister(u32);
    impl Debug;
    u8;

    pub globalscaler, set_globalscaler: 7, 0;
}
