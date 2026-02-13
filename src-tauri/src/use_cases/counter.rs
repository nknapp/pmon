use std::thread;
use std::time::Duration;

use crate::core::{StateSummary, StateSummarySink, StateSummaryDispatcher};

pub struct MonitoringService {
    dispatcher: StateSummaryDispatcher,
}

impl MonitoringService {
    pub fn new(dispatcher: StateSummaryDispatcher) -> Self {
        Self { dispatcher }
    }

    pub fn create_counter(self) {
        let mut count = 0u32;
        let dispatcher = self.dispatcher;
        thread::spawn(move || loop {
            thread::sleep(Duration::from_secs(2));
            count += 1;
            dispatcher.set_state_summary(state_for_count(count));
            println!("Sent background update #{}", count);
        });
    }
}

fn state_for_count(count: u32) -> StateSummary {
    if count == 0 {
        StateSummary::Ok
    } else if count % 2 == 0 {
        StateSummary::OkPending
    } else {
        StateSummary::FailurePending
    }
}
