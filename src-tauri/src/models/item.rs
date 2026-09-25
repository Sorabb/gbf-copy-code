use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    pub list_id: i64,
    pub code: String,
    pub created_at: i64,
    pub expires_at: i64,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExpiredItem {
    pub list_id: i64,
    pub code: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct AddCodesResult {
    pub added: usize,
    pub duplicates: usize,
    pub invalid: usize,
}
