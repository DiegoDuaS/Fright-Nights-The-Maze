use audioplayer::AudioPlayer;
use image::Rgb;
use minifb::{Key, Window, WindowOptions};
use std::time::{Duration, Instant};
use nalgebra_glm::Vec2;

mod framebuffer;
use framebuffer::Framebuffer;

mod player;
use player::Player;

mod castray;

mod textures;
use textures::GameTextures;

mod fileloader;
use fileloader::load_maze;

mod render;
use render::{render3d,render_minimap};

mod process_event;
use process_event::process_event;

mod audioplayer;


fn main() {
    let window_width = 900;
    let window_height = 550;
    let mut framebuffer = Framebuffer::new(window_width, window_height);
    let mut state = "main1";
    let asset_dir = "assets/";
    let textures = GameTextures::new(asset_dir);
    let minimap_scale = 0.2;
    let minimap_width = (framebuffer.width as f32 * minimap_scale) as usize;
    let minimap_height = (framebuffer.height as f32 * minimap_scale) as usize;
    let minimap_x = framebuffer.width - minimap_width - 80;
    let minimap_y = framebuffer.height - minimap_height - 420;

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
    let mut start_time1 = Instant::now(); 
    let mut start_time2 = Instant::now(); 
    let mut start_time3 = Instant::now(); 
    let night1_duration = Duration::new(20, 0);
    let night2_duration = Duration::new(20, 0); 
    let night3_duration = Duration::new(15, 0);
    let audio_player_main = AudioPlayer::new("sound/mainmenu.mp3");
    let audio_player_ambience = AudioPlayer::new("sound/ambience.mp3");
    audio_player_ambience.stop();
    let audio_player_jumpscare1 = AudioPlayer::new("sound/jumpscare.mp3");
    audio_player_jumpscare1.stop();
    let audio_player_win1 = AudioPlayer::new("sound/win.mp3");
    audio_player_win1.stop();
    let audio_player_jumpscare2 = AudioPlayer::new("sound/jumpscare.mp3");
    audio_player_jumpscare2.stop();
    let audio_player_win2 = AudioPlayer::new("sound/win.mp3");
    audio_player_win2.stop();
    let audio_player_jumpscare3 = AudioPlayer::new("sound/jumpscare.mp3");
    audio_player_jumpscare3.stop();
    let audio_player_win3 = AudioPlayer::new("sound/win.mp3");
    audio_player_win3.stop();
    
    // LABERINTO 1
    let maze_result1 = load_maze("./night1.txt");
    let (maze1, player_pos1, goal1) = match maze_result1 {
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

    // LABERINTO 2
    let maze_result2 = load_maze("./night2.txt");
    let (maze2, player_pos2, goal2) = match maze_result2 {
        Ok(m) => m,
        Err(e) => {
            println!("Error loading maze: {}", e);
            return;
        }
    };

    let initial_pos2 = match player_pos2 {
        Some((row, col)) => Vec2::new((col * 100) as f32, (row * 100) as f32),
        None => Vec2::new(100.0, 100.0),
    };

    let mut player2 = Player {
        pos: initial_pos2,
        a: 0.0,
        fov: std::f32::consts::PI / 3.0,
    };

    // LABERINTO 3
    let maze_result3 = load_maze("./night3.txt");
    let (maze3, player_pos3, goal3) = match maze_result3 {
        Ok(m) => m,
        Err(e) => {
            println!("Error loading maze: {}", e);
            return;
        }
    };

    let initial_pos3 = match player_pos3 {
        Some((row, col)) => Vec2::new((col * 100) as f32, (row * 100) as f32),
        None => Vec2::new(100.0, 100.0),
    };

    let mut player3 = Player {
        pos: initial_pos3,
        a: 0.0,
        fov: std::f32::consts::PI / 3.0,
    };


    let block_size: usize = window_width.min(window_height) / maze1.len().max(1);

    let mut frame_index = 0;
    let frame_duration = Duration::from_millis(100); // Adjust frame duration
    let mut last_frame_update = Instant::now();

    while window.is_open() {
        if window.is_key_down(Key::Escape) {
            break;
        }

        if state == "main1" {
            audio_player_main.play();
            
            let now = Instant::now();
            if now.duration_since(last_switch) >= Duration::new(3, 0) {
                current_screen = (current_screen + 1) % 2;
                startpage(&mut framebuffer, current_screen, window_width, window_height, &textures);
                last_switch = now;
            }
            if window.is_key_down(Key::Enter) {
                state = "night1start";
                last_switch = now;
                start_time1 = Instant::now(); 
                audio_player_main.stop();
            }
        }

        if state == "night1start" {
            framebuffer.clear();
            framebuffer.draw_text("NIGHT 1", window_width / 2 - 150, window_height / 2 - 50, 8);
            window.update_with_buffer(&framebuffer.buffer, window_width, window_height).unwrap();
            player1.pos = initial_pos1;

            let now = Instant::now();
            if now.duration_since(last_switch) >= Duration::new(3, 0) {
                state = "night1";
                last_switch = now;
                start_time1 = Instant::now(); 
            }
        }

        if state == "night1" {
            audio_player_ambience.play();
            framebuffer.clear();

            process_event(&mut player1,&window,&maze1,block_size);

            render3d(&mut framebuffer, &player1, window_width, window_height, &maze1, &textures);
            render_minimap(&mut framebuffer, &player1, &maze1, minimap_x, minimap_y, minimap_scale, window_width, window_height);
            let now = Instant::now();
            let frame_duration = now.duration_since(last_frame_time);
            let fps = 1.0 / frame_duration.as_secs_f32();
            last_frame_time = now;

            framebuffer.draw_text(&format!("FPS: {:.0}", fps), 10, 10, 5);

            let distance = calc_distance(&player1, block_size, &goal1);

            let threshold = 2.0;

            if distance <= threshold {
                audio_player_ambience.stop();
                state = "night1clear";
                last_switch = now;
            }

            if now.duration_since(start_time1) >= night1_duration {
                audio_player_ambience.stop();
                state = "night1lose";
                last_switch = now;
            }
        }

        if state == "night1clear" {
            audio_player_win1.play();
            framebuffer.clear();
            framebuffer.draw_text("I WAS THE FIRST", window_width / 2 - 250, window_height / 2 - 50, 4);
            framebuffer.draw_text("I HAVE SEEN EVERYTHING", window_width / 2 - 220, window_height / 2 - 15, 4);
            window.update_with_buffer(&framebuffer.buffer, window_width, window_height).unwrap();

            let now = Instant::now();
            if now.duration_since(last_switch) >= Duration::new(6, 0) {
                audio_player_win1.stop();
                state = "night2start";
                last_switch = now;
            }
        }

        if state == "night1lose" {
            audio_player_jumpscare1.play();
            let now = Instant::now();
            if now.duration_since(last_frame_update) >= frame_duration {
                frame_index = (frame_index + 1) % &textures.chicaloss.frame_count;
                last_frame_update = now;
            }

            framebuffer.clear();
            framebuffer.draw_text("GAME OVER", window_width / 2 - 150, window_height / 2 + 200, 5);
            framebuffer.draw_text("PRESS R TO RESTART OR Q TO QUIT", window_width / 2 - 250, window_height / 2 + 240, 3);
        
            framebuffer.draw_animated_image(&textures.chicaloss, frame_index, window_width, window_height - 100);

            window.update_with_buffer(&framebuffer.buffer, window_width, window_height).unwrap();
        
            if window.is_key_down(Key::R) {
                audio_player_jumpscare1.stop();
                state = "night1start";
                start_time1 = Instant::now(); 
            } else if window.is_key_down(Key::Q) {
                audio_player_jumpscare1.stop();
                state = "main1"; 
            }
        }


        if state == "night2start" {
            framebuffer.clear();
            framebuffer.draw_text("NIGHT 2", window_width / 2 - 150, window_height / 2 - 50, 8);
            window.update_with_buffer(&framebuffer.buffer, window_width, window_height).unwrap();
            player2.pos = initial_pos2;

            let now = Instant::now();
            if now.duration_since(last_switch) >= Duration::new(3, 0) {
                state = "night2";
                last_switch = now;
                start_time2 = Instant::now(); 
            }
        }

        if state == "night2" {
            framebuffer.clear();
            audio_player_ambience.play();

            process_event(&mut player2,&window,&maze2,block_size);

            render3d(&mut framebuffer, &player2, window_width, window_height, &maze2, &textures);
            render_minimap(&mut framebuffer, &player2, &maze2, minimap_x, minimap_y, minimap_scale, window_width, window_height);
            let now = Instant::now();
            let frame_duration = now.duration_since(last_frame_time);
            let fps = 1.0 / frame_duration.as_secs_f32();
            last_frame_time = now;

            framebuffer.draw_text(&format!("FPS: {:.0}", fps), 10, 10, 5);

            let distance = calc_distance(&player2, block_size, &goal1);

            let threshold = 2.0;

            if distance <= threshold {
                audio_player_ambience.stop();
                state = "night2clear";
                last_switch = now;
            }

            if now.duration_since(start_time2) >= night2_duration {
                audio_player_ambience.stop();
                state = "night2lose";
                last_switch = now;
            }
        }

        if state == "night2clear" {
            audio_player_win2.play();
            framebuffer.clear();
            framebuffer.draw_text("IS TIME TO FACE THE", window_width / 2 - 300, window_height / 2 - 50, 4);
            framebuffer.draw_text("CONSECUENCES OF YOUR FAILURE", window_width / 2 - 270, window_height / 2 - 15, 4);
            window.update_with_buffer(&framebuffer.buffer, window_width, window_height).unwrap();

            let now = Instant::now();
            if now.duration_since(last_switch) >= Duration::new(6, 0) {
                audio_player_win2.stop();
                state = "night3start";
                last_switch = now;
            }
        }


        if state == "night2lose" {
            audio_player_jumpscare2.play();
            let now = Instant::now();
            if now.duration_since(last_frame_update) >= frame_duration {
                frame_index = (frame_index + 1) % &textures.chicaloss.frame_count;
                last_frame_update = now;
            }

            framebuffer.clear();
            framebuffer.draw_text("GAME OVER", window_width / 2 - 150, window_height / 2 + 200, 5);
            framebuffer.draw_text("PRESS R TO RESTART OR Q TO QUIT", window_width / 2 - 250, window_height / 2 + 240, 3);
        
            framebuffer.draw_animated_image(&textures.bonnieloss, frame_index, window_width, window_height - 100);

            window.update_with_buffer(&framebuffer.buffer, window_width, window_height).unwrap();
        
            if window.is_key_down(Key::R) {
                audio_player_jumpscare2.stop();
                state = "night2start";
                start_time2 = Instant::now(); 
            } else if window.is_key_down(Key::Q) {
                audio_player_jumpscare2.stop();
                state = "main1"; 
            }
        }

        if state == "night3start" {
            framebuffer.clear();
            framebuffer.draw_text("NIGHT 3", window_width / 2 - 150, window_height / 2 - 50, 8);
            window.update_with_buffer(&framebuffer.buffer, window_width, window_height).unwrap();
            player3.pos = initial_pos3;

            let now = Instant::now();
            if now.duration_since(last_switch) >= Duration::new(3, 0) {
                state = "night3";
                last_switch = now;
                start_time3 = Instant::now(); 
            }
        }

        if state == "night3" {
            audio_player_ambience.play();
            framebuffer.clear();

            process_event(&mut player3,&window,&maze3,block_size);

            render3d(&mut framebuffer, &player3, window_width, window_height, &maze3, &textures);
            render_minimap(&mut framebuffer, &player3, &maze3, minimap_x, minimap_y, minimap_scale, window_width, window_height);
            let now = Instant::now();
            let frame_duration = now.duration_since(last_frame_time);
            let fps = 1.0 / frame_duration.as_secs_f32();
            last_frame_time = now;

            framebuffer.draw_text(&format!("FPS: {:.0}", fps), 10, 10, 5);

            let distance = calc_distance(&player3, block_size, &goal1);

            let threshold = 2.0;

            if distance <= threshold {
                audio_player_ambience.stop();
                state = "night3clear";
                last_switch = now;
            }

            if now.duration_since(start_time3) >= night3_duration {
                audio_player_ambience.stop();
                state = "night3lose";
                last_switch = now;
            }
        }

        if state == "night3clear" {
            audio_player_win3.play();
            framebuffer.clear();
            framebuffer.draw_text("TOMMORROW IS", window_width / 2 - 250, window_height / 2 - 50, 4);
            framebuffer.draw_text("ANOTHER DAY", window_width / 2 - 220, window_height / 2 - 15, 4);
            window.update_with_buffer(&framebuffer.buffer, window_width, window_height).unwrap();

            let now = Instant::now();
            if now.duration_since(last_switch) >= Duration::new(6, 0) {
                audio_player_win3.stop();
                state = "main1";
                last_switch = now;
            }
        }


        if state == "night3lose" {
            audio_player_jumpscare3.play();
            let now = Instant::now();
            if now.duration_since(last_frame_update) >= frame_duration {
                frame_index = (frame_index + 1) % &textures.chicaloss.frame_count;
                last_frame_update = now;
            }
            
            framebuffer.clear();
            framebuffer.draw_text("GAME OVER", window_width / 2 - 150, window_height / 2 + 200, 5);
            framebuffer.draw_text("PRESS R TO RESTART OR Q TO QUIT", window_width / 2 - 250, window_height / 2 + 240, 3);
        
            framebuffer.draw_animated_image(&textures.freddyloss, frame_index, window_width, window_height - 100);

            window.update_with_buffer(&framebuffer.buffer, window_width, window_height).unwrap();
        
            if window.is_key_down(Key::R) {
                audio_player_jumpscare3.stop();
                state = "night3start";
                start_time2 = Instant::now(); 
            } else if window.is_key_down(Key::Q) {
                audio_player_jumpscare3.stop();
                state = "main1"; 
            }
        }

        window.update_with_buffer(&framebuffer.buffer, window_width, window_height).unwrap();
        std::thread::sleep(std::time::Duration::from_millis(1));
    }
}




fn startpage(framebuffer: &mut Framebuffer, screen: usize, window_width: usize, window_height: usize,  textures: &GameTextures) {
    framebuffer.clear();

    match screen {
        0 => {
            framebuffer.draw_image(&textures.start1, window_width, window_height);
        }
        1 => {
            framebuffer.draw_image(&textures.start2, window_width, window_height);
        }
        _ => {}
    }
}

fn rgb_to_u32(rgb: Rgb<u8>) -> u32 {
    let r = rgb[0] as u32;
    let g = rgb[1] as u32;
    let b = rgb[2] as u32;
    0xFF000000 | (r << 16) | (g << 8) | b
}


fn calc_distance(player: &Player, block_size: usize, goal: &Vec<(usize, usize)>) -> f32{
    let player_col = (player.pos.x / block_size as f32) as usize;
    let player_row = (player.pos.y / block_size as f32) as usize;

    let goal_col = goal[0].1;
    let goal_row = goal[0].0;

    let dist_x = (goal_col as f32 - player_col as f32).powi(2);
    let dist_y = (goal_row as f32 - player_row as f32).powi(2);
    let distance = (dist_x + dist_y).sqrt();

    return 	distance;
    
}



