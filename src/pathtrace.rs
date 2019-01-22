use crate::helpers::Col;
use crate::intersect::*;
use crate::scene::{Ray, Scene, Sphere};
use crate::skybox::sky_box;
use cgmath::{dot, InnerSpace};
use ordered_float::OrderedFloat;

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

    let closest: Option<(usize, f32)> = spheres.iter().enumerate().filter_map(|(i, sphere)| {
        if Some(i) == ignore {
            None
        } else {
            Some((i, sphere.intersect(ray)?))
        }
    }).min_by_key(|(_, distance)| OrderedFloat(*distance));

    if depth_pass {
        if let Some((_, distance)) = closest {
            let d = distance / 20.0;
            col = Col::new(d, d, d).clamp(0.0, 1.0);
        }
        return col;
    }

    // Bounce:
    // Make new ray from old ray, bounce_point, and bounce_sphere
    // Recursively call intersect_spheres with new ray

    let conditions = bounce_count < max_bounces && !depth_pass;

    if conditions {
        if let Some((i, t)) = closest {
            let bounce_point = ray.pos + ray.dir * t;
            let bounce_sphere = &spheres[i];

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
    };

    return col;
}