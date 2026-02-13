#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NotificationState {
    Ok,
    OkPending,
    Failure,
    FailurePending,
}

pub trait NotificationStateController: Send + Sync {
    fn set_notification_state(&self, state: NotificationState);
}

pub struct NotificationStateDispatcher {
    controllers: Vec<Box<dyn NotificationStateController>>,
}

impl NotificationStateDispatcher {
    pub fn new() -> Self {
        Self {
            controllers: Vec::new(),
        }
    }

    pub fn with_controllers(controllers: Vec<Box<dyn NotificationStateController>>) -> Self {
        Self { controllers }
    }

    pub fn add_controller(&mut self, controller: Box<dyn NotificationStateController>) {
        self.controllers.push(controller);
    }
}

impl NotificationStateController for NotificationStateDispatcher {
    fn set_notification_state(&self, state: NotificationState) {
        for controller in &self.controllers {
            controller.set_notification_state(state);
        }
    }
}
