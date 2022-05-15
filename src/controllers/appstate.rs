use crate::gateways::db::Database;

// This struct represents state
#[derive(Clone)]
pub struct AppState<T: Clone + Database> {
    pub db: T,
}
