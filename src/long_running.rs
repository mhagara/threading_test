use std::collections::HashMap;
use std::sync::{Arc, Mutex, MutexGuard};
use std::thread;
use std::time::Duration;

pub struct LongRunning {
    items: HashMap<u32, String>,
}

pub struct LongRunningAsync {
    mutex: Arc<Mutex<LongRunning>>,
}

impl LongRunningAsync {
    pub fn run(&self) {
        let this = self.mutex.clone();
        thread::spawn(move || loop {
            let lock = LongRunningAsync::get_lock(&this);
            println!("{}", lock.items.len());
            drop(lock);
            thread::sleep(Duration::from_secs(1));
        });
    }

    pub fn add(&mut self, i: u32, s: String) {
        let mut lock = self.get_self_lock();
        lock.add(i, s);
        drop(lock);
    }

    fn get_lock(mutex: &Arc<Mutex<LongRunning>>) -> MutexGuard<LongRunning> {
        return mutex.lock().expect("mutex is poisoned");
    }

    fn get_self_lock(&self) -> MutexGuard<LongRunning> {
        return LongRunningAsync::get_lock(&self.mutex);
    }
}

impl LongRunning {
    pub fn async_new() -> LongRunningAsync {
        let inner = LongRunning {
            items: HashMap::new()
        };
        return LongRunningAsync { mutex: Arc::new(Mutex::new(inner)) };
    }

    pub fn add(&mut self, i: u32, s: String) {
        self.items.insert(i, s);
    }
}
