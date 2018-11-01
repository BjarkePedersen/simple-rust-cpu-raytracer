pub struct Viewport {
    pub distance_pass: bool,
    pub sample_iter: u32,
    pub time: Time,
}

pub struct Time {
    pub sum: f64,
    pub prev: f64,
    pub framecount: i32,
}
