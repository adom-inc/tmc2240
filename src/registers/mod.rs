use enum_iterator::Sequence;

pub mod adc;
pub mod direct_mode;
pub mod encoder;
pub mod general_config;
pub mod motor_driver;
pub mod velocity_dependent;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Sequence)]
#[repr(u8)]
#[allow(non_camel_case_types)]
pub enum RegisterAddress {
    /* General Config */
    GCONF = 0x00,
    GSTAT = 0x01,
    IFCNT = 0x02,
    NODECONF = 0x03,
    IOIN = 0x04,
    DRV_CONF = 0x0A,
    GLOBAL_SCALER = 0x0B,
    /* Velocity Dependent Configuration Registers */
    IHOLD_IRUN = 0x10,
    TPOWERDOWN = 0x11,
    TSTEP = 0x12,
    TPWMTHRS = 0x13,
    TCOOLTHRS = 0x14,
    THIGH = 0x15,
    /* Direct Mode Register */
    DIRECT_MODE = 0x2D,
    /* Encoder Registers */
    ENCMODE = 0x38,
    X_ENC = 0x39,
    ENC_CONST = 0x3A,
    ENC_STATUS = 0x3B,
    ENC_LATCH = 0x3C,
    /* ADC Registers */
    ADC_VSUPPLY_AIN = 0x50,
    ADC_TEMP = 0x51,
    OTW_OV_VTH = 0x52,
    /* Motor Driver Registers */
    MSLUT_0 = 0x60,
    MSLUT_1 = 0x61,
    MSLUT_2 = 0x62,
    MSLUT_3 = 0x63,
    MSLUT_4 = 0x64,
    MSLUT_5 = 0x65,
    MSLUT_6 = 0x66,
    MSLUT_7 = 0x67,
    MSLUTSEL = 0x68,
    MSLUTSTART = 0x69,
    MSCNT = 0x6A,
    MSCURACT = 0x6B,
    CHOPCONF = 0x6C,
    COOLCONF = 0x6D,
    DRV_STATUS = 0x6F,
    PWMCONF = 0x70,
    PWM_SCALE = 0x71,
    PWM_AUTO = 0x72,
    SG4_THRS = 0x74,
    SG4_RESULT = 0x75,
    SG4_IND = 0x76,
}
