use crate::player::Player;
use crate::Window;
use minifb::{Key, MouseMode};

pub fn process_event(player: &mut Player, window: &Window, maze: &Vec<Vec<char>>, block_size: usize){
    const MOVE_SPEED: f32 = 5.0;
    const ROTATION_SPEED: f32 = 0.2;
    const DEAD_ZONE: f32 = 250.0;

    // Movimiento con teclado
    if window.is_key_down(Key::W) {
        player.move_forward(MOVE_SPEED, &maze, block_size);
    }
    if window.is_key_down(Key::S) {
        player.move_backward(MOVE_SPEED, &maze, block_size);
    }
    if window.is_key_down(Key::A) {
        player.rotate(-ROTATION_SPEED);
    }
    if window.is_key_down(Key::D) {
        player.rotate(ROTATION_SPEED);
    }

    // Rotación con el mouse
    if let Some((mouse_x, _)) = window.get_mouse_pos(minifb::MouseMode::Discard) {
        let center_x = window.get_size().0 as f32 / 2.0;
        let left_dead_zone = center_x - DEAD_ZONE / 2.0;
        let right_dead_zone = center_x + DEAD_ZONE / 2.0;

        if mouse_x < left_dead_zone {
            let rotation_amount = ROTATION_SPEED * (left_dead_zone - mouse_x) / left_dead_zone;
            player.rotate(-rotation_amount);
        } else if mouse_x > right_dead_zone {
            let rotation_amount = ROTATION_SPEED * (mouse_x - right_dead_zone) / (window.get_size().0 as f32 - right_dead_zone);
            player.rotate(rotation_amount);
        }
    }
}