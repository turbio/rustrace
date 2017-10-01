mod screen;
mod vec2;
mod imageplane;
mod cam;
mod ray;
mod circle;

use screen::Screen;
use vec2::Vec2;
use imageplane::ImagePlane;
use cam::Cam;
use ray::Ray;
use circle::Circle;

fn main() {
    let mut screen = Screen::new(600, 600);

    for x in 0..screen.width {
        for y in 0..screen.height {
            screen.put(x as isize, y as isize, [255, 255, 255]);

        }
    }

    let c = Cam {
        pos: Vec2 {
            x: 0.5f64,
            y: 0.98f64,
        },
    };
    screen.draw(&c);

    let ip = ImagePlane::new();
    screen.draw(&ip);

    for i in 0..ip.pxls() {
        let on_plane = ip.point_at(i);

        let r = Ray {
            point: c.pos.clone(),
            direction: on_plane.sub(&c.pos.clone()),
        };
        screen.draw(&r);
    }

    let c = Circle {
        center: Vec2 { x: 0f64, y: 0f64 },
        radius: 0.25f64,
    };
    screen.draw(&c);

    screen.push("output.png");
}
