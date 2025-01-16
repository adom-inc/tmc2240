//! This driver is based on the information from the official TMC2240 datasheet
//! found here:
//! https://www.analog.com/media/en/technical-documentation/data-sheets/tmc2240_datasheet.pdf

#![no_std]

use embedded_hal_async::spi::SpiDevice;
use registers::RegisterAddress;

use crate::spi::{Instruction, OpCode, SpiStatus};

mod macros;
pub mod registers;
pub mod spi;

pub struct TMC2240<SPI>
where
    SPI: SpiDevice,
{
    spi: SPI,
}

impl<SPI> TMC2240<SPI>
where
    SPI: SpiDevice,
{
    pub fn new(spi: SPI) -> Self {
        Self { spi }
    }

    /// Sends a command and returns the data it received. Since IO on the
    /// TMC2240 is designed to allow for pipelining, this function returns the
    /// response from the *previous* call to [`send_raw`](TMC2240::send_raw).
    pub async fn send_raw(
        &mut self,
        op_code: OpCode,
        address: u8,
        data: u32,
    ) -> Result<(SpiStatus, u32), SPI::Error> {
        let mut buf = [0u8; 5];
        buf[0] = Instruction::new(op_code, address).into();
        buf[1..].copy_from_slice(&data.to_be_bytes());

        self.spi.transfer_in_place(&mut buf).await?;

        let status = SpiStatus(buf[0]);

        let mut data = [0u8; 4];
        data.copy_from_slice(&buf[1..]);
        let data = u32::from_be_bytes(data);

        Ok((status, data))
    }

    /// Performs a read operation on a single register. This requires
    /// transmitting 2 SPI datagrams since the IO on the TMS2240 is designed to
    /// be pipelined. To perform a pipelined read, use
    /// [`read_pipelined`](TMC2240::read_pipelined).
    pub async fn read_single(&mut self, address: u8) -> Result<u32, SPI::Error> {
        let _ = self.send_raw(OpCode::Read, address, 0).await?;
        let (_, data) = self.send_raw(OpCode::Read, address, 0).await?;

        Ok(data)
    }

    /// Performs a write operation on a single register. This requires
    /// transmitting 2 SPI datagrams since the IO on the TMS2240 is designed to
    /// be pipelined. To perform a pipelined write, use
    /// [`write_pipelined`](TMC2240::write_pipelined).
    pub async fn write_single(&mut self, address: u8, data: u32) -> Result<(), SPI::Error> {
        let _ = self.send_raw(OpCode::Write, address, data).await?;
        let (_, res) = self.send_raw(OpCode::Write, address, data).await?;

        // assert_eq!(data, res, "Write result did not match transmitted data!");

        Ok(())
    }

    pub async fn read_register<T: From<u32>>(
        &mut self,
        address: RegisterAddress,
    ) -> Result<T, SPI::Error> {
        Ok(self.read_single(address as u8).await?.into())
    }

    pub async fn write_register(
        &mut self,
        address: RegisterAddress,
        data: u32,
    ) -> Result<(), SPI::Error> {
        self.write_single(address as u8, data).await?;
        Ok(())
    }

    pub async fn modify_register<T: From<u32> + Into<u32>, F: FnOnce(&mut T)>(
        &mut self,
        address: RegisterAddress,
        f: F,
    ) -> Result<(), SPI::Error> {
        let mut reg: T = self.read_register(address).await?;
        f(&mut reg);
        self.write_register(address, reg.into()).await?;
        Ok(())
    }

    pub async fn read_pipelined(_address: u8) {
        // Note: Maybe use macros?
        todo!("Implement pipelined IO")
    }

    pub async fn write_pipelined(_address: u8) {
        // Note: Maybe use macros?
        todo!("Implement pipelined IO")
    }
}
