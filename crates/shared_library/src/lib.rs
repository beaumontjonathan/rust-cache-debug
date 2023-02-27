use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Person {
    id: Uuid,
    first_name: String,
    last_name: String,
}
