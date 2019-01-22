use crate::app::Viewport;
use crate::helpers::{clamp_min, clamp, Col};
use crate::scene::Camera;
use cgmath::{Matrix4, Vector3};
use minifb::{Key, MouseMode};

pub struct Movement {
    pub camera_movement: Vector3<f32>,
    pub mouse_movement: Vector3<f32>,
    pub moving: bool,
}

pub fn handle_movement(
    window: &mut minifb::Window,
    viewport: &mut Viewport,
    camera: &mut Camera,
    rgb_buffer: &mut Vec<(Col)>,
    movement: &mut Movement,
    rot: &mut Matrix4<f32>,
    keys_down: &mut Vec<Key>,
    display_width: &usize,
    display_height: &usize,
) {
    const MOVE_SPEED: f32 = 0.2;
    const ROT_SPEED: f32 = 0.1;

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
                // Key::Q => camera.rot.y += ROT_SPEED,
                // Key::E => camera.rot.y -= ROT_SPEED,
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
                    *rot = cgmath::Matrix4::from_angle_z(cgmath::Rad(camera.rot.z))
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
                // | Key::Q
                // | Key::E
                | Key::J
                | Key::L
                | Key::I
                | Key::M 
                | Key::Z
                | Key::X 
                => {
                    *rgb_buffer = vec![Col::new(0.0, 0.0, 0.0); display_width * display_height];
                    viewport.sample_iter = 0;

                    let pos = *rot * movement.camera_movement.extend(0.0);
                    let pos = pos.truncate();
                    camera.pos += pos;
                    camera.focal_length = clamp_min(camera.focal_length, 0.0);
                    camera.aperture_radius = clamp_min(camera.aperture_radius, 0.0);
                    camera.fov = clamp(camera.fov, std::f32::MIN_POSITIVE, 179.0);
                }

                Key::U => {
                    if !keys_down.contains(&key) {
                        viewport.overlays_enabled = !viewport.overlays_enabled;
                    }
                }

                Key::Enter => {
                    if !keys_down.contains(&key) {
                        viewport.depth_pass = !viewport.depth_pass;
                        *rgb_buffer = vec![Col::new(0.0, 0.0, 0.0); display_width * display_height];
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
        if movement.mouse_movement != Vector3::new(mouse.0 / 100 as f32, mouse.1 / 100 as f32, 0.0)
        {
            let mouse_delta = vec![
                -movement.mouse_movement.x + mouse.0 / 100.0,
                movement.mouse_movement.y - mouse.1 / 100.0,
            ];

            camera.rot.z -= mouse_delta[0] * camera.fov / 90.0;
            camera.rot.x += mouse_delta[1] * camera.fov / 90.0;

            // Constrain vertical rotation.
            if camera.rot.x > std::f32::consts::PI / 2.0 {
                camera.rot.x = std::f32::consts::PI / 2.0;
            } else if camera.rot.x < std::f32::consts::PI / -2.0 {
                camera.rot.x = std::f32::consts::PI / -2.0
            }

            movement.mouse_movement.x = mouse.0 / 100.0;
            movement.mouse_movement.y = mouse.1 / 100.0;

            *rot = cgmath::Matrix4::from_angle_z(cgmath::Rad(camera.rot.z))
                * cgmath::Matrix4::from_angle_y(cgmath::Rad(camera.rot.y))
                * cgmath::Matrix4::from_angle_x(cgmath::Rad(camera.rot.x));

            *rgb_buffer = vec![Col::new(0.0, 0.0, 0.0); display_width * display_height];
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
