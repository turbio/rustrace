use vec2::Vec2;
use screen::{Screen, Drawable};

pub struct ImagePlane {
    pix: Vec<[u8; 3]>,
    x1: Vec2,
    x2: Vec2,
}

impl ImagePlane {
    pub fn new() -> ImagePlane {
        ImagePlane {
            pix: vec![
                [50, 50, 50],
                [100, 100, 100],
                [150, 150, 150],
                [205, 205, 205],
                [50, 50, 50],
                [100, 100, 100],
                [150, 150, 150],
                [205, 205, 205],
                [50, 50, 50],
                [100, 100, 100],
                [150, 150, 150],
                [205, 205, 205],
                [50, 50, 50],
                [100, 100, 100],
                [150, 150, 150],
                [205, 205, 205],
            ],
            x1: Vec2 {
                x: 0.2f64 as f64,
                y: 0.5f64 as f64,
            },
            x2: Vec2 {
                x: 0.8f64 as f64,
                y: 0.5f64 as f64,
            },
        }
    }

    pub fn pxls(&self) -> usize {
        self.pix.len()
    }

    pub fn point_at(&self, at: usize) -> Vec2 {
        let perc = at as f64 / self.pix.len() as f64;
        self.x1.lerp(&self.x2, perc)
    }
}

impl Drawable for ImagePlane {
    fn render(&self, target: &mut Screen) {
        let (x1, y1) = target.project(&self.x1);
        let (x2, y2) = target.project(&self.x2);

        let (mut x_from, mut y_from, mut x_to, mut y_to) = (x1, y1, x2, y2);

        if x1 > x2 {
            x_from = x2;
            x_to = x1;
        }

        if y1 > y2 {
            y_from = y2;
            y_to = y1;
        }

        let dx = x_to - x_from;
        let dy = y_to - y_from;

        for x in x_from..(x_to + 1) {
            let perc = (x - x_from) as f64 / (x_to - x_from) as f64;
            let at = f64::round(perc * (self.pix.len() - 1) as f64) as usize;
            let color = self.pix[at];

            let y = y_from + dy * (x - x_from) / dx;
            target.put(x, y, color);
            target.put(x, y + 1, color);
            target.put(x, y + 2, color);
            target.put(x, y + 3, color);
        }
    }
}
