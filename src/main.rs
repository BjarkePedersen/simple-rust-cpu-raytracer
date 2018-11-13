extern crate cgmath;
extern crate minifb;
extern crate ordered_float;
extern crate rand;
extern crate rayon;
extern crate rgb;
extern crate time;

use helpers::*;
use movement::*;
use rays::intersect_sphere;
use scene::*;
use viewport::*;

mod app_time;
mod helpers;
mod movement;
mod rays;
mod scene;
mod viewport;

use cgmath::{InnerSpace, Vector3};
use minifb::{Key, Window, WindowOptions};
use ordered_float::OrderedFloat;
use rand::prelude::*;
use rayon::prelude::*;

const WIDTH: usize = 400;
const HEIGHT: usize = 400;
const FOV: f32 = 90.0;
const PIXEL_SIZE: f32 = 1.0 / WIDTH as f32;

fn main() {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    let mut rgb_buffer: Vec<(Col)> = vec![Col::new(0.0, 0.0, 0.0); WIDTH * HEIGHT];
    let mut window = Window::new("", WIDTH, HEIGHT, WindowOptions::default()).unwrap_or_else(|e| {
        panic!("{}", e);
    });

    let mut scene = initialize_scene();
    let mut viewport = Viewport {
        distance_pass: false,
        sample_iter: 0,
        time: Time {
            prev: app_time::timestamp(),
            sum: 0.0,
            framecount: 0,
        },
    };

    let uv_size = 2.0 * (rad(FOV) / 2.0).tan();
    let jitter_size =
        2.0 * scene.cameras[0].apeture_size * (1.0 - 1.0 / (scene.cameras[0].focus_distance - 1.0));

    let mut sorted_spheres = scene.spheres.clone();
    sorted_spheres.sort_by_key(|k| {
        OrderedFloat(clamp_min(
            0.0,
            distance(scene.cameras[0].pos, k.pos) - k.radius,
        ))
    });

    let mut movement = Movement {
        camera_movement: Vector3::new(0.0, 0.0, 0.0),
        mouse_movement: Vector3::new(0.0, 0.0, 0.0),
        moving: false,
    };

    // Main loop
    while window.is_open() && !window.is_key_down(Key::Escape) {
        app_time::update_time(
            &mut window,
            &mut viewport.time.prev,
            &mut viewport.time.framecount,
            &mut viewport.time.sum,
            &viewport.sample_iter,
        );

        let mut rot = cgmath::Matrix4::from_angle_z(cgmath::Rad(scene.cameras[0].rot.z))
            * cgmath::Matrix4::from_angle_y(cgmath::Rad(scene.cameras[0].rot.y))
            * cgmath::Matrix4::from_angle_x(cgmath::Rad(scene.cameras[0].rot.x));

        handle_movement(
            &mut window,
            &mut scene,
            &mut rgb_buffer,
            &mut viewport,
            &mut movement.camera_movement,
            &mut movement.mouse_movement,
            &mut movement.moving,
            &mut rot,
            &WIDTH,
            &HEIGHT,
        );

        if movement.moving {
            // Only need to sort spheres if camera has moved
            sorted_spheres.sort_by_key(|k| {
                OrderedFloat(clamp_min(
                    0.0,
                    distance(scene.cameras[0].pos, k.pos) - k.radius,
                ))
            });
        }

        rgb_buffer
            .par_iter_mut()
            .enumerate()
            .for_each(|(i, pixel)| {
                let mut rng = thread_rng();
                let mut col = Col::new(1.0, 1.0, 1.0);

                let jitter_x = rng.gen_range(-PIXEL_SIZE / 2.0, PIXEL_SIZE / 2.0);
                let jitter_z = rng.gen_range(-PIXEL_SIZE / 2.0, PIXEL_SIZE / 2.0);

                let apeture_jitter =
                    Vector3::new(jitter_x, 0.0, jitter_z) * 2.0 * scene.cameras[0].apeture_size;

                let aliasing_jitter = Vector3::new(
                    rng.gen_range(-PIXEL_SIZE / 2.0, PIXEL_SIZE / 2.0),
                    0.0,
                    rng.gen_range(-PIXEL_SIZE / 2.0, PIXEL_SIZE / 2.0),
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

                let apeture_jitter_mat4 = rot * apeture_jitter.extend(0.0);
                let apeture_jitter = apeture_jitter_mat4.truncate();

                let ray = Ray {
                    pos: apeture_jitter + scene.cameras[0].pos + aliasing_jitter,
                    dir: (line - apeture_jitter + aliasing_jitter).normalize(),
                };

                let sky_col = mix_col(
                    scene.sky.colors[0],
                    scene.sky.colors[1],
                    1.0 / ((line.z - apeture_jitter.z).abs() + 1.0),
                );

                let mut intersected = false;
                let mut closest_ray: f32 = std::f32::MAX;
                let mut first_intersect_distance = std::f32::MAX;
                // for sphere in &scene.spheres {
                for sphere in &sorted_spheres {
                    let intersect_point = intersect_sphere(&ray, &sphere);
                    if let Some(intersect_point) = intersect_point {
                        intersected = true;
                        let distance = distance(scene.cameras[0].pos, intersect_point);
                        if distance < first_intersect_distance {
                            first_intersect_distance = distance
                        };
                        if distance - first_intersect_distance > sphere.radius {
                            break;
                        }
                        if distance < closest_ray {
                            closest_ray = distance;
                            if !viewport.distance_pass {
                                // col = sphere.material.color;
                                col = mix_col(
                                    sphere.material.color,
                                    sky_col,
                                    clamp(0.0, 1.0, 1.0 / (distance / 20.0)),
                                );
                            } else {
                                col = Col::new(distance / 20.0, distance / 20.0, distance / 20.0);
                            }
                        }
                    }
                }
                // Sky color
                if !intersected {
                    col = sky_col;
                }
                pixel.r += col.r.powf(2.0);
                pixel.g += col.g.powf(2.0);
                pixel.b += col.b.powf(2.0);
            });

        viewport.sample_iter += 1;

        for (col_1, col_2) in rgb_buffer.iter().zip(buffer.iter_mut()) {
            let col = Col::new(
                (col_1.r / viewport.sample_iter as f32).sqrt(),
                (col_1.g / viewport.sample_iter as f32).sqrt(),
                (col_1.b / viewport.sample_iter as f32).sqrt(),
            );

            *col_2 = col_to_rgb_u32(col);
        }
        window.update_with_buffer(&buffer).unwrap();
    }
}
