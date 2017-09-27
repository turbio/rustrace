use vec2::Vec2;
use screen::{Screen, Drawable};

pub struct Ray {
    pub point: Vec2,
    pub direction: Vec2,
}

impl Drawable for Ray {
    fn render(&self, target: &mut Screen) {
        let (x1, y1) = target.project(&self.point);
        let (x2, y2) = target.project(&self.direction.add(&self.point));
        let color = [255, 0, 0];

        target.line(x1, y1, x2, y2, color);

        target.put(x1 + 1, y1, color);
        target.put(x1, y1 + 1, color);
        target.put(x1 + 1, y1 + 1, color);
    }

    //fn render(&self, target: &mut Screen) {
    //let (x1, y1) = target.project(&self.point);
    //let (x2, y2) = target.project(&self.point.sub(&self.direction));

    //let dx = x2 - x1;
    //let dy = y2 - y1;

    //let color = [255, 0, 0];

    //let mut x = x1;
    //let mut y = y1;

    //target.put(x + 1, y, color);
    //target.put(x, y + 1, color);
    //target.put(x + 1, y + 1, color);
    //while x > 0 && x < target.width as isize && dx != 0 {
    //target.put(x, y, color);

    //if x1 < x2 {
    //x -= 1;
    //} else {
    //x += 1;
    //}

    //y = y1 + dy * (x - x1) / dx;
    //}
    //}
}
