use ray::Ray;
use screen::Drawable;
use material::Material;
use vec2::Vec2;

pub trait Trace: Drawable {
    fn material(&self) -> Material;
    fn intersect(&self, ray: &Ray) -> Option<(f64, Vec2)>;
}
