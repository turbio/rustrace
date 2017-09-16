extern crate image;

use image::{ImageBuffer, Rgb};

struct Screen {
    width: u32,
    height: u32,
    image: ImageBuffer<Rgb<u8>, Vec<u8>>,
}

impl Screen {
    fn new(w: u32, h: u32) -> Screen {
        Screen {
            image: ImageBuffer::<Rgb<u8>, Vec<u8>>::new(w, h),
            width: w,
            height: h,
        }
    }

    fn put(&mut self, x: u32, y: u32, c: [u8; 3]) {
        self.image.get_pixel_mut(x, y).data = c;
    }

    fn push(&self, to: &str) {
        self.image.save(to).unwrap();
    }

    fn project(&self, v: Vec2) -> (i32, i32) {
        (
            (v.x * self.width as f64) as i32,
            (v.y * self.height as f64) as i32,
        )
    }

    fn line(&mut self, v1: Vec2, v2: Vec2) {
        let (x1, y1) = self.project(v1);
        let (x2, y2) = self.project(v2);

        let dx = x2 - x1;
        let dy = y2 - y1;

        for x in x1..(x2 + 1) {
            let y = y1 + dy * (x - x1) / dx;
            if x < self.width as i32 && y < self.height as i32 && x > 0 && y > 0 {
                self.put(x as u32, y as u32, [0, 0, 0]);
            }
        }
    }
}

struct Vec2 {
    x: f64,
    y: f64,
}

struct ImagePlane {
    x1: Vec2,
    x2: Vec2,
}

struct Camera {
    position: Vec2,
}

fn main() {
    let mut screen = Screen::new(300, 300);

    for x in 0..screen.width {
        for y in 0..screen.height {
            screen.put(x, y, [255, 255, 255])
        }
    }

    screen.line(Vec2 { x: -1f64, y: -1f64 }, Vec2 { x: 1f64, y: 1f64 });

    screen.push("output.png");
}
