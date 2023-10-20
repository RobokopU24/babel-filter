use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct FilterFormat {
  pub id: String,
  pub category: Vec<String>,
}