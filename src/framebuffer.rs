use nalgebra::Vector3;
use image::{DynamicImage, GenericImageView, ImageReader, Rgb};

use crate::{rgb_to_u32, textures::{AnimatedTexture, Texture}};

pub struct Framebuffer {
    pub width: usize,
    pub height: usize,
    pub buffer: Vec<u32>,
    background_color: u32,
    current_color: u32,
}

impl Framebuffer {
    pub fn new(width: usize, height: usize) -> Framebuffer {
        let buffer_size = width * height;
        let buffer = vec![0; buffer_size]; // Inicializa el buffer con 0 (representa color negro)
        Framebuffer {
            width,
            height,
            buffer,
            background_color: 0xFF000000, // Fondo negro transparente (RGBA: 255, 0, 0, 0)
            current_color: 0xFFFFFFFF,     // Color blanco opaco (RGBA: 255, 255, 255, 255)
        }
    }

    pub fn draw_text(&mut self, text: &str, x: usize, y: usize, fontsize: usize) {
        let font = [
        vec![0b01110, 0b10001, 0b11111, 0b10001, 0b10001], // A
        vec![0b11110, 0b10001, 0b11110, 0b10001, 0b11110], // B
        vec![0b01111, 0b10000, 0b10000, 0b10000, 0b01111], // C
        vec![0b11110, 0b10001, 0b10001, 0b10001, 0b11110], // D
        vec![0b11111, 0b10000, 0b11110, 0b10000, 0b11111], // E
        vec![0b11111, 0b10000, 0b11110, 0b10000, 0b10000], // F
        vec![0b01111, 0b10000, 0b10011, 0b10001, 0b01110], // G
        vec![0b10001, 0b10001, 0b11111, 0b10001, 0b10001], // H
        vec![0b11111, 0b00100, 0b00100, 0b00100, 0b11111], // I
        vec![0b00001, 0b00001, 0b00001, 0b10001, 0b01110], // J
        vec![0b10001, 0b10010, 0b11100, 0b10010, 0b10001], // K
        vec![0b10000, 0b10000, 0b10000, 0b10000, 0b11111], // L
        vec![0b10001, 0b11011, 0b10101, 0b10001, 0b10001], // M
        vec![0b10001, 0b11001, 0b10101, 0b10011, 0b10001], // N
        vec![0b01110, 0b10001, 0b10001, 0b10001, 0b01110], // O
        vec![0b11110, 0b10001, 0b11110, 0b10000, 0b10000], // P
        vec![0b01110, 0b10001, 0b10001, 0b10011, 0b01111], // Q
        vec![0b11110, 0b10001, 0b11110, 0b10010, 0b10001], // R
        vec![0b01111, 0b10000, 0b01110, 0b00001, 0b11110], // S
        vec![0b11111, 0b00100, 0b00100, 0b00100, 0b00100], // T
        vec![0b10001, 0b10001, 0b10001, 0b10001, 0b01110], // U
        vec![0b10001, 0b10001, 0b10001, 0b01010, 0b00100], // V
        vec![0b10001, 0b10001, 0b10001, 0b10101, 0b01010], // W
        vec![0b10001, 0b01010, 0b00100, 0b01010, 0b10001], // X
        vec![0b10001, 0b01010, 0b00100, 0b00100, 0b00100], // Y
        vec![0b11111, 0b00001, 0b01111, 0b10000, 0b11111], // Z
        vec![0b01110, 0b10001, 0b10001, 0b10001, 0b01110], // 0
        vec![0b00100, 0b01100, 0b00100, 0b00100, 0b11111], // 1
        vec![0b11111, 0b00001, 0b11111, 0b10000, 0b11111], // 2
        vec![0b11111, 0b00001, 0b01110, 0b00001, 0b11111], // 3
        vec![0b10001, 0b10001, 0b11111, 0b00001, 0b00001], // 4
        vec![0b11111, 0b10000, 0b11111, 0b00001, 0b11111], // 5
        vec![0b11111, 0b10000, 0b11111, 0b10001, 0b11111], // 6
        vec![0b11111, 0b00001, 0b00001, 0b00001, 0b00001], // 7
        vec![0b11111, 0b10001, 0b11111, 0b10001, 0b11111], // 8
        vec![0b11111, 0b10001, 0b11111, 0b00001, 0b11111], // 9
        vec![0b00000, 0b00100, 0b00000, 0b00100, 0b00000], // :
        vec![0b00000, 0b00000, 0b00000, 0b00000, 0b00000], // Espacio
        ];

    
        let scale = fontsize;  // Factor de escala para hacer el texto más grande
        let mut x_offset = x;
    
        for character in text.chars() {
            let idx = match character {
                'A'..='Z' => (character as usize) - ('A' as usize),
                '0'..='9' => (character as usize) - ('0' as usize) + 26,
                ':' => 36,
                ' ' => 37,
                _ => 37, // Espacio en blanco para caracteres no definidos
            };
    
            let pattern = &font[idx];
    
            for (i, row) in pattern.iter().enumerate() {
                for j in 0..5 {  // 5 es el ancho original del carácter
                    if row & (1 << (4 - j)) != 0 {  // Dibujar solo los bits encendidos
                        for dy in 0..scale {
                            for dx in 0..scale {
                                self.point(x_offset + j * scale + dx, y + i * scale + dy, 0xFFFFFFFF);
                            }
                        }
                    }
                }
            }
    
            x_offset += 5 * scale + scale;  // Mover la posición de inicio del siguiente carácter
        }
    }
    
    

    pub fn draw_image(&mut self, texture: &Texture, window_width: usize, window_height: usize) {
        let scale_x = texture.width as f32 / window_width as f32;
        let scale_y = texture.height as f32 / window_height as f32;

        for y in 0..window_height {
            for x in 0..window_width {
                let texture_x = (x as f32 * scale_x) as u32;
                let texture_y = (y as f32 * scale_y) as u32;

                if texture_x < texture.width && texture_y < texture.height {
                    let color = texture.get_pixel_color(texture_x, texture_y);
                    let color_u32 = rgb_to_u32(color);
                    self.point(x, y, color_u32);
                }
            }
        }
    }

    pub fn clear(&mut self) {
        for pixel in &mut self.buffer {
            *pixel = self.background_color;
        }
    }

    pub fn point(&mut self, x: usize, y: usize, color: u32) {
        if x < self.width && y < self.height {
            let index = y * self.width + x;
            self.buffer[index] = color;
        }
    }

    pub fn point_vertex(&mut self, vertex: Vector3<f32>, color: u32) {
        let x = vertex.x.round() as usize;
        let y = vertex.y.round() as usize;
        self.point(x, y, color);
    }

    pub fn set_background_color(&mut self, color: u32) {
        self.background_color = color;
    }

    pub fn set_current_color(&mut self, color: u32) {
        self.current_color = color;
    }

    pub fn flip_vertical(&mut self) {
        let mut temp_buffer = self.buffer.clone(); // Clonamos el buffer actual

        for y in 0..self.height {
            for x in 0..self.width {
                let index = y * self.width + x;
                let flipped_index = ((self.height - 1 - y) * self.width + x);
                self.buffer[index] = temp_buffer[flipped_index];
            }
        }
    }

    pub fn fill_rect(&mut self, x: usize, y: usize, width: usize, height: usize, color: u32) {
        for dy in 0..height {
            let y_offset = y + dy;
            if y_offset >= self.height {
                break; // Evitar dibujar fuera de los límites
            }

            for dx in 0..width {
                let x_offset = x + dx;
                if x_offset >= self.width {
                    break; // Evitar dibujar fuera de los límites
                }

                self.buffer[y_offset * self.width + x_offset] = color;
            }
        }
    }

    pub fn get_pixel(&self, x: usize, y: usize) -> u32 {
        if x < self.width && y < self.height {
            self.buffer[y * self.width + x]
        } else {
            // Devuelve un color predeterminado en caso de estar fuera de los límites
            0x00000000 // Color negro transparente (RGBA: 0, 0, 0, 0)
        }
    }

    pub fn draw_animated_image(&mut self, animated_texture: &AnimatedTexture, frame_index: usize, window_width: usize, window_height: usize) {
        // Ensure frame_index is within the bounds
        let frame_index = frame_index % animated_texture.frame_count;
    
        let scale_x = animated_texture.width as f32 / window_width as f32;
        let scale_y = animated_texture.height as f32 / window_height as f32;
    
        for y in 0..window_height {
            for x in 0..window_width {
                let texture_x = (x as f32 * scale_x) as u32;
                let texture_y = (y as f32 * scale_y) as u32;
    
                if texture_x < animated_texture.width && texture_y < animated_texture.height {
                    let color = animated_texture.get_pixel_color(frame_index, texture_x, texture_y);
                    let color_u32 = rgb_to_u32(color);
                    self.point(x, y, color_u32);
                }
            }
        }
    }
    
}
