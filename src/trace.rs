use ray::Ray;
use screen::Drawable;
use material::Material;

pub trait Trace: Drawable {
    fn material(&self) -> Material;
    fn intersect(&self, ray: &Ray) -> Option<f64>;
}
