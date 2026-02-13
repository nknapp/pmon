use std::sync;
use sync::RwLock;

mod global_state;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StateSummary {
    Ok,
    OkPending,
    Failure,
    FailurePending,
}

pub trait StateSummarySink: Send + Sync {
    fn set_state_summary(&self, state: StateSummary);
}

pub struct StateSummaryDispatcher {
    controllers: RwLock<Vec<Box<dyn StateSummarySink>>>,
}

impl StateSummaryDispatcher {
    pub fn new() -> Self {
        Self {
            controllers: RwLock::new(Vec::new()),
        }
    }

    pub fn add_controller(&self, controller: Box<dyn StateSummarySink>) {
        if let Ok(mut controllers) = self.controllers.write() {
            controllers.push(controller);
        }
    }
}

impl StateSummarySink for StateSummaryDispatcher {
    fn set_state_summary(&self, state: StateSummary) {
        if let Ok(controllers) = self.controllers.read() {
            for controller in controllers.iter() {
                controller.set_state_summary(state);
            }
        }
    }
}
