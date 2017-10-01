mod vec2;
mod imageplane;
mod cam;
mod ray;
mod circle;
mod color;
mod trace;
mod screen;
mod material;

use screen::{Screen, Drawable};
use material::Material;
use color::Color;
use vec2::Vec2;
use imageplane::ImagePlane;
use cam::Cam;
use ray::Ray;
use circle::Circle;
use trace::Trace;

struct Scene<'a> {
    objects: &'a Vec<&'a Trace>,
    image_plane: &'a ImagePlane,
    cam: &'a Cam,
}

fn trace(scene: &Scene, ray: &Ray, screen: &mut Screen) -> Color {
    let mut col = Color {
        r: 0.0f64,
        g: 0.0f64,
        b: 0.0f64,
    };

    let mut smallest: Option<f64> = None;
    for obj in scene.objects {
        let i = obj.intersect(ray);

        match (i, smallest) {
            (Some(pos), Some(sml)) => {
                if pos < sml {
                    col = obj.material().color;
                    smallest = Some(pos);
                }
            }
            (Some(pos), None) => {
                col = obj.material().color;
                smallest = Some(pos);
            }
            (None, _) => (),
        };
    }

    screen.put_screen(&ray.render().as_color(&col));

    col
}

fn render(scene: &Scene, screen: &mut Screen) {
    screen.put_screen(&scene.cam.render());
    for obj in scene.objects {
        screen.put_screen(&obj.render());
    }

    let mut rendered_plane = Box::new(scene.image_plane.clone());

    for i in 0..scene.image_plane.pxls() {
        let on_plane = scene.image_plane.point_at(i);

        let ray = Ray {
            point: scene.cam.pos.clone(),
            direction: on_plane.sub(&scene.cam.pos.clone()),
        };

        let c = trace(scene, &ray, screen);
        rendered_plane.put(i, c);
    }

    screen.put_screen(&rendered_plane.render());
}

fn main() {
    let a = &Circle {
        material: Material {
            color: Color {
                r: 1.0f64,
                g: 0.0f64,
                b: 0.0f64,
            },
        },
        center: Vec2 {
            x: 0.45f64,
            y: 0.2f64,
        },
        radius: 0.08f64,
    };

    let b = &Circle {
        material: Material {
            color: Color {
                r: 0.0f64,
                g: 1.0f64,
                b: 0.0f64,
            },
        },
        center: Vec2 {
            x: 0.17f64,
            y: 0.1f64,
        },
        radius: 0.25f64,
    };

    let c = &Circle {
        material: Material {
            color: Color {
                r: 0.0f64,
                g: 0.0f64,
                b: 1.0f64,
            },
        },
        center: Vec2 {
            x: 0.75f64,
            y: 0.3f64,
        },
        radius: 0.09f64,
    };

    let scene = Scene {
        objects: &vec![a, b, c],
        image_plane: &ImagePlane::new(
            64,
            Vec2 {
                x: 0.02f64 as f64,
                y: 0.5f64 as f64,
            },

            Vec2 {
                x: 0.98f64 as f64,
                y: 0.5f64 as f64,
            },
        ),
        cam: &Cam {
            pos: Vec2 {
                x: 0.5f64,
                y: 0.98f64,
            },
        },
    };

    let mut screen = Screen::new();

    render(&scene, &mut screen);

    screen.push("output.png");
}
