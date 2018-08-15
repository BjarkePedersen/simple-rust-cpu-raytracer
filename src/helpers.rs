use cgmath::Vector3;
use std::f32;
use {HEIGHT, WIDTH};

pub struct UV {
    pub x: f32,
    pub y: f32,
}

pub fn rgb(r: u32, g: u32, b: u32) -> u32 {
    let rg = (r << 8) | g;
    ((rg << 8) | b)
}

pub fn byte_to_rgb(hex: u32) -> (u8, u8, u8) {
    ((hex >> 16) as u8, (hex >> 8) as u8, hex as u8)
}

pub fn uv_8b_x(index: usize) -> u32 {
    ((index % WIDTH as usize) as f32 / (WIDTH as f32 / 255.0)) as u32
}

pub fn uv_8b_y(index: usize) -> u32 {
    ((index as f32 / WIDTH as f32) / (HEIGHT as f32 / 255.0)) as u32
}

pub fn uv(index: usize) -> UV {
    UV {
        x: (index % WIDTH as usize) as f32,
        y: (index as f32 / WIDTH as f32) as f32,
    }
}

pub fn uv_y(index: usize) -> u32 {
    (index as f32 / WIDTH as f32) as u32
}

pub fn distance(vec1: Vector3<f32>, vec2: Vector3<f32>) -> f32 {
    ((vec2.x - vec1.x).powi(2) + (vec2.y - vec1.y).powi(2) + (vec2.z - vec1.z).powi(2)).sqrt()
}

pub fn rad(deg: f32) -> f32 {
    deg * f32::consts::PI / 180.0
}

pub fn deg(rad: f32) -> f32 {
    rad * 180.0 / f32::consts::PI
}
