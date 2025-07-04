use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
};
use shared::*;
use uuid::Uuid;

use crate::AppState;

// Session handlers
pub async fn get_sessions(
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<Vec<WorkSessionWithTags>>>, StatusCode> {
    match state.db.get_sessions().await {
        Ok(sessions) => Ok(Json(ApiResponse::success(sessions))),
        Err(e) => {
            tracing::error!("Failed to get sessions: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn get_session(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<WorkSessionWithTags>>, StatusCode> {
    match state.db.get_session(id).await {
        Ok(Some(session)) => Ok(Json(ApiResponse::success(session))),
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(e) => {
            tracing::error!("Failed to get session {}: {}", id, e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn create_session(
    State(state): State<AppState>,
    Json(req): Json<CreateSessionRequest>,
) -> Result<Json<ApiResponse<WorkSession>>, StatusCode> {
    match state.db.create_session(req).await {
        Ok(session) => Ok(Json(ApiResponse::success(session))),
        Err(e) => {
            tracing::error!("Failed to create session: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn update_session(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(req): Json<UpdateSessionRequest>,
) -> Result<Json<ApiResponse<WorkSession>>, StatusCode> {
    match state.db.update_session(id, req).await {
        Ok(Some(session)) => Ok(Json(ApiResponse::success(session))),
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(e) => {
            tracing::error!("Failed to update session {}: {}", id, e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn delete_session(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<()>>, StatusCode> {
    match state.db.delete_session(id).await {
        Ok(true) => Ok(Json(ApiResponse::success(()))),
        Ok(false) => Err(StatusCode::NOT_FOUND),
        Err(e) => {
            tracing::error!("Failed to delete session {}: {}", id, e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

// Tag handlers
pub async fn get_tags(
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<Vec<Tag>>>, StatusCode> {
    match state.db.get_tags().await {
        Ok(tags) => Ok(Json(ApiResponse::success(tags))),
        Err(e) => {
            tracing::error!("Failed to get tags: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn get_tag(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<Tag>>, StatusCode> {
    match state.db.get_tag(id).await {
        Ok(Some(tag)) => Ok(Json(ApiResponse::success(tag))),
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(e) => {
            tracing::error!("Failed to get tag {}: {}", id, e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn create_tag(
    State(state): State<AppState>,
    Json(req): Json<CreateTagRequest>,
) -> Result<Json<ApiResponse<Tag>>, StatusCode> {
    match state.db.create_tag(req).await {
        Ok(tag) => Ok(Json(ApiResponse::success(tag))),
        Err(e) => {
            tracing::error!("Failed to create tag: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn update_tag(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(req): Json<UpdateTagRequest>,
) -> Result<Json<ApiResponse<Tag>>, StatusCode> {
    match state.db.update_tag(id, req).await {
        Ok(Some(tag)) => Ok(Json(ApiResponse::success(tag))),
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(e) => {
            tracing::error!("Failed to update tag {}: {}", id, e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn delete_tag(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<()>>, StatusCode> {
    match state.db.delete_tag(id).await {
        Ok(true) => Ok(Json(ApiResponse::success(()))),
        Ok(false) => Err(StatusCode::NOT_FOUND),
        Err(e) => {
            tracing::error!("Failed to delete tag {}: {}", id, e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}