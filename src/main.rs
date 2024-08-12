use std::fs::File;
use std::io::{self, BufRead, BufReader};
use image::Rgb;
use minifb::{Key, Window, WindowOptions};
use std::time::{Duration, Instant};
use nalgebra_glm::Vec2;

mod framebuffer;
use framebuffer::Framebuffer;

mod player;
use player::Player;

mod castray;
use castray::cast_ray;

mod textures;
use textures::GameTextures;
use textures::Texture;

fn main() {
    let window_width = 900;
    let window_height = 550;
    let mut framebuffer = Framebuffer::new(window_width, window_height);
    let mut state = "main1";
    let asset_dir = "assets/";
    let textures = GameTextures::new(asset_dir);

    framebuffer.clear();

    let mut window = Window::new(
        "Fright Nights: The Maze",
        window_width,
        window_height,
        WindowOptions::default(),
    ).unwrap();

    let mut current_screen = 0;
    let mut last_switch = Instant::now();
    let mut last_frame_time = Instant::now();

    // LABERINTO 1
    let maze_result1 = load_maze("./night1.txt");
    let (maze1, player_pos1, cupcakes1) = match maze_result1 {
        Ok(m) => m,
        Err(e) => {
            println!("Error loading maze: {}", e);
            return;
        }
    };

    let initial_pos1 = match player_pos1 {
        Some((row, col)) => Vec2::new((col * 100) as f32, (row * 100) as f32),
        None => Vec2::new(100.0, 100.0),
    };

    let mut player1 = Player {
        pos: initial_pos1,
        a: 0.0,
        fov: std::f32::consts::PI / 3.0,
    };

    let block_size1: usize = window_width.min(window_height) / maze1.len().max(1);

    while window.is_open() {
        if window.is_key_down(Key::Escape) {
            break;
        }

        if state == "main1" {
            let now = Instant::now();
            if now.duration_since(last_switch) >= Duration::new(3, 0) {
                current_screen = (current_screen + 1) % 2;
                startpage(&mut framebuffer, current_screen, window_width, window_height);
                last_switch = now;
            }
            if window.is_key_down(Key::Enter) {
                state = "night1start";
                last_switch = now;
            }
        }

        if state == "night1start" {
            framebuffer.clear();
            framebuffer.draw_text("Night 1", window_width / 2 - 150, window_height / 2 - 50, 8);
            window.update_with_buffer(&framebuffer.buffer, window_width, window_height).unwrap();

            let now = Instant::now();
            if now.duration_since(last_switch) >= Duration::new(3, 0) {
                state = "night1";
                last_switch = now;
            }
        }

        if state == "night1" {
            framebuffer.clear();

            if window.is_key_down(Key::W) {
                player1.move_forward(5.0, &maze1, block_size1);
            }
            if window.is_key_down(Key::S) {
                player1.move_backward(3.0, &maze1, block_size1);
            }
            if window.is_key_down(Key::A) {
                player1.rotate(-0.1);
            }
            if window.is_key_down(Key::D) {
                player1.rotate(0.1);
            }

            render3d(&mut framebuffer, &player1, window_width, window_height, "./night1.txt", &cupcakes1, &textures);
            let now = Instant::now();
            let frame_duration = now.duration_since(last_frame_time);
            let fps = 1.0 / frame_duration.as_secs_f32();
            last_frame_time = now;

            framebuffer.draw_text(&format!("FPS: {:.0}", fps), 10, 10, 5);
        }

        if state == "night2start" {
            continue;
        }

        if state == "night2" {
            // Lógica para la segunda noche
        }

        if state == "night3start" {
            continue;
        }

        if state == "night3" {
            // Lógica para la tercera noche
        }

        window.update_with_buffer(&framebuffer.buffer, window_width, window_height).unwrap();
        std::thread::sleep(std::time::Duration::from_millis(1));
    }
}



fn startpage(framebuffer: &mut Framebuffer, screen: usize, window_width: usize, window_height: usize) {
    framebuffer.clear();

    match screen {
        0 => {
            framebuffer.draw_image("assets/start1.png", window_width, window_height);
        }
        1 => {
            framebuffer.draw_image("assets/start2.png", window_width, window_height);
        }
        _ => {}
    }
}

pub fn load_maze(filename: &str) -> Result<(Vec<Vec<char>>, Option<(usize, usize)>, Vec<(usize, usize)>), io::Error> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let mut maze = Vec::new();
    let mut player_pos = None;
    let mut cupcakes = Vec::new();

    for (row_index, line) in reader.lines().enumerate() {
        let line = line?;
        let mut row = Vec::new();
        for (col_index, c) in line.chars().enumerate() {
            match c {
                'p' => {
                    player_pos = Some((row_index, col_index));
                    row.push(' '); // Reemplazar el carácter del jugador con un espacio para el renderizado
                }
                'C' => {
                    cupcakes.push((row_index, col_index));
                    row.push(' '); // Reemplazar el carácter del cupcake con un espacio para el renderizado
                }
                _ => row.push(c),
            }
        }
        maze.push(row);
    }

    Ok((maze, player_pos, cupcakes))
}


fn render3d(
    framebuffer: &mut Framebuffer,
    player: &Player,
    window_width: usize,
    window_height: usize,
    filename: &str,
    cupcakes: &[(usize, usize)],
    textures: &GameTextures,
) {
    let maze_result = load_maze(filename);
    let (maze, _player_pos, _cupcakes) = match maze_result {
        Ok(m) => m,
        Err(e) => {
            println!("Error loading maze: {}", e);
            return;
        }
    };

    let hw = framebuffer.width as f32 / 2.0;
    let hh = framebuffer.height as f32 / 2.0;

    // Renderizar el techo con gris oscuro
    framebuffer.fill_rect(0, 0, framebuffer.width, hh as usize, 0xFF171A1D);

    // Renderizar el suelo con morado más oscuro
    framebuffer.fill_rect(0, hh as usize, framebuffer.width, (framebuffer.height - hh as usize), 0x2E0854);

    let block_size = window_width.min(window_height) / maze.len().max(1);
    let step = 2; // Lanza un rayo cada 2 píxeles
    let num_rays = framebuffer.width / step;

    let texture = &textures.wall;
    let mut z_buffer = vec![f32::INFINITY; framebuffer.width as usize];

    for i in 0..num_rays {
        let current_ray = i as f32 / num_rays as f32;
        let a = player.a - (player.fov / 2.0) + (player.fov * current_ray);
        let intersect = cast_ray(framebuffer, &maze, player, a, block_size, false);

        let distance_to_wall = intersect.distance;
        let distance_to_projection_plane = hw / (player.fov / 2.0).tan();
        let wall_height = (block_size as f32 / distance_to_wall) * distance_to_projection_plane;

        let wall_top = hh - wall_height / 2.0;
        let wall_bottom = hh + wall_height / 2.0;

        let texture_width = texture.width as f32;
        let texture_height = texture.height as f32;

        for y in wall_top as usize..wall_bottom as usize {
            if y >= framebuffer.height {
                continue;
            }

            let texture_y = ((y as f32 - wall_top) / wall_height * texture_height) as u32;
            let texture_x = (intersect.tx as f32 / block_size as f32 * texture_width) as u32;

            let color = texture.get_pixel_color(texture_x, texture_y);
            let color_u32 = rgb_to_u32(color);

            if distance_to_wall < z_buffer[i * step as usize] {
                // Rellenar las columnas intermedias
                framebuffer.fill_rect(i * step as usize, y, step, 1, color_u32);
            }
        }

        z_buffer[i * step as usize] = distance_to_wall;
    }
}


fn rgb_to_u32(rgb: Rgb<u8>) -> u32 {
    let r = rgb[0] as u32;
    let g = rgb[1] as u32;
    let b = rgb[2] as u32;
    0xFF000000 | (r << 16) | (g << 8) | b
}

