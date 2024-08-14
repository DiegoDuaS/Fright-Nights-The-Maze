use crate::framebuffer::Framebuffer;
use crate::player::Player;

pub struct Intersect {
    pub distance: f32,
    pub impact: char,
    pub tx: usize,
}

pub fn cast_ray(
    framebuffer: &mut Framebuffer,
    maze: &Vec<Vec<char>>,
    player: &Player,
    angle: f32,
    block_size: usize,
    draw_line: bool,
) -> Intersect {
    let mut d = 0.0;
    let step_size = 1.0;

    framebuffer.set_current_color(0xFFDDDD);

    let cos_angle = angle.cos();
    let sin_angle = angle.sin();
    let player_x = player.pos.x;
    let player_y = player.pos.y;

    loop {
        let x = (player_x + d * cos_angle) as usize;
        let y = (player_y + d * sin_angle) as usize;

        let i = x / block_size;
        let j = y / block_size;

        let cell = maze.get(j).and_then(|row| row.get(i));

        if let Some(&cell_value) = cell {
            if cell_value != ' ' {
                let tx = x - i * block_size;
                let ty = y - j * block_size;

                let mut maxhit = ty;
                if 1 < tx && tx < block_size - 1 {
                    maxhit = tx;
                }

                let is_vertical = (angle % (std::f32::consts::PI / 2.0)).abs() < 0.01;

                return Intersect {
                    distance: d,
                    impact: cell_value,
                    tx: maxhit,
                };
            }
        }

        if draw_line {
            framebuffer.point(x, y, 0xFF800080);
        }

        d += step_size;
    }
}

pub fn cast_ray_minimap(framebuffer: &mut Framebuffer, maze: &Vec<Vec<char>>, player: &Player, angle: f32, block_size: usize, minimap_x: usize, minimap_y: usize, scale: f32) {
    let mut d = 0.0;
    let max_distance = 50.0; 

    while d < max_distance {
        let cos = d * angle.cos();
        let sin = d * angle.sin();

        let x = (player.pos.x + cos) as usize;
        let y = (player.pos.y + sin) as usize;

        // Ajusta las coordenadas para el minimapa
        let mx = minimap_x + (x as f32 * scale) as usize;
        let my = minimap_y + (y as f32 * scale) as usize;

        if mx >= framebuffer.width || my >= framebuffer.height {
            break; // Evita dibujar fuera de los límites del framebuffer
        }

        framebuffer.point(mx, my,0xFFFFFF);

        let i = x / block_size;
        let j = y / block_size;
        if maze.get(j).and_then(|row| row.get(i)) == Some(&'#') { // '#' representa un muro
            break;
        }
        d += 1.0;
    }
}
