use std::fs::File;
use std::io::{self, BufRead, BufReader};
use minifb::{Key, Window, WindowOptions};
use std::time::{Duration, Instant};
use nalgebra_glm::Vec2;

mod framebuffer;
use framebuffer::Framebuffer;

mod player;
use player::Player;

mod castray;
use castray::cast_ray;

fn main() {
    let window_width = 1200;
    let window_height = 700;
    let mut framebuffer = Framebuffer::new(window_width, window_height);
    let mut state = "main1";

    framebuffer.clear();

    let mut window = Window::new(
        "Fright Nights: The Maze",
        window_width,
        window_height,
        WindowOptions::default(),
    ).unwrap();

    let mut current_screen = 0;  // Comenzar con la primera pantalla
    let mut last_switch = Instant::now();

    //LABERINTO 1
    let maze_result1 = load_maze("./night1.txt");
        let (maze1, player_pos1) = match maze_result1 {
            Ok(m) => m,
            Err(e) => {
                println!("Error loading maze: {}", e);
                return;
            }
        };

    let initial_pos1 = match player_pos1 {
        Some((row, col)) => Vec2::new((col * 100) as f32, (row * 100) as f32),
            None => Vec2::new(100.0, 100.0), // Posición por defecto si no se encuentra el jugador en el archivo
    };

    let mut player1 = Player {
        pos: initial_pos1,
        a: 0.0, // Ángulo inicial
        fov: std::f32::consts::PI / 3.0,
    };

    let block_size1: usize = window_width.min(window_height) / maze1.len().max(1);

    //LABERINTO 2

    //LABERINTO 3

    while window.is_open() {
        if window.is_key_down(Key::Escape) {
            break;
        }

        if state == "main1" {
            let now = Instant::now();
            if now.duration_since(last_switch) >= Duration::new(3, 0) {
                // Alternar pantalla cada 3 segundos
                current_screen = (current_screen + 1) % 2;  // Alterna entre 0 y 1
                startpage(&mut framebuffer, current_screen, window_width, window_height);
                last_switch = now;
            }
            if window.is_key_down(Key::Enter) {
                state = "night1start";
                last_switch = now;
            }
        }

        if state == "night1start" {
            // Mostrar "Night 1" durante 3 segundos
            framebuffer.clear();
            framebuffer.draw_text("Night 1", window_width / 2  - 150 , window_height / 2 - 50 );
            window.update_with_buffer(&framebuffer.buffer, window_width, window_height).unwrap();
            
            let now = Instant::now();
            if now.duration_since(last_switch) >= Duration::new(3, 0) {
                state = "night1";
                last_switch = now; // Actualizar el tiempo de referencia
            }
        }

        if state == "night1" {
            framebuffer.clear();
            
            if window.is_key_down(Key::W) {
                player1.move_forward(5.0, &maze1, block_size1); 
            }
            if window.is_key_down(Key::S) {
                player1.move_backward(5.0, &maze1, block_size1); 
            }
            if window.is_key_down(Key::A) {
                player1.rotate(-0.1); 
            }
            if window.is_key_down(Key::D) {
                player1.rotate(0.1); 
            }

            render3d(&mut framebuffer, &player1, window_width, window_height, "./night1.txt");


        }

        if state == "night2start" {
            // Similar lógica para la segunda noche
            continue;
        }

        if state == "night2" {
            
        }

        if state == "night3start" {
            // Similar lógica para la tercera noche
            continue;
        }

        if state == "night3" {
        }

        window
            .update_with_buffer(&framebuffer.buffer, window_width, window_height)
            .unwrap();

        std::thread::sleep(std::time::Duration::from_millis(16));
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

pub fn load_maze(filename: &str) -> Result<(Vec<Vec<char>>, Option<(usize, usize)>), io::Error> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let mut maze = Vec::new();
    let mut player_pos = None;

    for (row_index, line) in reader.lines().enumerate() {
        let line = line?;
        let mut row = Vec::new();
        for (col_index, c) in line.chars().enumerate() {
            if c == 'p' {
                player_pos = Some((row_index, col_index));
                row.push(' '); // Reemplazar el carácter del jugador con un espacio para el renderizado
            } else {
                row.push(c);
            }
        }
        maze.push(row);
    }

    Ok((maze, player_pos))
}

fn render3d(
    framebuffer: &mut Framebuffer,
    player: &Player,
    window_width: usize,
    window_height: usize,
    filename: &str,
) {
    let maze_result = load_maze(filename);
    let (maze, _player_pos) = match maze_result {
        Ok(m) => m,
        Err(e) => {
            println!("Error loading maze: {}", e);
            return;
        }
    };

    let block_size = window_width.min(window_height) / maze.len().max(1);
    let num_rays = framebuffer.width;

    let hw = framebuffer.width as f32 / 2.0;
    let hh = framebuffer.height as f32 / 2.0;

    for i in 0..num_rays {
        let current_ray = i as f32 / num_rays as f32;
        let a = player.a - (player.fov / 2.0) + (player.fov * current_ray);
        let intersect = cast_ray(framebuffer, &maze, player, a, block_size, false);

        let distance_to_wall = intersect.distance;
        let distance_to_projection_plane = hw / (player.fov / 2.0).tan();
        let wall_height = (block_size as f32 / distance_to_wall) * distance_to_projection_plane;

        let wall_top = hh - wall_height / 2.0;
        let wall_bottom = hh + wall_height / 2.0;

        // Coordenada x en la textura
        

        for y in wall_top as usize..wall_bottom as usize {
            // Coordenada y en la textura

            framebuffer.point(i, y, 0xFF808080);
        }
    }
}


