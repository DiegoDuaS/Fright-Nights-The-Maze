use std::io::{self, BufRead, BufReader};
use minifb::{Key, Window, WindowOptions};
use std::time::{Duration, Instant};

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
            if now.duration_since(last_switch) >= Duration::new(5, 0) {
                // Alternar pantalla cada 5 segundos
                current_screen = (current_screen + 1) % 2;  // Alterna entre 0 y 1
                startpage(&mut framebuffer, current_screen, window_width, window_height);
                last_switch = now;
            }
            if window.is_key_down(Key::Enter) {
                state = "main2"
            }
        }

        if state == "main2"{
            beginpage(&mut framebuffer, window_width, window_height);
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
