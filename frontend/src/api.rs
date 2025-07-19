use gloo_net::http::Request;
use shared::{CreateSessionRequest, CreateTagRequest, Tag, UpdateSessionRequest, UpdateTagRequest, WorkSessionWithTags};
use uuid::Uuid;

const API_BASE: &str = "http://localhost:3000/api";

pub async fn get_sessions() -> Result<Vec<WorkSessionWithTags>, String> {
    let response = Request::get(&format!("{API_BASE}/sessions"))
        .send()
        .await
        .map_err(|e| format!("Request failed: {e}"))?;

    let text = response
        .text()
        .await
        .map_err(|e| format!("Failed to parse response: {e}"))?;

    serde_json::from_str(&text).map_err(|e| format!("Failed to parse response: {e}"))
}

pub async fn get_session(id: Uuid) -> Result<WorkSessionWithTags, String> {
    let response = Request::get(&format!("{API_BASE}/sessions/{id}"))
        .send()
        .await
        .map_err(|e| format!("Request failed: {e}"))?;

    let text = response
        .text()
        .await
        .map_err(|e| format!("Failed to parse response: {e}"))?;

    serde_json::from_str(&text).map_err(|e| format!("Failed to parse response: {e}"))
}

pub async fn create_session(req: CreateSessionRequest) -> Result<WorkSessionWithTags, String> {
    let response = Request::post(&format!("{API_BASE}/sessions"))
        .json(&req)
        .map_err(|e| format!("Failed to serialize request: {e}"))?
        .send()
        .await
        .map_err(|e| format!("Request failed: {e}"))?;

    let text = response
        .text()
        .await
        .map_err(|e| format!("Failed to parse response: {e}"))?;

    serde_json::from_str(&text).map_err(|e| format!("Failed to parse response: {e}"))
}

#[allow(dead_code)]
pub async fn update_session(id: Uuid, req: UpdateSessionRequest) -> Result<WorkSessionWithTags, String> {
    let response = Request::put(&format!("{API_BASE}/sessions/{id}"))
        .json(&req)
        .map_err(|e| format!("Failed to serialize request: {e}"))?
        .send()
        .await
        .map_err(|e| format!("Request failed: {e}"))?;

    let text = response
        .text()
        .await
        .map_err(|e| format!("Failed to parse response: {e}"))?;

    serde_json::from_str(&text).map_err(|e| format!("Failed to parse response: {e}"))
}

pub async fn delete_session(id: Uuid) -> Result<(), String> {
    let response = Request::delete(&format!("{API_BASE}/sessions/{id}"))
        .send()
        .await
        .map_err(|e| format!("Request failed: {e}"))?;

    if response.ok() {
        Ok(())
    } else {
        Err("Failed to delete session".to_string())
    }
}

pub async fn get_tags() -> Result<Vec<Tag>, String> {
    let response = Request::get(&format!("{API_BASE}/tags"))
        .send()
        .await
        .map_err(|e| format!("Request failed: {e}"))?;

    let text = response
        .text()
        .await
        .map_err(|e| format!("Failed to parse response: {e}"))?;

    serde_json::from_str(&text).map_err(|e| format!("Failed to parse response: {e}"))
}

pub async fn create_tag(req: CreateTagRequest) -> Result<Tag, String> {
    let response = Request::post(&format!("{API_BASE}/tags"))
        .json(&req)
        .map_err(|e| format!("Failed to serialize request: {e}"))?
        .send()
        .await
        .map_err(|e| format!("Request failed: {e}"))?;

    let text = response
        .text()
        .await
        .map_err(|e| format!("Failed to parse response: {e}"))?;

    serde_json::from_str(&text).map_err(|e| format!("Failed to parse response: {e}"))
}

pub async fn update_tag(id: Uuid, req: UpdateTagRequest) -> Result<Tag, String> {
    let response = Request::put(&format!("{API_BASE}/tags/{id}"))
        .json(&req)
        .map_err(|e| format!("Failed to serialize request: {e}"))?
        .send()
        .await
        .map_err(|e| format!("Request failed: {e}"))?;

    let text = response
        .text()
        .await
        .map_err(|e| format!("Failed to parse response: {e}"))?;

    serde_json::from_str(&text).map_err(|e| format!("Failed to parse response: {e}"))
}

pub async fn delete_tag(id: Uuid) -> Result<(), String> {
    let response = Request::delete(&format!("{API_BASE}/tags/{id}"))
        .send()
        .await
        .map_err(|e| format!("Request failed: {e}"))?;

    if response.ok() {
        Ok(())
    } else {
        Err("Failed to delete tag".to_string())
    }
}