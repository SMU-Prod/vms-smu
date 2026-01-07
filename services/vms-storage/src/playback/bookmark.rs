//! Bookmark Manager
//! Gerenciamento de bookmarks (marcadores temporais)

use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Bookmark {
    pub id: String,
    pub camera_id: String,
    pub timestamp: DateTime<Utc>,
    pub user_id: String,
    pub note: String,
    pub tags: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateBookmarkRequest {
    pub camera_id: String,
    pub timestamp: DateTime<Utc>,
    pub user_id: String,
    pub note: String,
    #[serde(default)]
    pub tags: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateBookmarkRequest {
    pub note: Option<String>,
    pub tags: Option<Vec<String>>,
}

pub struct BookmarkManager {
    // Em produção, seria RocksDB ou PostgreSQL
    // Por enquanto, in-memory para MVP
    bookmarks: Arc<RwLock<HashMap<String, Bookmark>>>,
}

impl BookmarkManager {
    pub fn new() -> Self {
        Self {
            bookmarks: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Criar novo bookmark
    pub async fn create_bookmark(&self, request: CreateBookmarkRequest) -> Result<Bookmark> {
        let bookmark = Bookmark {
            id: Uuid::new_v4().to_string(),
            camera_id: request.camera_id,
            timestamp: request.timestamp,
            user_id: request.user_id,
            note: request.note,
            tags: request.tags,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        let mut bookmarks = self.bookmarks.write().await;
        bookmarks.insert(bookmark.id.clone(), bookmark.clone());

        tracing::info!("Created bookmark: {}", bookmark.id);

        Ok(bookmark)
    }

    /// Listar bookmarks de uma câmera em um intervalo de tempo
    pub async fn list_bookmarks(
        &self,
        camera_id: &str,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
    ) -> Result<Vec<Bookmark>> {
        let bookmarks = self.bookmarks.read().await;

        let mut results: Vec<Bookmark> = bookmarks
            .values()
            .filter(|b| {
                b.camera_id == camera_id && b.timestamp >= start && b.timestamp <= end
            })
            .cloned()
            .collect();

        // Ordenar por timestamp
        results.sort_by_key(|b| b.timestamp);

        Ok(results)
    }

    /// Obter bookmark por ID
    pub async fn get_bookmark(&self, id: &str) -> Result<Option<Bookmark>> {
        let bookmarks = self.bookmarks.read().await;
        Ok(bookmarks.get(id).cloned())
    }

    /// Atualizar bookmark
    pub async fn update_bookmark(
        &self,
        id: &str,
        request: UpdateBookmarkRequest,
    ) -> Result<Option<Bookmark>> {
        let mut bookmarks = self.bookmarks.write().await;

        if let Some(bookmark) = bookmarks.get_mut(id) {
            if let Some(note) = request.note {
                bookmark.note = note;
            }
            if let Some(tags) = request.tags {
                bookmark.tags = tags;
            }
            bookmark.updated_at = Utc::now();

            tracing::info!("Updated bookmark: {}", id);

            Ok(Some(bookmark.clone()))
        } else {
            Ok(None)
        }
    }

    /// Deletar bookmark
    pub async fn delete_bookmark(&self, id: &str) -> Result<bool> {
        let mut bookmarks = self.bookmarks.write().await;
        let existed = bookmarks.remove(id).is_some();

        if existed {
            tracing::info!("Deleted bookmark: {}", id);
        }

        Ok(existed)
    }

    /// Listar todos os bookmarks de uma câmera
    pub async fn list_camera_bookmarks(&self, camera_id: &str) -> Result<Vec<Bookmark>> {
        let bookmarks = self.bookmarks.read().await;

        let mut results: Vec<Bookmark> = bookmarks
            .values()
            .filter(|b| b.camera_id == camera_id)
            .cloned()
            .collect();

        results.sort_by_key(|b| b.timestamp);

        Ok(results)
    }

    /// Buscar bookmarks por tags
    pub async fn search_by_tags(&self, tags: &[String]) -> Result<Vec<Bookmark>> {
        let bookmarks = self.bookmarks.read().await;

        let results: Vec<Bookmark> = bookmarks
            .values()
            .filter(|b| tags.iter().any(|tag| b.tags.contains(tag)))
            .cloned()
            .collect();

        Ok(results)
    }
}

impl Default for BookmarkManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_bookmark_crud() {
        let manager = BookmarkManager::new();

        // Create
        let bookmark = manager
            .create_bookmark(CreateBookmarkRequest {
                camera_id: "cam1".to_string(),
                timestamp: Utc::now(),
                user_id: "user1".to_string(),
                note: "Important event".to_string(),
                tags: vec!["critical".to_string()],
            })
            .await
            .unwrap();

        assert_eq!(bookmark.camera_id, "cam1");
        assert_eq!(bookmark.note, "Important event");

        // Read
        let found = manager.get_bookmark(&bookmark.id).await.unwrap();
        assert!(found.is_some());
        assert_eq!(found.unwrap().id, bookmark.id);

        // Update
        let updated = manager
            .update_bookmark(
                &bookmark.id,
                UpdateBookmarkRequest {
                    note: Some("Updated note".to_string()),
                    tags: None,
                },
            )
            .await
            .unwrap();

        assert!(updated.is_some());
        assert_eq!(updated.unwrap().note, "Updated note");

        // Delete
        let deleted = manager.delete_bookmark(&bookmark.id).await.unwrap();
        assert!(deleted);

        let found_after_delete = manager.get_bookmark(&bookmark.id).await.unwrap();
        assert!(found_after_delete.is_none());
    }

    #[tokio::test]
    async fn test_list_by_time_range() {
        let manager = BookmarkManager::new();

        let now = Utc::now();

        // Criar bookmarks em tempos diferentes
        for i in 0..5 {
            manager
                .create_bookmark(CreateBookmarkRequest {
                    camera_id: "cam1".to_string(),
                    timestamp: now - chrono::Duration::hours(i),
                    user_id: "user1".to_string(),
                    note: format!("Event {}", i),
                    tags: vec![],
                })
                .await
                .unwrap();
        }

        // Buscar últimas 3 horas
        let start = now - chrono::Duration::hours(3);
        let end = now + chrono::Duration::hours(1);

        let results = manager.list_bookmarks("cam1", start, end).await.unwrap();

        // Deve retornar 4 bookmarks (0, 1, 2, 3 horas atrás)
        assert_eq!(results.len(), 4);
    }
}
