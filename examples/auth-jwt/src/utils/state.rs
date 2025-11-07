use sqlx::{Pool, Sqlite};

#[derive(Clone)]
pub struct AppState {
    pub db_pool: Pool<Sqlite>,
}

impl AppState {
    pub fn new(db_pool: Pool<Sqlite>) -> Self {
        Self { db_pool }
    }
}
