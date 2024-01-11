use autopilot;
use rand::prelude::*;

fn main() {
    let screen_size = autopilot::screen::size();
    let mut rng = rand::thread_rng();
    loop {
        let (x, y) = (
            rng.gen_range(0.0..screen_size.width),
            rng.gen_range(0.0..screen_size.height),
        );
        autopilot::mouse::smooth_move(autopilot::geometry::Point::new(x, y), Some(2.0)).unwrap();
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
