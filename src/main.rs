extern crate cgmath;
extern crate minifb;
extern crate ordered_float;
extern crate rand;
extern crate rayon;
extern crate rgb;
extern crate time;

use app_time::*;
use helpers::*;
use movement::*;
use rays::intersect_sphere;
use render::*;
use scene::*;

mod app_time;
mod helpers;
mod movement;
mod rays;
mod render;
mod scene;

use cgmath::{InnerSpace, Vector3};
use minifb::{Key, MouseMode, Window, WindowOptions};
use ordered_float::OrderedFloat;
use rand::prelude::*;
use rayon::prelude::*;
use time::now;

const WIDTH: usize = 400;
const HEIGHT: usize = 400;
const FOV: f32 = 90.0;

fn main() {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    let mut rgb_buffer: Vec<(Col)> = vec![Col::new(0.0, 0.0, 0.0); WIDTH * HEIGHT];
    let mut window = Window::new("", WIDTH, HEIGHT, WindowOptions::default()).unwrap_or_else(|e| {
        panic!("{}", e);
    });

    let mut scene = initialize_scene();
    let mut render = Render {
        distance_pass: false,
        pixel_size: 1.0 / WIDTH as f32,
        sample_iter: 0,
        time: Time {
            prev: app_time::timestamp(),
            sum: 0.0,
            framecount: 0,
        },
    };

    let mut sorted_spheres = scene.spheres.clone();
    sorted_spheres.sort_by_key(|k| {
        OrderedFloat(clamp_min(
            0.0,
            distance(scene.cameras[0].pos, k.pos) - k.radius,
        ))
    });

    let mut mouse_movement = Vector3::new(0.0, 0.0, 0.0);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        app_time::update_time(
            &mut window,
            &mut render.time.prev,
            &mut render.time.framecount,
            &mut render.time.sum,
        );

        let mut rot = cgmath::Matrix4::from_angle_z(cgmath::Rad(scene.cameras[0].rot.z))
            * cgmath::Matrix4::from_angle_y(cgmath::Rad(scene.cameras[0].rot.y))
            * cgmath::Matrix4::from_angle_x(cgmath::Rad(scene.cameras[0].rot.x));

        let mut movement = Vector3::new(0.0, 0.0, 0.0);
        let mut moving = false;

        handle_movement(
            &mut window,
            &mut scene,
            &mut rgb_buffer,
            &mut render,
            &mut movement,
            &mut rot,
            &WIDTH,
            &HEIGHT,
        );

        handle_mouse_movement(
            &mut window,
            &mut scene,
            &mut rgb_buffer,
            &mut render,
            &mut movement,
            &mut mouse_movement,
            &mut moving,
            &mut rot,
            &WIDTH,
            &HEIGHT,
        );

        let uv_size = 2.0 * (rad(FOV) / 2.0).tan();
        let jitter_size = 2.0
            * scene.cameras[0].apeture_size
            * (1.0 - 1.0 / (scene.cameras[0].focus_distance - 1.0));

        if moving {
            // Only sort spheres if camera is moved
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

                let jitter_x = rng.gen_range(-render.pixel_size / 2.0, render.pixel_size / 2.0);
                let jitter_z = rng.gen_range(-render.pixel_size / 2.0, render.pixel_size / 2.0);

                let focus_jitter =
                    Vector3::new(jitter_x, 0.0, jitter_z) * 2.0 * scene.cameras[0].apeture_size;

                let aliasing_jitter = Vector3::new(
                    rng.gen_range(-render.pixel_size / 2.0, render.pixel_size / 2.0),
                    0.0,
                    rng.gen_range(-render.pixel_size / 2.0, render.pixel_size / 2.0),
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

                let sky_col = mix_col(
                    scene.sky.colors[0],
                    scene.sky.colors[1],
                    1.0 / ((line.z - focus_jitter.z).abs() + 1.0),
                );

                let mut intersected = false;
                let mut closest_ray: f32 = std::f32::MAX;
                let mut first_intersect_distance = std::f32::MAX;
                // for sphere in &scene.spheres {
                for sphere in &sorted_spheres {
                    let intersect_point = intersect_sphere(&ray, &sphere, &scene.cameras[0]);
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
                            if !render.distance_pass {
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

        render.sample_iter += 1;

        for (col_1, col_2) in rgb_buffer.iter().zip(buffer.iter_mut()) {
            let col = Col::new(
                (col_1.r / render.sample_iter as f32).sqrt(),
                (col_1.g / render.sample_iter as f32).sqrt(),
                (col_1.b / render.sample_iter as f32).sqrt(),
            );

            *col_2 = col_to_rgb_u32(col);
        }

        {
            // scene.spheres[0].pos.y = (framecount as f32 / 20.0).sin() * 10.0;
            // scene.cameras[0].pos.y = -5.0 + (framecount as f32 / 20.0).sin() * 1.0;
            // scene.cameras[0].pos.x = (framecount as f32 / 20.0).cos() * 1.0;
            // scene.cameras[0].pos.z = (framecount as f32 / 40.0).cos() * 2.0;
            // sample_iter = 0;
            // rgb_buffer = vec![(0, 0, 0); WIDTH * HEIGHT];
        }

        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window.update_with_buffer(&buffer).unwrap();
    }
}
