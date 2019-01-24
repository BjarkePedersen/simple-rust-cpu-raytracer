use crate::app::*;
use crate::helpers::*;
use crate::movement::*;
use crate::pathtrace::*;
use crate::scene::*;
use rand::prelude::*;

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
use rayon::prelude::*;

const WIDTH: usize = 400;
const HEIGHT: usize = 400;

fn main() {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    let mut render_buffer: Vec<(Col)> = vec![Col::new(0.0, 0.0, 0.0); WIDTH * HEIGHT];
    let mut window = Window::new("", WIDTH, HEIGHT, WindowOptions::default()).unwrap_or_else(|e| {
        panic!("{}", e);
    });

    let mut scene = initialize_scene();

    let mut viewport = Viewport {
        overlays_enabled: true,
        depth_pass: false,
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
        let jitter_size =
            scene.cameras[0].aperture_radius * 2.0 * (1.0 - 1.0 / (scene.cameras[0].focal_length));
        let pixel_size: f32 = 1.0 / WIDTH as f32 * image_plane_size / 2.0;

        // Iterate over pixels
        render_buffer
            .par_iter_mut()
            .enumerate()
            .for_each(|(i, pixel)| {
                let mut rng = thread_rng();
                // Create ray from camera
                let ray = camera_ray(
                    i,
                    &scene,
                    image_plane_size,
                    jitter_size,
                    pixel_size,
                    WIDTH,
                    HEIGHT,
                    &movement,
                    &mut rng,
                );

                // Trace ray
                let col = intersect_spheres(
                    3,
                    0,
                    &scene,
                    viewport.depth_pass,
                    &scene.spheres,
                    None,
                    &ray,
                    &mut rng,
                );

                // Update render buffer with result
                pixel.r += col.r.powi(2);
                pixel.g += col.g.powi(2);
                pixel.b += col.b.powi(2);
            });

        viewport.sample_iter += 1;

        // Update frame buffer with render buffer
        for (col_1, col_2) in render_buffer.iter().zip(buffer.iter_mut()) {
            let col = Col::new(
                clamp_max((col_1.r / viewport.sample_iter as f32).sqrt(), 1.0),
                clamp_max((col_1.g / viewport.sample_iter as f32).sqrt(), 1.0),
                clamp_max((col_1.b / viewport.sample_iter as f32).sqrt(), 1.0),
            );

            *col_2 = col_to_rgb_u32(col);
        }

        // Draw overlays
        if viewport.overlays_enabled {
            for wireframe in &mut scene.wireframes {
                wireframe.render(&mut buffer, &scene.cameras[0], &WIDTH, &HEIGHT);
            }
        }

        // Update window
        window.update_with_buffer(&buffer).unwrap();
    }
}
