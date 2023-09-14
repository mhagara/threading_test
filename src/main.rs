use std::thread;
use std::time::Duration;
use crate::long_running::LongRunning;

mod long_running;

fn main() {
    let mut l = LongRunning::async_new();

    l.run();

    let mut i = 0;
    loop {
        l.add(i, i.to_string());
        thread::sleep(Duration::from_secs(1));
        i = i + 1;
    }
}
