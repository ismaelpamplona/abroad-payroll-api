use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

pub mod get_by_id;
pub mod list;
pub mod save;

pub use get_by_id::get_by_id;
pub use list::list;
pub use save::handle_post_request;

#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct BankPayload {
    name: String,
    number: String,
}

#[derive(Serialize, FromRow, Debug)]
pub struct BankResponse {
    id: Uuid,
    name: String,
    number: String,
}

#[derive(Deserialize)]
pub struct BankFilter {
    names: Option<String>,
    numbers: Option<String>,
}
