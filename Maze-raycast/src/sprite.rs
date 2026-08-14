use crate::framebuffer::Framebuffer;
use crate::player::Player;

pub struct Sprite {
    pub x: f32,
    pub y: f32,
}

impl Sprite {
    pub fn new(x: f32, y: f32) -> Self {
        Sprite { x, y }
    }
}

fn normalize_angle(mut a: f32) -> f32 {
    while a > std::f32::consts::PI {
        a -= std::f32::consts::TAU;
    }
    while a < -std::f32::consts::PI {
        a += std::f32::consts::TAU;
    }
    a
}

pub fn draw_sprites(
    fb: &mut Framebuffer,
    player: &Player,
    sprites: &[Sprite],
    depth_buffer: &[f32],
    time: f32,
) {
    let w = fb.width as f32;
    let h = fb.height as f32;

    let pulse = 1.0 + 0.25 * (time * 3.0).sin();
    let glow = 0.5 + 0.5 * (time * 3.0).sin();

    for sprite in sprites {
        let dx = sprite.x - player.x;
        let dy = sprite.y - player.y;
        let dist = (dx * dx + dy * dy).sqrt();
        if dist < 0.1 {
            continue;
        }

        let rel_angle = normalize_angle(dy.atan2(dx) - player.angle);
        if rel_angle.abs() > player.fov / 2.0 + 0.3 {
            continue;
        }

        let perp_dist = (dist * rel_angle.cos()).max(0.0001);
        let screen_x = (0.5 + rel_angle / player.fov) * w;

        let size = (h / perp_dist) * 0.35 * pulse;
        if size < 1.0 {
            continue;
        }

        let x0 = (screen_x - size / 2.0).max(0.0) as i32;
        let x1 = (screen_x + size / 2.0).min(w - 1.0) as i32;
        let y_center = h / 2.0;
        let y0 = (y_center - size / 2.0).max(0.0) as i32;
        let y1 = (y_center + size / 2.0).min(h - 1.0) as i32;

        if x1 < x0 || y1 < y0 {
            continue;
        }

        for sx in x0..=x1 {
            if sx < 0 || sx as usize >= depth_buffer.len() {
                continue;
            }
            if depth_buffer[sx as usize] < perp_dist {
                continue;
            }

            let nx = (sx as f32 - screen_x) / (size / 2.0);
            for sy in y0..=y1 {
                let ny = (sy as f32 - y_center) / (size / 2.0);
                let r2 = nx * nx + ny * ny;
                if r2 > 1.0 {
                    continue;
                }

                let core = 1.0 - r2;
                let g = (150.0 + 100.0 * glow * core) as u32;
                let b = (255.0 * core.max(0.3)) as u32;
                let r = (80.0 + 80.0 * glow) as u32;
                let color = (r.min(255) << 16) | (g.min(255) << 8) | b.min(255);
                fb.set_pixel(sx as usize, sy as usize, color);
            }
        }
    }
}
