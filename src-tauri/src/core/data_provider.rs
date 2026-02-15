pub trait DataProvider: Send + Sync {
    fn refresh(&mut self);
}
