use crate::app::*;
use crate::helpers::*;
use crate::movement::*;
use crate::pathtrace::*;
use crate::scene::*;

mod app;
mod bresenham;
mod helpers;
mod intersect;
mod movement;
mod pathtrace;
mod scene;
mod skybox;

use cgmath::Vector3;
use minifb::{Key, Window, WindowOptions};
use rand::thread_rng;
use rayon::prelude::*;

const WIDTH: usize = 400;
const HEIGHT: usize = 400;

const CHROMATIC_ABERRATION_STRENGTH: f32 = 0.0;

fn main() {
    let mut output_buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    let mut render_buffer: Vec<Col> = vec![Col::new(0.0, 0.0, 0.0); WIDTH * HEIGHT];
    let mut window = Window::new("", WIDTH, HEIGHT, WindowOptions::default()).unwrap_or_else(|e| {
        panic!("{}", e);
    });

    let mut scene = initialize_scene();

    let mut viewport = Viewport {
        overlays_enabled: true,
        autofocus: true,
        depth_pass: false,
        normal_pass: false,
        sample_iter: 0,
        time: Time {
            prev: app::timestamp(),
            sum: 0.0,
            framecount: 0,
        },
    };

    let mut movement = Movement {
        camera_movement: Vector3::new(0.0, 0.0, 0.0),
        camera_rotation: cgmath::Matrix4::from_angle_z(cgmath::Rad(scene.cameras[0].rot.z))
            * cgmath::Matrix4::from_angle_y(cgmath::Rad(scene.cameras[0].rot.y))
            * cgmath::Matrix4::from_angle_x(cgmath::Rad(scene.cameras[0].rot.x)),
        mouse_movement: Vector3::new(0.0, 0.0, 0.0),
        moving: false,
    };
    let mut keys_down: Vec<Key> = vec![];

    // Main loop
    while window.is_open() && !window.is_key_down(Key::Escape) {
        app::update_time(&mut window, &mut viewport.time, &viewport.sample_iter);

        handle_input(
            &mut window,
            &mut viewport,
            &mut scene.cameras[0],
            &mut render_buffer,
            &mut movement,
            &mut keys_down,
            &WIDTH,
            &HEIGHT,
        );

        let image_plane_size = 2.0 * rad(scene.cameras[0].fov / 2.0).tan();

        autofocus(
            viewport.autofocus,
            WIDTH as f32,
            HEIGHT as f32,
            &mut scene,
            image_plane_size,
            &movement,
        );

        let jitter_size = scene.cameras[0].aperture_radius
            * 2.0
            * (1.0 - 1.0 / (scene.cameras[0].focal_length + 0.5));
        let pixel_size: f32 = 1.0 / WIDTH as f32 * image_plane_size / 2.0;

        // Iterate over pixels
        render_buffer
            .par_iter_mut()
            .enumerate()
            .for_each(|(i, pixel)| {
                let mut rng = thread_rng();
                // Create ray from camera
                let (ray, chromatic_aberration_len) = camera_ray(
                    i,
                    &scene,
                    image_plane_size,
                    jitter_size,
                    pixel_size,
                    WIDTH as f32,
                    HEIGHT as f32,
                    &movement,
                    &mut rng,
                    CHROMATIC_ABERRATION_STRENGTH
                );

                // Trace ray
                let col = intersect_spheres(
                    3,
                    10,
                    0,
                    0,
                    &scene,
                    viewport.depth_pass,
                    viewport.normal_pass,
                    &scene.spheres,
                    ObjectID::from(0),
                    &ray,
                    &mut rng,
                );

                if CHROMATIC_ABERRATION_STRENGTH > 0.0 {
                    let cr = chromatic_aberration_len/2.0 + 0.5;
                    let col = col * Col::from_hue(cr) * (1.0 / Col::from_hue(cr).luminance());
                }
                
                // Update render buffer with result
                *pixel += col;
            });

        viewport.sample_iter += 1;

        // Update frame buffer with render buffer
        for (col_1, col_2) in render_buffer.iter().zip(output_buffer.iter_mut()) {
            let col = Col::new(
                clamp_max(col_1.r / viewport.sample_iter as f32, 1.0),
                clamp_max(col_1.g / viewport.sample_iter as f32, 1.0),
                clamp_max(col_1.b / viewport.sample_iter as f32, 1.0),
            );

            *col_2 = col_to_rgb_u32(col);
        }

        // Draw overlays
        if viewport.overlays_enabled {
            for wireframe in &mut scene.wireframes {
                wireframe.render(&mut output_buffer, &scene.cameras[0], &WIDTH, &HEIGHT);
            }
        }

        // Update window
        window.update_with_buffer(&output_buffer).unwrap();
    }
}
