use crate::framebuffer::Framebuffer;
use crate::maze::Maze;
use crate::player::Player;
use crate::raycaster::cast_ray;
use crate::skybox::Skybox;
use crate::texture::Texture;

const CEILING_COLOR: u32 = 0x33334d;
const FLOOR_COLOR: u32 = 0x2b2b2b;

fn shade_floor(color: u32, dist: f32) -> u32 {
    let fog = (1.0 - (dist / 16.0).min(0.75)).max(0.25);
    let r = (((color >> 16) & 0xff) as f32 * fog) as u32;
    let g = (((color >> 8) & 0xff) as f32 * fog) as u32;
    let b = ((color & 0xff) as f32 * fog) as u32;
    (r << 16) | (g << 8) | b
}

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

fn shade_texel(color: u32, side: u8, dist: f32) -> u32 {
    let side_factor = if side == 1 { 0.65 } else { 1.0 };
    let fog = (1.0 - (dist / 16.0).min(0.75)).max(0.25);
    let factor = side_factor * fog;

    let r = (((color >> 16) & 0xff) as f32 * factor) as u32;
    let g = (((color >> 8) & 0xff) as f32 * factor) as u32;
    let b = ((color & 0xff) as f32 * factor) as u32;
    (r << 16) | (g << 8) | b
}

pub fn render(
    fb: &mut Framebuffer,
    maze: &Maze,
    player: &Player,
    depth_buffer: &mut Vec<f32>,
    skybox: Option<&Skybox>,
    floor_texture: Option<&Texture>,
    wall_textures: &[Texture],
) {
    let w = fb.width;
    let h = fb.height;
    let half_h = h as f32 / 2.0;

    depth_buffer.clear();
    depth_buffer.resize(w, f32::INFINITY);

    for x in 0..w {
        let camera_x = x as f32 / w as f32;
        let ray_angle = player.angle - player.fov / 2.0 + player.fov * camera_x;
        let cos_diff = (ray_angle - player.angle).cos();
        let cos_ra = ray_angle.cos();
        let sin_ra = ray_angle.sin();

        let hit = cast_ray(maze, player.x, player.y, ray_angle);

        let corrected_dist = (hit.dist * cos_diff).max(0.0001);
        depth_buffer[x] = corrected_dist;

        let line_height = (h as f32 / corrected_dist) as i32;
        let draw_start = (-line_height / 2 + h as i32 / 2).max(0) as usize;
        let draw_end = (line_height / 2 + h as i32 / 2).min(h as i32 - 1) as usize;

        match skybox {
            Some(sky) => {
                let u = ray_angle / std::f32::consts::TAU;
                for y in 0..draw_start {
                    let v = y as f32 / (h as f32 / 2.0);
                    fb.set_pixel(x, y, sky.sample(u, v));
                }
            }
            None => fb.draw_vline(x, 0, draw_start.saturating_sub(1), CEILING_COLOR),
        }

        match floor_texture {
            Some(tex) => {
                for y in (draw_end + 1)..h {
                    let p = y as f32 - half_h;
                    let row_dist = half_h / p;
                    let actual_dist = row_dist / cos_diff;
                    let floor_x = player.x + actual_dist * cos_ra;
                    let floor_y = player.y + actual_dist * sin_ra;
                    let color = tex.sample(floor_x, floor_y);
                    fb.set_pixel(x, y, shade_floor(color, actual_dist));
                }
            }
            None => {
                for y in (draw_end + 1)..h {
                    fb.set_pixel(x, y, FLOOR_COLOR);
                }
            }
        }

        let wall_texture = if wall_textures.is_empty() {
            None
        } else {
            let idx = (hit.map_x + hit.map_y).rem_euclid(wall_textures.len() as i32) as usize;
            Some(&wall_textures[idx])
        };

        match wall_texture {
            Some(tex) => {
                let tex_w = tex.width();
                let tex_h = tex.height();

                let mut tex_x = (hit.wall_x * tex_w as f32) as usize;
                let flip = (hit.side == 0 && cos_ra > 0.0) || (hit.side == 1 && sin_ra < 0.0);
                if flip {
                    tex_x = tex_w - 1 - tex_x;
                }
                tex_x = tex_x.min(tex_w - 1);

                let tex_step = tex_h as f32 / line_height.max(1) as f32;
                let mut tex_pos =
                    (draw_start as f32 - half_h + line_height as f32 / 2.0) * tex_step;

                for y in draw_start..=draw_end {
                    let tex_y = (tex_pos as usize).min(tex_h - 1);
                    tex_pos += tex_step;
                    let color = shade_texel(tex.texel(tex_x, tex_y), hit.side, corrected_dist);
                    fb.set_pixel(x, y, color);
                }
            }
            None => {
                let color = shade(base_color(hit.wall_type), hit.side, corrected_dist);
                fb.draw_vline(x, draw_start, draw_end, color);
            }
        }
    }
}
