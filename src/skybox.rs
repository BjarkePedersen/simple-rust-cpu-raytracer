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
    let tile_size = 5.0;
    let secondary_tile_factor = 4.0;
    let up = (ray.pos.z + 12.0).max(0.0) / tile_size;

    let big_tiles_x = tile((ray.dir.x * tile_size / ray.dir.z * up - ray.pos.x).cos());
    let big_tiles_y = tile((ray.dir.y * tile_size / ray.dir.z * up - ray.pos.y).cos());
    let small_tiles_x = tile(
        (ray.dir.x * tile_size * secondary_tile_factor / ray.dir.z * up
            - secondary_tile_factor * ray.pos.x)
            .cos(),
    );
    let small_tiles_y = tile(
        (ray.dir.y * tile_size * secondary_tile_factor / ray.dir.z * up
            - secondary_tile_factor * ray.pos.y)
            .cos(),
    );

    let ground = mix_col(
        Col::new(0.8, 0.8, 0.8),
        Col::new(0.2, 0.3, 0.4),
        big_tiles_x * big_tiles_y * small_tiles_x * small_tiles_y,
    );

    col = mix_col(col, ground, if ray.dir.z > 0.0 { 1.0 } else { 0.0 });

    return col;
}
