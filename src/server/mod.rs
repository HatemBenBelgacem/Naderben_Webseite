//! Nur auf dem Server: Datenbank, Anmeldung, Bild-Speicher.

pub mod auth;
pub mod db;
pub mod media;

use leptos::prelude::ServerFnError;

/// Datenbankfehler loggen, dem Browser nur eine allgemeine Meldung zeigen
pub fn db_err(e: sqlx::Error) -> ServerFnError {
    eprintln!("[db] {e}");
    ServerFnError::new("Datenbankfehler – bitte später erneut versuchen.")
}

pub fn user_err(msg: impl Into<String>) -> ServerFnError {
    ServerFnError::new(msg.into())
}
