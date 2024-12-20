use super::*;
use crate::register::{Register16Bit, Register8Bit};

impl<MODE, I2C, E> VL6180X<MODE, I2C>
where
    I2C: I2c<Error = E>,
{
    /// Reads a named 8-bit register
    pub(crate) fn read_named_register(&mut self, reg: Register8Bit) -> Result<u8, E> {
        self.read_register(reg as u16)
    }

    /// Reads an 8-bit register
    fn read_register(&mut self, reg: u16) -> Result<u8, E> {
        let mut data: [u8; 1] = [0];
        let reg: [u8; 2] = reg.to_be_bytes();

        self.com.write_read(self.config.address, &reg, &mut data)?;
        Ok(data[0])
    }

    /// Reads a named 16-bit register
    pub(crate) fn read_named_register_16bit(&mut self, reg: Register16Bit) -> Result<u16, E> {
        self.read_register_16bit(reg as u16)
    }

    /// Reads a 16-bit register
    fn read_register_16bit(&mut self, reg: u16) -> Result<u16, E> {
        let mut data: [u8; 2] = [0, 0];
        let reg: [u8; 2] = reg.to_be_bytes();

        self.com.write_read(self.config.address, &reg, &mut data)?;
        Ok(u16::from_be_bytes(data))
    }

    /// Reads a named 32-bit register
    fn read_named_register_32bit(&mut self, reg: Register16Bit) -> Result<u32, E> {
        self.read_register_32bit(reg as u16)
    }

    /// Reads a 32-bit register
    fn read_register_32bit(&mut self, reg: u16) -> Result<u32, E> {
        let mut data: [u8; 4] = [0, 0, 0, 0];
        let reg: [u8; 2] = reg.to_be_bytes();

        self.com.write_read(self.config.address, &reg, &mut data)?;
        Ok(u32::from_be_bytes(data))
    }

    pub(super) fn write_only_named_register(
        &mut self,
        reg: Register8Bit,
        code: u8,
    ) -> Result<(), E> {
        self.write_only_register(reg as u16, code)
    }

    pub(super) fn write_only_register(&mut self, reg: u16, code: u8) -> Result<(), E> {
        let reg = reg.to_be_bytes();
        let bytes: [u8; 3] = [reg[0], reg[1], code];
        self.com.write(self.config.address, &bytes)
    }

    pub(super) fn write_named_register(&mut self, reg: Register8Bit, code: u8) -> Result<(), E> {
        self.write_register(reg as u16, code)
    }

    pub(super) fn write_register(&mut self, reg: u16, code: u8) -> Result<(), E> {
        let mut buffer = [0];
        let reg = reg.to_be_bytes();
        let bytes: [u8; 3] = [reg[0], reg[1], code];
        self.com
            .write_read(self.config.address, &bytes, &mut buffer)
    }

    pub(super) fn write_named_register_16bit(
        &mut self,
        reg: Register16Bit,
        code: u16,
    ) -> Result<(), E> {
        self.write_register_16bit(reg as u16, code)
    }

    fn write_register_16bit(&mut self, reg: u16, code: u16) -> Result<(), E> {
        let mut buffer = [0];
        let code = code.to_be_bytes();
        let reg = reg.to_be_bytes();
        let bytes: [u8; 4] = [reg[0], reg[1], code[0], code[1]];
        self.com
            .write_read(self.config.address, &bytes, &mut buffer)
    }

    pub(super) fn write_named_register_32bit(
        &mut self,
        reg: Register16Bit,
        code: u32,
    ) -> Result<(), E> {
        self.write_register_32bit(reg as u32, code)
    }

    fn write_register_32bit(&mut self, reg: u32, code: u32) -> Result<(), E> {
        let mut buffer = [0];
        let code = code.to_be_bytes();
        let reg = reg.to_be_bytes();
        let bytes: [u8; 6] = [reg[0], reg[1], code[0], code[1], code[2], code[3]];
        self.com
            .write_read(self.config.address, &bytes, &mut buffer)
    }

    // fn write_6bytes(&mut self, reg: Register8Bit, bytes: [u8; 6]) -> Result<(), E> {
    //     let mut buf: [u8; 6] = [0, 0, 0, 0, 0, 0];
    //     self.com.write_read(
    //         self.config.address,
    //         &[
    //             reg as u8, bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5],
    //         ],
    //         &mut buf,
    //     )
    // }
}
