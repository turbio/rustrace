use vec2::Vec2;
use screen::{Screen, Drawable};
use std::f64::consts;

pub struct Circle {
    pub center: Vec2,
    pub radius: f64,
}

impl Circle {}

impl Drawable for Circle {
    fn render(&self, target: &mut Screen) {
        let mut d = 0.0f64;
        while d < consts::PI * 2.0f64 {
            let (x, y) = target.project(&Vec2 {
                x: f64::sin(d) / 8f64 + 0.25f64,
                y: f64::cos(d) / 8f64 + 0.25f64,
            });

            target.put(x, y, [128, 0, 255]);
            d += 0.0145f64;
        }
    }
}
