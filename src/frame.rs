use std::fs::File;
use std::io::prelude::*;

use crate::vec3::Vec3f;

pub type FrameBuffer = Vec<Vec<Vec3f>>;

#[derive(Copy, Clone)]
pub struct Frame {
    pub width: usize,
    pub height: usize,
    pub fov_deg: f32
}

impl Frame {
    pub fn new(x: usize, y: usize, fov_deg: f32) -> Frame {
        Frame {
            width: x,
            height: y,
            fov_deg: fov_deg
        }
    }

    pub fn init_frame(self) -> FrameBuffer {
        let mut fb: FrameBuffer = Vec::new();
        for y in 0..self.height {
            fb.push(Vec::new());
            for x in 0..self.width {
                fb[y].push(Vec3f::zero());
            }
        }
        fb
    }

    pub fn get_pixel(framebuf: FrameBuffer, x: usize, y:usize) -> Vec3f {
        framebuf[y][x]
    }

    pub fn set_pixel(framebuf: &mut FrameBuffer, x: usize, y: usize, rgb: Vec3f) {
        //println!("set x:{} y:{} to r{} g{} b{}", x, y, rgb.x, rgb.y, rgb.z);
        framebuf[y][x] = rgb;
    }

    pub fn save(self, framebuf: FrameBuffer , filename: &str) {
        let mut file = match File::create(filename) {
            Err(why) => panic!("couldn't create file: {}", why),
            Ok(file) => file
        };
        match file.write_all(format!("P6\n{} {}\n255\n", self.width, self.height).as_bytes()) {
            Err(why) => panic!("couldnt write file: {}", why),
            Ok(_) => println!("wrote")
        };
        for row in framebuf {
            for pxl in row {
                let byte_colors: Vec<u8> = pxl.to_vec().iter().map(|c| (c * 255.) as u8).collect();
                match file.write_all(&byte_colors) {
                    Err(why) => panic!("no writey {}", why),
                    Ok(_) => ()
                };
            }
        }
        println!("rendered {}.", filename);
    }
}

