extern crate minifb;
extern crate rand;
extern crate rgb;

use minifb::{Key, Window, WindowOptions};

const WIDTH: usize = 800;
const HEIGHT: usize = 800;

const CLIP_MIN: f32 = 0.001;

#[derive(Debug, Clone)]
struct Camera {
    pos: Vec3,
    rot: Vec3,
    fov: f32,
}

#[derive(Debug, Clone)]
struct Vec3 {
    x: f32,
    y: f32,
    z: f32,
}

#[derive(Debug, Clone)]
struct Sphere {
    pos: Vec3,
    radius: f32,
}

struct Ray {
    p1: Vec3,
    p2: Vec3,
}

fn rgb(r: u32, g: u32, b: u32) -> u32 {
    let rg = (r << 8) | g;
    ((rg << 8) | b)
}

fn uv_8b_x(index: usize) -> u32 {
    ((index % WIDTH as usize) as f32 / (WIDTH as f32 / 255.0)) as u32
}

fn uv_8b_y(index: usize) -> u32 {
    ((index as f32 / WIDTH as f32) / (HEIGHT as f32 / 255.0)) as u32
}

fn uv_x(index: usize) -> u32 {
    (index % WIDTH as usize) as u32
}

fn uv_y(index: usize) -> u32 {
    (index as f32 / WIDTH as f32) as u32
}

fn v3(x: f32, y: f32, z: f32) -> Vec3 {
    Vec3 { x: x, y: y, z: z }
}

fn intersect_sphere(ray: Ray, sphere: Sphere) -> Option<(Vec3, Vec3)> {
    let a = (ray.p2.x - ray.p1.x).powi(2)
        + (ray.p2.y - ray.p1.y).powi(2)
        + (ray.p2.z - ray.p1.z).powi(2);

    let b = 2.0 * ((ray.p2.x - ray.p1.x) * (ray.p1.x - sphere.pos.x)
        + (ray.p2.y - ray.p1.y) * (ray.p1.y - sphere.pos.y)
        + (ray.p2.z - ray.p1.z) * (ray.p1.z - sphere.pos.z));

    let c = sphere.pos.x.powi(2)
        + sphere.pos.y.powi(2)
        + sphere.pos.z.powi(2)
        + ray.p1.x.powi(2)
        + ray.p1.y.powi(2)
        + ray.p1.z.powi(2)
        - 2.0 * (sphere.pos.x * ray.p1.x + sphere.pos.y * ray.p1.y + sphere.pos.z * ray.p1.z)
        - sphere.radius.powi(2);

    let d = b.powi(2) - 4.0 * a * c;

    if d > 0.0 {
        let solution_1 = d.sqrt() / (2.0 * a);
        let solution_2 = d.sqrt() / (2.0 * a);

        println!("{}", solution_1);
        println!("{}", solution_2);

        return Some((
            v3(
                ray.p1.x + solution_1 * ray.p2.x,
                ray.p1.y + solution_1 * ray.p2.y,
                ray.p1.z + solution_1 * ray.p2.z,
            ),
            v3(
                ray.p1.x + solution_2 * ray.p2.x,
                ray.p1.y + solution_2 * ray.p2.y,
                ray.p1.z + solution_2 * ray.p2.z,
            ),
        ));
    } else {
        return None;
    }
}
fn check_intersect_sphere(ray: Ray, sphere: Sphere) -> bool {
    let a = (ray.p2.x - ray.p1.x).powi(2)
        + (ray.p2.y - ray.p1.y).powi(2)
        + (ray.p2.z - ray.p1.z).powi(2);

    let b = 2.0 * ((ray.p2.x - ray.p1.x) * (ray.p1.x - sphere.pos.x)
        + (ray.p2.y - ray.p1.y) * (ray.p1.y - sphere.pos.y)
        + (ray.p2.z - ray.p1.z) * (ray.p1.z - sphere.pos.z));

    let c = sphere.pos.x.powi(2)
        + sphere.pos.y.powi(2)
        + sphere.pos.z.powi(2)
        + ray.p1.x.powi(2)
        + ray.p1.y.powi(2)
        + ray.p1.z.powi(2)
        - 2.0 * (sphere.pos.x * ray.p1.x + sphere.pos.y * ray.p1.y + sphere.pos.z * ray.p1.z)
        - sphere.radius.powi(2);

    let d = b.powi(2) - 4.0 * a * c;

    return d > 0.0;
}

fn main() {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    let mut window = Window::new(
        "Test - ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    ).unwrap_or_else(|e| {
        panic!("{}", e);
    });

    let camera = Camera {
        pos: Vec3 {
            x: 0.0,
            y: -5.0,
            z: 0.0,
        },
        rot: Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
        fov: 35.0,
    };

    let sphere1 = Sphere {
        pos: Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
        radius: 1.0,
    };

    let ray1 = Ray {
        p1: camera.pos.clone(),
        p2: v3(0.0, 10.0, 0.0),
    };

    println!("{:?}", intersect_sphere(ray1, sphere1.clone()));

    let mut t: f32 = 0.0;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        t += 1.0;

        for (x, i) in buffer.iter_mut().enumerate() {
            // let uv_x = ((x % WIDTH as usize) as f32 / (WIDTH as f32 / 255.0)) as u32;
            // let uv_y = ((x as f32 / WIDTH as f32) / (HEIGHT as f32 / 255.0)) as u32;
            let val = if check_intersect_sphere(
                Ray {
                    p1: camera.pos.clone(),
                    p2: v3(
                        camera.pos.x
                            + (uv_x(x) as f32 - WIDTH as f32 / 2.0) * camera.fov / 50000.0
                                * CLIP_MIN,
                        camera.pos.y + CLIP_MIN,
                        camera.pos.z
                            + (uv_y(x) as f32 - HEIGHT as f32 / 2.0) * camera.fov / 50000.0
                                * CLIP_MIN,
                    ),
                },
                sphere1.clone(),
            ) {
                255
            } else {
                2
            };

            *i = rgb(val, val, val);
        }

        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window.update_with_buffer(&buffer).unwrap();
    }
}
