use crate::bresenham::Cube;
use crate::helpers::Col;
use crate::scene::{Camera, Sphere, WorldObject};
use cgmath::Vector3;
use rand::StdRng;

pub struct BoundingVolume<'a> {
    pub bounds: [Vector3<f32>; 8],
    pub world_objects: Option<&'a [&'a (dyn WorldObject)]>,
    pub children: (
        Option<Box<BoundingVolume<'a>>>,
        Option<Box<BoundingVolume<'a>>>,
    ),
    pub level: i32,
}

impl BoundingVolume<'_> {
    pub fn new<'a>(
        bounds: [Vector3<f32>; 8],
        world_objects: Option<&'a [&'a (dyn WorldObject)]>,
        children: (
            Option<Box<BoundingVolume<'a>>>,
            Option<Box<BoundingVolume<'a>>>,
        ),
        level: i32,
    ) -> BoundingVolume<'a> {
        BoundingVolume {
            bounds,
            world_objects,
            children,
            level,
        }
    }

    pub fn draw(
        &self,
        buffer: &mut Vec<u32>,
        camera: &Camera,
        display_width: &'static usize,
        display_height: &'static usize,
        rng: &mut StdRng,
        draw_level: i32,
    ) {
        // Draw children
        if let Some(child) = &self.children.0 {
            child.draw(
                buffer,
                camera,
                display_width,
                display_height,
                rng,
                draw_level,
            );
        }
        if let Some(child) = &self.children.1 {
            child.draw(
                buffer,
                camera,
                display_width,
                display_height,
                rng,
                draw_level,
            );
        }

        if self.level < draw_level {
            return;
        }

        let cube = Cube::new(self.bounds, Col::from_random_hue(rng));

        // Draw cube
        cube.draw(buffer, camera, display_width, display_height);
    }
}

pub fn construct_bvh<'a>(
    objects: &'a [&'a (dyn WorldObject)],
    objects_len: usize,
    level: i32,
) -> Box<BoundingVolume<'a>> {
    let bounds = get_bounds(&objects);
    let mid = (objects_len - 1) / 2;
    let (left, right) = objects.split_at(mid);
    let right_len = objects_len - mid;
    let left_len = objects_len - right_len;

    return Box::new(BoundingVolume::new(
        bounds,
        Some(objects),
        (
            if left_len > 2 {
                Some(construct_bvh(left, left_len, level + 1))
            } else {
                None
            },
            if right_len > 2 {
                Some(construct_bvh(right, right_len, level + 1))
            } else {
                None
            },
        ),
        level,
    ));
}

pub fn get_bounds<'a>(objects: &'a [&'a (dyn WorldObject)]) -> [Vector3<f32>; 8] {
    let mut max = [std::f32::MIN, std::f32::MIN, std::f32::MIN];
    let mut min = [std::f32::MAX, std::f32::MAX, std::f32::MAX];

    for object in objects.iter() {
        let pos = object.pos();
        let pos = [pos.x, pos.y, pos.z];
        let radius = object.radius();

        for ((pos, max), min) in pos.iter().zip(max.iter_mut()).zip(min.iter_mut()) {
            let greatest = pos + radius;
            let least = pos - radius;
            if greatest > *max {
                *max = greatest;
            }
            if least < *min {
                *min = least;
            }
        }
    }

    let max = Vector3::new(max[0], max[1], max[2]);
    let min = Vector3::new(min[0], min[1], min[2]);

    return [
        Vector3::new(min.x, min.y, min.z),
        Vector3::new(min.x, max.y, min.z),
        Vector3::new(max.x, min.y, min.z),
        Vector3::new(max.x, max.y, min.z),
        Vector3::new(min.x, min.y, max.z),
        Vector3::new(min.x, max.y, max.z),
        Vector3::new(max.x, min.y, max.z),
        Vector3::new(max.x, max.y, max.z),
    ];
}
