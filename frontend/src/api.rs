use gloo_net::http::Request;
use shared::{ApiResponse, CreateSessionRequest, CreateTagRequest, Tag, UpdateSessionRequest, UpdateTagRequest, WorkSession, WorkSessionWithTags};
use uuid::Uuid;

const API_BASE: &str = "http://localhost:8080/api";

// Helper function to parse API responses
fn parse_api_response<T>(text: &str) -> Result<T, String> 
where 
    T: serde::de::DeserializeOwned,
{
    let api_response: ApiResponse<T> = serde_json::from_str(text)
        .map_err(|e| format!("Failed to parse API response: {}", e))?;
    
    if api_response.success {
        api_response.data.ok_or_else(|| "No data in successful response".to_string())
    } else {
        Err(api_response.message.unwrap_or_else(|| "Unknown API error".to_string()))
    }
}

pub async fn get_sessions() -> Result<Vec<WorkSessionWithTags>, String> {
    let response = Request::get(&format!("{API_BASE}/sessions"))
        .send()
        .await
        .map_err(|e| format!("Request failed: {e}"))?;

    let text = response
        .text()
        .await
        .map_err(|e| format!("Failed to get response text: {e}"))?;

    parse_api_response(&text)
}

pub async fn get_session(id: Uuid) -> Result<WorkSessionWithTags, String> {
    let response = Request::get(&format!("{API_BASE}/sessions/{id}"))
        .send()
        .await
        .map_err(|e| format!("Request failed: {e}"))?;

    let text = response
        .text()
        .await
        .map_err(|e| format!("Failed to get response text: {e}"))?;

    parse_api_response(&text)
}

pub async fn create_session(req: CreateSessionRequest) -> Result<WorkSession, String> {
    let response = Request::post(&format!("{API_BASE}/sessions"))
        .json(&req)
        .map_err(|e| format!("Failed to serialize request: {e}"))?
        .send()
        .await
        .map_err(|e| format!("Request failed: {e}"))?;

    let text = response
        .text()
        .await
        .map_err(|e| format!("Failed to get response text: {e}"))?;

    parse_api_response(&text)
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
        .map_err(|e| format!("Failed to get response text: {e}"))?;

    parse_api_response(&text)
}

pub async fn delete_session(id: Uuid) -> Result<(), String> {
    let response = Request::delete(&format!("{API_BASE}/sessions/{id}"))
        .send()
        .await
        .map_err(|e| format!("Request failed: {e}"))?;

    if response.ok() {
        let text = response
            .text()
            .await
            .map_err(|e| format!("Failed to get response text: {e}"))?;
        
        // Handle empty response body for DELETE requests
        if text.trim().is_empty() {
            Ok(())
        } else {
            parse_api_response::<()>(&text)
        }
    } else {
        let text = response
            .text()
            .await
            .unwrap_or_else(|_| "Failed to delete session".to_string());
        
        // Try to parse as ApiResponse to get error message
        if let Ok(api_response) = serde_json::from_str::<ApiResponse<()>>(&text) {
            if !api_response.success {
                return Err(api_response.message.unwrap_or_else(|| "Failed to delete session".to_string()));
            }
        }
        
        Err(format!("Failed to delete session: {}", text))
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
        .map_err(|e| format!("Failed to get response text: {e}"))?;

    parse_api_response(&text)
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
        .map_err(|e| format!("Failed to get response text: {e}"))?;

    parse_api_response(&text)
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
        .map_err(|e| format!("Failed to get response text: {e}"))?;

    parse_api_response(&text)
}

pub async fn delete_tag(id: Uuid) -> Result<(), String> {
    let response = Request::delete(&format!("{API_BASE}/tags/{id}"))
        .send()
        .await
        .map_err(|e| format!("Request failed: {e}"))?;

    if response.ok() {
        let text = response
            .text()
            .await
            .map_err(|e| format!("Failed to get response text: {e}"))?;
        
        // Handle empty response body for DELETE requests
        if text.trim().is_empty() {
            Ok(())
        } else {
            parse_api_response::<()>(&text)
        }
    } else {
        let text = response
            .text()
            .await
            .unwrap_or_else(|_| "Failed to delete tag".to_string());
        
        // Try to parse as ApiResponse to get error message
        if let Ok(api_response) = serde_json::from_str::<ApiResponse<()>>(&text) {
            if !api_response.success {
                return Err(api_response.message.unwrap_or_else(|| "Failed to delete tag".to_string()));
            }
        }
        
        Err(format!("Failed to delete tag: {}", text))
    }
}