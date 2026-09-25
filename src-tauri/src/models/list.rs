use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ListInfo {
    pub id: i64,
    pub name: String,
    pub created_at: i64,
}
