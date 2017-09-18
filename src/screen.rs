extern crate image;

use self::image::{ImageBuffer, Rgb};
use vec2::Vec2;

pub struct Screen {
    pub width: u32,
    pub height: u32,
    image: ImageBuffer<Rgb<u8>, Vec<u8>>,
}

impl Screen {
    pub fn new(w: u32, h: u32) -> Screen {
        Screen {
            image: ImageBuffer::<Rgb<u8>, Vec<u8>>::new(w, h),
            width: w,
            height: h,
        }
    }

    pub fn put(&mut self, x: isize, y: isize, c: [u8; 3]) {
        if x < self.width as isize && y < self.height as isize && x > 0 && y > 0 {
            self.image.get_pixel_mut(x as u32, y as u32).data = c;
        }
    }

    pub fn push(&self, to: &str) {
        self.image.save(to).unwrap();
    }

    pub fn project(&self, v: &Vec2) -> (isize, isize) {
        (
            (v.x * self.width as f64) as isize,
            (v.y * self.height as f64) as isize,
        )
    }

    pub fn line(&mut self, v1: Vec2, v2: Vec2) {}

    pub fn draw(&mut self, d: &Drawable) {
        d.render(self)
    }
}

pub trait Drawable {
    fn render(&self, &mut Screen);
}
