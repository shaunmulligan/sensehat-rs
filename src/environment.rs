#[warn(dead_code)]

// Constants for the HTS221 sensor
pub const HTS221_ADDR: u16 = 0x5f;
pub const POWER_UP: u8 = 0x80;
pub const CTRL_REG1: u8 = 0x20;
pub const ODR0_SET: u8 = 0x1;
pub const HUMIDITY_READY: u8 = 0x2;
pub const STATUS_REG: u8 = 0x27;
pub const HUMIDITY_L_REG: u8 = 0x28;
pub const HUMIDITY_H_REG: u8 = 0x29;
