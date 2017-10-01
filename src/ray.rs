use vec2::Vec2;
use screen::{Screen, Drawable};

pub struct Ray {
    pub point: Vec2,
    pub direction: Vec2,
}

impl Drawable for Ray {
    fn render(&self) -> Screen {
        let mut target = Screen::new();

        let (x1, y1) = target.project(&self.point);

        // uggly hack (for now?)
        let (x2, y2) = target.project(&self.direction
            .add(&self.direction)
            .add(&self.direction)
            .add(&self.direction)
            .add(&self.point));
        let color = [0, 0, 0];

        target.put_line(x1, y1, x2, y2, color);

        target.put(x1 + 1, y1, color);
        target.put(x1, y1 + 1, color);
        target.put(x1 + 1, y1 + 1, color);

        target
    }
}
