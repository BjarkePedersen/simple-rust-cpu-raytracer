use std::time::{Duration, Instant};

pub struct Viewport {
    pub overlays_enabled: bool,
    pub autofocus: bool,
    pub depth_pass: bool,
    pub normal_pass: bool,
    pub sample_iter: u32,
    pub time: Time,
}

pub struct Time {
    pub start: Instant,
    pub sum: Duration,
    pub prev: Instant,
    pub framecount: i32,
}

pub fn timestamp() -> Instant {
    Instant::now()
    // // let millis: f64 = timespec.sec as f64 + (timespec.nsec as f64 / 1000.0 / 1000.0 / 1000.0);
    // let duration = Duration::new();
    // println!("skrrrrrrt {:?}", timespec.t);
    // let millis = 0.0;
    // millis
}

pub fn update_time(window: &mut minifb::Window, time: &mut Time, sample_iter: &u32) {
    time.framecount += 1;
    let now = timestamp();

    if time.framecount % 30 == 0 {
        let diff = &(now - time.prev).as_millis().to_string()[..];
        let iterations = &(sample_iter).to_string()[..];
        window
        .set_title(&("ms: ".to_owned() + diff + &"   iterations: ".to_owned() + iterations)[..]);
        time.sum = Duration::new(0,0);
    }
    
    time.prev = now;
}
