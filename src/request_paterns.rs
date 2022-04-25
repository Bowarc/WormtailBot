use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct UptimeResponse {
    pub data: Vec<UptimeResponseData>,
    pub pagination: serde_json::Value,
}

#[derive(Deserialize, Debug)]
pub struct UptimeResponseData {
    pub id: String,
    pub user_id: String,
    pub user_login: String,
    pub user_name: String,
    pub game_id: String,
    pub game_name: String,
    pub r#type: String,
    pub title: String,
    pub viewer_count: i32,
    pub started_at: String,
    pub language: String,
    pub thumbnail_url: String,
    pub tag_ids: Vec<String>,
    pub is_mature: bool,
}
