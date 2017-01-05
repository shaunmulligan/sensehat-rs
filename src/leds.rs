// extern crate framebuffer;
// extern crate glob;
use std::io;
use std::io::Error as err;
use std::io::prelude::*;
use std::fs::File;
use std::error::Error;
use std::path::PathBuf;

use framebuffer::Framebuffer;
use glob::glob;

// const W:u8 = 8;
const H:u8 = 8;
const LINE_LENGTH:u8 = 16;
// const BYTESPP:u8 = 2;

// Allows clear to accept no args, rbg values or a color object.
// #[macro_export]
// macro_rules! clear {
//     ($a: expr, $b: expr, $c: expr) => {clear(&Color{r:$a, g:$b, b:$c})};
//     ($a: expr) => { clear($a) };
//     () => { clear(&Color{r:0, g:0, b:0}) };
// }

#[derive(Debug)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

pub struct Leds;

impl Leds {

	pub fn _get_sensehat_fb_device() -> Result<PathBuf, &'static str> {

	    for entry in glob("/sys/class/graphics/fb*").expect("Failed to read /sys/class/graphics/") {
	        match entry {
	            Ok(result) => {
	                let name_path = result.join("name");
	                match name_path.exists() {
	                    true => {

	                        let mut file = match File::open(&name_path) {
	                            Err(why) => {
	                                panic!("couldn't open {}: {}",
	                                       name_path.display(),
	                                       why.description())
	                            }
	                            Ok(file) => file,
	                        };

	                        let mut s = String::new();
	                        match file.read_to_string(&mut s) {
	                            Err(why) => {
	                                panic!("couldn't read {}: {}",
	                                       name_path.display(),
	                                       why.description())
	                            }
	                            Ok(_) => {
	                                match s.as_ref() {
	                                    "RPi-Sense FB\n" => {

	                                        let fb_dev = name_path.parent().unwrap().strip_prefix("/sys/class/graphics/").unwrap();
                                        	let fb_path = PathBuf::from("/dev").join(fb_dev);
	                                        return Ok(fb_path);
	                                    }
	                                    _ => {
	                                        println!("This isn't what we are looking for");
	                                        continue;
	                                    }
	                                }
	                            }
	                        }

	                    }
	                    false => println!("This fb has no name file"),
	                }

	            }
	            Err(e) => {
	                println!("Error reading path {:?}", e);
	                return Err("Error getting framebuffer device");
	            }
	        }
	    };
	    Err("No framebuffer devices found")
	}	

    /// Accepts a Vec containing 64 smaller array of [R,G,B] pixels and
	/// updates the LED matrix. R,G,B elements must intergers between 0
	/// and 255
	pub fn set_pixels(f: &mut Framebuffer, pixel_list: &mut Vec< [u8 ; 3] >) {

	    let mut frame = vec![0u8; (LINE_LENGTH * H) as usize];
	    let mut temp_index = 0;

	    for (_, line) in pixel_list.chunks_mut(8 as usize).enumerate() {
	        for (_, p) in line.chunks_mut(1 as usize).enumerate() {
	            let color = vec_to_color(p[0]);
	            let (lo, hi) = color_to_pixel(&color);
	            let index0 = temp_index;
	            let index1 = index0 + 1;
	            temp_index = index1 + 1;
	            frame[index0] = lo;
	            frame[index1] = hi;
	        }
	    }
	 

	    let _ = f.write_frame(&frame);
	}

	pub fn get_pixels() -> Result< [u8 ; 3], err> {
		let fb = Leds::_get_sensehat_fb_device().unwrap();
		let mut f = try!(File::open(fb));
		let mut buffer = vec![0u8; (LINE_LENGTH * H) as usize];

		// read up to 10 bytes
		try!(f.read(&mut buffer));
		println!("{:?}", buffer);
		let mut array: [u8; 3] = [0; 3];
		return Ok(array)
	}

    pub fn clear(mut f: &mut Framebuffer, c: &Color) {
	    println!("Clearing with color: {:?}", c);
	    let mut pixel_list = vec![[c.r, c.g, c.b]; 64];
	    Leds::set_pixels(&mut f, &mut pixel_list);
	}
}

//======== Helpers ========

/// converts rgb values to 2  8-bit RGB565, which represents a pixel.
fn color_to_pixel(c: &Color) -> (u8, u8) {
    let r = ((c.r >> 3) & 0x1F) as u16;
    let g = ((c.g >> 2) & 0x3F) as u16;
    let b = ((c.b >> 3) & 0x1F) as u16;
    let output = (r  << 11) + (g << 5) + b ;

    return split_u16(output)
}

fn vec_to_color(v: [u8; 3]) -> Color {
    return Color{r: v[0], g: v[1], b:v[2]}
}

fn split_u16 (val: u16) -> (u8, u8){
    let mask = 0b1111111100000000;
    let lo = (val & !mask) as u8;
    let hi = ((val & mask) >> 8) as u8;
    return (lo, hi)
}

