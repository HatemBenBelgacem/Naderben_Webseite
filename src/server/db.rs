use std::sync::OnceLock;

use leptos::prelude::ServerFnError;
use sqlx::{postgres::PgPoolOptions, PgPool};

use super::{auth, db_err};
use crate::{
    data::{Category, PhotoItem, VideoItem, SEED_PHOTOS, SEED_VIDEOS},
    models::{ADMIN_ROLE_ID, OWNER_EMAIL},
};

/// Lokale Standardverbindung, passend zu `docker-compose.yml`
const DEFAULT_URL: &str = "postgres://naderben:naderben@localhost:5432/naderben";

static POOL: OnceLock<PgPool> = OnceLock::new();

pub fn pool() -> &'static PgPool {
    POOL.get().expect("Datenbank wurde nicht initialisiert")
}

/// Verbindung öffnen, Migrationen ausführen, Startdaten und Admin-Konto anlegen.
///
/// `DATABASE_URL` zeigt auf PostgreSQL, z. B. `postgres://naderben:naderben@localhost:5432/naderben`.
/// Lokal startet die Datenbank mit `docker compose up -d`, auf Railway liefert der PostgreSQL-Dienst die URL.
pub async fn init() -> Result<(), Box<dyn std::error::Error>> {
    let url = std::env::var("DATABASE_URL").unwrap_or_else(|_| DEFAULT_URL.into());
    let pool = PgPoolOptions::new().max_connections(8).connect(&url).await?;

    sqlx::migrate!("./migrations").run(&pool).await?;
    seed_portfolio(&pool).await?;
    ensure_owner(&pool).await?;
    sqlx::query("DELETE FROM sessions WHERE expires_at < $1")
        .bind(auth::now())
        .execute(&pool)
        .await?;

    println!("[db] PostgreSQL bereit");
    let _ = POOL.set(pool);
    Ok(())
}

/// Beim allerersten Start die Platzhalter-Galerien einspielen
async fn seed_portfolio(pool: &PgPool) -> Result<(), sqlx::Error> {
    let seeded: Option<(String,)> = sqlx::query_as("SELECT value FROM app_meta WHERE key = 'portfolio_seeded'")
        .fetch_optional(pool)
        .await?;
    if seeded.is_some() {
        return Ok(());
    }

    let mut tx = pool.begin().await?;
    for (i, p) in SEED_PHOTOS.iter().enumerate() {
        sqlx::query(
            "INSERT INTO photos (category, title, image_url, thumb_url, width, height, sort_order) VALUES ($1, $2, $3, $4, $5, $6, $7)",
        )
        .bind(p.category.slug())
        .bind(p.title)
        .bind(p.url(1800))
        .bind(p.url(800))
        .bind(i64::from(p.width) * 200)
        .bind(i64::from(p.height) * 200)
        .bind(i as i64)
        .execute(&mut *tx)
        .await?;
    }
    for (i, v) in SEED_VIDEOS.iter().enumerate() {
        sqlx::query("INSERT INTO videos (category, title, client, duration, youtube_id, sort_order) VALUES ($1, $2, $3, $4, $5, $6)")
            .bind(v.category.slug())
            .bind(v.title)
            .bind(v.client)
            .bind(v.duration)
            .bind(v.youtube_id)
            .bind(i as i64)
            .execute(&mut *tx)
            .await?;
    }
    sqlx::query("INSERT INTO app_meta (key, value) VALUES ('portfolio_seeded', to_char(now() AT TIME ZONE 'UTC', 'YYYY-MM-DD HH24:MI:SS'))")
        .execute(&mut *tx)
        .await?;
    tx.commit().await?;
    println!("[db] Startdaten für das Portfolio eingespielt");
    Ok(())
}

/// Das Konto von OWNER_EMAIL existiert immer, ist aktiv und hat die Admin-Rolle.
async fn ensure_owner(pool: &PgPool) -> Result<(), Box<dyn std::error::Error>> {
    let existing: Option<(i64,)> = sqlx::query_as("SELECT id FROM users WHERE email = $1")
        .bind(OWNER_EMAIL)
        .fetch_optional(pool)
        .await?;

    let env_password = std::env::var("ADMIN_PASSWORD").ok().filter(|p| !p.trim().is_empty());

    if let Some((id,)) = existing {
        sqlx::query("UPDATE users SET role_id = $1, active = TRUE WHERE id = $2")
            .bind(ADMIN_ROLE_ID)
            .bind(id)
            .execute(pool)
            .await?;

        // Notfall-Reset: ADMIN_PASSWORD_RESET=1 setzt das Passwort auf ADMIN_PASSWORD
        let reset = std::env::var("ADMIN_PASSWORD_RESET").is_ok_and(|v| v == "1" || v.eq_ignore_ascii_case("true"));
        if reset {
            let Some(password) = env_password else {
                eprintln!("[auth] ADMIN_PASSWORD_RESET ist gesetzt, aber ADMIN_PASSWORD fehlt – kein Reset.");
                return Ok(());
            };
            if let Err(ServerFnError::ServerError(msg)) = auth::validate_password(&password) {
                return Err(format!("ADMIN_PASSWORD: {msg}").into());
            }
            let hash = auth::hash_password(&password).await.map_err(|e| e.to_string())?;
            sqlx::query("UPDATE users SET password_hash = $1 WHERE id = $2").bind(hash).bind(id).execute(pool).await?;
            sqlx::query("DELETE FROM sessions WHERE user_id = $1").bind(id).execute(pool).await?;
            println!(
                "[auth] Passwort des Inhaber-Kontos wurde zurückgesetzt. \
                 ADMIN_PASSWORD_RESET jetzt wieder entfernen."
            );
        }
        return Ok(());
    }

    let (password, generated) = match env_password {
        Some(p) => (p, false),
        None => (auth::random_password(), true),
    };
    let hash = auth::hash_password(&password).await.map_err(|e| e.to_string())?;
    sqlx::query("INSERT INTO users (email, name, password_hash, role_id) VALUES ($1, $2, $3, $4)")
        .bind(OWNER_EMAIL)
        .bind("Hatem Ben Belgacem")
        .bind(hash)
        .bind(ADMIN_ROLE_ID)
        .execute(pool)
        .await?;

    if generated {
        println!(
            "[auth] Admin-Konto {OWNER_EMAIL} angelegt. Startpasswort: {password}\n\
             [auth] Bitte nach der ersten Anmeldung unter /admin/konto ändern \
             (oder vor dem ersten Start ADMIN_PASSWORD setzen)."
        );
    } else {
        println!("[auth] Admin-Konto {OWNER_EMAIL} mit ADMIN_PASSWORD angelegt.");
    }
    Ok(())
}

type PhotoRow = (i64, String, String, String, Option<String>, Option<i64>, Option<i64>);
type VideoRow = (i64, String, String, String, String, String);

pub async fn load_photos() -> Result<Vec<PhotoItem>, ServerFnError> {
    let rows: Vec<PhotoRow> = sqlx::query_as(
        "SELECT id, category, title, image_url, thumb_url, width, height FROM photos ORDER BY sort_order, id",
    )
    .fetch_all(pool())
    .await
    .map_err(db_err)?;

    Ok(rows
        .into_iter()
        .filter_map(|(id, category, title, image_url, thumb_url, width, height)| {
            Some(PhotoItem {
                id,
                category: Category::from_slug(&category)?,
                title,
                image_url,
                thumb_url,
                width: width.map(|w| w as u32),
                height: height.map(|h| h as u32),
            })
        })
        .collect())
}

pub async fn load_videos() -> Result<Vec<VideoItem>, ServerFnError> {
    let rows: Vec<VideoRow> = sqlx::query_as(
        "SELECT id, category, title, client, duration, youtube_id FROM videos ORDER BY sort_order, id",
    )
    .fetch_all(pool())
    .await
    .map_err(db_err)?;

    Ok(rows
        .into_iter()
        .filter_map(|(id, category, title, client, duration, youtube_id)| {
            Some(VideoItem {
                id,
                category: Category::from_slug(&category)?,
                title,
                client,
                duration,
                youtube_id,
            })
        })
        .collect())
}

/// Element innerhalb seines Bereichs um eine Position verschieben (`table` ist `photos` oder `videos`)
pub async fn move_item(table: &str, id: i64, direction: i32) -> Result<(), ServerFnError> {
    debug_assert!(table == "photos" || table == "videos");
    let pool = pool();
    let category: Option<(String,)> = sqlx::query_as(&format!("SELECT category FROM {table} WHERE id = $1"))
        .bind(id)
        .fetch_optional(pool)
        .await
        .map_err(db_err)?;
    let Some((category,)) = category else {
        return Err(super::user_err("Eintrag nicht gefunden."));
    };

    let mut ids: Vec<(i64,)> = sqlx::query_as(&format!(
        "SELECT id FROM {table} WHERE category = $1 ORDER BY sort_order, id"
    ))
    .bind(&category)
    .fetch_all(pool)
    .await
    .map_err(db_err)?;

    let Some(pos) = ids.iter().position(|(i,)| *i == id) else { return Ok(()) };
    let target = pos as i64 + direction.signum() as i64;
    if target < 0 || target >= ids.len() as i64 {
        return Ok(());
    }
    ids.swap(pos, target as usize);

    let mut tx = pool.begin().await.map_err(db_err)?;
    for (order, (item_id,)) in ids.iter().enumerate() {
        sqlx::query(&format!("UPDATE {table} SET sort_order = $1 WHERE id = $2"))
            .bind(order as i64)
            .bind(item_id)
            .execute(&mut *tx)
            .await
            .map_err(db_err)?;
    }
    tx.commit().await.map_err(db_err)?;
    Ok(())
}

/// Nächste freie Position am Ende eines Bereichs
pub async fn next_sort_order(table: &str, category: &str) -> Result<i64, ServerFnError> {
    let (max,): (Option<i64>,) = sqlx::query_as(&format!("SELECT MAX(sort_order) FROM {table} WHERE category = $1"))
        .bind(category)
        .fetch_one(pool())
        .await
        .map_err(db_err)?;
    Ok(max.map(|m| m + 1).unwrap_or(0))
}
