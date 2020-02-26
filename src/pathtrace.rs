use crate::helpers::{clamp, uv, Col};
use crate::intersect::*;
use crate::movement::Movement;
use crate::scene::{Ray, Scene, Sphere};
use crate::skybox::sky_box;
use cgmath::{dot, InnerSpace, Vector3};
use ordered_float::OrderedFloat;
use rand::{Rng, ThreadRng};

// Create ray from camera
pub fn camera_ray(
    i: usize,
    scene: &Scene,
    image_plane_size: f32,
    jitter_size: f32,
    pixel_size: f32,
    width: usize,
    height: usize,
    movement: &Movement,
    rng: &mut ThreadRng,
) -> Ray {
    let jitter_angle = rng.gen_range(0.0, 1.0) * std::f32::consts::PI * 2.0;
    let jitter_length = (rng.gen_range(0.0, 1.0) as f32).sqrt();
    let jitter_x = jitter_length * jitter_angle.cos();
    let jitter_z = jitter_length * jitter_angle.sin();

    let aperture_jitter =
        Vector3::new(jitter_x, 0.0, jitter_z) * 2.0 * scene.cameras[0].aperture_radius;

    let anti_aliasing_jitter = Vector3::new(
        rng.gen_range(-1.0, 1.0) * pixel_size,
        0.0,
        rng.gen_range(-1.0, 1.0) * pixel_size,
    );

    let dir = {
        let uv = uv(width * height - i - 1);

        Vector3::new(
            ((uv.x - width as f32 / 2.0) / height as f32) * -image_plane_size
                + jitter_x * jitter_size,
            1.0,
            ((uv.y - height as f32 / 2.0) / height as f32) * image_plane_size
                + jitter_z * jitter_size,
        ) - aperture_jitter
            + anti_aliasing_jitter
    };

    let dir = (movement.camera_rotation * dir.extend(0.0)).truncate();

    let combined_jitter = aperture_jitter + anti_aliasing_jitter;
    let combined_jitter = (movement.camera_rotation * combined_jitter.extend(0.0)).truncate();

    return Ray {
        pos: scene.cameras[0].pos + combined_jitter,
        dir: dir.normalize(),
    };
}

// Create ray from camera with no jittering (used for autofocus)
pub fn camera_ray_simple(
    i: usize,
    scene: &Scene,
    image_plane_size: f32,
    width: usize,
    height: usize,
    movement: &Movement,
) -> Ray {
    let dir = {
        let uv = uv(width * height - i - 1);

        Vector3::new(
            ((uv.x - width as f32 / 2.0) / height as f32) * -image_plane_size,
            1.0,
            ((uv.y - height as f32 / 2.0) / height as f32) * image_plane_size,
        )
    };

    let dir = (movement.camera_rotation * dir.extend(0.0)).truncate();

    return Ray {
        pos: scene.cameras[0].pos,
        dir: dir.normalize(),
    };
}

pub fn intersect_spheres(
    max_bounces: i32,
    bounce_count: i32,
    scene: &Scene,
    depth_pass: bool,
    spheres: &[Sphere],
    ignore: Option<usize>,
    ray: &Ray,
    rng: &mut ThreadRng,
) -> Col {
    let mut col = sky_box(scene, ray);

    let closest: Option<(usize, f32)> = spheres
        .iter()
        .enumerate()
        .filter_map(|(i, sphere)| {
            if Some(i) == ignore {
                // Prevents intersection with reflected sphere
                None
            } else {
                Some((i, sphere.intersect(ray)?))
            }
        })
        .min_by_key(|(_, distance)| OrderedFloat(*distance));

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

            let roughness = bounce_sphere.material.roughness;

            // Reflected vector
            let specular = d - 2.0 * dot(d, n) * n;

            let metallic = bounce_sphere.material.metallic;

            // Schlick aproximation
            let n1: f32 = 1.0;
            let n2: f32 = 1.5;
            let r0 = ((n1 - n2) / (n1 + n2)).powi(2);
            let cos_x = -dot(n, d);

            let x = 1.0 - cos_x;
            let fresnel = clamp(r0 + (1.0 - r0) * x.powi(5), 0.0, 1.0);

            // BRDF
            let diffuse = n + Vector3::new(
                rng.gen_range(-0.5, 0.5) * std::f32::consts::PI,
                rng.gen_range(-0.5, 0.5) * std::f32::consts::PI,
                rng.gen_range(-0.5, 0.5) * std::f32::consts::PI,
            );

            let specular = diffuse * roughness + specular * (1.0 - roughness);

            // Reflected rays
            let ray_specular = Ray {
                pos: bounce_point,
                dir: specular.normalize(),
            };
            let ray_diffuse = Ray {
                pos: bounce_point,
                dir: diffuse.normalize(),
            };

            let specular = intersect_spheres(
                max_bounces,
                bounce_count + 1,
                &scene,
                depth_pass,
                &spheres,
                Some(i),
                &ray_specular,
                rng,
            );

            let diffuse = intersect_spheres(
                max_bounces,
                bounce_count + 1,
                &scene,
                depth_pass,
                &spheres,
                Some(i),
                &ray_diffuse,
                rng,
            ) * bounce_sphere.material.color;

            let emission =
                bounce_sphere.material.emission_color * bounce_sphere.material.emission_intensity;

            let dielectric = specular * fresnel + diffuse * (1.0 - fresnel) + emission;

            return dielectric * (1.0 - metallic)
                + specular * bounce_sphere.material.color * metallic;
        }
    };

    return col;
}

pub fn raycast(spheres: &[Sphere], ray: Ray) -> Option<Vector3<f32>> {
    let closest: Option<(usize, f32)> = spheres
        .iter()
        .enumerate()
        .filter_map(|(i, sphere)| Some((i, sphere.intersect(&ray)?)))
        .min_by_key(|(_, distance)| OrderedFloat(*distance));

    if let Some((_, t)) = closest {
        let bounce_point = ray.pos + ray.dir * t;
        return Some(bounce_point);
    } else {
        return None;
    };
}
