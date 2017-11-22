mod vec2;
mod imageplane;
mod cam;
mod ray;
mod circle;
mod color;
mod trace;
mod screen;
mod material;
mod light;

use screen::{Screen, Drawable};
use material::Material;
use color::Color;
use vec2::Vec2;
use imageplane::ImagePlane;
use cam::Cam;
use ray::Ray;
use circle::Circle;
use trace::{Trace, Intersection};
use light::Light;

use std::ffi::CString;
use std::ffi::CStr;
use std::os::raw::c_char;

extern crate serde_json;
use serde_json::{Value, Error};

pub struct Scene<'a> {
    objects: &'a Vec<&'a Trace>,
    image_plane: &'a ImagePlane,
    cam: &'a Cam,
    lights: &'a Vec<&'a Light>,
    ambient: &'a Color,
}

fn occluded(scene: &Scene, shadow_ray: &Ray) -> bool {
    for obj2 in scene.objects {
        let inter = obj2.intersect(shadow_ray);

        if inter.is_none() {
            continue;
        }

        let inter = inter.unwrap();

        if inter.distance > 0.0f64 && inter.distance < 1.0f64 {
            return true;
        }
    }

    false
}

fn trace(scene: &Scene, ray: &Ray, screen: &mut Screen) -> Color {
    let mut col = Color {
        r: 0.0f64,
        g: 0.0f64,
        b: 0.0f64,
    };

    let mut intersection: Option<Intersection> = None;

    for obj in scene.objects {
        let inter = obj.intersect(ray);

        match (inter, intersection.clone()) {
            (Some(pos), Some(sml)) => {
                if pos.distance < sml.distance {
                    col = obj.material().ambient.mult(scene.ambient);
                    intersection = Some(pos);
                }
            }
            (Some(pos), None) => {
                col = obj.material().ambient.mult(scene.ambient);
                intersection = Some(pos);
            }
            (_, _) => (),
        };
    }

    if intersection.is_none() {
        return col;
    }

    let intersection = intersection.unwrap();

    let intersect_at = ray.point.add(&ray.direction.scale(intersection.distance));

    let (p_x, p_y) = screen.project(&intersect_at);
    let (d_x, d_y) = screen.project(&intersection.normal.scale(0.02f64).add(&intersect_at));

    screen.put_line(p_x, p_y, d_x, d_y, col.rgb());

    for light in scene.lights {
        let light_vec = light.position.sub(&intersect_at).normalize();
        let light_dot = light_vec.dot(&intersection.normal);

        if light_dot < 0.0f64 {
            continue;
        }

        let shadow_ray = Ray {
            point: intersect_at.clone(),
            direction: light.position.sub(&intersect_at),
        };

        if occluded(&scene, &shadow_ray) {
            screen.put_screen(&shadow_ray.render());
            continue;
        }

        let diffuse = &intersection
            .object
            .material()
            .diffuse
            .mult(&light.diffuse)
            .scale(light_dot);


        let (d_x, d_y) = screen.project(&light_vec.scale(0.02f64).add(&intersect_at));
        screen.put_line(p_x, p_y, d_x, d_y, diffuse.rgb());

        col = col.add(diffuse);

        let reflect_vec = intersection.normal.scale(2.0f64 * light_dot).sub(
            &light_vec,
        );

        let view_vec = scene.cam.pos.sub(&intersect_at).normalize();

        let reflec_view_dot = reflect_vec.dot(&view_vec);

        let specular = light
            .specular
            .mult(&intersection.object.material().specular)
            .scale(reflec_view_dot.powf(
                intersection.object.material().shininess,
            ));

        col = col.add(&specular);
    }

    if intersection.object.material().reflectivity > 0.0f64 {
        let v = ray.direction.scale(-1.0f64).normalize();
        let reflect_vec = intersection
            .normal
            .scale(2.0f64 * v.dot(&intersection.normal))
            .sub(&v);

        let r = Ray {
            point: intersect_at.add(&Vec2 {
                x: 0.0001f64,
                y: 0.0001f64,
            }),
            direction: reflect_vec,
        };

        col = col.add(&trace(scene, &r, screen).scale(
            intersection
                .object
                .material()
                .reflectivity,
        ));
    }

    col = col.clamp();

    screen.put_screen(&ray.render().as_color(&col));

    col
}

pub fn render(scene: &Scene, screen: &mut Screen) {
    screen.put_screen(&scene.cam.render());

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


    for obj in scene.objects {
        screen.put_screen(&obj.render());
    }

    for light in scene.lights {
        screen.put_screen(&light.render());
    }

    screen.put_screen(&rendered_plane.render());
}

fn my_string_safe(i: *mut c_char) -> String {
    unsafe { CStr::from_ptr(i).to_string_lossy().into_owned() }
}

#[no_mangle]
pub fn render_serial_scene(scene: *mut c_char, target: *mut u8, w: usize, h: usize) {
    let scene = my_string_safe(scene);

    let target: &mut [u8] = unsafe { std::slice::from_raw_parts_mut(target, w * h * 4) };

    for i in 0..target.len() {
        target[i] = 0;
    }

    let mut screen = Screen::new();

    let o1 = &Circle {
        material: Material {
            ambient: Color {
                r: 1.0f64,
                g: 0.0f64,
                b: 0.0f64,
            },
            diffuse: Color {
                r: 1.0f64,
                g: 0.0f64,
                b: 0.0f64,
            },
            specular: Color {
                r: 0.0f64,
                g: 0.0f64,
                b: 0.0f64,
            },
            shininess: 0.0f64,
            reflectivity: 0.3f64,
        },
        center: Vec2 {
            x: 0.5f64,
            y: 0.3f64,
        },
        radius: 0.08f64,
    };

    let o2 = &Circle {
        material: Material {
            ambient: Color {
                r: 0.0f64,
                g: 1.0f64,
                b: 0.0f64,
            },
            diffuse: Color {
                r: 0.0f64,
                g: 1.0f64,
                b: 0.0f64,
            },
            specular: Color {
                r: 0.8f64,
                g: 0.8f64,
                b: 0.8f64,
            },
            shininess: 20.0f64,
            reflectivity: 0.5f64,
        },
        center: Vec2 {
            x: 0.17f64,
            y: 0.1f64,
        },
        radius: 0.25f64,
    };

    let o3 = &Circle {
        material: Material {
            ambient: Color {
                r: 0.0f64,
                g: 0.0f64,
                b: 1.0f64,
            },
            diffuse: Color {
                r: 0.0f64,
                g: 0.0f64,
                b: 1.0f64,
            },
            specular: Color {
                r: 0.0f64,
                g: 0.0f64,
                b: 0.0f64,
            },
            shininess: 0.0f64,
            reflectivity: 0.0f64,
        },
        center: Vec2 {
            x: 0.75f64,
            y: 0.3f64,
        },
        radius: 0.09f64,
    };

    let l1 = &Light {
        position: Vec2 {
            x: 0.7f64,
            y: 0.6f64,
        },
        diffuse: Color {
            r: 0.8f64,
            g: 0.8f64,
            b: 0.8f64,
        },
        specular: Color {
            r: 0.8f64,
            g: 0.8f64,
            b: 0.8f64,
        },
    };

    let scene = Scene {
        ambient: &Color {
            r: 0.1f64,
            g: 0.1f64,
            b: 0.1f64,
        },
        objects: &vec![o1, o2, o3],
        lights: &vec![l1],
        image_plane: &ImagePlane::new(
            32,
            Vec2 {
                x: 0.01f64 as f64,
                y: 0.6f64 as f64,
            },

            Vec2 {
                x: 0.99f64 as f64,
                y: 0.6f64 as f64,
            },
        ),
        cam: &Cam {
            pos: Vec2 {
                x: 0.5f64,
                y: 0.98f64,
            },
        },
    };

    render(&scene, &mut screen);

    screen.push_to_arr(target).unwrap();
}

fn main() {}
