use cgmath::Vector3;
use helpers::Col;

use rand::prelude::*;

#[derive(Debug, Clone)]
pub struct Camera {
    pub pos: Vector3<f32>,
    pub rot: Vector3<f32>,
    pub focal_length: f32,
    pub sensor_size: f32,
    pub focus_distance: f32,
    pub apeture_size: f32,
}

#[derive(Debug, Clone)]
pub struct Sphere {
    pub pos: Vector3<f32>,
    pub radius: f32,
    pub material: Material,
}

#[derive(Debug, Clone)]
pub struct Light {
    pub pos: Vector3<f32>,
    pub radius: f32,
    pub material: Material,
    pub intensity: f32,
}

#[derive(Debug, Clone)]
pub struct Sky {
    pub colors: Vec<Col>,
    pub intensity: f32,
}

pub struct Ray {
    pub pos: Vector3<f32>,
    pub dir: Vector3<f32>,
}

#[derive(Debug, Clone)]
pub struct Material {
    pub color: Col,
}

#[derive(Debug, Clone)]
pub struct Scene {
    pub cameras: Vec<Camera>,
    pub spheres: Vec<Sphere>,
    pub sky: Sky,
}

pub fn initialize_scene() -> Scene {
    let mut rng = thread_rng();

    let mut spheres = vec![
        Sphere {
            pos: Vector3::new(0.0, 0.0, 2.0),
            radius: 1.0,
            material: Material {
                color: Col::new(0.0, 0.2, 0.6),
            },
        },
        Sphere {
            pos: Vector3::new(0.0, 0.0, 0.0),
            radius: 2.0,
            material: Material {
                color: Col::new(1.0, 0.2, 0.2),
            },
        },
    ];

    for _i in 0..50 {
        spheres.push(Sphere {
            pos: Vector3::new(
                rng.gen_range(-5.0, 5.0),
                rng.gen_range(-5.0, 5.0),
                rng.gen_range(-5.0, 5.0),
            ),
            radius: rng.gen_range(0.25, 1.0),
            material: Material {
                color: Col::new(
                    rng.gen_range(0.0, 1.0),
                    rng.gen_range(0.0, 1.0),
                    rng.gen_range(0.0, 1.0),
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
            focus_distance: 5.0,
            apeture_size: 100.0,
        }],
        spheres: spheres,
        sky: Sky {
            colors: vec![Col::new(0.9, 0.875, 0.85), Col::new(0.078, 0.4, 1.0)],
            intensity: 1.0,
        },
    };

    return scene;
}
