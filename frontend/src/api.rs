use gloo_net::http::Request;
use shared::*;
use uuid::Uuid;

const API_BASE: &str = "http://localhost:8080/api";

pub async fn get_sessions() -> Result<Vec<WorkSessionWithTags>, String> {
    let response = Request::get(&format!("{}/sessions", API_BASE))
        .send()
        .await
        .map_err(|e| format!("Request failed: {}", e))?;

    if response.ok() {
        let api_response: ApiResponse<Vec<WorkSessionWithTags>> = response
            .json()
            .await
            .map_err(|e| format!("Failed to parse response: {}", e))?;
        
        api_response.data.ok_or_else(|| "No data in response".to_string())
    } else {
        Err(format!("Request failed with status: {}", response.status()))
    }
}

pub async fn get_session(id: Uuid) -> Result<WorkSessionWithTags, String> {
    let response = Request::get(&format!("{}/sessions/{}", API_BASE, id))
        .send()
        .await
        .map_err(|e| format!("Request failed: {}", e))?;

    if response.ok() {
        let api_response: ApiResponse<WorkSessionWithTags> = response
            .json()
            .await
            .map_err(|e| format!("Failed to parse response: {}", e))?;
        
        api_response.data.ok_or_else(|| "No data in response".to_string())
    } else {
        Err(format!("Request failed with status: {}", response.status()))
    }
}

pub async fn create_session(req: CreateSessionRequest) -> Result<WorkSession, String> {
    let response = Request::post(&format!("{}/sessions", API_BASE))
        .json(&req)
        .map_err(|e| format!("Failed to serialize request: {}", e))?
        .send()
        .await
        .map_err(|e| format!("Request failed: {}", e))?;

    if response.ok() {
        let api_response: ApiResponse<WorkSession> = response
            .json()
            .await
            .map_err(|e| format!("Failed to parse response: {}", e))?;
        
        api_response.data.ok_or_else(|| "No data in response".to_string())
    } else {
        Err(format!("Request failed with status: {}", response.status()))
    }
}

pub async fn update_session(id: Uuid, req: UpdateSessionRequest) -> Result<WorkSession, String> {
    let response = Request::put(&format!("{}/sessions/{}", API_BASE, id))
        .json(&req)
        .map_err(|e| format!("Failed to serialize request: {}", e))?
        .send()
        .await
        .map_err(|e| format!("Request failed: {}", e))?;

    if response.ok() {
        let api_response: ApiResponse<WorkSession> = response
            .json()
            .await
            .map_err(|e| format!("Failed to parse response: {}", e))?;
        
        api_response.data.ok_or_else(|| "No data in response".to_string())
    } else {
        Err(format!("Request failed with status: {}", response.status()))
    }
}

pub async fn delete_session(id: Uuid) -> Result<(), String> {
    let response = Request::delete(&format!("{}/sessions/{}", API_BASE, id))
        .send()
        .await
        .map_err(|e| format!("Request failed: {}", e))?;

    if response.ok() {
        Ok(())
    } else {
        Err(format!("Request failed with status: {}", response.status()))
    }
}

pub async fn get_tags() -> Result<Vec<Tag>, String> {
    let response = Request::get(&format!("{}/tags", API_BASE))
        .send()
        .await
        .map_err(|e| format!("Request failed: {}", e))?;

    if response.ok() {
        let api_response: ApiResponse<Vec<Tag>> = response
            .json()
            .await
            .map_err(|e| format!("Failed to parse response: {}", e))?;
        
        api_response.data.ok_or_else(|| "No data in response".to_string())
    } else {
        Err(format!("Request failed with status: {}", response.status()))
    }
}

pub async fn create_tag(req: CreateTagRequest) -> Result<Tag, String> {
    let response = Request::post(&format!("{}/tags", API_BASE))
        .json(&req)
        .map_err(|e| format!("Failed to serialize request: {}", e))?
        .send()
        .await
        .map_err(|e| format!("Request failed: {}", e))?;

    if response.ok() {
        let api_response: ApiResponse<Tag> = response
            .json()
            .await
            .map_err(|e| format!("Failed to parse response: {}", e))?;
        
        api_response.data.ok_or_else(|| "No data in response".to_string())
    } else {
        Err(format!("Request failed with status: {}", response.status()))
    }
}

pub async fn update_tag(id: Uuid, req: UpdateTagRequest) -> Result<Tag, String> {
    let response = Request::put(&format!("{}/tags/{}", API_BASE, id))
        .json(&req)
        .map_err(|e| format!("Failed to serialize request: {}", e))?
        .send()
        .await
        .map_err(|e| format!("Request failed: {}", e))?;

    if response.ok() {
        let api_response: ApiResponse<Tag> = response
            .json()
            .await
            .map_err(|e| format!("Failed to parse response: {}", e))?;
        
        api_response.data.ok_or_else(|| "No data in response".to_string())
    } else {
        Err(format!("Request failed with status: {}", response.status()))
    }
}

pub async fn delete_tag(id: Uuid) -> Result<(), String> {
    let response = Request::delete(&format!("{}/tags/{}", API_BASE, id))
        .send()
        .await
        .map_err(|e| format!("Request failed: {}", e))?;

    if response.ok() {
        Ok(())
    } else {
        Err(format!("Request failed with status: {}", response.status()))
    }
}