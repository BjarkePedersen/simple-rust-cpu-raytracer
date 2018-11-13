pub fn timestamp() -> f64 {
    let timespec = time::get_time();
    let mills: f64 = timespec.sec as f64 + (timespec.nsec as f64 / 1000.0 / 1000.0 / 1000.0);
    mills
}

pub fn update_time(
    window: &mut minifb::Window,
    prev: &mut f64,
    framecount: &mut i32,
    time_sum: &mut f64,
    sample_iter: &u32,
) {
    *framecount += 1;
    if *framecount % 30 == 0 {
        let ms = &((*time_sum / 30.0).round()).to_string()[..];
        let iterations = &(sample_iter).to_string()[..];
        window
            .set_title(&("ms: ".to_owned() + ms + &"   iterations: ".to_owned() + iterations)[..]);
        *time_sum = 0.0;
    }
    *time_sum += (timestamp() - *prev) * 1000.0;
    *prev = timestamp();
}
