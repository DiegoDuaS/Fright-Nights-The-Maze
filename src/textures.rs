use std::{fs::File, io::BufReader};

use image::{
    codecs::gif::GifDecoder, AnimationDecoder, Frame, GenericImageView, ImageDecoder, ImageReader,
    Pixel, Rgb,
};

pub struct GameTextures {
    pub wall: Texture,
    pub start1: Texture,
    pub start2: Texture,
    pub exitwall: Texture,
    pub chicaloss: AnimatedTexture
    
}

impl GameTextures {
    pub fn new(asset_dir: &str) -> Self {
        let wall = format!("{}{}", asset_dir, "wall1.png");
        let start1 = format!("{}{}", asset_dir, "start1.png");
        let start2 = format!("{}{}", asset_dir, "start2.png");
        let exitwall = format!("{}{}", asset_dir, "exit1.png");
        let chicaloss = format!("{}{}", asset_dir, "chica.gif");

        let wall = Texture::new(&wall);
        let start1 = Texture::new(&start1);
        let start2 = Texture::new(&start2);
        let exitwall = Texture::new(&exitwall);
        let chicaloss = AnimatedTexture::new(&chicaloss);

        GameTextures {
            wall,
            start1,
            start2,
            exitwall,
            chicaloss,
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
        if x >= self.width || y >= self.height {
            return Rgb([0, 0, 0]); // Retorna un color predeterminado si fuera de límites
        }
        let idx = (y * self.width + x) as usize;
        self.colors[idx]
    }
}

pub struct AnimatedTexture {
    pub width: u32,
    pub height: u32,
    frames: Vec<Frame>,
    pub frame_count: usize,
}

impl AnimatedTexture {
    pub fn new(file_path: &str) -> Self {
        let file_in = BufReader::new(File::open(file_path).unwrap());
        let decoder = GifDecoder::new(file_in).unwrap();
        let (width, height) = decoder.dimensions();
        let frames = decoder.into_frames();
        let frames = frames.collect_frames().expect("error decoding gif");
        let frame_count = frames.len();

        Self {
            width,
            height,
            frames,
            frame_count,
        }
    }

    /// Gets the color of the pixel positioned on the frame `t`.
    pub fn get_pixel_color(&self, t: usize, x: u32, y: u32) -> Rgb<u8> {
        let frame = &self.frames[t];
        let pixel = frame.buffer().get_pixel(x, y).to_rgb();
        pixel
    }
}




