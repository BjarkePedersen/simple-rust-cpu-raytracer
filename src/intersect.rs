use crate::scene::{Ray, Sphere};
use cgmath::{InnerSpace, Vector3};

pub trait Intersect {
    fn intersect(&self, ray: &Ray) -> Option<(Vector3<f32>)>;
}

impl Intersect for Sphere {
    fn intersect(&self, ray: &Ray) -> Option<(Vector3<f32>)> {
        let o = ray.pos;
        let l = ray.dir;
        let c = self.pos;
        let r = self.radius;

        let o_c = o - c;
        let l_dot_o_c = l.dot(o_c);

        let discriminant = l_dot_o_c.powi(2) - o_c.magnitude2() + r.sqrt();

        if discriminant > 0.0 && l_dot_o_c < 0.0 {
            let discriminant_sqrt = discriminant.sqrt();

            let d_1 = -l_dot_o_c + discriminant_sqrt;
            let d_2 = -l_dot_o_c - discriminant_sqrt;

            let solution = d_1.min(d_2);
            return Some(o + solution * l);
        } else {
            return None;
        }

        // let o = ray.pos;
        // let c = self.pos;
        // let dir = ray.dir;

        // let r = self.radius;
        // let l = c - o;
        // // Vec3f L = center - orig;
        // let tca = l.dot(ray.dir);
        // // float tca = L*dir;
        // let d2 = l.dot(l) - tca.powi(2);
        // // float d2 = L*L - tca*tca;
        // let thc = (r.powi(2) - d2).sqrt();
        // if d2 > r.powi(2) {
        //     return None;
        // }

        // let mut t0 = tca - thc;
        // let t1 = tca + thc;

        // if t0 < 0.0 {
        //     t0 = t1;
        // }
        // if t0 < 0.0 {
        //     return None;
        // }
        // return Some(o + t0 * dir);

        // float thc = sqrtf(radius*radius - d2);
        // t0       = tca - thc;
        // float t1 = tca + thc;
        // if (t0 < 0) t0 = t1;
        // if (t0 < 0) return false;
        // return true;
    }
}
