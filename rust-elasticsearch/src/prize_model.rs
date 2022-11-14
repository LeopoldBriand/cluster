use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Prize {
    year: String,
    category: String,
    #[serde(default = "default_laureates")]
    laureates: Vec<Laureate>
}
#[derive(Debug, Deserialize, Serialize)]
pub struct Laureate {
    #[serde(default = "default_names")]
    firstname: String,
    #[serde(default = "default_names")]
    surname: String,
    motivation: String
}

fn default_names() -> String {
    "unknown".to_string()
}

fn default_laureates() -> Vec<Laureate> {
    vec![]
}