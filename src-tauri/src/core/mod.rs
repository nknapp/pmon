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
    controllers: Vec<Box<dyn StateSummarySink>>,
}

impl StateSummaryDispatcher {
    pub fn new() -> Self {
        Self {
            controllers: Vec::new(),
        }
    }

    pub fn with_controllers(controllers: Vec<Box<dyn StateSummarySink>>) -> Self {
        Self { controllers }
    }

    pub fn add_controller(&mut self, controller: Box<dyn StateSummarySink>) {
        self.controllers.push(controller);
    }
}

impl StateSummarySink for StateSummaryDispatcher {
    fn set_state_summary(&self, state: StateSummary) {
        for controller in &self.controllers {
            controller.set_state_summary(state);
        }
    }
}
