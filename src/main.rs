use std::io::{self, BufRead, BufReader};
use minifb::{Key, Window, WindowOptions};
use std::time::{Duration, Instant};
use std::thread;

mod framebuffer;
use framebuffer::Framebuffer;


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

    while window.is_open() {
        if window.is_key_down(Key::Escape) {
            break;
        }

        if state == "main1"{
            let now = Instant::now();
            if now.duration_since(last_switch) >= Duration::new(3, 0) {
                // Alternar pantalla cada 5 segundos
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

        if state == "night1"{
            framebuffer.clear();
        }

        if state == "night2start"{
            continue;
        }

        if state == "night2"{
            continue;
        }

        if state == "night3start"{
            continue;
        }

        if state == "night3"{
            continue;
        }

        // Mostrar el contenido del framebuffer en la ventana
        window.update_with_buffer(&framebuffer.buffer, window_width, window_height).unwrap();
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


fn beginpage(framebuffer: &mut Framebuffer, window_width: usize, window_height: usize){
    framebuffer.draw_image("assets/starte.png", window_width, window_height);
}
