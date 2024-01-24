use autopilot;
use rand::prelude::*;
use std::sync::{
    atomic::{AtomicBool, Ordering::Relaxed},
    Arc,
};
use windows::Win32::System::Power::{
    SetThreadExecutionState, ES_CONTINUOUS, ES_DISPLAY_REQUIRED, ES_SYSTEM_REQUIRED,
};

fn keep_thread_awake() {
    let flags = ES_CONTINUOUS | ES_SYSTEM_REQUIRED | ES_DISPLAY_REQUIRED;

    unsafe { SetThreadExecutionState(flags) };
}

fn let_thread_sleep() {
    unsafe {
        SetThreadExecutionState(Default::default());
    }
}

fn main() {
    keep_thread_awake();

    let term = Arc::new(AtomicBool::new(false));
    signal_hook::flag::register(signal_hook::consts::SIGTERM, Arc::clone(&term)).unwrap();
    while !term.load(Relaxed) {
        let screen_size = autopilot::screen::size();
        let mut rng = rand::thread_rng();
        let (x, y) = (
            rng.gen_range(0.0..screen_size.width),
            rng.gen_range(0.0..screen_size.height),
        );
        autopilot::mouse::smooth_move(autopilot::geometry::Point::new(x, y), Some(2.0)).unwrap();
        std::thread::sleep(std::time::Duration::from_secs(1));
    }

    let_thread_sleep()
}
