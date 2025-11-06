use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

// Global in-memory store (id → original URL)
static URL_STORE: Lazy<Arc<Mutex<HashMap<String, String>>>> =
    Lazy::new(|| Arc::new(Mutex::new(HashMap::new())));

/// Generate a 6-character short ID (e.g. "aB3k9X")
pub fn generate_id() -> String {
    nanoid::nanoid!(6)
}

/// Create a short URL given the base URL (e.g., "http://localhost:8000")
pub fn shorten_url(original_url: &str, base_url: &str) -> String {
    let id = generate_id();
    save_url(&id, original_url);
    format!("{}/{}", base_url.trim_end_matches('/'), id)
}

/// Save the mapping between short ID and original URL
pub fn save_url(id: &str, original_url: &str) {
    let mut store = URL_STORE.lock().unwrap();
    store.insert(id.to_string(), original_url.to_string());
}

/// Retrieve the original URL by short ID
#[allow(dead_code)]
pub fn get_original_url(id: &str) -> Option<String> {
    let store = URL_STORE.lock().unwrap();
    store.get(id).cloned()
}
