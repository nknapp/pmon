pub trait DataProvider: Send + Sync {
    fn refresh();
}
