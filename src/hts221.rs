#[warn(dead_code)]

use i2cdev::linux::LinuxI2CDevice;
use i2cdev::core::I2CDevice;

// Constants for the HTS221 sensor
pub const HTS221_ADDR: u16 		= 0x5f;

// 8 bit Register addresses
const AV_CONF: u8 			= 0x10;
const POWER_UP: u8 			= 0x80;
const CTRL_REG1: u8 		= 0x20;
// const CTRL_REG2: u8 		= 0x21;
// const CTRL_REG3: u8 		= 0x22;
const STATUS_REG: u8 		= 0x27;
const ODR0_SET: u8 			= 0x1;
const HUMIDITY_READY: u8 	= 0x2;
const HUMIDITY_L_REG: u8 	= 0x28;
const HUMIDITY_H_REG: u8 	= 0x29;
const TEMP_READY: u8 		= 0x1;
const TEMP_L_REG: u8     	= 0x2A;
const TEMP_H_REG: u8     	= 0x2B;
const T0_DEGC_X8: u8 		= 0x32;
const T1_DEGC_X8: u8 		= 0x33;
const T1_T0_MSB: u8 		= 0x35;
const H0_RH_X2: u8 			= 0x30;
const H1_RH_X2: u8 			= 0x31;
// const REG_WHO_AM_I: u8 = 0x0F; // Return value is 0xBC

const H0_T0_OUT_L: u8    	= 0x36;
const H0_T0_OUT_H: u8    	= 0x37;
const H1_T0_OUT_L: u8    	= 0x3A;
const H1_T0_OUT_H: u8    	= 0x3B;
const T0_OUT_L: u8       	= 0x3C;
const T0_OUT_H: u8       	= 0x3D;
const T1_OUT_L: u8       	= 0x3E;
const T1_OUT_H: u8       	= 0x3F;

pub struct Hts221;

impl Hts221 {
    pub fn init(i2cdev: &mut LinuxI2CDevice){
        println!("Initialising Humdity Sensor");
        // Setup the humidity sensor in read mode
        let ctl_reg = i2cdev.smbus_read_byte_data(CTRL_REG1).unwrap();
        let mut result = POWER_UP | ctl_reg;
        result = result | ODR0_SET;
        i2cdev.smbus_write_byte_data(CTRL_REG1, result).unwrap();
    }

    pub fn configure(i2cdev: &mut LinuxI2CDevice) {
        // Set AV_CONF = 0x1b
        i2cdev.smbus_write_byte_data(AV_CONF, 0x1b).unwrap();
    }

    pub fn get_calibration(i2cdev: &mut LinuxI2CDevice) -> Vec<f32> {
		// Calculate h_slope and h_offset
        
        let h0_r_h = i2cdev.smbus_read_byte_data(H0_RH_X2).unwrap() as f32/2.0;
        let h1_r_h = i2cdev.smbus_read_byte_data(H1_RH_X2).unwrap() as f32/2.0;

        let h0_t0_l = i2cdev.smbus_read_byte_data(H0_T0_OUT_L).unwrap();
        let h0_t0_h = i2cdev.smbus_read_byte_data(H0_T0_OUT_H).unwrap();

        let h0_t0_out  = data_convert(h0_t0_h, h0_t0_l) as f32;

        let h1_t0_l = i2cdev.smbus_read_byte_data(H1_T0_OUT_L).unwrap();
        let h1_t0_h = i2cdev.smbus_read_byte_data(H1_T0_OUT_H).unwrap();

        let h1_t0_out  = data_convert(h1_t0_h, h1_t0_l) as f32;

        let h_slope = (h1_r_h - h0_r_h) / (h1_t0_out - h0_t0_out);
        let h_offset = h0_r_h  - (h_slope * h0_t0_out);

        let t0_deg_c_x8 = i2cdev.smbus_read_byte_data(T0_DEGC_X8).unwrap();
        let t1_t0_msb = i2cdev.smbus_read_byte_data(T1_T0_MSB).unwrap();
        let t0_deg_c = ((t0_deg_c_x8 as u16) + (1 << 8) * (t1_t0_msb & 0x03) as u16) as f32/ 8.0;

        let t1_deg_c_x8 = i2cdev.smbus_read_byte_data(T1_DEGC_X8).unwrap();
        let t1_t0_msb = i2cdev.smbus_read_byte_data(T1_T0_MSB).unwrap();
        let t1_deg_c = ((t1_deg_c_x8 as u16) + (1 << 6) * (t1_t0_msb & 0x0C) as u16) as f32/ 8.0; 

        let t0_out_h = i2cdev.smbus_read_byte_data(T0_OUT_H).unwrap();
        let t0_out_l = i2cdev.smbus_read_byte_data(T0_OUT_L).unwrap();

        let t0_out = data_convert(t0_out_h, t0_out_l) as f32;

        let t1_out_h = i2cdev.smbus_read_byte_data(T1_OUT_H).unwrap();
        let t1_out_l = i2cdev.smbus_read_byte_data(T1_OUT_L).unwrap();
        
        let t1_out = data_convert(t1_out_h, t1_out_l) as f32;

        let t_slope = (t1_deg_c - t0_deg_c)/(t1_out - t0_out);
        let t_offset = t0_deg_c - (t_slope * t0_out);

        return vec![t_slope, t_offset, h_slope, h_offset]
    }

        /// Read the raw u16 data for humidity
    pub fn get_raw_humidity(i2cdev: &mut LinuxI2CDevice) -> u16 {

        // Wait until humidity status ready is true
        loop {
            // TODO: handle error case here
            let status_reg = i2cdev.smbus_read_byte_data(STATUS_REG).unwrap();

            let hum_status = status_reg & HUMIDITY_READY;

            // TODO: add a timeout route, since this will block forever
            match hum_status {
                2 => {
                    // TODO: handle errors on byte reads
                    let h_reg =
                        i2cdev.smbus_read_byte_data(HUMIDITY_H_REG).unwrap() ;//as u16;
                    let l_reg =
                        i2cdev.smbus_read_byte_data(HUMIDITY_L_REG).unwrap() ;//as u16;

                    let output = data_convert(h_reg, l_reg);
                    
                    return output as u16;

                }
                _ => {
                    //println!("Not ready to read humidity");
                    continue;
                }
            }
        }

    }

	pub fn get_raw_temperature(i2cdev: &mut LinuxI2CDevice) -> u16 {

	        // Wait until Temperature status ready is true
	        loop {
	            // TODO: handle error case here
	            let status_reg = i2cdev.smbus_read_byte_data(STATUS_REG).unwrap();

	            let temp_status = status_reg & TEMP_READY;

	            // TODO: add a timeout route, since this will block forever
	            match temp_status {
	                1 => {
	                    // TODO: handle errors on byte reads
	                    let h_reg =
	                        i2cdev.smbus_read_byte_data(TEMP_H_REG).unwrap() ;//as u16;

	                    let l_reg =
	                        i2cdev.smbus_read_byte_data(TEMP_L_REG).unwrap() ;//as u16;

	                    let output = data_convert(h_reg, l_reg);
	                    
	                    return output as u16;

	                }
	                _ => {
	                    //println!("Not ready to read temperature");
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