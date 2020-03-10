use crate::helpers::{clamp, uv, Col, ObjectID};
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
        from_wormhole: false,
        from_object_id: ObjectID::from(0),
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
        from_wormhole: false,
        from_object_id: ObjectID::from(0),
    };
}

pub fn intersect_spheres(
    max_bounces: i32,
    max_wormhole_bounces: i32,
    bounce_count: i32,
    wormhole_bounce_count: i32,
    scene: &Scene,
    depth_pass: bool,
    spheres: &[Sphere],
    from_object_id: ObjectID,
    ray: &Ray,
    rng: &mut ThreadRng,
) -> Col {
    let mut col = sky_box(scene, ray);

    let closest: Option<(usize, f32)> = spheres
        .iter()
        .enumerate()
        .filter_map(|(i, sphere)| {
            if sphere.material.wormhole_params.is_wormhole {
                if ray.from_wormhole {
                    if from_object_id == sphere.material.wormhole_params.other_end_object_id {
                        // Ignore the wormhole the ray exited from
                        None
                    } else {
                        Some((i, sphere.intersect(ray)?))
                    }
                } else {
                    Some((i, sphere.intersect(ray)?))
                }
            } else if sphere.object_id == from_object_id {
                // Prevents intersection with reflected spher
                None
            } else {
                Some((i, sphere.intersect(ray)?))
            }
        })
        .min_by_key(|(_, distance)| OrderedFloat(*distance));

    if depth_pass {
        if bounce_count < max_bounces {
            if let Some((i, distance)) = closest {
                let d = 1.0 - 1.0 / (distance + 1.0);
                col = Col::new(d, d, d).clamp(0.0, 1.0);
            }
        }
        return col;
    }

    // Bounce:
    // Make new ray from old ray, bounce_point, and bounce_sphere
    // Recursively call intersect_spheres with new ray

    let should_terminate =
        bounce_count > max_bounces || wormhole_bounce_count > max_wormhole_bounces;

    if !should_terminate {
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

            let is_wormhole = bounce_sphere.material.wormhole_params.is_wormhole;
            let wormhole_factor = 1.0 - clamp(1.0 * r0 + (1.0 - r0) * x.powi(2), 0.0, 1.0);

            if is_wormhole {
                let angle: f32 = 0.2;
                let mut dir = ray.dir;

                let ray = Ray {
                    pos: ray.pos
                        + bounce_sphere.material.wormhole_params.wormhole_offset * wormhole_factor,
                    dir: dir,
                    from_wormhole: true,
                    from_object_id: bounce_sphere.object_id.clone(),
                };

                col = intersect_spheres(
                    max_bounces,
                    max_wormhole_bounces,
                    bounce_count,
                    wormhole_bounce_count + 1,
                    &scene,
                    depth_pass,
                    &spheres,
                    bounce_sphere.object_id,
                    &ray,
                    rng,
                );
                return col;
            } else if metallic < 1.0 {
                // Reflected rays
                let ray_specular = Ray {
                    pos: bounce_point,
                    dir: specular.normalize(),
                    from_wormhole: false,
                    from_object_id: ObjectID::from(0),
                };
                let ray_diffuse = Ray {
                    pos: bounce_point,
                    dir: diffuse.normalize(),
                    from_wormhole: false,
                    from_object_id: ObjectID::from(0),
                };

                let specular = intersect_spheres(
                    max_bounces,
                    max_wormhole_bounces,
                    bounce_count + 1,
                    wormhole_bounce_count,
                    &scene,
                    depth_pass,
                    &spheres,
                    bounce_sphere.object_id,
                    &ray_specular,
                    rng,
                );

                let diffuse = intersect_spheres(
                    max_bounces,
                    max_wormhole_bounces,
                    bounce_count + 1,
                    wormhole_bounce_count,
                    &scene,
                    depth_pass,
                    &spheres,
                    bounce_sphere.object_id,
                    &ray_diffuse,
                    rng,
                ) * bounce_sphere.material.color;

                let emission = bounce_sphere.material.emission_color
                    * bounce_sphere.material.emission_intensity;

                let dielectric = specular * fresnel + diffuse * (1.0 - fresnel) + emission;

                col = dielectric * (1.0 - metallic)
                    + specular * bounce_sphere.material.color * metallic;
            } else {
                let specular = specular * (1.0 - roughness) + diffuse * roughness;
                let ray_specular = Ray {
                    pos: bounce_point,
                    dir: specular.normalize(),
                    from_wormhole: false,
                    from_object_id: ObjectID::from(0),
                };

                col = intersect_spheres(
                    max_bounces,
                    max_wormhole_bounces,
                    bounce_count + 1,
                    wormhole_bounce_count,
                    &scene,
                    depth_pass,
                    &spheres,
                    bounce_sphere.object_id,
                    &ray_specular,
                    rng,
                ) * bounce_sphere.material.color;
            }
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

    if let Some((i, distance)) = closest {
        let bounce_point = if spheres[i].material.wormhole_params.is_wormhole {
            let bounce_point = ray.pos + ray.dir * distance;
            let bounce_sphere = &spheres[i];

            // Normal at intersection point
            let n = (bounce_point - bounce_sphere.pos).normalize();

            // Incoming ray vector
            let d = ray.dir;

            let n1: f32 = 1.0;
            let n2: f32 = 1.5;
            let r0 = ((n1 - n2) / (n1 + n2)).powi(2);
            let cos_x = -dot(n, d);

            let x = 1.0 - cos_x;
            let wormhole_factor = 1.0 - clamp(1.0 * r0 + (1.0 - r0) * x.powi(2), 0.0, 1.0);

            ray.pos
                + ray.dir * distance
                + spheres[i].material.wormhole_params.wormhole_offset * wormhole_factor
        } else {
            ray.pos + ray.dir * distance
        };
        return Some(bounce_point);
    } else {
        return None;
    };
}
