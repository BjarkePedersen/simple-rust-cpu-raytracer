use crate::helpers::{mix_col, Col};
use crate::scene::{Ray, Scene};

pub fn sky_box(scene: &Scene, ray: &Ray) -> Col {
    let mut col = mix_col(scene.sky.colors[0], scene.sky.colors[1], ray.dir.z.abs());
    fn tile(x: f32) -> f32 {
        if x > 0.99 {
            0.0
        } else {
            1.0
        }
    }
    let ground = mix_col(
        Col::new(0.8, 0.8, 0.8),
        Col::new(0.2, 0.3, 0.4),
        tile((ray.dir.x * 10.0 / ray.dir.z).cos())
            * tile((ray.dir.y * 10.0 / ray.dir.z).cos())
            * tile((ray.dir.x * 40.0 / ray.dir.z).cos())
            * tile((ray.dir.y * 40.0 / ray.dir.z).cos()),
    );

    col = mix_col(col, ground, if ray.dir.z > 0.0 { 1.0 } else { 0.0 });

    return col;
}
