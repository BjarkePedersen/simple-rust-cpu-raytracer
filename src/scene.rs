use cgmath::Vector3;
use rgb;

use rand::prelude::*;

#[derive(Debug, Clone)]
pub struct Camera {
    pub pos: Vector3<f32>,
    pub rot: Vector3<f32>,
    pub focal_length: f32,
    pub sensor_size: f32,
}

#[derive(Debug, Clone)]
pub struct Sphere {
    pub pos: Vector3<f32>,
    pub radius: f32,
    pub material: Material,
}

pub struct Ray {
    pub p1: Vector3<f32>,
    pub p2: Vector3<f32>,
}

#[derive(Debug, Clone)]
pub struct Material {
    pub color: u32,
}

#[derive(Debug, Clone)]
pub struct Scene {
    pub cameras: Vec<Camera>,
    pub spheres: Vec<Sphere>,
}

pub fn initialize_scene() -> Scene {
    let mut rng = thread_rng();

    let mut spheres = vec![
        Sphere {
            pos: Vector3::new(0.0, 0.0, 2.0),
            radius: 1.0,
            material: Material {
                color: rgb(0, 50, 155),
            },
        },
        Sphere {
            pos: Vector3::new(0.0, 0.0, 0.0),
            radius: 2.0,
            material: Material {
                color: rgb(255, 50, 50),
            },
        },
    ];

    for _i in 0..10 {
        spheres.push(Sphere {
            pos: Vector3::new(
                rng.gen_range(-5.0, 5.0),
                rng.gen_range(-5.0, 5.0),
                rng.gen_range(-5.0, 5.0),
            ),
            radius: rng.gen_range(0.25, 1.0),
            material: Material {
                color: rgb(
                    rng.gen_range(0, 200),
                    rng.gen_range(0, 200),
                    rng.gen_range(0, 200),
                ),
            },
        });
    }

    let scene = Scene {
        cameras: vec![Camera {
            pos: Vector3::new(0.0, -5.0, 0.0),
            rot: Vector3::new(0.0, 0.0, 0.0),
            focal_length: 0.35,
            sensor_size: 1.0,
        }],
        spheres: spheres,
    };

    return scene;
}
