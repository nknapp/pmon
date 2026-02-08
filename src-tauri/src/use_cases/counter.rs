use std::thread;
use std::time::Duration;

#[derive(Clone, serde::Serialize)]
pub struct CounterData {
    count: u32,
    message: String,
}

pub trait StatusObserver: Send + Sync {
    fn on_update(&self, data: CounterData);
}

pub struct MonitoringService {
    // We hold the observer as a Trait Object
    observer: Box<dyn StatusObserver>,
}

impl MonitoringService {
    // Constructor
    pub fn new(observer: Box<dyn StatusObserver>) -> Self {
        Self { observer }
    }
    pub fn create_counter(self) {
        let mut count = 0u32;
        let observer = self.observer; // take ownership
        thread::spawn(move || loop {
            thread::sleep(Duration::from_secs(2));
            count += 1;
            let data = CounterData {
                count,
                message: format!("Background task update #{}", count),
            };
            observer.on_update(data);

            println!("Sent background update #{}", count);
        });
    }
}
