pub mod router;
pub use router::init_keys_router; // реэкспортируем функцию, чтобы можно было писать keys::router()

mod jwt_service;
mod key_store;
mod keys_repo;
mod keys_repo_impl;
mod models;

pub fn mount() {}
