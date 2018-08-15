use cgmath::InnerSpace;
use cgmath::Vector3;
use scene::Ray;
use Sphere;

pub fn intersect_sphere(ray: &Ray, sphere: &Sphere) -> Option<(Vector3<f32>)> {
    let o = ray.p1;
    let l = (ray.p2 - ray.p1).normalize();
    let c = sphere.pos;
    let r = sphere.radius;

    // let ray_direction = Vector3::new(
    //     ray.p2.x - ray.p1.x,
    //     ray.p2.y - ray.p1.y,
    //     ray.p2.z - ray.p1.z,
    // );

    // let camera_direction = vec![0.0, 1.0, 0.0];

    let o_c = o - c;
    let l_dot_o_c = l.dot(o_c);

    let discriminant = l_dot_o_c.powi(2) - o_c.magnitude2() + r.powi(2);

    if discriminant > 0.0 {
        let discriminant_sqrt = discriminant.sqrt();

        let d_1 = -l_dot_o_c + discriminant_sqrt;
        let d_2 = -l_dot_o_c - discriminant_sqrt;

        let solution = d_1.min(d_2);
        return Some(Vector3::new(
            o.x + solution * ray.p2.x,
            o.y + solution * ray.p2.y,
            o.z + solution * ray.p2.z,
        ));
    } else {
        return None;
    }
}
