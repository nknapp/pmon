use std::sync::RwLock;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StateSummary {
    Ok,
    OkPending,
    Failure,
    FailurePending,
}

pub trait StateSummaryAdapter: Send + Sync {
    fn set_state_summary(&self, state: StateSummary);
}

pub struct StateSummaryGateway {
    controllers: RwLock<Vec<Box<dyn StateSummaryAdapter>>>,
}

impl StateSummaryGateway {
    pub fn new() -> Self {
        Self {
            controllers: RwLock::new(Vec::new()),
        }
    }

    pub fn add_controller(&self, controller: Box<dyn StateSummaryAdapter>) {
        if let Ok(mut controllers) = self.controllers.write() {
            controllers.push(controller);
        }
    }
}

impl StateSummaryAdapter for StateSummaryGateway {
    fn set_state_summary(&self, state: StateSummary) {
        if let Ok(controllers) = self.controllers.read() {
            for controller in controllers.iter() {
                controller.set_state_summary(state);
            }
        }
    }
}
