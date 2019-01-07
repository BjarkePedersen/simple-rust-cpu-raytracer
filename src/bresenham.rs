use crate::helpers::Col;
use crate::scene::Camera;
use cgmath::{EuclideanSpace, Matrix4, Point3, Vector2, Vector3, Vector4};

pub struct Line2d {
    pub x0: i32,
    pub y0: i32,
    pub x1: i32,
    pub y1: i32,
}

impl Line2d {
    pub fn new(x0: i32, y0: i32, x1: i32, y1: i32) -> Line2d {
        Line2d {
            x0: x0,
            y0: y0,
            x1: x1,
            y1: y1,
        }
    }

    pub fn clamp(&self, width: usize, height: usize) -> Option<Line2d> {
        let p1 = Vector2::new(self.x0 as f32, self.y0 as f32);
        let p2 = Vector2::new(self.x1 as f32, self.y1 as f32);

        let r = p2 - p1;
        let t = if p2.x < 0.0 {
            -p1.x / r.x
        } else if p2.x > width as f32 {
            (width as f32 - p1.x) / r.x
        } else {
            1.0
        };
        let p2 = p1 + r * t;

        let r = p2 - p1;
        let t = if p2.y < 0.0 {
            -p1.y / r.y
        } else if p2.y > height as f32 {
            (height as f32 - p1.y) / r.y
        } else {
            1.0
        };
        let p2 = p1 + r * t;

        let r = p1 - p2;
        let t = if p1.x < 0.0 {
            -p2.x / r.x
        } else if p1.x > width as f32 {
            (width as f32 - p2.x) / r.x
        } else {
            1.0
        };
        let p1 = p2 + r * t;

        let r = p1 - p2;
        let t = if p1.y < 0.0 {
            -p2.y / r.y
        } else if p1.y > height as f32 {
            (height as f32 - p2.y) / r.y
        } else {
            1.0
        };
        let p1 = p2 + r * t;

        if !(p1.x + p2.x + p1.y + p2.y).is_nan() && !(p1.x + p2.x + p1.y + p2.y).is_infinite() {
            Some(Line2d {
                x0: p1.x as i32,
                y0: p1.y as i32,
                x1: p2.x as i32,
                y1: p2.y as i32,
            })
        } else {
            None
        }
    }
}

fn plot_line(
    line: Line2d,
    width: &'static usize,
    height: &'static usize,
) -> impl Iterator<Item = (i32, i32)> {
    let x0 = line.x0;
    let y0 = line.y0;
    let x1 = line.x1;
    let y1 = line.y1;

    fn plot_line_low(x0: i32, y0: i32, x1: i32, y1: i32) -> impl Iterator<Item = (i32, i32)> {
        let dx = x1 - x0;
        let mut dy = y1 - y0;
        let mut yi = 1;

        if dy < 0 {
            yi = -1;
            dy = -dy;
        }
        let mut d = 2 * dy - dx;
        let mut y = y0;

        return (x0..x1).map(move |x| {
            let coordinates = (x, y);
            if d > 0 {
                y = y + yi;
                d = d - 2 * dx;
            }
            d = d + 2 * dy;
            coordinates
        });
    };

    let coordinates = if (y1 - y0).abs() < (x1 - x0).abs() {
        if x0 > x1 {
            plot_line_low(x1, y1, x0, y0)
        } else {
            plot_line_low(x0, y0, x1, y1)
        }
    } else {
        if y0 > y1 {
            plot_line_low(y1, x1, y0, x0)
        } else {
            plot_line_low(y0, x0, y1, x1)
        }
    };
    coordinates
        .filter(move |(x, y)| *x < (*width as i32) && *x > 0 && *y < (*height as i32) && *y > 0)
        .map(move |(x, y)| {
            if (y1 - y0).abs() < (x1 - x0).abs() {
                (x, y)
            } else {
                (y, x)
            }
        })
}

pub struct Line3d {
    pub p1: Vector3<f32>,
    pub p2: Vector3<f32>,
    pub color: Col,
}

impl Line3d {
    pub fn new(p1: Vector3<f32>, p2: Vector3<f32>, color: Col) -> Line3d {
        Line3d {
            p1: p1,
            p2: p2,
            color: color,
        }
    }

    pub fn render_line(
        &self,
        camera: &Camera,
        width: &'static usize,
        height: &'static usize,
    ) -> impl Iterator<Item = (i32, i32)> {
        let matrix: Matrix4<f32> = cgmath::PerspectiveFov {
            fovy: cgmath::Rad(camera.fov * std::f32::consts::PI / 180.0),
            aspect: 1.0,
            near: 1.0,
            far: 10.0,
        }
        .into();

        let coord1 = (self.p1 - camera.pos).extend(0.0);
        let coord2 = (self.p2 - camera.pos).extend(0.0);

        let rot = cgmath::Matrix4::from_angle_z(cgmath::Rad(camera.rot.z))
            * cgmath::Matrix4::from_angle_y(cgmath::Rad(camera.rot.y))
            * cgmath::Matrix4::from_angle_x(cgmath::Rad(camera.rot.x));

        let dir = rot * Vector4::new(0.0, 1.0, 0.0, 0.0);

        let dir = Matrix4::look_at_dir(
            Point3::from_vec(camera.pos),
            dir.truncate(),
            Vector3::new(0.0, 0.0, 1.0),
        );

        let coord1 = matrix * dir * coord1;
        let coord2 = matrix * dir * coord2;

        // println!("{:?} {:?}", coord1, coord2);

        let half_height = *height as f32 / -2.0;
        let half_width = *width as i32 / 2;

        let line = Line2d::new(
            (half_height * coord1.x / coord1.w) as i32 + half_width,
            (half_height * coord1.y / coord1.w) as i32 + -half_height as i32,
            (half_height * coord2.x / coord2.w) as i32 + half_width,
            (half_height * coord2.y / coord2.w) as i32 + -half_height as i32,
        )
        .clamp(*width, *height);

        if coord1.w > 0.0 && coord2.w > 0.0 {
            if let Some(line) = line {
                either::Right(plot_line(line, width, height))
            } else {
                either::Left(std::iter::empty())
            }
        } else {
            either::Left(std::iter::empty())
        }
    }
}
