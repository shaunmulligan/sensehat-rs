extern crate i2cdev;

mod environment;

use environment::*;

use i2cdev::linux::{LinuxI2CDevice, LinuxI2CError};
use i2cdev::core::I2CDevice;

const I2C_DEV: &'static str = "/dev/i2c-1";

#[derive(Debug)]
pub struct SenseHat<LinuxI2CDevice> {
    // the i2c instance to environment sensor
    environment: LinuxI2CDevice,
    _t_slope: f32,
    _t_offset: f32,
    _h_slope: f32,
    _h_offset: f32,
}

impl SenseHat<LinuxI2CDevice> {
    /// Create a new SenseHat handle for the given I2C_DEV
    pub fn new() -> Result<SenseHat<LinuxI2CDevice>, LinuxI2CError> {
        // Try create an i2c reference to environment sensor
        let dev = try!(LinuxI2CDevice::new(I2C_DEV, HTS221_ADDR));

        //TODO: figure out how to initialise calibration variables here, currently can't figure out how to
        // get self into the ::new constructor.
        Ok(SenseHat { environment: dev, _t_slope: 0.0, _t_offset: 0.0, _h_slope: 0.0, _h_offset: 0.0 })
    }

    /// Initialise all the sensors and calibration variables
    pub fn init(&mut self) {
        self.hts221_init();
        self.hts221_configure();
        let calib = self.get_calibration_variables();
        self._t_slope = calib[0];
        self._t_offset = calib[1];
        self._h_slope = calib[2];
        self._h_offset = calib[3];
    }

    /// Initialises the HTS221 Temperature & Humidity sensor
    fn hts221_init(&mut self) {
        // Setup the humidity sensor in read mode
        let ctl_reg = self.environment.smbus_read_byte_data(CTRL_REG1).unwrap();
        let mut result = POWER_UP | ctl_reg;
        result = result | ODR0_SET;
        self.environment.smbus_write_byte_data(CTRL_REG1, result).unwrap();
    }

    /// Configure the way HTS221 outputs data
    fn hts221_configure(&mut self) {
        // Set AV_CONF = 0x1b
        self.environment.smbus_write_byte_data(AV_CONF, 0x1b).unwrap();
    }

    fn get_calibration_variables(&mut self) -> Vec<f32>{

        // Calculate h_slope and h_offset
        
        let h0_r_h = self.environment.smbus_read_byte_data(H0_RH_X2).unwrap() as f32/2.0;
        let h1_r_h = self.environment.smbus_read_byte_data(H1_RH_X2).unwrap() as f32/2.0;

        let h0_t0_l = self.environment.smbus_read_byte_data(H0_T0_OUT_L).unwrap();
        let h0_t0_h = self.environment.smbus_read_byte_data(H0_T0_OUT_H).unwrap();

        let h0_t0_out  = data_convert(h0_t0_h, h0_t0_l) as f32;

        let h1_t0_l = self.environment.smbus_read_byte_data(H1_T0_OUT_L).unwrap();
        let h1_t0_h = self.environment.smbus_read_byte_data(H1_T0_OUT_H).unwrap();

        let h1_t0_out  = data_convert(h1_t0_h, h1_t0_l) as f32;

        let h_slope = (h1_r_h - h0_r_h) / (h1_t0_out - h0_t0_out);
        let h_offset = h0_r_h  - (h_slope * h0_t0_out);

        let t0_deg_c_x8 = self.environment.smbus_read_byte_data(T0_DEGC_X8).unwrap();
        let t1_t0_msb = self.environment.smbus_read_byte_data(T1_T0_MSB).unwrap();
        let t0_deg_c = ((t0_deg_c_x8 as u16) + (1 << 8) * (t1_t0_msb & 0x03) as u16) as f32/ 8.0;

        let t1_deg_c_x8 = self.environment.smbus_read_byte_data(T1_DEGC_X8).unwrap();
        let t1_t0_msb = self.environment.smbus_read_byte_data(T1_T0_MSB).unwrap();
        let t1_deg_c = ((t1_deg_c_x8 as u16) + (1 << 6) * (t1_t0_msb & 0x0C) as u16) as f32/ 8.0; 

        let t0_out_h = self.environment.smbus_read_byte_data(T0_OUT_H).unwrap();
        let t0_out_l = self.environment.smbus_read_byte_data(T0_OUT_L).unwrap();

        let t0_out = data_convert(t0_out_h, t0_out_l) as f32;

        let t1_out_h = self.environment.smbus_read_byte_data(T1_OUT_H).unwrap();
        let t1_out_l = self.environment.smbus_read_byte_data(T1_OUT_L).unwrap();
        
        let t1_out = data_convert(t1_out_h, t1_out_l) as f32;

        let t_slope = (t1_deg_c - t0_deg_c)/(t1_out - t0_out);
        let t_offset = t0_deg_c - (t_slope * t0_out);

        return vec![t_slope, t_offset, h_slope, h_offset]
    }

    /// Gets the percentage of relative humidity from the humidity sensor.
    pub fn get_humidity(&mut self) -> f32 {
        let hum_raw = self.get_raw_humidity() as i16;
        let humidity = self._h_slope*(hum_raw as f32)  + self._h_offset;
        return humidity
    }

    /// Read the raw u16 data for humidity
    fn get_raw_humidity(&mut self) -> u16 {

        // Wait until humidity status ready is true
        loop {
            // TODO: handle error case here
            let status_reg = self.environment.smbus_read_byte_data(STATUS_REG).unwrap();

            let hum_status = status_reg & HUMIDITY_READY;

            // TODO: add a timeout route, since this will block forever
            match hum_status {
                2 => {
                    // TODO: handle errors on byte reads
                    let h_reg =
                        self.environment.smbus_read_byte_data(HUMIDITY_H_REG).unwrap() ;//as u16;
                    let l_reg =
                        self.environment.smbus_read_byte_data(HUMIDITY_L_REG).unwrap() ;//as u16;

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

    pub fn get_temperature(&mut self) -> f32 {
        return self.get_temperature_from_humidity()
    }

    pub fn get_temperature_from_humidity(&mut self) -> f32 {

        let temp_raw = self.get_raw_temperature() as f32;
        let temperature = self._t_slope*temp_raw + self._t_offset;

        return temperature
    }

    fn get_raw_temperature(&mut self) -> u16 {

        // Wait until Temperature status ready is true
        loop {
            // TODO: handle error case here
            let status_reg = self.environment.smbus_read_byte_data(STATUS_REG).unwrap();

            let temp_status = status_reg & TEMP_READY;

            // TODO: add a timeout route, since this will block forever
            match temp_status {
                1 => {
                    // TODO: handle errors on byte reads
                    let h_reg =
                        self.environment.smbus_read_byte_data(TEMP_H_REG).unwrap() ;//as u16;

                    let l_reg =
                        self.environment.smbus_read_byte_data(TEMP_L_REG).unwrap() ;//as u16;

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


//======= Helpers =======

/// converts 2 u8 registers into a signed i16
/// data_convert (high_reg, low_reg)
fn data_convert(h: u8, l: u8) -> i16{
    let output = ((h as u16) << 8) | (l as u16);
    return output as i16
}