use std::{fs::File, io::BufReader};

use image::{
    GenericImageView, ImageDecoder, ImageReader,
    Pixel, Rgb,
};

pub struct GameTextures {
    pub wall: Texture,
    pub cupcake: Texture,
    pub start1: Texture,
    pub start2: Texture
    
}

impl GameTextures {
    pub fn new(asset_dir: &str) -> Self {
        let wall = format!("{}{}", asset_dir, "wall1.png");
        let cupcake = format!("{}{}", asset_dir, "cupcake.png");
        let start1 = format!("{}{}", asset_dir, "start1.png");
        let start2 = format!("{}{}", asset_dir, "start2.png");

        let wall = Texture::new(&wall);
        let cupcake = Texture::new(&cupcake);
        let start1 = Texture::new(&start1);
        let start2 = Texture::new(&start2);

        GameTextures {
            wall,
            cupcake,
            start1,
            start2,
            
        }
    }
}

pub struct Texture {
    pub width: u32,
    pub height: u32,
    colors: Vec<Rgb<u8>>,
}


impl Texture {
    pub fn new(file_path: &str) -> Self {
        let image = ImageReader::open(file_path).unwrap().decode().unwrap();
        let width = image.width();
        let height = image.height();

        let size = (width * height) as usize;
        let mut colors = Vec::with_capacity(size);

        for y in 0..height {
            for x in 0..width {
                let pixel = image.get_pixel(x, y).to_rgb();
                colors.push(pixel);
            }
        }

        Texture {
            width,
            height,
            colors,
        }
    }

    pub fn get_pixel_color(&self, x: u32, y: u32) -> Rgb<u8> {
        let idx = (y * self.width + x) as usize;
        self.colors[idx]
    }
}
