//! Typen für Benutzer, Rollen und Verwaltung, die Server und Browser teilen.

use serde::{Deserialize, Serialize};

/// Wer sich mit dieser E-Mail anmeldet, ist immer Admin.
/// Das Konto kann weder gelöscht, deaktiviert noch herabgestuft werden.
pub const OWNER_EMAIL: &str = "hatem.benbelgacem@outlook.com";

pub const MIN_PASSWORD_LEN: usize = 10;

/// Der angemeldete Benutzer mit den Rechten seiner Rolle
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SessionUser {
    pub id: i64,
    pub name: String,
    pub email: String,
    pub role: String,
    pub can_edit_content: bool,
    pub can_manage_users: bool,
    pub can_manage_roles: bool,
}

impl SessionUser {
    /// Darf irgendeinen Bereich der Verwaltung sehen
    pub fn has_admin_access(&self) -> bool {
        self.can_edit_content || self.can_manage_users || self.can_manage_roles
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RoleRow {
    pub id: i64,
    pub name: String,
    pub description: String,
    pub can_edit_content: bool,
    pub can_manage_users: bool,
    pub can_manage_roles: bool,
    pub is_system: bool,
    pub user_count: i64,
}

impl RoleRow {
    /// Die Admin-Rolle hat immer alle Rechte
    pub fn is_admin(&self) -> bool {
        self.id == ADMIN_ROLE_ID
    }
}

pub const ADMIN_ROLE_ID: i64 = 1;
/// Standardrolle für neue Benutzer
pub const USER_ROLE_ID: i64 = 3;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserRow {
    pub id: i64,
    pub email: String,
    pub name: String,
    pub role_id: i64,
    pub role_name: String,
    pub active: bool,
    pub created_at: String,
}

impl UserRow {
    pub fn is_owner(&self) -> bool {
        self.email.eq_ignore_ascii_case(OWNER_EMAIL)
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContentEntry {
    pub key: String,
    pub value: String,
}

/// Ergebnis eines Bild-Uploads
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MediaInfo {
    pub url: String,
    pub width: Option<u32>,
    pub height: Option<u32>,
}
