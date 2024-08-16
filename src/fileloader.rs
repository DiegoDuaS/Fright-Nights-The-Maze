use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub fn load_maze(filename: &str) -> Result<(Vec<Vec<char>>, Option<(usize, usize)>, Vec<(usize, usize)>), io::Error> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let mut maze = Vec::new();
    let mut player_pos = None;
    let mut goals = Vec::new(); // Usar solo objetivos ('G')

    for (row_index, line) in reader.lines().enumerate() {
        let line = line?;
        let mut row = Vec::new();
        for (col_index, c) in line.chars().enumerate() {
            match c {
                'p' => {
                    player_pos = Some((row_index, col_index));
                    row.push(' '); // Reemplazar el carácter del jugador con un espacio para el renderizado
                }
                'G' => {
                    goals.push((row_index, col_index)); // Almacenar las posiciones de los objetivos
                    row.push(' '); // Reemplazar el carácter del objetivo con un espacio para el renderizado
                }
                _ => row.push(c),
            }
        }
        maze.push(row);
    }

    Ok((maze, player_pos, goals)) // Retornar solo los objetivos
}