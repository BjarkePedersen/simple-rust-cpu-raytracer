pub struct Viewport {
    pub overlays_enabled: bool,
    pub autofocus: bool,
    pub depth_pass: bool,
    pub normal_pass: bool,
    pub sample_iter: u32,
    pub time: Time,
}

pub struct Time {
    pub sum: f64,
    pub prev: f64,
    pub framecount: i32,
}

pub fn timestamp() -> f64 {
    let timespec = time::get_time();
    let mills: f64 = timespec.sec as f64 + (timespec.nsec as f64 / 1000.0 / 1000.0 / 1000.0);
    mills
}

pub fn update_time(window: &mut minifb::Window, time: &mut Time, sample_iter: &u32) {
    time.framecount += 1;
    if time.framecount % 30 == 0 {
        let ms = &((time.sum / 30.0).round()).to_string()[..];
        let iterations = &(sample_iter).to_string()[..];
        window
            .set_title(&("ms: ".to_owned() + ms + &"   iterations: ".to_owned() + iterations)[..]);
        time.sum = 0.0;
    }
    time.sum += (timestamp() - time.prev) * 1000.0;
    time.prev = timestamp();
}
