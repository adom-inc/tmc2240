use bitfield::bitfield;
use derive_more::{From, Into};

bitfield! {
    /// ADC_VSUPPLY_AIN
    #[derive(Clone, Copy, PartialEq, Eq, From, Into)]
    pub struct AdcMotorVoltageAndAinRegister(u32);
    impl Debug;
    u16;

    pub adc_ain, _: 16, 28;
    pub adc_vsupply, _: 12, 0;
}

bitfield! {
    /// ADC_TEMP
    #[derive(Clone, Copy, PartialEq, Eq, From, Into)]
    pub struct AdcTempRegister(u32);
    impl Debug;
    u16;

    pub adc_temp, _: 12, 0;
}

bitfield! {
    /// OTW_OV_VTH
    #[derive(Clone, Copy, PartialEq, Eq, From, Into)]
    pub struct OverTemperatureWarningAndOverVoltageThresholdRegister(u32);
    impl Debug;
    u16;

    pub overtempprewarning_vth, _: 28, 16;
    pub overvoltage_vth, _: 0, 12;
}
