extern crate image;

use self::image::{ImageBuffer, Rgba};
use vec2::Vec2;
use color::Color;

pub struct Screen {
    pub width: u32,
    pub height: u32,
    image: ImageBuffer<Rgba<u8>, Vec<u8>>,
}

impl Screen {
    pub fn new() -> Screen {
        let w = 800;
        let h = 800;

        let screen = Screen {
            image: ImageBuffer::<Rgba<u8>, Vec<u8>>::new(w, h),
            width: w,
            height: h,
        };

        screen
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

    pub fn project_x(&self, x: f64) -> isize {
        (x * self.width as f64) as isize
    }

    pub fn as_color(&self, color: &Color) -> Screen {
        let to_color = color.rgb();

        let mut target = Screen::new();

        for (x, y, p) in self.image.enumerate_pixels() {
            let tp = target.image.get_pixel_mut(x, y);
            if p[3] > 0 {
                tp[0] = to_color[0];
                tp[1] = to_color[1];
                tp[2] = to_color[2];
                tp[3] = p[3];
            }
        }

        target
    }

    pub fn put(&mut self, x: isize, y: isize, c: [u8; 3]) {
        if x < self.width as isize && y < self.height as isize && x > 0 && y > 0 {
            self.image.put_pixel(
                x as u32,
                y as u32,
                Rgba([c[0], c[1], c[2], 255]),
            )
        }
    }

    pub fn put_screen(&mut self, screen: &Screen) {
        for (x, y, p) in self.image.enumerate_pixels_mut() {
            let source = screen.image.get_pixel(x, y);

            if source[3] > 0 {
                p[0] = source[0];
                p[1] = source[1];
                p[2] = source[2];
                p[3] = 255;
            }
        }
    }

    pub fn put_line(&mut self, x1: isize, y1: isize, x2: isize, y2: isize, c: [u8; 3]) {
        let x_from;
        let y_from;
        let x_to;
        let y_to;

        if (x1 - x2).abs() > (y1 - y2).abs() {
            if x1 < x2 {
                x_from = x1;
                x_to = x2;
                y_from = y1;
                y_to = y2;
            } else {
                x_from = x2;
                x_to = x1;
                y_from = y2;
                y_to = y1;
            }


            let dx = x_to - x_from;
            let dy = y_to - y_from;

            for x in x_from..(x_to + 1) {
                if dx == 0 {
                    continue;
                }

                let y = y_from + dy * (x - x_from) / dx;
                self.put(x, y, c);
            }
        } else {
            if y1 < y2 {
                x_from = x1;
                x_to = x2;
                y_from = y1;
                y_to = y2;
            } else {
                x_from = x2;
                x_to = x1;
                y_from = y2;
                y_to = y1;
            }


            let dx = x_to - x_from;
            let dy = y_to - y_from;

            for y in y_from..(y_to + 1) {
                if dy == 0 {
                    continue;
                }

                let x = x_from + dx * (y - y_from) / dy;
                self.put(x, y, c);
            }
        }
    }
}

pub trait Drawable {
    fn render(&self) -> Screen;
}
