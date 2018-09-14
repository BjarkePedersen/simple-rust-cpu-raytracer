extern crate cgmath;
extern crate minifb;
extern crate rand;
extern crate rayon;
extern crate rgb;

use cgmath::Vector3;
use helpers::*;
use minifb::{Key, Window, WindowOptions};
use rand::prelude::*;
use rayon::prelude::*;
use rays::intersect_sphere;
use scene::*;

mod helpers;
mod rays;
mod scene;

const WIDTH: usize = 400;
const HEIGHT: usize = 400;
const FOV: f32 = 90.0;

const MOVE_SPEED: f32 = 0.2;
const ROT_SPEED: f32 = 0.05;

fn main() {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    let mut rgb_buffer: Vec<(u32, u32, u32)> = vec![(0, 0, 0); WIDTH * HEIGHT];

    let mut window = Window::new(
        "Test - ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    ).unwrap_or_else(|e| {
        panic!("{}", e);
    });

    let mut scene = initialize_scene();
    let mut distance_pass = false;
    let mut sample_iter: u32 = 0;

    let pixel_size = 1.0 / WIDTH as f32;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        window.get_keys().map(|keys| {
            for t in keys {
                match t {
                    Key::W => scene.cameras[0].pos.y += MOVE_SPEED,
                    Key::S => scene.cameras[0].pos.y -= MOVE_SPEED,
                    Key::A => scene.cameras[0].pos.x += MOVE_SPEED,
                    Key::D => scene.cameras[0].pos.x -= MOVE_SPEED,
                    Key::Space => scene.cameras[0].pos.z += MOVE_SPEED,
                    Key::LeftShift => scene.cameras[0].pos.z -= MOVE_SPEED,
                    Key::Left => scene.cameras[0].rot.z -= ROT_SPEED,
                    Key::Right => scene.cameras[0].rot.z += ROT_SPEED,
                    Key::Up => scene.cameras[0].rot.x += ROT_SPEED,
                    Key::Down => scene.cameras[0].rot.x -= ROT_SPEED,
                    _ => (),
                };
                match t {
                    Key::W
                    | Key::S
                    | Key::A
                    | Key::D
                    | Key::Space
                    | Key::LeftShift
                    | Key::Left
                    | Key::Right
                    | Key::Up
                    | Key::Down => {
                        rgb_buffer.iter_mut().for_each(|col| {
                            *col = (0, 0, 0);
                        });
                        sample_iter = 0;
                    }
                    Key::Enter => {
                        distance_pass = !distance_pass;
                        rgb_buffer.iter_mut().for_each(|col| {
                            *col = (0, 0, 0);
                        });
                        rgb_buffer = vec![(0, 0, 0); WIDTH * HEIGHT];
                        sample_iter = 0;
                    }
                    _ => (),
                };
            }
        });

        rgb_buffer
            .par_iter_mut()
            .enumerate()
            .for_each(|(i, pixel)| {
                let mut rng = thread_rng();
                let mut col = rgb(255, 255, 255);

                let mut closest_ray: f32 = std::f32::MAX;

                let wut = Vector3::new(
                    ((uv(WIDTH * HEIGHT - i - 1).x - WIDTH as f32 / 2.0) / HEIGHT as f32)
                        * 2.0
                        * (rad(FOV) / 2.0).tan()
                        + rng.gen_range(-pixel_size / 2.0, pixel_size / 2.0),
                    1.0,
                    ((uv(WIDTH * HEIGHT - i - 1).y - HEIGHT as f32 / 2.0) / HEIGHT as f32)
                        * 2.0
                        * (rad(FOV) / 2.0).tan()
                        + rng.gen_range(-pixel_size / 2.0, pixel_size / 2.0),
                );

                let x = cgmath::Matrix4::from_angle_x(cgmath::Rad(scene.cameras[0].rot.x));
                let y = cgmath::Matrix4::from_angle_y(cgmath::Rad(scene.cameras[0].rot.y));
                let z = cgmath::Matrix4::from_angle_z(cgmath::Rad(scene.cameras[0].rot.z));

                let newRay = x * y * z * wut.extend(0.0);

                let eh = newRay.truncate();

                let ray = Ray {
                    p1: scene.cameras[0].pos,
                    p2: eh + scene.cameras[0].pos,
                };

                for sphere in &scene.spheres {
                    let intersect_point = intersect_sphere(&ray, &sphere, &scene.cameras[0]);
                    if let Some(intersect) = intersect_point {
                        let distance = distance(scene.cameras[0].pos, intersect);
                        if distance < closest_ray {
                            closest_ray = distance;
                            if !distance_pass {
                                col = sphere.material.color;
                            } else {
                                col = rgb(
                                    distance as u32 * distance as u32 / 10,
                                    distance as u32 * distance as u32 / 10,
                                    distance as u32 * distance as u32 / 10,
                                );
                            }
                        }
                    }
                }
                pixel.0 += byte_to_rgb(col).0 as u32;
                pixel.1 += byte_to_rgb(col).1 as u32;
                pixel.2 += byte_to_rgb(col).2 as u32;
            });

        sample_iter += 1;

        for (col_1, col_2) in rgb_buffer.iter().zip(buffer.iter_mut()) {
            *col_2 = rgb(
                col_1.0 as u32 / sample_iter,
                col_1.1 as u32 / sample_iter,
                col_1.2 as u32 / sample_iter,
            );
        }

        {
            // scene.spheres[0].pos.y = (t / 20.0).sin() * 10.0;
            // scene.cameras[0].pos.y = -5.0 + (t / 20.0).sin() * 1.0;
            // scene.cameras[0].pos.x = (t / 20.0).cos() * 1.0;
            // scene.cameras[0].pos.z = (t / 40.0).cos() * 2.0;
            // sample_iter = 0;
            // rgb_buffer = vec![(0, 0, 0); WIDTH * HEIGHT];
        }

        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window.update_with_buffer(&buffer).unwrap();
    }
}