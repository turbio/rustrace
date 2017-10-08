use vec2::Vec2;
use color::Color;
use circle::Circle;
use material::Material;
use screen::{Drawable, Screen};

pub struct Light {
    pub position: Vec2,
    pub diffuse: Color,
    pub specular: Color,
}

impl Drawable for Light {
    fn render(&self) -> Screen {
        let mut target = Screen::new();

        let m = Material {
            ambient: self.diffuse.clone(),
            diffuse: Color {
                r: 1.0f64,
                g: 1.0f64,
                b: 1.0f64,
            },
            specular: Color {
                r: 1.0f64,
                g: 1.0f64,
                b: 1.0f64,
            },
            shininess: 0.0f64,
            reflectivity: 0.0f64,
        };

        let c1 = Circle {
            center: self.position.clone(),
            radius: 0.0125f64,
            material: m.clone(),
        };

        let c2 = Circle {
            center: self.position.clone(),
            radius: 0.0025f64,
            material: m.clone(),
        };

        let c3 = Circle {
            center: self.position.clone(),
            radius: 0.009f64,
            material: m,
        };

        target.put_screen(&c1.render());
        target.put_screen(&c2.render());
        target.put_screen(&c3.render());

        target
    }
}
