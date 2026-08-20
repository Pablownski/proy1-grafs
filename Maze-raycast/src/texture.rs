pub struct Texture {
    width: usize,
    height: usize,
    pixels: Vec<u32>,
}

impl Texture {
    pub fn load(path: &str) -> Option<Self> {
        let img = match image::open(path) {
            Ok(img) => img.to_rgb8(),
            Err(e) => {
                eprintln!("texture: no se pudo cargar '{path}': {e}");
                return None;
            }
        };

        let width = img.width() as usize;
        let height = img.height() as usize;
        let pixels = img
            .pixels()
            .map(|p| ((p[0] as u32) << 16) | ((p[1] as u32) << 8) | p[2] as u32)
            .collect();

        Some(Texture { width, height, pixels })
    }

    /// Samples the texture, tiling infinitely: `u` and `v` wrap around [0, 1).
    pub fn sample(&self, u: f32, v: f32) -> u32 {
        let u = u.rem_euclid(1.0);
        let v = v.rem_euclid(1.0);
        let x = ((u * self.width as f32) as usize).min(self.width - 1);
        let y = ((v * self.height as f32) as usize).min(self.height - 1);
        self.pixels[y * self.width + x]
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    /// Direct pixel access by integer texel coordinates (no wrapping/clamping).
    pub fn texel(&self, x: usize, y: usize) -> u32 {
        self.pixels[y * self.width + x]
    }
}
