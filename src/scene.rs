use crate::bresenham::Line3d;
use crate::helpers::ObjectID;
use crate::helpers::{col_to_rgb_u32, Col};
use cgmath::Vector3;
use rand::{thread_rng, Rng};
use std::rc::Rc;

pub trait WorldObject {
    fn new(pos: Vector3<f32>, radius: f32, material: Material, object_id: ObjectID) -> Self
    where
        Self: Sized;
    fn pos(&self) -> Vector3<f32>;
    fn radius(&self) -> f32;
    fn object_id(&self) -> ObjectID;
}

#[derive(Debug, Clone)]
pub struct Camera {
    pub pos: Vector3<f32>,
    pub rot: Vector3<f32>,
    pub fov: f32,
    pub focal_length: f32,
    pub aperture_radius: f32,
    pub object_id: ObjectID,
}

#[derive(Debug, Clone)]
pub struct Sphere {
    pub pos: Vector3<f32>,
    pub radius: f32,
    pub material: Material,
    pub object_id: ObjectID,
}

impl WorldObject for Sphere {
    fn new(pos: Vector3<f32>, radius: f32, material: Material, object_id: ObjectID) -> Sphere {
        Sphere {
            pos,
            radius,
            material,
            object_id,
        }
    }

    fn pos(&self) -> Vector3<f32> {
        self.pos
    }
    fn radius(&self) -> f32 {
        self.radius
    }

    fn object_id(&self) -> ObjectID {
        self.object_id
    }
}

#[derive(Debug, Clone)]
pub struct Light {
    pub pos: Vector3<f32>,
    pub radius: f32,
    pub material: Material,
    pub intensity: f32,
    pub object_id: ObjectID,
}

#[derive(Debug, Clone)]
pub struct Sky {
    pub colors: Vec<Col>,
    pub intensity: f32,
}

pub struct Ray {
    pub pos: Vector3<f32>,
    pub dir: Vector3<f32>,
    pub from_wormhole: bool,
    pub from_object_id: ObjectID,
}

#[derive(Debug, Clone)]
pub struct WormholeParams {
    pub is_wormhole: bool,
    pub wormhole_offset: Vector3<f32>,
    pub other_end_object_id: ObjectID,
}

impl WormholeParams {
    fn none() -> WormholeParams {
        WormholeParams {
            is_wormhole: false,
            wormhole_offset: Vector3::new(0.0, 0.0, 0.0),
            other_end_object_id: ObjectID::from(0),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Material {
    pub color: Col,
    pub metallic: f32,
    pub roughness: f32,
    pub emission_color: Col,
    pub emission_intensity: f32,
    pub wormhole_params: WormholeParams,
}

#[derive(Debug, Clone)]
pub struct Wireframe {
    pub lines: Vec<Line3d>,
}

impl Wireframe {
    pub fn new(lines: Vec<Line3d>) -> Wireframe {
        Wireframe { lines: lines }
    }

    pub fn render(
        &self,
        buffer: &mut Vec<u32>,
        camera: &Camera,
        display_width: &'static usize,
        display_height: &'static usize,
    ) {
        for line in self.lines.iter() {
            line.render(buffer, camera, display_width, display_height);
        }
    }
}

#[derive(Debug, Clone)]
pub struct Scene {
    pub cameras: Vec<Camera>,
    pub spheres: Vec<Sphere>,
    // pub spheres: Vec<Rc<Sphere>>,
    pub sky: Sky,
    pub wireframes: Vec<Wireframe>,
}

pub fn initialize_scene() -> Scene {
    let mut rng = thread_rng();

    let mut object_id = ObjectID::from(0);

    let mut scene = Scene {
        cameras: vec![Camera {
            pos: Vector3::new(10.0, -10.0, 10.0),
            rot: Vector3::new(0.0, 0.0, 0.0),
            fov: 90.0,
            focal_length: 8.0,
            aperture_radius: 0.015,
            object_id: object_id.next(),
        }],
        spheres: vec![],
        sky: Sky {
            colors: vec![Col::new(0.3, 0.6, 0.9), Col::new(0.9, 0.9, 0.9)],
            intensity: 1.0,
        },
        wireframes: vec![],
    };

    let wormhole_pos = Vector3::new(0.0, 0.0, 8.0);
    let wormhole_offset = Vector3::new(-2.0, 12.0, -6.0);

    // Spheres

    let mut spheres = vec![
        // Wireframe sphere
        Sphere {
            // Rc::new(Sphere {
            pos: Vector3::new(10.0, 2.0, 1.0),
            radius: 1.0,
            material: Material {
                color: Col::new(0.1, 0.1, 0.1),
                metallic: 0.0,
                roughness: 1.0,
                emission_color: Col::new(1.0, 1.0, 1.0),
                emission_intensity: 0.0,
                wormhole_params: WormholeParams::none(),
            },
            object_id: object_id.next(),
        },
        // Origin
        Sphere {
            // Rc::new(Sphere {
            pos: Vector3::new(2.0, 0.0, 0.0),
            radius: 0.3,
            material: Material {
                color: Col::new(1.0, 0.0, 0.0),
                metallic: 0.0,
                roughness: 1.0,
                emission_color: Col::new(1.0, 1.0, 1.0),
                emission_intensity: 0.0,
                wormhole_params: WormholeParams::none(),
            },
            object_id: object_id.next(),
        },
        Sphere {
            // Rc::new(Sphere {
            pos: Vector3::new(0.0, 2.0, 0.0),
            radius: 0.3,
            material: Material {
                color: Col::new(0.0, 1.0, 0.0),
                metallic: 0.0,
                roughness: 1.0,
                emission_color: Col::new(1.0, 1.0, 1.0),
                emission_intensity: 0.0,
                wormhole_params: WormholeParams::none(),
            },
            object_id: object_id.next(),
        },
        Sphere {
            // Rc::new(Sphere {
            pos: Vector3::new(0.0, 0.0, 2.0),
            radius: 0.3,
            material: Material {
                color: Col::new(0.1, 0.3, 1.0),
                metallic: 0.0,
                roughness: 1.0,
                emission_color: Col::new(1.0, 1.0, 1.0),
                emission_intensity: 0.0,
                wormhole_params: WormholeParams::none(),
            },
            object_id: object_id.next(),
        },
        // Light
        Sphere {
            // Rc::new(Sphere {
            pos: Vector3::new(-6.0, 0.0, 2.0),
            radius: 3.0,
            material: Material {
                color: Col::new(0.0, 0.0, 0.0),
                metallic: 0.0,
                roughness: 1.0,
                emission_color: Col::new(1.0, 1.0, 1.0),
                // emission_color: Col::new(4.0, 2.0, 1.0),
                emission_intensity: 1.0,
                wormhole_params: WormholeParams::none(),
            },
            object_id: object_id.next(),
        },
        // Wormhole pt. 1
        Sphere {
            // Rc::new(Sphere {
            pos: wormhole_pos,
            radius: 2.0,
            material: Material {
                color: Col::new(0.0, 0.0, 0.0),
                metallic: 0.0,
                roughness: 1.0,
                emission_color: Col::new(1.0, 1.0, 1.0),
                emission_intensity: 1.0,
                wormhole_params: WormholeParams {
                    is_wormhole: true,
                    wormhole_offset: wormhole_offset,
                    other_end_object_id: object_id.next() + 1,
                },
            },
            object_id: object_id,
        },
        // Wormhole pt. 2
        Sphere {
            // Rc::new(Sphere {
            pos: wormhole_pos + wormhole_offset,
            radius: 2.0,
            material: Material {
                color: Col::new(0.0, 0.0, 0.0),
                metallic: 0.0,
                roughness: 1.0,
                emission_color: Col::new(1.0, 1.0, 1.0),
                emission_intensity: 1.0,
                wormhole_params: WormholeParams {
                    is_wormhole: true,
                    wormhole_offset: wormhole_offset * -1.0,
                    other_end_object_id: object_id,
                },
            },
            object_id: object_id.next(),
        },
    ];
    for i in 0..6 {
        spheres.push(Sphere {
            // spheres.push(Rc::new(Sphere {
            pos: Vector3::new(-7.5 + 2.5 * i as f32, 8.0, 1.0),
            radius: 1.0,
            material: Material {
                color: Col::new(1.0, 1.0, 1.0),
                metallic: 1.0,
                roughness: (i as f32 / 6.0).powi(2),
                emission_color: Col::new(1.0, 1.0, 1.0),
                emission_intensity: 0.0,
                wormhole_params: WormholeParams::none(),
            },
            object_id: object_id.next(),
        });
    }

    for i in 0..6 {
        spheres.push(Sphere {
            // spheres.push(Rc::new(Sphere {
            pos: Vector3::new(-7.5 + 2.5 * i as f32, 8.0, -2.0),
            radius: 1.0,
            material: Material {
                color: Col::new(1.0, 1.0, 1.0),
                metallic: 1.0,
                roughness: 0.0,
                emission_color: Col::new(1.0, 1.0, 1.0),
                emission_intensity: 0.0,
                wormhole_params: WormholeParams::none(),
            },
            object_id: object_id.next(),
        });
    }

    for _ in 0..50 {
        let rnd = rng.gen_range(0.0, 1.0);

        spheres.push(Sphere {
            // spheres.push(Rc::new(Sphere {
            pos: Vector3::new(
                rng.gen_range(-5.0, 5.0),
                rng.gen_range(-5.0, 5.0),
                rng.gen_range(-5.0, 5.0),
            ),
            radius: rng.gen_range(0.5, 1.0),
            material: Material {
                color: Col::new(
                    rng.gen_range(0.0, 1.0),
                    rng.gen_range(0.0, 1.0),
                    rng.gen_range(0.0, 1.0),
                ),
                metallic: if rnd < 0.5 { 0.0 } else { 1.0 },
                roughness: 0.0,
                emission_color: Col::new(1.0, 1.0, 1.0),
                emission_intensity: 0.0,
                wormhole_params: WormholeParams::none(),
            },
            object_id: object_id.next(),
        });
    }
    scene.spheres = spheres;

    // Wireframes

    let mut wireframes = vec![];

    let origin = Wireframe::new(vec![
        Line3d::new(
            Vector3::new(0.0, 0.0, 0.0),
            Vector3::new(1.0, 0.0, 0.0),
            Col::new(1.0, 0.0, 0.0),
        ),
        Line3d::new(
            Vector3::new(0.0, 0.0, 0.0),
            Vector3::new(0.0, 1.0, 0.0),
            Col::new(0.0, 1.0, 0.0),
        ),
        Line3d::new(
            Vector3::new(0.0, 0.0, 0.0),
            Vector3::new(0.0, 0.0, 1.0),
            Col::new(0.1, 0.3, 1.0),
        ),
    ]);

    let mut sphere_test: Wireframe = Wireframe::new(vec![]);
    let v_iter = 8.0;
    let h_iter = v_iter * 2.0;

    for l in 0..v_iter as usize {
        for r in 0..h_iter as usize {
            let radius = 1.0;
            let col = Col::new(0.5, 0.5, 0.5);
            let t1 = l as f32;
            let t2 = l as f32 + 1.0;

            let w1 = (t1 as f32 / v_iter * std::f32::consts::PI).sin() * radius;
            let w2 = (t2 as f32 / v_iter * std::f32::consts::PI).sin() * radius;

            // Vertical line

            let line1_p1 = Vector3::new(
                (r as f32 / h_iter * 2.0 * std::f32::consts::PI).sin() * w1,
                (r as f32 / h_iter * 2.0 * std::f32::consts::PI).cos() * w1,
                (t1 as f32 / v_iter * std::f32::consts::PI).cos() * radius,
            );

            let line1_p2 = Vector3::new(
                (r as f32 / h_iter * 2.0 * std::f32::consts::PI).sin() * w2,
                (r as f32 / h_iter * 2.0 * std::f32::consts::PI).cos() * w2,
                (t2 as f32 / v_iter * std::f32::consts::PI).cos() * radius,
            );

            // Horizontal line

            let line2_p1 = Vector3::new(
                (r as f32 / h_iter * 2.0 * std::f32::consts::PI).sin() * w1,
                (r as f32 / h_iter * 2.0 * std::f32::consts::PI).cos() * w1,
                (t1 as f32 / v_iter * std::f32::consts::PI).cos() * radius,
            );

            let line2_p2 = Vector3::new(
                ((r + 1) as f32 / h_iter * 2.0 * std::f32::consts::PI).sin() * w1,
                ((r + 1) as f32 / h_iter * 2.0 * std::f32::consts::PI).cos() * w1,
                (t1 as f32 / v_iter * std::f32::consts::PI).cos() * radius,
            );

            let pos = scene.spheres[0].pos;
            let line1 = Line3d::new(line1_p1 + pos, line1_p2 + pos, col);
            let line2 = Line3d::new(line2_p1 + pos, line2_p2 + pos, col);
            sphere_test.lines.push(line1);
            sphere_test.lines.push(line2);
        }
    }

    wireframes.push(origin);
    wireframes.push(sphere_test);

    scene.wireframes = wireframes;

    return scene;
}
