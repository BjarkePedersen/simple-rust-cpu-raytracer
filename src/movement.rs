use cgmath::{Matrix4, Vector3};
use helpers::Col;
use minifb::{Key, MouseMode};
use render::Render;
use scene::Scene;

pub fn handle_movement(
    window: &mut minifb::Window,
    scene: &mut Scene,
    rgb_buffer: &mut Vec<(Col)>,
    render: &mut Render,
    movement: &mut Vector3<f32>,
    rot: &mut Matrix4<f32>,
    WIDTH: &usize,
    HEIGHT: &usize,
) {
    const MOVE_SPEED: f32 = 0.2;
    const ROT_SPEED: f32 = 0.1;

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
                Key::J => scene.cameras[0].focus_distance -= 0.1,
                Key::L => scene.cameras[0].focus_distance += 0.1,
                Key::M => scene.cameras[0].apeture_size -= 10.0,
                Key::I => scene.cameras[0].apeture_size += 10.0,
                _ => (),
            };
            match t {
                Key::Left | Key::Right | Key::Up | Key::Down | Key::Q | Key::E => {
                    *rot = cgmath::Matrix4::from_angle_z(cgmath::Rad(scene.cameras[0].rot.z))
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
                    *rgb_buffer = vec![Col::new(0.0, 0.0, 0.0); WIDTH * HEIGHT];
                    render.sample_iter = 0;

                    let pos = *rot * movement.extend(0.0);
                    let pos = pos.truncate();
                    scene.cameras[0].pos += pos;
                }
                Key::Enter => {
                    render.distance_pass = !render.distance_pass;
                    *rgb_buffer = vec![Col::new(0.0, 0.0, 0.0); WIDTH * HEIGHT];
                    render.sample_iter = 0;
                }
                _ => (),
            };
        }
    });
}

pub fn handle_mouse_movement(
    window: &mut minifb::Window,
    scene: &mut Scene,
    rgb_buffer: &mut Vec<(Col)>,
    render: &mut Render,
    movement: &mut Vector3<f32>,
    mouse_movement: &mut Vector3<f32>,
    moving: &mut bool,
    rot: &mut Matrix4<f32>,
    WIDTH: &usize,
    HEIGHT: &usize,
) {
    window.get_unscaled_mouse_pos(MouseMode::Pass).map(|mouse| {
        if *mouse_movement != Vector3::new(mouse.0 / 100 as f32, mouse.1 / 100 as f32, 0.0) {
            mouse_movement.x = mouse.0 / 100 as f32;
            mouse_movement.y = mouse.1 / 100 as f32;

            scene.cameras[0].rot.z = -mouse_movement.x;
            scene.cameras[0].rot.x = -mouse_movement.y;

            *rot = cgmath::Matrix4::from_angle_z(cgmath::Rad(scene.cameras[0].rot.z))
                * cgmath::Matrix4::from_angle_y(cgmath::Rad(scene.cameras[0].rot.y))
                * cgmath::Matrix4::from_angle_x(cgmath::Rad(scene.cameras[0].rot.x));

            *rgb_buffer = vec![Col::new(0.0, 0.0, 0.0); WIDTH * HEIGHT];
            render.sample_iter = 0;

            *moving = true;
        } else {
            *moving = false;
        }
    });

    if *movement == Vector3::new(0.0, 0.0, 0.0) {
        *moving = false;
    } else {
        *moving = true;
    }
}
