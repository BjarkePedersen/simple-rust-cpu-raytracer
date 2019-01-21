use crate::app::Viewport;
use crate::helpers::{clamp, distance, mix_col, Col};
use crate::scene::{Ray, Scene, Sphere};
use crate::sky_box::sky_box;
use cgmath::{dot, InnerSpace, Vector3};

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
    max_bounces: i32,
    bounce_count: i32,
    scene: &Scene,
    viewport: &Viewport,
    spheres: &Vec<Sphere>,
    ray: &Ray,
) -> Col {
    let mut col = sky_box(scene, ray);
    let mut closest_intersection: f32 = std::f32::MAX;
    let mut bounce_point: Option<(Vector3<f32>)> = None;
    let mut bounce_sphere: Option<&Sphere> = None;

    for sphere in spheres {
        let intersect_point = sphere.intersect(ray);
        if let Some(intersect_point) = intersect_point {
            let distance = distance(scene.cameras[0].pos, intersect_point);

            if distance < closest_intersection {
                closest_intersection = distance;

                bounce_point = Some(intersect_point.clone());
                bounce_sphere = Some(&sphere);

                if !viewport.distance_pass {
                    col = Col::new(0.0, 0.0, 0.0);
                } else {
                    let d = distance / 20.0;
                    col = Col::new(d, d, d).clamp(0.0, 1.0);
                }
            }
        }
    }

    // Bounce:
    // Make new ray from old ray, bounce_point, and bounce_sphere
    // Recursively call intersect_spheres with new ray
    if bounce_count < max_bounces {
        if let Some(bounce_point) = bounce_point {
            if let Some(bounce_sphere) = bounce_sphere {
                // Normal at intersection point
                let n = bounce_point - bounce_sphere.pos;

                // Incoming ray vector
                let d = ray.dir;

                // Reflected vector
                let dir = d - 2.0 * (dot(d, n)) * n;

                // Reflected ray
                let ray = Ray {
                    pos: bounce_point,
                    dir: dir.normalize(),
                };

                return intersect_spheres(
                    max_bounces,
                    bounce_count + 1,
                    &scene,
                    &viewport,
                    &spheres,
                    &ray,
                ) * bounce_sphere.material.color;
            }
        }
    };

    return col;
}
