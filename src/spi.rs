use bitfield::bitfield;
use derive_more::{From, Into};

bitfield! {
    /// Defined in Table 1 (SPI Datagram Structure)
    #[derive(Clone, Copy, PartialEq, Eq, From, Into)]
    pub struct Instruction(u8);
    impl Debug;
    u8;
    pub write, set_write: 7;
    pub address, set_address: 6, 0;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OpCode {
    Read,
    Write,
}

impl Instruction {
    pub fn new(op_code: OpCode, address: u8) -> Self {
        let mut i = Self(0);

        i.set_write(op_code == OpCode::Write);
        i.set_address(address);

        i
    }
}

bitfield! {
    /// Defined in Table 3 (SPI_STATUS)
    #[derive(Clone, Copy, PartialEq, Eq, From, Into)]
    pub struct SpiStatus(u8);
    impl Debug;
    u8;
    pub stand_still, set_stand_still: 3;
    pub stall_guard_2, set_stall_guard_2: 2;
    pub driver_error, set_driver_error: 1;
    pub reset_flag, set_reset_flag: 0;
}
