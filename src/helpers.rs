use cgmath::Vector3;
use std::f64;
use std::ops::Add;
use std::ops::Div;
use std::ops::Mul;
use std::ops::Sub;
use {HEIGHT, WIDTH};

pub struct UV {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug, Clone, Copy)]
pub struct Col {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl Col {
    pub fn new(r: f64, g: f64, b: f64) -> Col {
        Col { r: r, g: g, b: b }
    }
}

impl Add<f64> for Col {
    type Output = Col;

    fn add(self, val: f64) -> Col {
        Col {
            r: self.r + val,
            g: self.g + val,
            b: self.b + val,
        }
    }
}

impl Add<Col> for Col {
    type Output = Col;

    fn add(self, col2: Col) -> Col {
        Col {
            r: self.r + col2.r,
            g: self.g + col2.g,
            b: self.b + col2.b,
        }
    }
}

impl Sub<f64> for Col {
    type Output = Col;

    fn sub(self, val: f64) -> Col {
        Col {
            r: self.r + val,
            g: self.g + val,
            b: self.b + val,
        }
    }
}

impl Sub<Col> for Col {
    type Output = Col;

    fn sub(self, col2: Col) -> Col {
        Col {
            r: self.r - col2.r,
            g: self.g - col2.g,
            b: self.b - col2.b,
        }
    }
}

impl Mul<f64> for Col {
    type Output = Col;

    fn mul(self, val: f64) -> Col {
        Col {
            r: self.r * val,
            g: self.g * val,
            b: self.b * val,
        }
    }
}

impl Div<f64> for Col {
    type Output = Col;

    fn div(self, val: f64) -> Col {
        Col {
            r: self.r / val,
            g: self.g / val,
            b: self.b / val,
        }
    }
}

pub fn mix_col(col1: Col, col2: Col, mix: f64) -> Col {
    col1 * mix + col2 * (1.0 - mix)
}

pub fn col_to_rgb_u32(rgb: Col) -> u32 {
    rgb_u32(
        (rgb.r * 255.0) as u32,
        (rgb.g * 255.0) as u32,
        (rgb.b * 255.0) as u32,
    )
}

pub fn rgb_u32(r: u32, g: u32, b: u32) -> u32 {
    let rg = (r << 8) | g;
    ((rg << 8) | b)
}

pub fn byte_to_rgb(hex: u32) -> (u8, u8, u8) {
    ((hex >> 16) as u8, (hex >> 8) as u8, hex as u8)
}

pub fn uv(index: usize) -> UV {
    UV {
        x: (index % WIDTH as usize) as f64,
        y: (index as f64 / WIDTH as f64) as f64,
    }
}

pub fn distance(vec1: Vector3<f64>, vec2: Vector3<f64>) -> f64 {
    ((vec2.x - vec1.x).powi(2) + (vec2.y - vec1.y).powi(2) + (vec2.z - vec1.z).powi(2)).sqrt()
}

pub fn rad(deg: f64) -> f64 {
    deg * f64::consts::PI / 180.0
}

// pub fn deg(rad: f64) -> f64 {
//     rad * 180.0 / f64::consts::PI
// }
