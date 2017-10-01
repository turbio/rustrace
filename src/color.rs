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
}
