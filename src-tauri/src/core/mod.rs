#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NotificationState {
    Ok,
    OkPending,
    Failure,
    FailurePending,
}

pub trait NotificationStateController {
    fn set_notification_state(&self, state: NotificationState);
}
