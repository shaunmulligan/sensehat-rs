extern crate framebuffer;

use framebuffer::{KdMode, Framebuffer};

const W:u8 = 8;
const H:u8 = 8;
const LINE_LENGTH:u8 = 16;
const BYTESPP:u8 = 2;

// Allows clear to accept no args, rbg values or a color object.
macro_rules! clear {
    ($a: expr, $b: expr, $c: expr) => {clear(&Color{r:$a, g:$b, b:$c})};
    ($a: expr) => { clear($a) };
    () => { clear(&Color{r:0, g:0, b:0}) };
}

#[derive(Debug)]
struct Color {
    r: u8,
    g: u8,
    b: u8,
}

//Algorithm copied from:
//https://en.wikipedia.org/wiki/Mandelbrot_set
fn main() {
    let mut framebuffer = Framebuffer::new("/dev/fb1").unwrap();

    // let w = framebuffer.var_screen_info.xres;
    // let h = framebuffer.var_screen_info.yres;
    // println!("width = {}", w);
    // println!("height = {}", h);
    // let line_length = framebuffer.fix_screen_info.line_length;
    // println!("line_length = {}", line_length);
    // let bytespp = framebuffer.var_screen_info.bits_per_pixel / 8;
    // println!("bytespp = {}", bytespp);

    //let mut frame = vec![0u8; (LINE_LENGTH * H) as usize];

    // let red = Color{r:255, g:0, b:0};
    // let blue = Color{r:0, g:0, b:255};
    // for (r, line) in frame.chunks_mut(LINE_LENGTH as usize).enumerate() {
    //     for (c, p) in line.chunks_mut(BYTESPP as usize).enumerate() {

    //         let (lo, hi) = color_to_pixel(&red);
    //         p[0] = lo;
    //         p[1] = hi;
             
    //     }
    // }
 

    // let _ = framebuffer.write_frame(&frame);

    // clear!();
    // clear!(&red);
    // clear!(1,1,1);

    let mut pixel_list = vec![  [0, 0, 255], [255, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], 
                                [0, 0, 255], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], 
                                [0, 0, 255], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], 
                                [0, 0, 255], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], 
                                [0, 0, 255], [0, 0, 0], [0, 255, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], 
                                [0, 0, 255], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], 
                                [0, 0, 255], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], 
                                [0, 0, 255], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0]];

    set_pixels(&mut framebuffer, &mut pixel_list);

    loop{
        //println!("Looping forever");
    }
}


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

fn clear(f: &Framebuffer, c: &Color) {
    println!("Clearing with color: {:?}", c);
    let pixel_list = vec![[c.r, c.g, c.b]; 64];
    set_pixels(&mut f, &mut pixel_list);
}


/// Accepts a Vec containing 64 smaller array of [R,G,B] pixels and
/// updates the LED matrix. R,G,B elements must intergers between 0
/// and 255
fn set_pixels(f: &mut Framebuffer, pixel_list: &mut Vec< [u8; 3] >) {

    let mut frame = vec![0u8; (LINE_LENGTH * H) as usize];
    let mut temp_index = 0;

    for (r, line) in pixel_list.chunks_mut(8 as usize).enumerate() {
        for (c, p) in line.chunks_mut(1 as usize).enumerate() {
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