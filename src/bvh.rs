use crate::bresenham::Cube;
use crate::helpers::{Axis, Col};
use crate::scene::{Camera, WorldObject};
use cgmath::Vector3;
use rand::StdRng;
use std::fmt;
use std::rc::Rc;

// pub struct BoundingVolume {
pub struct BoundingVolume {
    pub bounds: BoundingBox,
    pub world_objects: Vec<Rc<dyn WorldObject>>,
    // pub world_objects: &'a [Rc<(dyn WorldObject)>],
    // pub world_objects: &'a [&'a (dyn WorldObject)],
    pub children: (Option<Box<BoundingVolume>>, Option<Box<BoundingVolume>>),
    pub level: i32,
}

impl BoundingVolume {
    pub fn new(
        bounds: BoundingBox,
        world_objects: Vec<Rc<dyn WorldObject>>,
        children: (Option<Box<BoundingVolume>>, Option<Box<BoundingVolume>>),
        level: i32,
    ) -> BoundingVolume {
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

        let cube = Cube::new(self.bounds.into(), Col::from_random_hue(rng));

        // Draw cube
        cube.draw(buffer, camera, display_width, display_height);
    }
}

pub fn get_by_axis(vec: Vector3<f32>, axis: Axis) -> f32 {
    match axis.into() {
        0 => vec.x,
        1 => vec.y,
        _ => vec.z,
    }
}

pub fn construct_bvh(
    // objects: [&'a impl WorldObject],
    // objects: &'a mut [&'a dyn WorldObject],
    objects: Vec<Rc<dyn WorldObject>>,
    // objects: [&'a (dyn WorldObject)],
    // objects: &'a Vec<&'a dyn WorldObject>,
    // objects: Option<&[&'a dyn WorldObject]>,
    // objects: Box<[Box<&'a dyn WorldObject>]>,
    // objects: Box<[&'a (dyn WorldObject)]>,
    // objects: Box<[Box<&'a (dyn WorldObject)>]>,
    level: i32,
    split_axis: Axis,
) -> Box<BoundingVolume> {
    let bounds = get_bounds(&objects);

    println!("\nAxis {}", split_axis);
    println!("Bounds {}", bounds);
    let midpoint = get_by_axis(bounds.local_coordinates().max_point(), split_axis) / 2.0;
    println!("Midpoint {}", midpoint);

    // Split objects by midpoint of parent BVH
    // let mut left: Vec<&'a dyn WorldObject> = vec![];
    // let mut right: Vec<&'a dyn WorldObject> = vec![];
    // let mut indices: Vec<usize> = vec![];
    // for (i, object) in objects.iter().enumerate() {
    //     if get_by_axis(object.pos(), split_axis) + object.radius() < midpoint {
    //         left.push(*object);
    //     } else {
    //         right.push(*object);
    //     }
    // }

    // let mut left: Vec<&dyn WorldObject> = objects
    //     .iter()
    //     .filter(|object| get_by_axis(object.pos(), split_axis) + object.radius() > midpoint)
    //     .collect();
    // let mut right: Vec<&&dyn WorldObject> = objects
    //     .iter()
    //     .filter(|object| get_by_axis(object.pos(), split_axis) + object.radius() <= midpoint)
    //     .collect();

    let mut objects = objects.clone();
    let mut i = objects.iter_mut().partition_in_place(|object| {
        get_by_axis(object.pos(), split_axis) + object.radius() > midpoint
    });
    let mut left_len = objects[0..i].len();
    let mut right_len = objects[i..0].len();
    // let mut left_len = left.len();
    // let mut right_len = right.len();
    let mut split_attempts = 1;

    while left_len == 0 || right_len == 0 && split_attempts < 3 {
        // Could not split any further on this axis. Try again with a different axis

        // left = vec![];
        // right = vec![];
        // for object in objects.iter() {
        //     if get_by_axis(object.pos(), split_axis) + object.radius() < midpoint {
        //         left.push(*object);
        //     } else {
        //         right.push(*object);
        //     }
        // }
        // left_len = left.len();
        // right_len = right.len();
        //     split_attempts += 1;
        i = objects.iter_mut().partition_in_place(|object| {
            get_by_axis(object.pos(), split_axis) + object.radius() > midpoint
        });
        left_len = objects[0..i].len();
        right_len = objects[i..0].len();
        split_attempts += 1;
    }
    let (left, right) = (objects[0..i].to_vec(), objects[i..0].to_vec());

    return Box::new(BoundingVolume {
        bounds,
        world_objects: objects,
        children: if split_attempts == 3 {
            (None, None)
        } else {
            (
                Some(construct_bvh(left, level + 1, split_axis.next())),
                Some(construct_bvh(right, level + 1, split_axis.next())),
            )
        },
        level,
    });
}

pub fn get_bounds(objects: &Vec<Rc<dyn WorldObject>>) -> BoundingBox {
    // pub fn get_bounds(objects: &'a [&'a (dyn WorldObject)]) -> BoundingBox {
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

    return BoundingBox::new([
        Vector3::new(min.x, min.y, min.z),
        Vector3::new(min.x, max.y, min.z),
        Vector3::new(max.x, min.y, min.z),
        Vector3::new(max.x, max.y, min.z),
        Vector3::new(min.x, min.y, max.z),
        Vector3::new(min.x, max.y, max.z),
        Vector3::new(max.x, min.y, max.z),
        Vector3::new(max.x, max.y, max.z),
    ]);
}

#[derive(Copy, Clone)]
pub struct BoundingBox {
    val: [Vector3<f32>; 8],
}

impl BoundingBox {
    fn new(val: [Vector3<f32>; 8]) -> BoundingBox {
        BoundingBox { val }
    }
    fn local_coordinates(&self) -> BoundingBox {
        BoundingBox {
            val: [
                self.val[0] - self.val[0],
                self.val[1] - self.val[0],
                self.val[2] - self.val[0],
                self.val[3] - self.val[0],
                self.val[4] - self.val[0],
                self.val[5] - self.val[0],
                self.val[6] - self.val[0],
                self.val[7] - self.val[0],
            ],
        }
    }

    fn max_point(&self) -> Vector3<f32> {
        self.val[7]
    }
}

impl Into<[Vector3<f32>; 8]> for BoundingBox {
    fn into(self) -> [Vector3<f32>; 8] {
        self.val
    }
}

impl fmt::Display for BoundingBox {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.val)
    }
}
