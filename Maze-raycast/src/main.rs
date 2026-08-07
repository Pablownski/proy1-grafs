mod framebuffer;
mod maze;
mod player;
mod raycaster;
mod render;

use std::time::Instant;

use minifb::{Key, Window, WindowOptions};

use framebuffer::Framebuffer;
use maze::Maze;
use player::Player;

const WIDTH: usize = 1024;
const HEIGHT: usize = 768;

fn main() {
    let maze = Maze::generate();
    let mut player = Player::new(maze.spawn.0, maze.spawn.1);

    let mut framebuffer = Framebuffer::new(WIDTH, HEIGHT);

    let mut window = Window::new("Maze Raycaster", WIDTH, HEIGHT, WindowOptions::default())
        .expect("no se pudo crear la ventana");

    let mut last_time = Instant::now();
    let mut last_mouse_x = WIDTH as f32 / 2.0;
    let mut won = false;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let now = Instant::now();
        let dt = (now - last_time).as_secs_f32().min(0.1);
        last_time = now;

        if !won {
            player.update(&window, &maze, dt, &mut last_mouse_x);

            if maze.is_goal(player.x, player.y) {
                won = true;
                println!("¡Meta alcanzada! Presiona ESC para salir.");
            }
        }

        framebuffer.clear();
        render::render(&mut framebuffer, &maze, &player);

        let fps = if dt > 0.0 { 1.0 / dt } else { 0.0 };
        let title = if won {
            format!("Maze Raycaster - {:.0} FPS - ¡META ALCANZADA!", fps)
        } else {
            format!("Maze Raycaster - {:.0} FPS", fps)
        };
        window.set_title(&title);

        window
            .update_with_buffer(&framebuffer.buffer, WIDTH, HEIGHT)
            .unwrap();
    }
}
