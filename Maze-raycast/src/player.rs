use minifb::{Key, MouseMode, Window};

use crate::maze::Maze;

pub struct Player {
    pub x: f32,
    pub y: f32,
    pub angle: f32,
    pub fov: f32,
}

const MOVE_SPEED: f32 = 3.0; 
const ROT_SPEED: f32 = 2.5; 
const MOUSE_SENSITIVITY: f32 = 0.005;
const RADIUS: f32 = 0.2; 

impl Player {
    pub fn new(x: f32, y: f32) -> Self {
        Player {
            x,
            y,
            angle: 0.0,
            fov: std::f32::consts::FRAC_PI_3,
        }
    }

    pub fn update(&mut self, window: &Window, maze: &Maze, dt: f32, last_mouse_x: &mut f32) {
        if window.is_key_down(Key::Left) || window.is_key_down(Key::A) {
            self.angle -= ROT_SPEED * dt;
        }
        if window.is_key_down(Key::Right) || window.is_key_down(Key::D) {
            self.angle += ROT_SPEED * dt;
        }

        if let Some((mx, _my)) = window.get_mouse_pos(MouseMode::Pass) {
            let delta = mx - *last_mouse_x;
            self.angle += delta * MOUSE_SENSITIVITY;
            *last_mouse_x = mx;
        }

        let mut move_x = 0.0f32;
        let mut move_y = 0.0f32;
        if window.is_key_down(Key::W) {
            move_x += self.angle.cos();
            move_y += self.angle.sin();
        }
        if window.is_key_down(Key::S) {
            move_x -= self.angle.cos();
            move_y -= self.angle.sin();
        }

        let step = MOVE_SPEED * dt;
        let new_x = self.x + move_x * step;
        let new_y = self.y + move_y * step;

        if move_x != 0.0 {
            let check_x = new_x + RADIUS * move_x.signum();
            if !maze.is_wall_f(check_x, self.y) {
                self.x = new_x;
            }
        }
        if move_y != 0.0 {
            let check_y = new_y + RADIUS * move_y.signum();
            if !maze.is_wall_f(self.x, check_y) {
                self.y = new_y;
            }
        }
    }
}
