use screen::{Screen, Drawable};

#[derive(Clone)]
pub struct Vec2 {
    pub x: f64,
    pub y: f64,
}

impl Drawable for Vec2 {
    fn render(&self, target: &mut Screen) {
        let (x, y) = target.project(self);
        target.put(x, y, [0, 0, 0]);
        target.put(x + 1, y, [0, 0, 0]);
        target.put(x, y + 1, [0, 0, 0]);
        target.put(x + 1, y + 1, [0, 0, 0]);
    }
}

impl Vec2 {
    pub fn add(&self, v: &Vec2) -> Vec2 {
        Vec2 {
            x: self.x + v.x,
            y: self.y + v.y,
        }
    }

    pub fn lerp(&self, v: &Vec2, s: f64) -> Vec2 {
        self.scale(1f64 - s).add(&v.scale(s))
    }


    pub fn sub(&self, v: &Vec2) -> Vec2 {
        Vec2 {
            x: self.x - v.x,
            y: self.y - v.y,
        }
    }

    pub fn scale(&self, s: f64) -> Vec2 {
        Vec2 {
            x: self.x * s,
            y: self.y * s,
        }
    }

    //fn mult(&self, v: Vec2) -> Vec2 {
    //Vec2 {
    //x: self.x * v.x,
    //y: self.y * v.y,
    //}
    //}
}
