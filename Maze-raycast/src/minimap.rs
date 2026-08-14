use crate::framebuffer::Framebuffer;
use crate::maze::Maze;
use crate::player::Player;

const CELL_PX_SMALL: usize = 5;
const CELL_PX_LARGE: usize = 16;
const MARGIN: usize = 12;
const BG_COLOR: u32 = 0x0d0d12;
const BORDER_COLOR: u32 = 0xe0e0e0;
const PLAYER_COLOR: u32 = 0xffdd33;
const DIR_COLOR: u32 = 0xffffff;

fn wall_color(wall_type: u8) -> u32 {
    match wall_type {
        1 => 0xc62828,
        2 => 0x1e64c8,
        3 => 0x3ca03c,
        4 => 0xd2a028,
        _ => 0x969696,
    }
}

pub fn draw_minimap(fb: &mut Framebuffer, maze: &Maze, player: &Player, large: bool) {
    let cell_px = if large { CELL_PX_LARGE } else { CELL_PX_SMALL };
    let map_w = maze.width * cell_px;
    let map_h = maze.height * cell_px;

    if map_w + 2 * MARGIN > fb.width || map_h + 2 * MARGIN > fb.height {
        return;
    }

    let (ox, oy) = if large {
        ((fb.width - map_w) / 2, (fb.height - map_h) / 2)
    } else {
        (fb.width - map_w - MARGIN, MARGIN)
    };

    if large {
        for y in 0..fb.height {
            for x in 0..fb.width {
                fb.set_pixel(x, y, 0x000000);
            }
        }
    }

    for y in 0..map_h + 4 {
        for x in 0..map_w + 4 {
            let color = if x < 2 || y < 2 || x >= map_w + 2 || y >= map_h + 2 {
                BORDER_COLOR
            } else {
                BG_COLOR
            };
            fb.set_pixel(ox - 2 + x, oy - 2 + y, color);
        }
    }

    for cy in 0..maze.height {
        for cx in 0..maze.width {
            let cell = maze.cells[cy][cx];
            if cell == 0 {
                continue;
            }
            let color = wall_color(cell);
            for py in 0..cell_px {
                for px in 0..cell_px {
                    fb.set_pixel(ox + cx * cell_px + px, oy + cy * cell_px + py, color);
                }
            }
        }
    }

    let (gx, gy) = maze.goal;
    for py in 0..cell_px {
        for px in 0..cell_px {
            fb.set_pixel(ox + gx * cell_px + px, oy + gy * cell_px + py, 0x22ff22);
        }
    }

    let px = ox as f32 + player.x * cell_px as f32;
    let py = oy as f32 + player.y * cell_px as f32;

    let player_radius = if large { 3i32 } else { 1i32 };
    for dy in -player_radius..=player_radius {
        for dx in -player_radius..=player_radius {
            fb.set_pixel((px as i32 + dx).max(0) as usize, (py as i32 + dy).max(0) as usize, PLAYER_COLOR);
        }
    }

    let dir_len = cell_px as f32 * 1.5;
    let steps = dir_len as i32;
    for i in 0..steps {
        let t = i as f32;
        let x = (px + player.angle.cos() * t) as usize;
        let y = (py + player.angle.sin() * t) as usize;
        fb.set_pixel(x, y, DIR_COLOR);
    }
}
