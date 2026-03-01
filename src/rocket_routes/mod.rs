use rocket::http::Status;
use rocket_db_pools::Database;

#[derive(Database)]
#[database("db")]
pub struct Db(rocket_db_pools::diesel::PgPool);

pub mod crates;
pub mod rustaceans;

/// Returns `(Status::InternalServerError, message)` and logs the error. Use in `Result<_, (Status, String)>` handlers.
pub fn server_error(e: impl std::error::Error) -> (Status, String) {
    rocket::error!("Server error: {}", e);
    (Status::InternalServerError, e.to_string())
}

/// Returns `(Status::NotFound, message)`. Use in `Result<_, (Status, String)>` handlers.
pub fn not_found_error(msg: impl Into<String>) -> (Status, String) {
    (Status::NotFound, msg.into())
}