use std::sync::Arc;

use crate::core::StateSummaryGateway;

pub trait DataProvider: Send + Sync {
    fn refresh(&mut self);
    fn start(&mut self, state_summary_gateway: Arc<StateSummaryGateway>);
    fn stop(&mut self);
}
