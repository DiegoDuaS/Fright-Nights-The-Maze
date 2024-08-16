use image::Rgb;

use crate::{castray::{cast_ray, cast_ray_minimap}, framebuffer::Framebuffer, player::Player, textures::GameTextures};

fn cell_to_texture_color(cell: char, tx: u32, ty: u32, textures: &GameTextures) -> u32 {
    let default_color = 0x000000;

    let color = match cell {
        '+' | '-' | '|' => textures.wall.get_pixel_color(tx, ty),
        'G' => textures.exitwall.get_pixel_color(tx, ty),
        'g' => textures.exitwall.get_pixel_color(tx, ty),
        _ => return default_color,
    };

    rgb_to_u32(color)
}


pub fn render3d(
    framebuffer: &mut Framebuffer,
    player: &Player,
    window_width: usize,
    window_height: usize,
    maze: &Vec<Vec<char>>,
    textures: &GameTextures,
) {
    let hw = framebuffer.width as f32 / 2.0;
    let hh = framebuffer.height as f32 / 2.0;

    // Renderizar el techo con gris oscuro
    framebuffer.fill_rect(0, 0, framebuffer.width, hh as usize, 0xFF171A1D);

    // Renderizar el suelo con morado más oscuro
    framebuffer.fill_rect(0, hh as usize, framebuffer.width, framebuffer.height - hh as usize, 0x2E0854);

    let block_size = window_width.min(window_height) / maze.len().max(1);
    let step = 2; // Lanza un rayo cada 2 píxeles
    let num_rays = framebuffer.width / step;

    let mut z_buffer = vec![f32::INFINITY; framebuffer.width as usize];

    // Renderizar paredes
    for i in 0..num_rays {
        let current_ray = i as f32 / num_rays as f32;
        let a = player.a - (player.fov / 2.0) + (player.fov * current_ray);
        let intersect = cast_ray(framebuffer, &maze, player, a, block_size, false); // Pasamos el goal como un Vec

        let distance_to_wall = intersect.distance;
        let distance_to_projection_plane = hw / (player.fov / 2.0).tan();
        let wall_height = (block_size as f32 / distance_to_wall) * distance_to_projection_plane;

        let wall_top = hh - wall_height / 2.0;
        let wall_bottom = hh + wall_height / 2.0;

        let texture_width = textures.wall.width as f32;
        let texture_height = textures.wall.height as f32;

        for y in wall_top as usize..wall_bottom as usize {
            if y >= framebuffer.height {
                continue;
            }

            let texture_y = ((y as f32 - wall_top) / wall_height * texture_height) as u32;
            let texture_x = (intersect.tx as f32 / block_size as f32 * texture_width) as u32;

            // Usa `cell_to_texture_color` para obtener el color
            let color = cell_to_texture_color(intersect.impact, texture_x, texture_y, textures);
            let color_u32 = color; // color ya es un `u32` ahora

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

fn draw_cell(framebuffer: &mut Framebuffer, x0: usize, y0: usize, block_size: usize, cell: char) {
    // Seleccionar un color según la celda
    let color = match cell {
        '#' => 0xFFFFFFFF, // Pared (negro)
        ' ' => 0x2E0854, // Camino (morado)
        'p' => 0xFF000000 , // Jugador (blanco)
        'g' => 0xFF00FF00, // Meta (verde)
        _ => 0xFFFFFFFF,   // Otros (rojo)
    };

    // Dibujar el rectángulo
    for y in y0..(y0 + block_size) {
        for x in x0..(x0 + block_size) {
            framebuffer.point(x, y, color);
        }
    }
}

pub fn render_minimap(framebuffer: &mut Framebuffer, player: &Player, maze: &Vec<Vec<char>>, minimap_x: usize, minimap_y: usize, minimap_scale: f32,window_width: usize,
    window_height: usize ) {
    let block_size = ((window_width.min(window_height) / maze.len().max(1)) as f32 * minimap_scale) as usize;

    for row in 0..maze.len() {
        for col in 0..maze[row].len() {
            let cell = maze[row][col];
            let xo = minimap_x + col * block_size;
            let yo = minimap_y + row * block_size;
            draw_cell(framebuffer, xo, yo, block_size, cell);
        }
    }

    let player_x = minimap_x + (player.pos.x as f32 * minimap_scale) as usize;
    let player_y = minimap_y + (player.pos.y as f32 * minimap_scale) as usize;
    framebuffer.point(player_x, player_y, 0xFF0000);

    let num_rays = 50;
    for i in 0..num_rays {
        let current_ray = i as f32 / num_rays as f32;
        let angle = player.a - (player.fov / 2.0) + (player.fov * current_ray);
        cast_ray_minimap(framebuffer, &maze, player, angle, block_size, minimap_x, minimap_y, minimap_scale);
    }
}



