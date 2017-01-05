# SenseHat Rust library

This crate implements an API to the Raspberry Pi [SenseHat](https://www.raspberrypi.org/products/sense-hat/). It tries as best as possible to emulate the python API defined here: https://pythonhosted.org/sense-hat/api/

## Usage:

Clone this repo, and make sure you have the following prerequisite installed. __Note:__ Currently this will only work on linux machines.

### Prerequisite:
* Docker
* rustc and Cargo: install using `curl https://sh.rustup.rs -sSf | sh`
* [cross](https://github.com/japaric/cross): install using cargo install cross
* [resin device toolbox](https://www.npmjs.com/package/resin-device-toolbox)

Once you have all that setup, you can just run the `run-local.sh` script to test your code on a resinOS device connected to the network.

### API Interface:

#### Environmental sensors

- [x] get_humidity()
- [x] get_temperature()
- [x] get_temperature_from_humidity()
- [x] get_temperature_from_pressure()
- [x] get_pressure()

#### LED Matrix

- [ ] set_rotation()
- [ ] flip_h()
- [ ] flip_v()
- [x] set_pixels()
- [ ] get_pixels() -- Need to figure out how to read back from the framebuffer
- [ ] set_pixel()
- [ ] get_pixel()
- [ ] load_image()
- [x] clear() -- currently can only clear to black.
- [ ] show_message()
- [ ] show_letter()
- [ ] low_light()
- [ ] gamma
- [ ] gamma_reset

#### IMU Sensor

- [ ] set_imu_config()
- [ ] get_orientation_radians()
- [ ] get_orientation_degrees()
- [ ] get_orientation()
- [ ] get_compass()
- [ ] get_compass_raw()
- [ ] get_gyroscope()
- [ ] get_gyroscope_raw()
- [ ] get_accelerometer()
- [ ] get_accelerometer_raw()

#### Joystick

- [ ] InputEvent
- [ ] wait_for_event()
- [ ] get_events()
- [ ] direction_up, direction_left, direction_right, direction_down, direction_middle, direction_any


### Internals:

## i2cdetect:
```
$ i2cdetect -y 1
     0  1  2  3  4  5  6  7  8  9  a  b  c  d  e  f
00:          -- -- -- -- -- -- -- -- -- -- -- -- -- 
10: -- -- -- -- -- -- -- -- -- -- -- -- 1c -- -- -- 
20: -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- 
30: -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- 
40: -- -- -- -- -- -- UU -- -- -- -- -- -- -- -- -- 
50: -- -- -- -- -- -- -- -- -- -- -- -- 5c -- -- 5f 
60: -- -- -- -- -- -- -- -- -- -- 6a -- -- -- -- -- 
70: -- -- -- -- -- -- -- --   
```
1c == Magnetometer
6a == The IMU (Accelerometer and Magnetometer) through a LSM9DS1
5c == LPS25H Pressure/Temperature sensor
5f == HTS221 Humidity/Temperature sensor

info from https://pinout.xyz/pinout/sense_hat




[	0, 0, 0, 0, 15, 0, 15, 0, 15, 0, 15, 0, 0, 0, 0, 0,
	0, 0, 15, 0, 0, 0, 0, 0, 0, 0, 0, 0, 15, 0, 0, 0, 
	15, 0, 0, 0, 15, 0, 0, 0, 0, 0, 15, 0, 0, 0, 15, 0, 
	15, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 15, 0, 
	15, 0, 0, 0, 15, 0, 0, 0, 0, 0, 15, 0, 0, 0, 15, 0, 
	15, 0, 0, 0, 0, 0, 15, 0, 15, 0, 0, 0, 0, 0, 15, 0, 
	0, 0, 15, 0, 0, 0, 0, 0, 0, 0, 0, 0, 15, 0, 0, 0, 
	0, 0, 0, 0, 15, 0, 15, 0, 15, 0, 15, 0, 0, 0, 0, 0]


(u8, u8)	== 
(15, 0) 	== (00001111, 00000000)
