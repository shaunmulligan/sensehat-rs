#[warn(dead_code)]

use i2cdev::linux::LinuxI2CDevice;
use i2cdev::core::I2CDevice;

//  LPS25H I2C Slave Addresses
pub const LPS25H_ADDR: u16        	 = 0x5c;
// const LPS25H_ADDRESS1             0x5d;
// const LPS25H_REG_ID               0x0f;
// const LPS25H_ID: u8                  = 0xbd;

//	Register map

// const LPS25H_REF_P_XL: u8            = 0x08;
// const LPS25H_REF_P_XH: u8            = 0x09;
const LPS25H_RES_CONF: u8            = 0x10;
const LPS25H_CTRL_REG_1: u8          = 0x20;
const LPS25H_CTRL_REG_2: u8          = 0x21;
// const LPS25H_CTRL_REG_3: u8          = 0x22;
// const LPS25H_CTRL_REG_4: u8          = 0x23;
// const LPS25H_INT_CFG: u8             = 0x24;
// const LPS25H_INT_SOURCE: u8          = 0x25;
const LPS25H_STATUS_REG: u8          = 0x27;
const LPS25H_PRESS_OUT_XL: u8        = 0x28;
const LPS25H_PRESS_OUT_L: u8         = 0x29;
const LPS25H_PRESS_OUT_H: u8         = 0x2a;
const LPS25H_TEMP_OUT_L: u8          = 0x2b;
const LPS25H_TEMP_OUT_H: u8          = 0x2c;
const LPS25H_FIFO_CTRL: u8           = 0x2e;
// const LPS25H_FIFO_STATUS: u8         = 0x2f;
// const LPS25H_THS_P_L: u8             = 0x30;
// const LPS25H_THS_P_H: u8             = 0x31;
// const LPS25H_RPDS_L: u8              = 0x39;
// const LPS25H_RPDS_H: u8              = 0x3a;

pub struct Lps25h;

impl Lps25h {
    pub fn init(i2cdev: &mut LinuxI2CDevice){
    	println!("Initialising Pressure Sensor");
    	i2cdev.smbus_write_byte_data(LPS25H_CTRL_REG_1, 0xC4).unwrap();
  		i2cdev.smbus_write_byte_data( LPS25H_RES_CONF, 0x05).unwrap();
  		i2cdev.smbus_write_byte_data(LPS25H_FIFO_CTRL, 0xC0).unwrap();
  		i2cdev.smbus_write_byte_data(LPS25H_CTRL_REG_2, 0x40).unwrap();
    }

    pub fn get_pressure(i2cdev: &mut LinuxI2CDevice) -> f32 {
        return (Lps25h::get_raw_pressure(i2cdev) as f32 )/4096.0
    }

    pub fn get_temperature(i2cdev: &mut LinuxI2CDevice) -> f32 {
    	return (Lps25h::get_raw_temperature(i2cdev) as f32)/ 480.0 + 42.5
    }

    fn get_raw_pressure(i2cdev: &mut LinuxI2CDevice) -> u32 {
    	// Wait until pressure status ready is true
        loop {
            // TODO: handle error case here
            let status_reg = i2cdev.smbus_read_byte_data(LPS25H_STATUS_REG).unwrap();
            let status = status_reg & 2;
            // TODO: add a timeout route, since this will block forever
            match status {
                2 => {
                    // TODO: handle errors on byte reads
                    let xl_reg = i2cdev.smbus_read_byte_data(LPS25H_PRESS_OUT_XL).unwrap();
                    let h_reg = i2cdev.smbus_read_byte_data(LPS25H_PRESS_OUT_H).unwrap() ;
                    let l_reg = i2cdev.smbus_read_byte_data(LPS25H_PRESS_OUT_L).unwrap() ;

                    let output = ((h_reg as u32) << 16) | ((l_reg as u32) << 8) | (xl_reg as u32);
                    return output;

                }
                _ => {
                    //println!("Not ready to read pressure");
                    continue;
                }
            }
        }
    }

    fn get_raw_temperature(i2cdev: &mut LinuxI2CDevice) -> i16{
    	// Wait until temp status ready is true
        loop {
            // TODO: handle error case here
            let status_reg = i2cdev.smbus_read_byte_data(LPS25H_STATUS_REG).unwrap();
            let status = status_reg & 1;
            // TODO: add a timeout route, since this will block forever
            match status {
                1 => {
                    // TODO: handle errors on byte reads
                    let h_reg = i2cdev.smbus_read_byte_data(LPS25H_TEMP_OUT_H).unwrap() ;
                    let l_reg = i2cdev.smbus_read_byte_data(LPS25H_TEMP_OUT_L).unwrap() ;

                    let output = data_convert(h_reg, l_reg);
                    return output;
                }
                _ => {
                    //println!("Not ready to read pressure");
                    continue;
                }
            }
        }
    }
}

//======= Helpers Utilities =======

/// converts 2 u8 registers into a signed i16
/// data_convert (high_reg, low_reg)
fn data_convert(h: u8, l: u8) -> i16{
    let output = ((h as u16) << 8) | (l as u16);
    return output as i16
}
