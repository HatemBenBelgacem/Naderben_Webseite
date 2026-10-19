//! Passwörter (Argon2), Sessions per HttpOnly-Cookie und Rechteprüfung.

use std::time::{Duration, SystemTime, UNIX_EPOCH};

use argon2::{
    password_hash::{rand_core::{OsRng, RngCore}, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use axum::http::{header, HeaderMap, HeaderValue};
use leptos::prelude::*;
use sha2::{Digest, Sha256};

use super::{db::pool, db_err, user_err};
use crate::models::{SessionUser, MIN_PASSWORD_LEN, OWNER_EMAIL};

pub const SESSION_COOKIE: &str = "nb_session";
const SESSION_DAYS: i64 = 30;

pub fn now() -> i64 {
    SystemTime::now().duration_since(UNIX_EPOCH).map(|d| d.as_secs() as i64).unwrap_or(0)
}

// ───────────── Passwörter ─────────────

/// Argon2 ist absichtlich rechenintensiv – daher im Blocking-Thread
pub async fn hash_password(password: &str) -> Result<String, ServerFnError> {
    let password = password.to_owned();
    tokio::task::spawn_blocking(move || {
        let salt = SaltString::generate(&mut OsRng);
        Argon2::default()
            .hash_password(password.as_bytes(), &salt)
            .map(|h| h.to_string())
            .map_err(|_| user_err("Passwort konnte nicht gespeichert werden."))
    })
    .await
    .map_err(|_| user_err("Interner Fehler."))?
}

pub async fn verify_password(password: &str, hash: &str) -> bool {
    let (password, hash) = (password.to_owned(), hash.to_owned());
    tokio::task::spawn_blocking(move || {
        PasswordHash::new(&hash)
            .map(|parsed| Argon2::default().verify_password(password.as_bytes(), &parsed).is_ok())
            .unwrap_or(false)
    })
    .await
    .unwrap_or(false)
}

pub fn validate_password(password: &str) -> Result<(), ServerFnError> {
    if password.chars().count() < MIN_PASSWORD_LEN {
        return Err(user_err(format!("Das Passwort muss mindestens {MIN_PASSWORD_LEN} Zeichen lang sein.")));
    }
    if password.len() > 200 {
        return Err(user_err("Das Passwort ist zu lang."));
    }
    Ok(())
}

pub fn random_password() -> String {
    const CHARS: &[u8] = b"ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnpqrstuvwxyz23456789";
    let mut bytes = [0u8; 16];
    OsRng.fill_bytes(&mut bytes);
    bytes.iter().map(|b| CHARS[*b as usize % CHARS.len()] as char).collect()
}

// ───────────── Sessions ─────────────

fn new_token() -> String {
    let mut bytes = [0u8; 32];
    OsRng.fill_bytes(&mut bytes);
    hex::encode(bytes)
}

fn hash_token(token: &str) -> String {
    hex::encode(Sha256::digest(token.as_bytes()))
}

/// Im Produktivbetrieb (HTTPS auf Railway) nur über sichere Verbindungen senden
fn cookie_secure() -> bool {
    std::env::var("LEPTOS_ENV").map(|e| e.eq_ignore_ascii_case("PROD")).unwrap_or(false)
}

fn cookie(value: &str, max_age: i64) -> String {
    let secure = if cookie_secure() { "; Secure" } else { "" };
    format!("{SESSION_COOKIE}={value}; Path=/; HttpOnly; SameSite=Lax; Max-Age={max_age}{secure}")
}

fn set_cookie(value: String) {
    if let (Some(resp), Ok(v)) = (use_context::<leptos_axum::ResponseOptions>(), HeaderValue::from_str(&value)) {
        resp.append_header(header::SET_COOKIE, v);
    }
}

/// Session-Token aus dem Cookie der aktuellen Anfrage
pub async fn request_token() -> Option<String> {
    let headers: HeaderMap = leptos_axum::extract().await.ok()?;
    headers
        .get_all(header::COOKIE)
        .iter()
        .filter_map(|v| v.to_str().ok())
        .flat_map(|v| v.split(';'))
        .filter_map(|pair| pair.trim().split_once('='))
        .find(|(name, _)| *name == SESSION_COOKIE)
        .map(|(_, value)| value.to_string())
        .filter(|v| v.len() == 64)
}

pub async fn start_session(user_id: i64) -> Result<(), ServerFnError> {
    let token = new_token();
    let max_age = SESSION_DAYS * 24 * 3600;
    sqlx::query("INSERT INTO sessions (token_hash, user_id, expires_at) VALUES (?, ?, ?)")
        .bind(hash_token(&token))
        .bind(user_id)
        .bind(now() + max_age)
        .execute(pool())
        .await
        .map_err(db_err)?;
    set_cookie(cookie(&token, max_age));
    Ok(())
}

pub async fn end_session() -> Result<(), ServerFnError> {
    if let Some(token) = request_token().await {
        sqlx::query("DELETE FROM sessions WHERE token_hash = ?")
            .bind(hash_token(&token))
            .execute(pool())
            .await
            .map_err(db_err)?;
    }
    set_cookie(cookie("", 0));
    Ok(())
}

/// Alle Sessions eines Benutzers beenden – optional ausser der aktuellen
pub async fn end_sessions_of(user_id: i64, keep_current: bool) -> Result<(), ServerFnError> {
    let keep = if keep_current { request_token().await.map(|t| hash_token(&t)) } else { None };
    sqlx::query("DELETE FROM sessions WHERE user_id = ? AND token_hash != ?")
        .bind(user_id)
        .bind(keep.unwrap_or_default())
        .execute(pool())
        .await
        .map_err(db_err)?;
    Ok(())
}

type UserRow = (i64, String, String, String, bool, bool, bool);

fn to_session_user((id, name, email, role, edit, users, roles): UserRow) -> SessionUser {
    SessionUser { id, name, email, role, can_edit_content: edit, can_manage_users: users, can_manage_roles: roles }
}

const USER_SELECT: &str = "SELECT u.id, u.name, u.email, r.name, r.can_edit_content, r.can_manage_users, r.can_manage_roles
     FROM users u JOIN roles r ON r.id = u.role_id";

pub async fn user_by_id(id: i64) -> Result<Option<SessionUser>, ServerFnError> {
    let row: Option<UserRow> = sqlx::query_as(&format!("{USER_SELECT} WHERE u.id = ? AND u.active = 1"))
        .bind(id)
        .fetch_optional(pool())
        .await
        .map_err(db_err)?;
    Ok(row.map(to_session_user))
}

/// Angemeldeter Benutzer der aktuellen Anfrage (oder `None`)
pub async fn current_user() -> Result<Option<SessionUser>, ServerFnError> {
    let Some(token) = request_token().await else { return Ok(None) };
    let row: Option<UserRow> = sqlx::query_as(&format!(
        "{USER_SELECT} JOIN sessions s ON s.user_id = u.id
         WHERE s.token_hash = ? AND s.expires_at > ? AND u.active = 1"
    ))
    .bind(hash_token(&token))
    .bind(now())
    .fetch_optional(pool())
    .await
    .map_err(db_err)?;
    Ok(row.map(to_session_user))
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Perm {
    LoggedIn,
    EditContent,
    ManageUsers,
    ManageRoles,
    /// Benutzer- oder Rollenverwaltung (z. B. um die Rollenliste zu sehen)
    UsersOrRoles,
}

/// Bricht mit Fehler ab, wenn niemand angemeldet ist oder das Recht fehlt
pub async fn require(perm: Perm) -> Result<SessionUser, ServerFnError> {
    let user = current_user().await?.ok_or_else(|| user_err("Bitte zuerst anmelden."))?;
    let allowed = match perm {
        Perm::LoggedIn => true,
        Perm::EditContent => user.can_edit_content,
        Perm::ManageUsers => user.can_manage_users,
        Perm::ManageRoles => user.can_manage_roles,
        Perm::UsersOrRoles => user.can_manage_users || user.can_manage_roles,
    };
    if allowed {
        Ok(user)
    } else {
        Err(user_err("Dafür fehlt dir die Berechtigung."))
    }
}

pub fn is_owner(email: &str) -> bool {
    email.eq_ignore_ascii_case(OWNER_EMAIL)
}

/// Gegen Durchprobieren von Passwörtern: fehlgeschlagene Anmeldungen bremsen
pub async fn failed_login_delay() {
    tokio::time::sleep(Duration::from_millis(600)).await;
}
