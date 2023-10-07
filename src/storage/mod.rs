mod storage;
mod db_storage;
mod data;

pub use self::storage::DataStorage;
pub use self::db_storage::DbStorage;
pub use self::data::*;


/// Error message for DB consistency violation.
const CONSISTENCY_VIOLATION: &str = "Cannot remove item from DB because of another items referencing it";
