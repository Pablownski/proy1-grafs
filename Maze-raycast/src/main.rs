mod audio;
mod framebuffer;
mod gamepad;
mod maze;
mod minimap;
mod player;
mod raycaster;
mod render;
mod skybox;
mod sprite;
mod text;
mod welcome;

use std::time::Instant;

use minifb::{Key, KeyRepeat, Window, WindowOptions};

use audio::AudioSystem;
use framebuffer::Framebuffer;
use gamepad::GamepadInput;
use maze::{Maze, LEVEL_SEEDS};
use player::Player;
use skybox::Skybox;
use sprite::Sprite;

const WIDTH: usize = 1024;
const HEIGHT: usize = 768;
const FPS_OPTIONS: [usize; 3] = [30, 60, 120];

const MUSIC_PATH: &str = "assets/Taylor Swift - Shake It Off.mp3";
const SFX_WIN_PATH: &str = "assets/sfx_win.mp3";
const SKYBOX_PATH: &str = "assets/skybox/skybox-space.png";

#[derive(PartialEq)]
enum GameState {
    Welcome,
    Playing,
    Won,
}

fn start_level(idx: usize) -> (Maze, Player, Vec<Sprite>) {
    let maze = Maze::generate(LEVEL_SEEDS[idx]);
    let mut player = Player::new(maze.spawn.0, maze.spawn.1);
    player.angle = maze.spawn_facing_angle();
    let sprites = maze
        .sprite_cells()
        .into_iter()
        .map(|(x, y)| Sprite::new(x, y))
        .collect();
    (maze, player, sprites)
}

fn main() {
    let mut framebuffer = Framebuffer::new(WIDTH, HEIGHT);

    let mut window = Window::new("Maze Raycaster", WIDTH, HEIGHT, WindowOptions::default())
        .expect("no se pudo crear la ventana");

    let mut fps_index = 0usize;
    window.set_target_fps(FPS_OPTIONS[fps_index]);

    let mut audio = AudioSystem::new();
    audio.play_music_loop(MUSIC_PATH);

    let mut gamepad_input = GamepadInput::new();
    let skybox = Skybox::load(SKYBOX_PATH);

    let mut selected_level = 0usize;
    let mut state = GameState::Welcome;

    let (mut maze, mut player, mut sprites) = start_level(selected_level);

    let mut last_time = Instant::now();
    let start_time = Instant::now();
    let mut last_mouse_x = WIDTH as f32 / 2.0;
    let mut map_large = false;
    let mut depth_buffer: Vec<f32> = Vec::with_capacity(WIDTH);
    let mut smoothed_fps = FPS_OPTIONS[fps_index] as f32;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let now = Instant::now();
        let dt = (now - last_time).as_secs_f32().min(0.1);
        last_time = now;
        let time = start_time.elapsed().as_secs_f32();

        if window.is_key_pressed(Key::F, KeyRepeat::No) {
            fps_index = (fps_index + 1) % FPS_OPTIONS.len();
            window.set_target_fps(FPS_OPTIONS[fps_index]);
        }

        let gp_state = gamepad_input.poll();

        match state {
            GameState::Welcome => {
                if window.is_key_pressed(Key::Key1, KeyRepeat::No) {
                    selected_level = 0;
                }
                if window.is_key_pressed(Key::Key2, KeyRepeat::No) {
                    selected_level = 1;
                }
                if window.is_key_pressed(Key::Key3, KeyRepeat::No) {
                    selected_level = 2;
                }
                if window.is_key_pressed(Key::Down, KeyRepeat::Yes) {
                    selected_level = (selected_level + 1) % LEVEL_SEEDS.len();
                }
                if window.is_key_pressed(Key::Up, KeyRepeat::Yes) {
                    selected_level = (selected_level + LEVEL_SEEDS.len() - 1) % LEVEL_SEEDS.len();
                }
                if window.is_key_pressed(Key::Enter, KeyRepeat::No) {
                    (maze, player, sprites) = start_level(selected_level);
                    state = GameState::Playing;
                }

                welcome::draw_welcome_screen(&mut framebuffer, selected_level);
            }
            GameState::Playing => {
                if window.is_key_pressed(Key::M, KeyRepeat::No) {
                    map_large = !map_large;
                }

                player.update(&window, &maze, dt, &mut last_mouse_x, gp_state);

                if maze.is_goal(player.x, player.y) {
                    state = GameState::Won;
                    audio.play_sfx(SFX_WIN_PATH);
                    println!("¡Meta alcanzada! Presiona ENTER para continuar o ESC para salir.");
                }

                framebuffer.clear();
                render::render(&mut framebuffer, &maze, &player, &mut depth_buffer, skybox.as_ref());
                sprite::draw_sprites(&mut framebuffer, &player, &sprites, &depth_buffer, time);
                minimap::draw_minimap(&mut framebuffer, &maze, &player, map_large);
            }
            GameState::Won => {
                if window.is_key_pressed(Key::M, KeyRepeat::No) {
                    map_large = !map_large;
                }

                let has_next_level = selected_level + 1 < LEVEL_SEEDS.len();

                if window.is_key_pressed(Key::Enter, KeyRepeat::No) {
                    if has_next_level {
                        selected_level += 1;
                        (maze, player, sprites) = start_level(selected_level);
                        state = GameState::Playing;
                    } else {
                        state = GameState::Welcome;
                    }
                }

                framebuffer.clear();
                render::render(&mut framebuffer, &maze, &player, &mut depth_buffer, skybox.as_ref());
                sprite::draw_sprites(&mut framebuffer, &player, &sprites, &depth_buffer, time);
                minimap::draw_minimap(&mut framebuffer, &maze, &player, map_large);

                if has_next_level {
                    text::draw_centered_banner(
                        &mut framebuffer,
                        &["META ALCANZADA !", "ENTER: SIGUIENTE NIVEL"],
                        4,
                    );
                } else {
                    text::draw_centered_banner(
                        &mut framebuffer,
                        &["COMPLETASTE TODO !", "ENTER: MENU"],
                        4,
                    );
                }
            }
        }

        let fps = if dt > 0.0 { 1.0 / dt } else { 0.0 };
        smoothed_fps = smoothed_fps * 0.9 + fps * 0.1;

        let fps_line = format!(
            "FPS: {:.0} / {}  F CAMBIA",
            smoothed_fps, FPS_OPTIONS[fps_index]
        );
        text::draw_text(&mut framebuffer, &fps_line, 10, 10, 2, 0x33ff66);

        let title = match state {
            GameState::Won => format!("Maze Raycaster - {:.0} FPS - ¡META ALCANZADA!", fps),
            _ => format!("Maze Raycaster - {:.0} FPS", fps),
        };
        window.set_title(&title);

        window
            .update_with_buffer(&framebuffer.buffer, WIDTH, HEIGHT)
            .unwrap();
    }
}
