use crate::helpers::{distance, Col};
use crate::intersect::*;
use crate::scene::{Ray, Scene, Sphere};
use crate::skybox::sky_box;
use cgmath::{dot, InnerSpace, Vector3};

pub fn intersect_spheres(
    max_bounces: i32,
    bounce_count: i32,
    scene: &Scene,
    depth_pass: bool,
    spheres: &[Sphere],
    ignore: Option<usize>,
    ray: &Ray,
) -> Col {
    let mut col = sky_box(scene, ray);
    let mut closest_intersection: f32 = std::f32::MAX;
    let mut bounce_point: Option<(Vector3<f32>)> = None;
    let mut bounce_sphere: Option<(usize, &Sphere)> = None;

    for (i, sphere) in spheres.iter().enumerate() {
        if Some(i) == ignore {
            continue;
        }
        let solution = sphere.intersect(ray);
        if let Some(solution) = solution {
            let intersect_point = ray.pos + solution * ray.dir;
            let distance = distance(scene.cameras[0].pos, intersect_point);

            if distance < closest_intersection {
                closest_intersection = distance;

                bounce_point = Some(intersect_point.clone());
                bounce_sphere = Some((i, &sphere));

                if !depth_pass {
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

    let conditions = bounce_count < max_bounces && !depth_pass;

    if conditions {
        if let Some(bounce_point) = bounce_point {
            if let Some((i, bounce_sphere)) = bounce_sphere {
                // Normal at intersection point
                let n = (bounce_point - bounce_sphere.pos).normalize();

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
                    depth_pass,
                    &spheres,
                    Some(i),
                    &ray,
                ) * bounce_sphere.material.color;
            }
        }
    };

    return col;
}
