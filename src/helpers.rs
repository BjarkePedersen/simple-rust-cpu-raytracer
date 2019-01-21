use crate::WIDTH;
use cgmath::Vector3;
use std::f32;
use std::ops::Add;
use std::ops::Div;
use std::ops::Mul;
use std::ops::Sub;

pub fn clamp<T: PartialOrd>(val: T, min: T, max: T) -> T {
    if val < min {
        min
    } else if val > max {
        max
    } else {
        val
    }
}

pub fn clamp_min<T: PartialOrd>(val: T, min: T) -> T {
    if val < min {
        min
    } else {
        val
    }
}

pub fn clamp_max<T: PartialOrd>(val: T, max: T) -> T {
    if val > max {
        max
    } else {
        val
    }
}

pub struct UV {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Clone, Copy)]
pub struct Col {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl Col {
    pub fn new(r: f32, g: f32, b: f32) -> Col {
        Col { r: r, g: g, b: b }
    }

    pub fn clamp(&self, min: f32, max: f32) -> Col {
        Col {
            r: clamp(self.r, min, max),
            g: clamp(self.r, min, max),
            b: clamp(self.r, min, max),
        }
    }
}

impl Add<f32> for Col {
    type Output = Col;

    fn add(self, val: f32) -> Col {
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

impl Sub<f32> for Col {
    type Output = Col;

    fn sub(self, val: f32) -> Col {
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

impl Mul<f32> for Col {
    type Output = Col;

    fn mul(self, val: f32) -> Col {
        Col {
            r: self.r * val,
            g: self.g * val,
            b: self.b * val,
        }
    }
}

impl Mul<Col> for Col {
    type Output = Col;

    fn mul(self, col2: Col) -> Col {
        Col {
            r: self.r * col2.r,
            g: self.g * col2.g,
            b: self.b * col2.b,
        }
    }
}

impl Div<f32> for Col {
    type Output = Col;

    fn div(self, val: f32) -> Col {
        Col {
            r: self.r / val,
            g: self.g / val,
            b: self.b / val,
        }
    }
}

impl Div<Col> for Col {
    type Output = Col;

    fn div(self, col2: Col) -> Col {
        Col {
            r: self.r / col2.r,
            g: self.g / col2.g,
            b: self.b / col2.b,
        }
    }
}

pub fn mix_col(col1: Col, col2: Col, mix: f32) -> Col {
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

pub fn uv(index: usize) -> UV {
    UV {
        x: (index % WIDTH as usize) as f32,
        y: (index as f32 / WIDTH as f32) as f32,
    }
}

pub fn distance(vec1: Vector3<f32>, vec2: Vector3<f32>) -> f32 {
    ((vec2.x - vec1.x).powi(2) + (vec2.y - vec1.y).powi(2) + (vec2.z - vec1.z).powi(2)).sqrt()
}

pub fn rad(deg: f32) -> f32 {
    deg * f32::consts::PI / 180.0
}
