use nalgebra::Vector3;
use image::GenericImageView;

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
        let font = vec![
            // N
            vec![
                0b10001,
                0b11001,
                0b10101,
                0b10011,
                0b10001,
            ],
            // I
            vec![
                0b11111,
                0b00100,
                0b00100,
                0b00100,
                0b11111,
            ],
            // G
            vec![
                0b01110,
                0b10001,
                0b10000,
                0b10011,
                0b01111,
            ],
            // H
            vec![
                0b10001,
                0b11111,
                0b10001,
                0b10001,
                0b10001,
            ],
            // T
            vec![
                0b11111,
                0b00100,
                0b00100,
                0b00100,
                0b00100,
            ],
            // " "
            vec![
                0b00000,
                0b00000,
                0b00000,
                0b00000,
                0b00000,
            ],
            // 0
            vec![
                0b01110,
                0b10001,
                0b10001,
                0b10001,
                0b01110,
            ],
            // 1
            vec![
                0b00100,
                0b01100,
                0b00100,
                0b00100,
                0b11111,
            ],
            // 2
            vec![
                0b11111,
                0b00001,
                0b11111,
                0b10000,
                0b11111,
            ],
            // 3
            vec![
                0b11111,
                0b00001,
                0b01110,
                0b00001,
                0b11111,
            ],
            // 4
            vec![
                0b10001,
                0b10001,
                0b11111,
                0b00001,
                0b00001,
            ],
            // 5
            vec![
                0b11111,
                0b10000,
                0b11111,
                0b00001,
                0b11111,
            ],
            // 6
            vec![
                0b11111,
                0b10000,
                0b11111,
                0b10001,
                0b11111,
            ],
            // 7
            vec![
                0b11111,
                0b00001,
                0b00001,
                0b00001,
                0b00001,
            ],
            // 8
            vec![
                0b11111,
                0b10001,
                0b11111,
                0b10001,
                0b11111,
            ],
            // 9
            vec![
                0b11111,
                0b10001,
                0b11111,
                0b00001,
                0b11111,
            ],
            // F
            vec![
                0b11111,
                0b10000,
                0b11110,
                0b10000,
                0b10000,
            ],
            // P
            vec![
                0b11110,
                0b10001,
                0b11110,
                0b10000,
                0b10000,
            ],
            // S
            vec![
                0b01111,
                0b10000,
                0b01110,
                0b00001,
                0b11110,
            ],
            // :
            vec![
                0b00000,
                0b00100,
                0b00000,
                0b00100,
                0b00000,
            ],
        ];

    
        let scale = fontsize;  // Factor de escala para hacer el texto más grande
        let mut x_offset = x;
    
        for character in text.chars() {
            let idx = match character {
                'N' => 0,
                'i' => 1,
                'g' => 2,
                'h' => 3,
                't' => 4,
                ' ' => 5,
                '0' => 6,
                '1' => 7,
                '2' => 8,
                '3' => 9,
                '4' => 10,
                '5' => 11,
                '6' => 12,
                '7' => 13,
                '8' => 14,
                '9' => 15,
                'F' => 16,
                'P' => 17,
                'S' => 18,
                ':' => 19,
                _ => 5, // Espacio en blanco para caracteres no definidos
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
    
    

    pub fn draw_image(&mut self, image_path: &str, window_width: usize, window_height: usize) {
        let img = image::open(image_path).expect("Failed to open image");

        // Redimensionar la imagen a las dimensiones de la ventana
        let resized_img = img.resize(window_width as u32, window_height as u32, image::imageops::FilterType::Lanczos3);

        for y in 0..window_height {
            for x in 0..window_width {
                if x < resized_img.width() as usize && y < resized_img.height() as usize {
                    let pixel = resized_img.get_pixel(x as u32, y as u32);
                    let rgba = pixel.0;

                    let color = (rgba[0] as u32) << 16  // Red
                              | (rgba[1] as u32) << 8   // Green
                              | (rgba[2] as u32)        // Blue
                              | (rgba[3] as u32) << 24; // Alpha
                    self.point(x, y, color);
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
}
