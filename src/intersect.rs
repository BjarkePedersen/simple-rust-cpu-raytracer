use crate::app::Viewport;
use crate::helpers::{clamp, distance, mix_col, Col};
use crate::scene::{Ray, Scene, Sphere};
use cgmath::InnerSpace;
use cgmath::Vector3;

pub trait Intersect {
    fn intersect(&self, ray: &Ray) -> Option<(Vector3<f32>)>;
}

impl Intersect for Sphere {
    fn intersect(&self, ray: &Ray) -> Option<(Vector3<f32>)> {
        let o = ray.pos;
        let l = ray.dir;
        let c = self.pos;
        let r = self.radius;

        let o_c = o - c;
        let l_dot_o_c = l.dot(o_c);

        let discriminant = l_dot_o_c.powi(2) - o_c.magnitude2() + r.powi(2);

        if discriminant > 0.0 && l_dot_o_c < 0.0 {
            let discriminant_sqrt = discriminant.sqrt();

            let d_1 = -l_dot_o_c + discriminant_sqrt;
            let d_2 = -l_dot_o_c - discriminant_sqrt;

            let solution = d_1.min(d_2);
            return Some(o + solution * l);
        } else {
            return None;
        }
    }
}

pub fn intersect_spheres(
    scene: &Scene,
    viewport: &Viewport,
    sorted_spheres: &Vec<Sphere>,
    ray: &Ray,
    col: &mut Col,
) {
    let mut closest_ray: f32 = std::f32::MAX;
    let mut first_intersect_distance = std::f32::MAX;

    for sphere in sorted_spheres {
        let intersect_point = sphere.intersect(ray);
        if let Some(intersect_point) = intersect_point {
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
                    // Fog
                    *col = mix_col(
                        sphere.material.color,
                        *col,
                        clamp(1.0 / (distance / 20.0), 0.0, 1.0),
                    );
                } else {
                    *col =
                        Col::new(distance / 20.0, distance / 20.0, distance / 20.0).clamp(0.0, 1.0);
                }
            }
        }
    }
}
