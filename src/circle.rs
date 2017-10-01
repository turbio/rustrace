use std::f64;

use vec2::Vec2;
use ray::Ray;
use screen::{Screen, Drawable};
use trace::Trace;
use material::Material;

pub struct Circle {
    pub center: Vec2,
    pub radius: f64,
    pub material: Material,
}

impl Circle {}

impl Drawable for Circle {
    fn render(&self) -> Screen {
        let mut target = Screen::new();

        let (xc, yc) = target.project(&Vec2 {
            x: self.center.x,
            y: self.center.y,
        });

        let r = target.project_x(self.radius);
        let mut d = 3 - 2 * r;
        let mut x = 0;
        let mut y = r;

        let c = self.material.color.rgb();

        while y >= x {
            target.put(xc + x, yc + y, c);
            target.put(xc - x, yc + y, c);
            target.put(xc + x, yc - y, c);
            target.put(xc - x, yc - y, c);
            target.put(xc + y, yc + x, c);
            target.put(xc - y, yc + x, c);
            target.put(xc + y, yc - x, c);
            target.put(xc - y, yc - x, c);

            x += 1;

            if d > 0 {
                y -= 1;
                d = d + 4 * (x - y) + 10;
            } else {
                d = d + 4 * x + 6;
            }
        }

        target
    }
}

impl Trace for Circle {
    fn material(&self) -> Material {
        self.material.clone()
    }

    fn intersect(&self, ray: &Ray) -> Option<f64> {
        let ray_to_self = ray.point.sub(&self.center);

        let a = ray.direction.len().powi(2);
        let b = ray.direction.dot(&ray_to_self) * 2.0f64;
        let c = ray_to_self.len().powi(2) - self.radius.powi(2);

        let disc = b.powi(2) - (4.0f64 * a * c);

        if disc < 0.0f64 {
            return None;
        }


        let t1 = (-b + (b.powi(2) - 4.0f64 * a * c).sqrt()) / (2.0f64 * a);
        let t2 = (-b - (b.powi(2) - 4.0f64 * a * c).sqrt()) / (2.0f64 * a);

        let t = t1.min(t2);

        if t < 0.0f64 {
            return None;
        }

        return Some(t);
    }
}
