pub use text_storage::TextStorage;
pub use storage_interface::StorageInterface;
pub use sqlite_storage::SqliteStorage;

mod storage_interface;
mod text_storage;
mod sqlite_storage;
