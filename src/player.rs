use nalgebra_glm::Vec2;

pub struct Player {
    pub pos: Vec2,
    pub a: f32, // Ángulo en radianes
    pub fov: f32,
}

impl Player {
    fn check_collision(&self, new_pos: Vec2, maze: &Vec<Vec<char>>, block_size: usize) -> bool {
        let x = (new_pos.x / block_size as f32).floor() as usize;
        let y = (new_pos.y / block_size as f32).floor() as usize;
        if let Some(row) = maze.get(y) {
            if let Some(&cell) = row.get(x) {
                return cell != ' '; // True si hay colisión (la celda no está vacía)
            }
        }
        false // No hay colisión si está fuera del laberinto o es un espacio vacío
    }

    pub fn move_forward(&mut self, distance: f32, maze: &Vec<Vec<char>>, block_size: usize) {
        let new_pos = self.pos + Vec2::new(self.a.cos(), self.a.sin()) * distance;
        if !self.check_collision(new_pos, maze, block_size) {
            self.pos = new_pos;
        } else {
            println!("Collision detected, cannot move forward.");
        }
    }

    pub fn move_backward(&mut self, distance: f32, maze: &Vec<Vec<char>>, block_size: usize) {
        let new_pos = self.pos - Vec2::new(self.a.cos(), self.a.sin()) * distance;
        if !self.check_collision(new_pos, maze, block_size) {
            self.pos = new_pos;
        } else {
            println!("Collision detected, cannot move backward.");
        }
    }

    pub fn rotate(&mut self, angle: f32) {
        self.a += angle;
    }
}


