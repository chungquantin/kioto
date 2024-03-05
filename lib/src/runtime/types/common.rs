/// After thread starts / before thread stops
pub type Callback = std::sync::Arc<dyn Fn() + Send + Sync>;
