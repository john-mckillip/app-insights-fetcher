use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct QueryResponse {
    pub tables: Vec<Table>,
}

#[derive(Debug, Deserialize)]
pub struct Table {
    #[allow(dead_code)]
    pub columns: Vec<Column>,
    pub rows: Vec<Vec<serde_json::Value>>,
}

#[derive(Debug, Deserialize)]
pub struct Column {
    #[allow(dead_code)]
    pub name: String,
    #[allow(dead_code)]
    #[serde(rename = "type")]
    pub column_type: String,
}

#[derive(Debug)]
pub struct Exception {
    pub timestamp: String,
    pub exception_type: String,
    pub message: String,
    pub operation_name: String,
}
