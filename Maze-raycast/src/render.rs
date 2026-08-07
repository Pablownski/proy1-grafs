use crate::framebuffer::Framebuffer;
use crate::maze::Maze;
use crate::player::Player;
use crate::raycaster::cast_ray;

const CEILING_COLOR: u32 = 0x33334d;
const FLOOR_COLOR: u32 = 0x2b2b2b;

fn base_color(wall_type: u8) -> (f32, f32, f32) {
    match wall_type {
        1 => (198.0, 40.0, 40.0),  
        2 => (30.0, 100.0, 200.0), 
        3 => (60.0, 160.0, 60.0),  
        4 => (210.0, 160.0, 40.0),
        _ => (150.0, 150.0, 150.0),
    }
}

fn shade(color: (f32, f32, f32), side: u8, dist: f32) -> u32 {
    let side_factor = if side == 1 { 0.65 } else { 1.0 };
    let fog = (1.0 - (dist / 16.0).min(0.75)).max(0.25);
    let factor = side_factor * fog;

    let r = (color.0 * factor) as u32;
    let g = (color.1 * factor) as u32;
    let b = (color.2 * factor) as u32;
    (r << 16) | (g << 8) | b
}

pub fn render(fb: &mut Framebuffer, maze: &Maze, player: &Player) {
    let w = fb.width;
    let h = fb.height;

    for y in 0..h / 2 {
        for x in 0..w {
            fb.set_pixel(x, y, CEILING_COLOR);
        }
    }
    for y in h / 2..h {
        for x in 0..w {
            fb.set_pixel(x, y, FLOOR_COLOR);
        }
    }

    for x in 0..w {
        let camera_x = x as f32 / w as f32; 
        let ray_angle = player.angle - player.fov / 2.0 + player.fov * camera_x;

        let hit = cast_ray(maze, player.x, player.y, ray_angle);

        let corrected_dist = (hit.dist * (ray_angle - player.angle).cos()).max(0.0001);

        let line_height = (h as f32 / corrected_dist) as i32;
        let draw_start = (-line_height / 2 + h as i32 / 2).max(0) as usize;
        let draw_end = (line_height / 2 + h as i32 / 2).min(h as i32 - 1) as usize;

        let color = shade(base_color(hit.wall_type), hit.side, corrected_dist);
        fb.draw_vline(x, draw_start, draw_end, color);
    }
}
