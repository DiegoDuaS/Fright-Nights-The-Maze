use crate::framebuffer::Framebuffer;
use crate::player::Player;

pub struct Intersect{
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
    let step_size = 1.0; // Tamaño del paso del rayo en unidades de píxeles

    framebuffer.set_current_color(0xFFDDDD); // Color del rayo

    loop {
        let cos = d * angle.cos();
        let sin = d * angle.sin();
        let x = (player.pos.x + cos) as usize;
        let y = (player.pos.y + sin) as usize;

        let i = x / block_size;
        let j = y / block_size;

        let tx = x - i * block_size;

        if maze.get(j).and_then(|row| row.get(i)).map_or(false, |&cell| cell != ' ') {
            return Intersect {
                distance: d,
                impact: maze[j][i],
                tx: tx,
            };
        }

        if draw_line {
            framebuffer.point(x, y, 0xFF800080);
        }

        d += step_size;
    }
}
