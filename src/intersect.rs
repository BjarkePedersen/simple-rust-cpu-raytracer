use crate::scene::{Ray, Sphere};
use cgmath::{dot, InnerSpace};

pub trait Intersect {
    fn intersect(&self, ray: &Ray) -> Option<f32>;
}

impl Intersect for Sphere {
    fn intersect(&self, ray: &Ray) -> Option<f32> {
        let r_squared = self.radius.powi(2);
        let l = self.pos - ray.pos;
        let tca = l.dot(ray.dir);
        let d2 = l.dot(l) - tca.powi(2);
        let thc = (r_squared - d2).sqrt();

        if d2 > r_squared {
            return None;
        }

        let mut t0 = tca - thc;
        let t1 = tca + thc;

        if t0 < 0.0 {
            t0 = t1;
        }
        if t0 < 0.0 {
            return None;
        }
        if dot(ray.pos + ray.dir * t0 - self.pos, ray.dir) > 0.0 {
            return None;
        }

        return Some(t0);
    }
}
