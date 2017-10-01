use vec2::Vec2;
use screen::{Screen, Drawable};
use color::Color;

#[derive(Clone)]
pub struct ImagePlane {
    pix: Vec<Color>,
    x1: Vec2,
    x2: Vec2,
}

impl ImagePlane {
    pub fn new(size: usize, x1: Vec2, x2: Vec2) -> ImagePlane {

        let mut p = Vec::new();
        for i in 0..size {
            let c = if i % 3 == 0 {

                Color {
                    r: 1.0f64,
                    g: 0.0f64,
                    b: 0.0f64,
                }
            } else if i % 3 == 1 {
                Color {
                    r: 0.0f64,
                    g: 1.0f64,
                    b: 0.0f64,
                }
            } else {

                Color {
                    r: 0.0f64,
                    g: 0.0f64,
                    b: 1.0f64,
                }
            };

            p.push(c)
        }

        ImagePlane {
            pix: p,
            x1: x1,
            x2: x2,
        }
    }

    pub fn put(&mut self, at: usize, c: Color) {
        self.pix[at] = c;
    }

    pub fn pxls(&self) -> usize {
        self.pix.len()
    }

    pub fn point_at(&self, at: usize) -> Vec2 {
        // so point will be right in the center of the pixel
        let perc = at as f64 / self.pix.len() as f64;
        let next_perc = (at + 1) as f64 / self.pix.len() as f64;

        self.x1.lerp(&self.x2, (perc + next_perc) / 2.0f64)
    }
}

impl Drawable for ImagePlane {
    fn render(&self) -> Screen {
        let mut target = Screen::new();

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
            let perc = (x - x_from) as f64 / (x_from - x_to).abs() as f64;
            let mut at = (perc * self.pix.len() as f64).floor() as usize;

            if at >= self.pix.len() {
                at = self.pix.len() - 1;
            }

            let color = &self.pix[at];

            let y = y_from + dy * (x - x_from) / dx;
            target.put(x, y, color.rgb());
            target.put(x, y + 1, color.rgb());
            target.put(x, y + 2, color.rgb());
            target.put(x, y + 3, color.rgb());

        }

        target
    }
}
