extern crate cgmath;
extern crate minifb;
extern crate rand;
extern crate rayon;
extern crate rgb;

use cgmath::{Vector3, InnerSpace};
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
const ROT_SPEED: f32 = 0.1;

fn main() {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    let mut rgb_buffer: Vec<(Col)> = vec![Col::new(0.0, 0.0, 0.0); WIDTH * HEIGHT];

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
    let mut focus_distance: f32 = 5.0;
    let mut apeture_size: f32 = 100.0;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let mut rot = cgmath::Matrix4::from_angle_z(cgmath::Rad(scene.cameras[0].rot.z))
            * cgmath::Matrix4::from_angle_y(cgmath::Rad(scene.cameras[0].rot.y))
            * cgmath::Matrix4::from_angle_x(cgmath::Rad(scene.cameras[0].rot.x));

        let mut movement = Vector3::new(0.0, 0.0, 0.0);

        window.get_keys().map(|keys| {
            for t in keys {
                match t {
                    Key::W => movement.y += MOVE_SPEED,
                    Key::S => movement.y -= MOVE_SPEED,
                    Key::A => movement.x += MOVE_SPEED,
                    Key::D => movement.x -= MOVE_SPEED,
                    Key::Space => movement.z += MOVE_SPEED,
                    Key::LeftShift => movement.z -= MOVE_SPEED,
                    Key::Left => scene.cameras[0].rot.z -= ROT_SPEED,
                    Key::Right => scene.cameras[0].rot.z += ROT_SPEED,
                    Key::Up => scene.cameras[0].rot.x += ROT_SPEED,
                    Key::Down => scene.cameras[0].rot.x -= ROT_SPEED,
                    Key::Q => scene.cameras[0].rot.y += ROT_SPEED,
                    Key::E => scene.cameras[0].rot.y -= ROT_SPEED,
                    Key::J => focus_distance -= 0.1,
                    Key::L => focus_distance += 0.1,
                    Key::M => apeture_size -= 10.0,
                    Key::I => apeture_size += 10.0,
                    _ => (),
                };
                match t {
                    Key::Left | Key::Right | Key::Up | Key::Down => {
                        rot = cgmath::Matrix4::from_angle_z(cgmath::Rad(scene.cameras[0].rot.z))
                            * cgmath::Matrix4::from_angle_y(cgmath::Rad(scene.cameras[0].rot.y))
                            * cgmath::Matrix4::from_angle_x(cgmath::Rad(scene.cameras[0].rot.x));
                    }
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
                    | Key::Down
                    | Key::Q
                    | Key::E
                    | Key::J
                    | Key::L
                    | Key::I
                    | Key::M => {
                        rgb_buffer = vec![Col::new(0.0, 0.0, 0.0); WIDTH * HEIGHT];
                        sample_iter = 0;

                        let pos = rot * movement.extend(0.0);
                        let pos = pos.truncate();
                        scene.cameras[0].pos += pos;
                    }
                    Key::Enter => {
                        distance_pass = !distance_pass;
                        rgb_buffer = vec![Col::new(0.0, 0.0, 0.0); WIDTH * HEIGHT];
                        sample_iter = 0;
                    }
                    _ => (),
                };
            }
        });

        let uv_size = 2.0 * (rad(FOV) / 2.0).tan();
        let jitter_size = 2.0 * apeture_size * (1.0 - 1.0 / (focus_distance - 1.0));

        rgb_buffer
            .par_iter_mut()
            .enumerate()
            .for_each(|(i, pixel)| {
                let mut rng = thread_rng();
                let mut col = Col::new(1.0, 1.0, 1.0);

                let mut closest_ray: f32 = std::f32::MAX;

                let jitter_x = rng.gen_range(-pixel_size / 2.0, pixel_size / 2.0);
                let jitter_z = rng.gen_range(-pixel_size / 2.0, pixel_size / 2.0);

                let focus_jitter = Vector3::new(jitter_x, 0.0, jitter_z) * 2.0 * apeture_size;

                // Anti aliasing
                let aliasing_jitter = Vector3::new(
                    rng.gen_range(-pixel_size / 2.0, pixel_size / 2.0),
                    0.0,
                    rng.gen_range(-pixel_size / 2.0, pixel_size / 2.0),
                );

                let line = {
                    let uv = uv(WIDTH * HEIGHT - i - 1);

                    Vector3::new(
                        ((uv.x - WIDTH as f32 / 2.0) / HEIGHT as f32) * uv_size
                            + jitter_x * jitter_size,
                        1.0,
                        ((uv.y - HEIGHT as f32 / 2.0) / HEIGHT as f32) * uv_size
                            + jitter_z * jitter_size,
                    )
                };

                let ray1_mat4 = rot * line.extend(0.0);
                let line = ray1_mat4.truncate();

                let focus_jitter_mat4 = rot * focus_jitter.extend(0.0);
                let focus_jitter = focus_jitter_mat4.truncate();

                let ray = Ray {
                    pos: focus_jitter + scene.cameras[0].pos + aliasing_jitter,
                    dir: (line - focus_jitter + aliasing_jitter).normalize(),
                };

                let mut intersected = false;
                for sphere in &scene.spheres {
                    let intersect_point = intersect_sphere(&ray, &sphere, &scene.cameras[0] );
                    if let Some(intersect) = intersect_point {
                        intersected = true;
                        let distance = distance(scene.cameras[0].pos, intersect);
                        if distance < closest_ray {
                            closest_ray = distance;
                            if !distance_pass {
                                col = sphere.material.color;
                            } else {
                                col =
                                    Col::new(distance / 20.0, distance / 20.0, distance / 20.0);
                            }
                        }
                    }
                }
                // Sky color
                if !intersected {
                    col = mix_col(
                        scene.sky.colors[0],
                        scene.sky.colors[1],
                        1.0 / ((line.z - focus_jitter.z).abs() + 1.0),
                    )
                }
                pixel.r += col.r.powf(2.0);
                pixel.g += col.g.powf(2.0);
                pixel.b += col.b.powf(2.0);
            });

        sample_iter += 1;

        for (col_1, col_2) in rgb_buffer.iter().zip(buffer.iter_mut()) {
            let col = Col::new(
                (col_1.r / sample_iter as f32).sqrt(),
                (col_1.g / sample_iter as f32).sqrt(),
                (col_1.b / sample_iter as f32).sqrt(),
            );

            *col_2 = col_to_rgb_u32(col);
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
