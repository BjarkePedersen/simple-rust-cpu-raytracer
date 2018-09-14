use cgmath::InnerSpace;
use cgmath::Matrix4;
use cgmath::Vector3;
use cgmath::Vector4;
use scene::Camera;
use scene::Ray;
use Sphere;

pub fn intersect_sphere(ray: &Ray, sphere: &Sphere, camera: &Camera) -> Option<(Vector3<f32>)> {
    let o = ray.p1;
    let p = ray.p2;
    let l = (p - o).normalize();
    let c = sphere.pos;
    let r = sphere.radius;

    // let up = Vector3::new(0.0, 0.0, 1.0);
    // let target = camera.pos + Vector3::new(0.0, 1.0, 0.0);

    // let zaxis = (o - target).normalize();
    // let xaxis = (up.cross(zaxis)).normalize();
    // let yaxis = zaxis.cross(xaxis);

    // let orientation = Matrix4::new(
    //     xaxis[0], yaxis[0], zaxis[0], 0.0, xaxis[1], yaxis[1], zaxis[1], 0.0, xaxis[2], yaxis[2],
    //     zaxis[2], 0.0, 0.0, 0.0, 0.0, 1.0,
    // );

    // let translation = Matrix4::new(
    //     1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, -o[0], -o[1], -o[2], 1.0,
    // );

    // let wat = orientation * translation;

    // let eh = p.extend(1.0) * wat;

    // let weew = eh.truncate();

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
            o.x + solution * p.x,
            o.y + solution * p.y,
            o.z + solution * p.z,
        ));
    } else {
        return None;
    }
}
