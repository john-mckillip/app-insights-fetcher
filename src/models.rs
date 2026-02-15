use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct QueryResponse {
    pub tables: Vec<Table>,
}

#[derive(Debug, Deserialize)]
pub struct Table {
    pub rows: Vec<Vec<serde_json::Value>>,
}

#[derive(Debug)]
pub struct Exception {
    pub timestamp: String,
    pub exception_type: String,
    pub message: String,
    pub operation_name: String,
}