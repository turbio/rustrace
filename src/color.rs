#[derive(Clone)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl Color {
    pub fn rgb(&self) -> [u8; 3] {
        [
            (self.r * 255.0f64) as u8,
            (self.g * 255.0f64) as u8,
            (self.b * 255.0f64) as u8,
        ]
    }

    pub fn add(&self, c: &Color) -> Color {
        Color {
            r: self.r + c.r,
            g: self.g + c.g,
            b: self.b + c.b,
        }
    }

    pub fn mult(&self, c: &Color) -> Color {
        Color {
            r: self.r * c.r,
            g: self.g * c.g,
            b: self.b * c.b,
        }
    }

    pub fn scale(&self, s: f64) -> Color {
        Color {
            r: self.r * s,
            g: self.g * s,
            b: self.b * s,
        }
    }

    pub fn clamp(&self) -> Color {
        Color {
            r: self.r.min(1.0f64).max(0.0f64),
            g: self.g.min(1.0f64).max(0.0f64),
            b: self.b.min(1.0f64).max(0.0f64),
        }
    }
}
