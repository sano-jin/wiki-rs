use crate::gateways::db::Database;
use chrono::{DateTime, Utc};
use std::collections::HashMap;
// use std::ops::{Deref, DerefMut};
// use std::sync::{Arc, Mutex};
use std::sync::Mutex;
use uuid::Uuid;

/// This struct represents state
// #[derive(Clone)]
pub struct AppState<T: Clone + Database> {
    // database pool
    pub db: T,

    // The uuid and the last login time of logged in users
    // pub logged_in_users: Arc<Mutex<HashMap<Uuid, DateTime<Utc>>>>,
    pub logged_in_users: Mutex<HashMap<Uuid, DateTime<Utc>>>,
}

impl<T: Clone + Database> AppState<T> {
    pub fn new(db: T) -> Self {
        AppState {
            db: db,
            // logged_in_users: Arc::new(Mutex::new(HashMap::new())),
            logged_in_users: Mutex::new(HashMap::new()),
        }
    }
}

// impl<T: Clone + Database> Deref for AppState<T> {
//     type Target = T;
//
//     fn deref(&self) -> &Self::Target {
//         &self.value
//     }
// }
//
// impl<T: Clone + Database> DerefMut for AppState<T> {
//     fn deref_mut(&mut self) -> &mut Self::Target {
//         &mut self.value
//     }
// }
