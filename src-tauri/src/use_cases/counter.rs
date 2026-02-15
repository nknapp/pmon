use std::sync::Arc;
use std::thread;
use std::time::Duration;

use crate::core::{StateSummary, StateSummaryAdapter, StateSummaryGateway};

pub struct MonitoringService {
    state_summary_gateway: Arc<StateSummaryGateway>,
}

impl MonitoringService {
    pub fn new(state_summary_gateway: Arc<StateSummaryGateway>) -> Self {
        Self {
            state_summary_gateway,
        }
    }

    pub fn create_counter(self) {
        let mut count = 0u32;
        let state_summary_gateway = self.state_summary_gateway;
        thread::spawn(move || loop {
            thread::sleep(Duration::from_secs(2));
            count += 1;
            state_summary_gateway.set_state_summary(state_for_count(count));
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
