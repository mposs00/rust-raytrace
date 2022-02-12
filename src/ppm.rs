use std::fs::File;
use std::io::prelude::*;

pub type FrameBuffer = Vec<Vec<[f32; 3]>>;

#[derive(Clone)]
pub struct Frame {
    pub frame: FrameBuffer,
    pub width: usize,
    pub height: usize
}

impl Frame {
    pub fn new(width: usize, height: usize) -> Frame {
        Frame {
            frame: vec![vec![[0.5; 3]; width]; height],
            width: width,
            height: height
        }
    }

    pub fn set_pixel_rgb(&mut self, x: usize, y: usize, rgb: &Vec<f32>) {
        self.frame[y][x] = [rgb[0], rgb[1], rgb[2]];
    }

    pub fn save(self, filename: &str) {
        let mut file = match File::create(filename) {
            Err(why) => panic!("couldn't create file: {}", why),
            Ok(file) => file
        };
        match file.write_all(format!("P6\n{} {}\n255\n", self.width, self.height).as_bytes()) {
            Err(why) => panic!("couldnt write file: {}", why),
            Ok(_) => println!("wrote")
        };
        for row in self.frame {
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