use anyhow::Result;
use chrono::Utc;
use shared::*;
use sqlx::PgPool;
use uuid::Uuid;

pub struct Database {
    pool: PgPool,
}

impl Database {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    // Session operations
    pub async fn create_session(&self, req: CreateSessionRequest) -> Result<WorkSession> {
        let session_id = Uuid::new_v4();
        let now = Utc::now();

        let mut tx = self.pool.begin().await?;

        // Insert session
        sqlx::query!(
            "INSERT INTO work_sessions (id, duration_seconds, description, created_at, updated_at) 
             VALUES ($1, $2, $3, $4, $5)",
            session_id,
            req.duration_seconds,
            req.description,
            now,
            now
        )
        .execute(&mut *tx)
        .await?;

        // Insert session tags
        for tag_id in &req.tag_ids {
            sqlx::query!(
                "INSERT INTO session_tags (session_id, tag_id) VALUES ($1, $2)",
                session_id,
                tag_id
            )
            .execute(&mut *tx)
            .await?;
        }

        tx.commit().await?;

        Ok(WorkSession {
            id: session_id,
            duration_seconds: req.duration_seconds,
            description: req.description,
            created_at: now,
            updated_at: now,
        })
    }

    pub async fn get_session(&self, id: Uuid) -> Result<Option<WorkSessionWithTags>> {
        let session = sqlx::query_as!(
            WorkSession,
            "SELECT id, duration_seconds, description, created_at, updated_at 
             FROM work_sessions WHERE id = $1",
            id
        )
        .fetch_optional(&self.pool)
        .await?;

        if let Some(session) = session {
            let tags = self.get_session_tags(id).await?;
            Ok(Some(WorkSessionWithTags {
                id: session.id,
                duration_seconds: session.duration_seconds,
                description: session.description,
                created_at: session.created_at,
                updated_at: session.updated_at,
                tags,
            }))
        } else {
            Ok(None)
        }
    }

    pub async fn get_sessions(&self) -> Result<Vec<WorkSessionWithTags>> {
        let sessions = sqlx::query_as!(
            WorkSession,
            "SELECT id, duration_seconds, description, created_at, updated_at 
             FROM work_sessions ORDER BY created_at DESC"
        )
        .fetch_all(&self.pool)
        .await?;

        let mut sessions_with_tags = Vec::new();
        for session in sessions {
            let tags = self.get_session_tags(session.id).await?;
            sessions_with_tags.push(WorkSessionWithTags {
                id: session.id,
                duration_seconds: session.duration_seconds,
                description: session.description,
                created_at: session.created_at,
                updated_at: session.updated_at,
                tags,
            });
        }

        Ok(sessions_with_tags)
    }

    pub async fn update_session(&self, id: Uuid, req: UpdateSessionRequest) -> Result<Option<WorkSession>> {
        let mut tx = self.pool.begin().await?;

        // Update session
        let updated_session = sqlx::query_as!(
            WorkSession,
            "UPDATE work_sessions 
             SET duration_seconds = COALESCE($2, duration_seconds),
                 description = COALESCE($3, description),
                 updated_at = $4
             WHERE id = $1
             RETURNING id, duration_seconds, description, created_at, updated_at",
            id,
            req.duration_seconds,
            req.description,
            Utc::now()
        )
        .fetch_optional(&mut *tx)
        .await?;

        if let Some(_session) = &updated_session {
            // Update tags if provided
            if let Some(tag_ids) = req.tag_ids {
                // Delete existing tags
                sqlx::query!("DELETE FROM session_tags WHERE session_id = $1", id)
                    .execute(&mut *tx)
                    .await?;

                // Insert new tags
                for tag_id in tag_ids {
                    sqlx::query!(
                        "INSERT INTO session_tags (session_id, tag_id) VALUES ($1, $2)",
                        id,
                        tag_id
                    )
                    .execute(&mut *tx)
                    .await?;
                }
            }
        }

        tx.commit().await?;
        Ok(updated_session)
    }

    pub async fn delete_session(&self, id: Uuid) -> Result<bool> {
        let mut tx = self.pool.begin().await?;

        // Delete session tags first
        sqlx::query!("DELETE FROM session_tags WHERE session_id = $1", id)
            .execute(&mut *tx)
            .await?;

        // Delete session
        let result = sqlx::query!("DELETE FROM work_sessions WHERE id = $1", id)
            .execute(&mut *tx)
            .await?;

        tx.commit().await?;
        Ok(result.rows_affected() > 0)
    }

    // Tag operations
    pub async fn create_tag(&self, req: CreateTagRequest) -> Result<Tag> {
        let tag_id = Uuid::new_v4();
        let now = Utc::now();

        let tag = sqlx::query_as!(
            Tag,
            "INSERT INTO tags (id, name, color, created_at) VALUES ($1, $2, $3, $4) 
             RETURNING id, name, color, created_at",
            tag_id,
            req.name,
            req.color,
            now
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(tag)
    }

    pub async fn get_tags(&self) -> Result<Vec<Tag>> {
        let tags = sqlx::query_as!(
            Tag,
            "SELECT id, name, color, created_at FROM tags ORDER BY name"
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(tags)
    }

    pub async fn get_tag(&self, id: Uuid) -> Result<Option<Tag>> {
        let tag = sqlx::query_as!(
            Tag,
            "SELECT id, name, color, created_at FROM tags WHERE id = $1",
            id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(tag)
    }

    pub async fn update_tag(&self, id: Uuid, req: UpdateTagRequest) -> Result<Option<Tag>> {
        let tag = sqlx::query_as!(
            Tag,
            "UPDATE tags 
             SET name = COALESCE($2, name),
                 color = COALESCE($3, color)
             WHERE id = $1
             RETURNING id, name, color, created_at",
            id,
            req.name,
            req.color
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(tag)
    }

    pub async fn delete_tag(&self, id: Uuid) -> Result<bool> {
        let mut tx = self.pool.begin().await?;

        // Delete session tags first
        sqlx::query!("DELETE FROM session_tags WHERE tag_id = $1", id)
            .execute(&mut *tx)
            .await?;

        // Delete tag
        let result = sqlx::query!("DELETE FROM tags WHERE id = $1", id)
            .execute(&mut *tx)
            .await?;

        tx.commit().await?;
        Ok(result.rows_affected() > 0)
    }

    // Helper methods
    async fn get_session_tags(&self, session_id: Uuid) -> Result<Vec<Tag>> {
        let tags = sqlx::query_as!(
            Tag,
            "SELECT t.id, t.name, t.color, t.created_at 
             FROM tags t 
             JOIN session_tags st ON t.id = st.tag_id 
             WHERE st.session_id = $1
             ORDER BY t.name",
            session_id
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(tags)
    }
}