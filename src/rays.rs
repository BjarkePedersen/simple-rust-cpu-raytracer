use cgmath::InnerSpace;
use cgmath::Vector3;
use scene::Ray;
use Sphere;

pub fn intersect_sphere(ray: &Ray, sphere: &Sphere) -> Option<(Vector3<f32>)> {
    let o = ray.pos;
    let l = ray.dir;
    let c = sphere.pos;
    let r = sphere.radius;

    let o_c = o - c;
    let l_dot_o_c = l.dot(o_c);

    let discriminant = l_dot_o_c.powi(2) - o_c.magnitude2() + r.powi(2);

    if discriminant > 0.0 && l_dot_o_c < 0.0 {
        let discriminant_sqrt = discriminant.sqrt();

        let d_1 = -l_dot_o_c + discriminant_sqrt;
        let d_2 = -l_dot_o_c - discriminant_sqrt;

        let solution = d_1.min(d_2);
        return Some(o + solution * l);
    } else {
        return None;
    }
}
