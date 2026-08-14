use std::collections::VecDeque;


const LOGICAL_COLS: usize = 12;
const LOGICAL_ROWS: usize = 9;

pub const LEVEL_SEEDS: [u32; 3] = [0x1234_5678, 0x9e37_79b9, 0x85eb_ca6b];
pub const LEVEL_NAMES: [&str; 3] = ["NIVEL 1", "NIVEL 2", "NIVEL 3"];

pub struct Maze {
    
    pub cells: Vec<Vec<u8>>,
    pub width: usize,
    pub height: usize,
    pub spawn: (f32, f32),
    pub goal: (usize, usize),
}

struct Rng(u32);

impl Rng {
    fn next(&mut self) -> u32 {
        let mut x = self.0;
        x ^= x << 13;
        x ^= x >> 17;
        x ^= x << 5;
        self.0 = x;
        x
    }

    fn range(&mut self, n: usize) -> usize {
        (self.next() as usize) % n
    }
}

impl Maze {
    pub fn generate(seed: u32) -> Self {
        let width = LOGICAL_COLS * 2 + 1;
        let height = LOGICAL_ROWS * 2 + 1;

        let mut cells = vec![vec![1u8; width]; height];
        let mut visited = vec![vec![false; LOGICAL_COLS]; LOGICAL_ROWS];
        let mut rng = Rng(seed);

        let mut stack = vec![(0usize, 0usize)];
        visited[0][0] = true;
        cells[1][1] = 0;

        while let Some(&(cx, cy)) = stack.last() {
            let mut neighbors: Vec<(usize, usize, i32, i32)> = Vec::new();
            if cx > 0 && !visited[cy][cx - 1] {
                neighbors.push((cx - 1, cy, -1, 0));
            }
            if cx + 1 < LOGICAL_COLS && !visited[cy][cx + 1] {
                neighbors.push((cx + 1, cy, 1, 0));
            }
            if cy > 0 && !visited[cy - 1][cx] {
                neighbors.push((cx, cy - 1, 0, -1));
            }
            if cy + 1 < LOGICAL_ROWS && !visited[cy + 1][cx] {
                neighbors.push((cx, cy + 1, 0, 1));
            }

            if neighbors.is_empty() {
                stack.pop();
                continue;
            }

            let (nx, ny, dx, dy) = neighbors[rng.range(neighbors.len())];
            visited[ny][nx] = true;

            let wall_x = (cx as i32 * 2 + 1 + dx) as usize;
            let wall_y = (cy as i32 * 2 + 1 + dy) as usize;
            cells[wall_y][wall_x] = 0;
            cells[ny * 2 + 1][nx * 2 + 1] = 0;

            stack.push((nx, ny));
        }

        for y in 0..height {
            for x in 0..width {
                if cells[y][x] == 1 {
                    let qx = x >= width / 2;
                    let qy = y >= height / 2;
                    cells[y][x] = match (qx, qy) {
                        (false, false) => 1,
                        (true, false) => 2,
                        (false, true) => 3,
                        (true, true) => 4,
                    };
                }
            }
        }

        let spawn_cell = (1usize, 1usize);
        let goal_cell = Self::farthest_cell(&cells, width, height, spawn_cell);

        Maze {
            cells,
            width,
            height,
            spawn: (spawn_cell.0 as f32 + 0.5, spawn_cell.1 as f32 + 0.5),
            goal: goal_cell,
        }
    }

    fn farthest_cell(
        cells: &[Vec<u8>],
        width: usize,
        height: usize,
        start: (usize, usize),
    ) -> (usize, usize) {
        let mut dist = vec![vec![-1i32; width]; height];
        let mut queue = VecDeque::new();
        dist[start.1][start.0] = 0;
        queue.push_back(start);

        let mut farthest = start;
        let mut farthest_dist = 0;

        while let Some((x, y)) = queue.pop_front() {
            let d = dist[y][x];
            if d > farthest_dist {
                farthest_dist = d;
                farthest = (x, y);
            }

            let candidates = [
                (x.wrapping_sub(1), y),
                (x + 1, y),
                (x, y.wrapping_sub(1)),
                (x, y + 1),
            ];
            for (nx, ny) in candidates {
                if nx < width && ny < height && cells[ny][nx] == 0 && dist[ny][nx] == -1 {
                    dist[ny][nx] = d + 1;
                    queue.push_back((nx, ny));
                }
            }
        }

        farthest
    }

    pub fn is_wall(&self, x: i32, y: i32) -> bool {
        if x < 0 || y < 0 || x as usize >= self.width || y as usize >= self.height {
            return true;
        }
        self.cells[y as usize][x as usize] != 0
    }

    pub fn is_wall_f(&self, x: f32, y: f32) -> bool {
        self.is_wall(x.floor() as i32, y.floor() as i32)
    }

    pub fn wall_type(&self, x: i32, y: i32) -> u8 {
        if x < 0 || y < 0 || x as usize >= self.width || y as usize >= self.height {
            return 1;
        }
        self.cells[y as usize][x as usize]
    }

    pub fn is_goal(&self, x: f32, y: f32) -> bool {
        (x.floor() as usize, y.floor() as usize) == self.goal
    }

    /// Angle the player should start facing so they look down an open
    /// corridor instead of straight into a wall.
    pub fn spawn_facing_angle(&self) -> f32 {
        let cx = self.spawn.0.floor() as i32;
        let cy = self.spawn.1.floor() as i32;

        const DIRS: [(i32, i32, f32); 4] = [
            (1, 0, 0.0),
            (0, 1, std::f32::consts::FRAC_PI_2),
            (-1, 0, std::f32::consts::PI),
            (0, -1, -std::f32::consts::FRAC_PI_2),
        ];

        for (dx, dy, angle) in DIRS {
            if !self.is_wall(cx + dx, cy + dy) {
                return angle;
            }
        }
        0.0
    }

    /// Picks a couple of open cells along the spawn->goal path (roughly at
    /// 1/3 and 2/3 of the way) to place collectible sprites on.
    pub fn sprite_cells(&self) -> Vec<(f32, f32)> {
        let spawn = (self.spawn.0.floor() as usize, self.spawn.1.floor() as usize);
        let mut dist = vec![vec![-1i32; self.width]; self.height];
        let mut queue = VecDeque::new();
        dist[spawn.1][spawn.0] = 0;
        queue.push_back(spawn);

        let mut ordered = vec![spawn];
        while let Some((x, y)) = queue.pop_front() {
            let d = dist[y][x];
            let candidates = [
                (x.wrapping_sub(1), y),
                (x + 1, y),
                (x, y.wrapping_sub(1)),
                (x, y + 1),
            ];
            for (nx, ny) in candidates {
                if nx < self.width && ny < self.height && self.cells[ny][nx] == 0 && dist[ny][nx] == -1
                {
                    dist[ny][nx] = d + 1;
                    queue.push_back((nx, ny));
                    ordered.push((nx, ny));
                }
            }
        }

        let goal_dist = dist[self.goal.1][self.goal.0].max(1);
        let targets = [goal_dist / 3, goal_dist * 2 / 3];
        targets
            .iter()
            .filter_map(|&t| {
                ordered
                    .iter()
                    .filter(|&&(x, y)| (x, y) != spawn && (x, y) != self.goal)
                    .min_by_key(|&&(x, y)| (dist[y][x] - t).abs())
                    .map(|&(x, y)| (x as f32 + 0.5, y as f32 + 0.5))
            })
            .collect()
    }
}
