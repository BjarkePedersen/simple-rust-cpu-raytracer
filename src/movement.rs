use crate::app::Viewport;
use crate::helpers::{clamp, clamp_min, distance, Col};
use crate::pathtrace::{camera_ray_simple, raycast};
use crate::scene::{Camera, Scene};
use cgmath::{Matrix4, Vector3};
use minifb::{Key, MouseMode};

pub struct Movement {
    pub camera_movement: Vector3<f32>,
    pub camera_rotation: Matrix4<f32>,
    pub mouse_movement: Vector3<f32>,
    pub moving: bool,
}

pub fn handle_input(
    window: &mut minifb::Window,
    viewport: &mut Viewport,
    camera: &mut Camera,
    render_buffer: &mut Vec<(Col)>,
    movement: &mut Movement,
    keys_down: &mut Vec<Key>,
    display_width: &usize,
    display_height: &usize,
) {
    const MOVE_SPEED: f32 = 0.2;
    const ROT_SPEED: f32 = 0.1;
    const MOUSE_SENSITIVITY: f32 = 100.0;

    window.get_keys().map(|keys| {
        for key in keys {
            match key {
                Key::W => movement.camera_movement.y += MOVE_SPEED,
                Key::S => movement.camera_movement.y -= MOVE_SPEED,
                Key::A => movement.camera_movement.x -= MOVE_SPEED,
                Key::D => movement.camera_movement.x += MOVE_SPEED,
                Key::Space => movement.camera_movement.z += MOVE_SPEED,
                Key::LeftShift => movement.camera_movement.z -= MOVE_SPEED,
                Key::Left => camera.rot.z += ROT_SPEED,
                Key::Right => camera.rot.z -= ROT_SPEED,
                Key::Up => camera.rot.x += ROT_SPEED,
                Key::Down => camera.rot.x -= ROT_SPEED,
                Key::J => camera.focal_length *= 0.9,
                Key::L => camera.focal_length /= 0.9,
                Key::I => camera.aperture_radius += 0.01,
                Key::M => camera.aperture_radius -= 0.01,
                Key::Z => camera.fov *= 0.95,
                Key::X => camera.fov /= 0.95,
                _ => (),
            };
            match key {
                Key::Left | Key::Right | Key::Up | Key::Down | Key::Q | Key::E => {
                    movement.camera_rotation =
                        cgmath::Matrix4::from_angle_z(cgmath::Rad(camera.rot.z))
                            * cgmath::Matrix4::from_angle_y(cgmath::Rad(camera.rot.y))
                            * cgmath::Matrix4::from_angle_x(cgmath::Rad(camera.rot.x));
                }
                _ => (),
            };
            match key {
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
                | Key::J
                | Key::L
                | Key::I
                | Key::M
                | Key::Z
                | Key::X => {
                    *render_buffer = vec![Col::new(0.0, 0.0, 0.0); display_width * display_height];
                    viewport.sample_iter = 0;

                    let pos = movement.camera_rotation * movement.camera_movement.extend(0.0);
                    let pos = pos.truncate();
                    camera.pos += pos;
                    camera.focal_length = clamp_min(camera.focal_length, 0.0);
                    camera.aperture_radius = clamp_min(camera.aperture_radius, 0.0);
                    camera.fov = clamp(camera.fov, std::f32::MIN_POSITIVE, 179.0);
                }

                // Toggle overlays
                Key::U => {
                    if !keys_down.contains(&key) {
                        viewport.overlays_enabled = !viewport.overlays_enabled;
                    }
                }

                // Toggle autofocus
                Key::N => {
                    if !keys_down.contains(&key) {
                        viewport.autofocus = !viewport.autofocus;
                        *render_buffer =
                            vec![Col::new(0.0, 0.0, 0.0); display_width * display_height];
                        viewport.sample_iter = 0;
                    }
                }

                Key::Enter => {
                    if !keys_down.contains(&key) {
                        viewport.depth_pass = !viewport.depth_pass;
                        *render_buffer =
                            vec![Col::new(0.0, 0.0, 0.0); display_width * display_height];
                        viewport.sample_iter = 0;
                    }
                }
                _ => (),
            };
        }
    });

    // Reset keys_down
    *keys_down = vec![];
    window.get_keys().map(|keys| {
        keys.iter().for_each(|key| keys_down.push(*key));
    });

    // Mouse movement
    window.get_unscaled_mouse_pos(MouseMode::Pass).map(|mouse| {
        if movement.mouse_movement
            != Vector3::new(mouse.0 / MOUSE_SENSITIVITY, mouse.1 / 100 as f32, 0.0)
        {
            let mouse_delta = Vector3::new(
                movement.mouse_movement.y - mouse.1 / MOUSE_SENSITIVITY,
                0.0,
                movement.mouse_movement.x - mouse.0 / MOUSE_SENSITIVITY,
            );

            camera.rot += mouse_delta * camera.fov / 90.0;

            // Constrain vertical rotation.
            camera.rot.x = clamp(
                camera.rot.x,
                std::f32::consts::PI / -2.0,
                std::f32::consts::PI / 2.0,
            );

            movement.mouse_movement.x = mouse.0 / MOUSE_SENSITIVITY;
            movement.mouse_movement.y = mouse.1 / MOUSE_SENSITIVITY;

            movement.camera_rotation = cgmath::Matrix4::from_angle_z(cgmath::Rad(camera.rot.z))
                * cgmath::Matrix4::from_angle_y(cgmath::Rad(camera.rot.y))
                * cgmath::Matrix4::from_angle_x(cgmath::Rad(camera.rot.x));

            *render_buffer = vec![Col::new(0.0, 0.0, 0.0); display_width * display_height];
            viewport.sample_iter = 0;

            movement.moving = true;
        } else {
            movement.moving = false;
        }
    });

    if movement.camera_movement == Vector3::new(0.0, 0.0, 0.0) {
        movement.moving = false;
    } else {
        movement.moving = true;
    }

    movement.camera_movement = Vector3::new(0.0, 0.0, 0.0);
}

pub fn autofocus(
    autofocus: bool,
    width: usize,
    height: usize,
    scene: &mut Scene,
    image_plane_size: f32,
    movement: &Movement,
) {
    if autofocus {
        let focus_probe = camera_ray_simple(
            width * height / 2 - width / 2,
            scene,
            image_plane_size,
            width,
            height,
            movement,
        );

        match raycast(&scene.spheres, focus_probe) {
            Some(point) => scene.cameras[0].focal_length = distance(scene.cameras[0].pos, point),
            None => scene.cameras[0].focal_length = 200.0,
        };
    }
}
