use crate::maze::Maze;

pub struct RayHit {
    pub dist: f32,
    pub wall_type: u8,
    pub side: u8,
}

pub fn cast_ray(maze: &Maze, ox: f32, oy: f32, angle: f32) -> RayHit {
    let ray_dir_x = angle.cos();
    let ray_dir_y = angle.sin();

    let mut map_x = ox.floor() as i32;
    let mut map_y = oy.floor() as i32;

    let delta_dist_x = if ray_dir_x == 0.0 {
        f32::INFINITY
    } else {
        (1.0 / ray_dir_x).abs()
    };
    let delta_dist_y = if ray_dir_y == 0.0 {
        f32::INFINITY
    } else {
        (1.0 / ray_dir_y).abs()
    };

    let (step_x, mut side_dist_x) = if ray_dir_x < 0.0 {
        (-1, (ox - map_x as f32) * delta_dist_x)
    } else {
        (1, (map_x as f32 + 1.0 - ox) * delta_dist_x)
    };
    let (step_y, mut side_dist_y) = if ray_dir_y < 0.0 {
        (-1, (oy - map_y as f32) * delta_dist_y)
    } else {
        (1, (map_y as f32 + 1.0 - oy) * delta_dist_y)
    };

    let mut side;
    loop {
        if side_dist_x < side_dist_y {
            side_dist_x += delta_dist_x;
            map_x += step_x;
            side = 0;
        } else {
            side_dist_y += delta_dist_y;
            map_y += step_y;
            side = 1;
        }

        if maze.is_wall(map_x, map_y) {
            break;
        }
    }

    let dist = if side == 0 {
        (map_x as f32 - ox + (1 - step_x) as f32 / 2.0) / ray_dir_x
    } else {
        (map_y as f32 - oy + (1 - step_y) as f32 / 2.0) / ray_dir_y
    };

    RayHit {
        dist: dist.abs(),
        wall_type: maze.wall_type(map_x, map_y),
        side,
    }
}
