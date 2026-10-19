//! Hochgeladene Bilder liegen als BLOB in SQLite und werden unter `/media/{id}` ausgeliefert.

use axum::{
    extract::Path,
    http::{header, StatusCode},
    response::{IntoResponse, Response},
};
use imagesize::ImageType;
use leptos::prelude::ServerFnError;

use super::{db::pool, db_err, user_err};
use crate::models::MediaInfo;

pub const MAX_UPLOAD_BYTES: usize = 15 * 1024 * 1024;

pub async fn serve(Path(id): Path<i64>) -> Response {
    let row: Result<Option<(String, Vec<u8>)>, _> = sqlx::query_as("SELECT content_type, data FROM media WHERE id = ?")
        .bind(id)
        .fetch_optional(pool())
        .await;

    match row {
        Ok(Some((content_type, data))) => (
            [
                (header::CONTENT_TYPE, content_type),
                // IDs werden nie wiederverwendet → unbegrenzt cachebar
                (header::CACHE_CONTROL, "public, max-age=31536000, immutable".to_string()),
                (header::X_CONTENT_TYPE_OPTIONS, "nosniff".to_string()),
            ],
            data,
        )
            .into_response(),
        Ok(None) => StatusCode::NOT_FOUND.into_response(),
        Err(e) => {
            eprintln!("[media] {e}");
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

/// Bild prüfen (Format anhand der Datei selbst, nicht der Endung) und speichern
pub async fn store(filename: &str, bytes: Vec<u8>) -> Result<MediaInfo, ServerFnError> {
    let content_type = match imagesize::image_type(&bytes) {
        Ok(ImageType::Jpeg) => "image/jpeg",
        Ok(ImageType::Png) => "image/png",
        Ok(ImageType::Webp) => "image/webp",
        Ok(ImageType::Gif) => "image/gif",
        _ => return Err(user_err("Nur JPG, PNG, WebP oder GIF sind erlaubt.")),
    };
    let (width, height) = imagesize::blob_size(&bytes)
        .map(|s| (Some(s.width as u32), Some(s.height as u32)))
        .unwrap_or((None, None));

    let (id,): (i64,) = sqlx::query_as(
        "INSERT INTO media (filename, content_type, data, width, height) VALUES (?, ?, ?, ?, ?) RETURNING id",
    )
    .bind(filename.chars().take(200).collect::<String>())
    .bind(content_type)
    .bind(bytes)
    .bind(width)
    .bind(height)
    .fetch_one(pool())
    .await
    .map_err(db_err)?;

    Ok(MediaInfo { url: format!("/media/{id}"), width, height })
}

/// Hochgeladenes Bild löschen, wenn es nirgends mehr verwendet wird
pub async fn cleanup(url: &str) -> Result<(), ServerFnError> {
    let Some(id) = url.strip_prefix("/media/").and_then(|s| s.parse::<i64>().ok()) else {
        return Ok(());
    };
    let (uses,): (i64,) = sqlx::query_as(
        "SELECT (SELECT COUNT(*) FROM photos WHERE image_url = ?1 OR thumb_url = ?1)
              + (SELECT COUNT(*) FROM content WHERE value = ?1)",
    )
    .bind(url)
    .fetch_one(pool())
    .await
    .map_err(db_err)?;

    if uses == 0 {
        sqlx::query("DELETE FROM media WHERE id = ?").bind(id).execute(pool()).await.map_err(db_err)?;
    }
    Ok(())
}

/// Erlaubte Bildquellen: eigene Uploads oder externe HTTPS-Adressen
pub fn validate_image_url(url: &str) -> Result<(), ServerFnError> {
    let ok = url.is_empty()
        || (url.starts_with("/media/") && url[7..].chars().all(|c| c.is_ascii_digit()))
        || (url.starts_with("https://") && !url.chars().any(|c| c.is_whitespace() || "\"'()<>\\".contains(c)));
    if ok {
        Ok(())
    } else {
        Err(user_err("Bild-Adressen müssen mit https:// beginnen oder ein hochgeladenes Bild sein."))
    }
}
