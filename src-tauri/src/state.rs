use std::sync::{Arc, Mutex};

use rusqlite::Connection;

pub struct AppState {
    pub db: Mutex<Connection>,
}

impl AppState {
    pub fn new(conn: Connection) -> Arc<Self> {
        Arc::new(Self {
            db: Mutex::new(conn),
        })
    }
}
