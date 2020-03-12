use crate::HEIGHT;
use crate::WIDTH;
use cgmath::{Vector2, Vector3};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};
use std::{f32, fmt};

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

    pub fn luminance(&self) -> f32 {
        return (self.r + self.g + self.b) / 3.0;
    }

    pub fn powf(&self, power: f32) -> Col {
        Col {
            r: self.r.powf(power),
            g: self.g.powf(power),
            b: self.b.powf(power),
        }
    }

    pub fn powi(&self, power: i32) -> Col {
        Col {
            r: self.r.powi(power),
            g: self.g.powi(power),
            b: self.b.powi(power),
        }
    }

    pub fn red() -> Col {
        Col::new(1.0, 0.0, 0.0)
    }
    pub fn green() -> Col {
        Col::new(0.0, 1.0, 0.0)
    }
    pub fn blue() -> Col {
        Col::new(0.0, 0.0, 1.0)
    }
    pub fn yellow() -> Col {
        Col::new(1.0, 1.0, 0.0)
    }
    pub fn cyan() -> Col {
        Col::new(0.0, 1.0, 1.0)
    }
    pub fn magenta() -> Col {
        Col::new(1.0, 0.0, 1.0)
    }
    pub fn black() -> Col {
        Col::new(0.0, 0.0, 0.0)
    }
    pub fn white() -> Col {
        Col::new(1.0, 1.0, 1.0)
    }
    pub fn grey() -> Col {
        Col::new(0.5, 0.5, 0.5)
    }
    pub fn light_grey() -> Col {
        Col::new(0.75, 0.75, 0.75)
    }
    pub fn dark_grey() -> Col {
        Col::new(0.25, 0.25, 0.25)
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

impl AddAssign for Col {
    fn add_assign(&mut self, other: Self) {
        self.r += other.r;
        self.g += other.g;
        self.b += other.b;
    }
}

impl AddAssign<f32> for Col {
    fn add_assign(&mut self, other: f32) {
        *self = *self + other;
    }
}

impl SubAssign for Col {
    fn sub_assign(&mut self, other: Self) {
        self.r -= other.r;
        self.g -= other.g;
        self.b -= other.b;
    }
}

impl SubAssign<f32> for Col {
    fn sub_assign(&mut self, other: f32) {
        *self = *self - other;
    }
}

impl MulAssign for Col {
    fn mul_assign(&mut self, other: Self) {
        self.r *= other.r;
        self.g *= other.g;
        self.b *= other.b;
    }
}

impl MulAssign<f32> for Col {
    fn mul_assign(&mut self, other: f32) {
        *self = *self * other;
    }
}

impl DivAssign for Col {
    fn div_assign(&mut self, other: Self) {
        self.r /= other.r;
        self.g /= other.g;
        self.b /= other.b;
    }
}

impl DivAssign<f32> for Col {
    fn div_assign(&mut self, other: f32) {
        *self = *self / other;
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
    (rg << 8) | b
}

pub fn uv(index: usize) -> UV {
    UV {
        x: (index % WIDTH as usize) as f32,
        y: (index as f32 / WIDTH as f32).floor() as f32,
    }
}

pub fn uv_to_pixel_coordinates(x: f32, y: f32) -> Vector2<i32> {
    Vector2::new((x * WIDTH as f32) as i32, (y * HEIGHT as f32) as i32)
}

pub fn rad(deg: f32) -> f32 {
    deg * f32::consts::PI / 180.0
}

pub fn length(vector: Vector3<f32>) -> f32 {
    ((vector.x).powi(2) + (vector.y).powi(2) + (vector.z).powi(2)).sqrt()
}

pub fn distance(p1: Vector3<f32>, p2: Vector3<f32>) -> f32 {
    length(p2 - p1)
}

#[derive(Debug, Clone, Copy)]
pub struct ObjectID {
    val: i32,
}

impl ObjectID {
    pub fn next(&mut self) -> ObjectID {
        self.val += 1;
        return ObjectID::from(self.val);
    }
}

impl fmt::Display for ObjectID {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.val)
    }
}

impl From<i32> for ObjectID {
    fn from(w: i32) -> ObjectID {
        ObjectID { val: w }
    }
}

impl PartialEq for ObjectID {
    fn eq(&self, other: &Self) -> bool {
        self.val == other.val
    }
}

impl Add<i32> for ObjectID {
    type Output = ObjectID;

    fn add(self, val: i32) -> ObjectID {
        ObjectID::from(self.val + val)
    }
}
