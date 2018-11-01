
pub fn timestamp() -> f64 {
    let timespec = time::get_time();
    let mills: f64 = timespec.sec as f64 + (timespec.nsec as f64 / 1000.0 / 1000.0 / 1000.0);
    mills
}

pub fn update_time(window: &mut minifb::Window, prev: &mut f64, framecount: &mut i32, time_sum: &mut f64) {
    *framecount += 1;
    if *framecount % 30 == 0 {
        window.set_title(&(*time_sum / 30.0).to_string()[..]);
        *time_sum = 0.0;
    }
    *time_sum += (timestamp() - *prev)*1000.0;
    *prev = timestamp();
}