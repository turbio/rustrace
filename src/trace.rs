use ray::Ray;
use screen::Drawable;
use material::Material;
use vec2::Vec2;

#[derive(Clone)]
pub struct Intersection<'a> {
    pub distance: f64,
    pub normal: Vec2,
    pub object: &'a Trace,
}

pub trait Trace: Drawable {
    fn material(&self) -> Material;
    fn intersect(&self, ray: &Ray) -> Option<Intersection>;
}
