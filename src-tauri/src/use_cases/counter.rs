use std::thread;
use std::time::Duration;

use crate::core::{NotificationState, NotificationStateController, NotificationStateDispatcher};

pub struct MonitoringService {
    dispatcher: NotificationStateDispatcher,
}

impl MonitoringService {
    pub fn new(dispatcher: NotificationStateDispatcher) -> Self {
        Self { dispatcher }
    }

    pub fn create_counter(self) {
        let mut count = 0u32;
        let dispatcher = self.dispatcher;
        thread::spawn(move || loop {
            thread::sleep(Duration::from_secs(2));
            count += 1;
            dispatcher.set_notification_state(state_for_count(count));
            println!("Sent background update #{}", count);
        });
    }
}

fn state_for_count(count: u32) -> NotificationState {
    if count == 0 {
        NotificationState::Ok
    } else if count % 2 == 0 {
        NotificationState::OkPending
    } else {
        NotificationState::FailurePending
    }
}
