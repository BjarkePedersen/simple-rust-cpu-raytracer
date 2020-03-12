use crate::bresenham::Cube;
use crate::helpers::Col;
use crate::scene::{Camera, WorldObject};
use cgmath::Vector3;

pub struct BoundingVolume {
    pub points: [Vector3<f32>; 8],
    // pub child_object: Option<Box<dyn WorldObject>>,
    // pub child_node: Option<&'a BoundingVolume<'a>>,
}

impl BoundingVolume {
    pub fn draw(
        &self,
        buffer: &mut Vec<u32>,
        camera: &Camera,
        display_width: &'static usize,
        display_height: &'static usize,
    ) {
        // Create cube from points
        let cube = Cube::new(self.points, Col::red());

        // Draw cube
        cube.draw(buffer, camera, display_width, display_height);
    }
}

// pub struct Partition2d<'a> {
//     pub bounds: [Vector2<f32>; 4],
//     pub child_object: Option<Box<dyn WorldObject>>,
//     pub child_node: Option<&'a Partition2d<'a>>,
// }

// impl Partition2d<'_> {
//     pub fn new<'a>(
//         bounds: [Vector2<f32>; 4],
//         child_object: Option<Box<dyn WorldObject>>,
//         child_node: Option<&'a Partition2d<'a>>,
//     ) -> Partition2d<'a> {
//         Partition2d {
//             bounds,
//             child_object,
//             child_node,
//         }
//     }

//     pub fn render(
//         &self,
//         buffer: &mut Vec<u32>,
//         display_width: &'static usize,
//         display_height: &'static usize,
//     ) {
//         // Render all child partitions
//         if let Some(child_node) = self.child_node {
//             child_node.render(buffer, display_width, display_height);
//         }

//         // Render partition
//         let bounding_box = Polygon {
//             points: vec![
//                 uv_to_pixel_coordinates(self.bounds[0].x, self.bounds[0].y),
//                 uv_to_pixel_coordinates(self.bounds[1].x, self.bounds[1].y),
//                 uv_to_pixel_coordinates(self.bounds[2].x, self.bounds[2].y),
//                 uv_to_pixel_coordinates(self.bounds[3].x, self.bounds[3].y),
//             ],
//             color: Col::new(1.0, 0.0, 0.0),
//         };
//         bounding_box.draw(buffer, display_width, display_height);
//     }

//     pub fn consilidate(&mut self, objects: &Vec<Box<dyn WorldObject>>, camera: &Camera) {
//         for object in objects.iter() {
//             let size = length(object.radius() * (object.pos() - camera.pos));
//             println!("\n{:?}", camera.pos);
//             println!("{:?} {}", object.pos(), object.radius());
//             println!("size: {}", size);

//             // TODO: Find coordinates of the object with respect to the frame
//             // TODO: Create approximate bounding box from coordinates + radius
//             // TODO: Handle all the tree stuff
//         }
//     }
// }
