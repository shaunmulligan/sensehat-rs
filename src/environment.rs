#[warn(dead_code)]

// Constants for the HTS221 sensor
pub const HTS221_ADDR: u16 		= 0x5f;

// 8 bit Register addresses
pub const AV_CONF: u8 			= 0x10;
pub const POWER_UP: u8 			= 0x80;
pub const CTRL_REG1: u8 		= 0x20;
//pub const CTRL_REG2: u8 		= 0x21;
//pub const CTRL_REG3: u8 		= 0x22;
pub const STATUS_REG: u8 		= 0x27;
pub const ODR0_SET: u8 			= 0x1;
pub const HUMIDITY_READY: u8 	= 0x2;
pub const HUMIDITY_L_REG: u8 	= 0x28;
pub const HUMIDITY_H_REG: u8 	= 0x29;
pub const TEMP_READY: u8 		= 0x1;
pub const TEMP_L_REG: u8     	= 0x2A;
pub const TEMP_H_REG: u8     	= 0x2B;
pub const T0_DEGC_X8: u8 		= 0x32;
pub const T1_DEGC_X8: u8 		= 0x33;
pub const T1_T0_MSB: u8 		= 0x35;
pub const H0_RH_X2: u8 			= 0x30;
pub const H1_RH_X2: u8 			= 0x31;
//pub const REG_WHO_AM_I: u8 = 0x0F; // Return value is 0xBC

pub const H0_T0_OUT_L: u8    	= 0x36;
pub const H0_T0_OUT_H: u8    	= 0x37;
pub const H1_T0_OUT_L: u8    	= 0x3A;
pub const H1_T0_OUT_H: u8    	= 0x3B;
pub const T0_OUT_L: u8       	= 0x3C;
pub const T0_OUT_H: u8       	= 0x3D;
pub const T1_OUT_L: u8       	= 0x3E;
pub const T1_OUT_H: u8       	= 0x3F;
