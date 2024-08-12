use std::{fs::File, io::BufReader};

use image::{
    GenericImageView, ImageDecoder, ImageReader,
    Pixel, Rgb,
};

pub struct GameTextures {
    pub wall: Texture,
    pub cupcake: Texture,
    
}

impl GameTextures {
    pub fn new(asset_dir: &str) -> Self {
        let wall = format!("{}{}", asset_dir, "wall1.png");
        let cupcake = format!("{}{}", asset_dir, "cupcake.png");

        let wall = Texture::new(&wall);
        let cupcake = Texture::new(&cupcake);

        GameTextures {
            wall,
            cupcake,
            
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
