pub struct Framebuffer {
    pub width: usize,
    pub height: usize,
    pub buffer: Vec<u32>,
}

impl Framebuffer {
    pub fn new(width: usize, height: usize) -> Self {
        Framebuffer {
            width,
            height,
            buffer: vec![0; width * height],
        }
    }

    pub fn clear(&mut self) {
        for pixel in self.buffer.iter_mut() {
            *pixel = 0;
        }
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, color: u32) {
        if x < self.width && y < self.height {
            self.buffer[y * self.width + x] = color;
        }
    }

    pub fn draw_vline(&mut self, x: usize, y0: usize, y1: usize, color: u32) {
        if x >= self.width || y0 > y1 {
            return;
        }
        let y1 = y1.min(self.height.saturating_sub(1));
        for y in y0..=y1 {
            self.set_pixel(x, y, color);
        }
    }
}
