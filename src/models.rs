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

// ───────────── Shop ─────────────

/// Eine Zeile im Warenkorb. Name und Preis kommen direkt aus dem Produkt.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CartLine {
    pub product_id: i64,
    pub name: String,
    pub price_cents: i64,
    pub stock: i64,
    pub quantity: i64,
}

impl CartLine {
    pub fn line_total_cents(&self) -> i64 {
        self.price_cents * self.quantity
    }
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Cart {
    pub lines: Vec<CartLine>,
}

impl Cart {
    pub fn item_count(&self) -> i64 {
        self.lines.iter().map(|l| l.quantity).sum()
    }

    pub fn total_cents(&self) -> i64 {
        self.lines.iter().map(CartLine::line_total_cents).sum()
    }
}

/// Bestätigung nach einer Bestellung
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OrderConfirmation {
    pub id: i64,
    pub total_cents: i64,
    pub item_count: i64,
}

/// Obergrenzen für Preis und Lagerbestand (Schutz vor Tippfehlern)
pub const MAX_PRICE_CENTS: i64 = 10_000_000;
pub const MAX_STOCK: i64 = 100_000;

/// Ein Produkt im Shop. Der Preis steht in Rappen (ganze Zahl).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Product {
    pub id: i64,
    pub name: String,
    pub description: String,
    pub price_cents: i64,
    pub stock: i64,
}

/// „25.50“ oder „25,5“ → 2550. Höchstens zwei Nachkommastellen, keine Vorzeichen.
pub fn parse_price(input: &str) -> Option<i64> {
    let s = input.trim().replace(',', ".");
    let (whole, frac) = s.split_once('.').unwrap_or((s.as_str(), ""));
    if whole.is_empty() && frac.is_empty() {
        return None;
    }
    if !whole.chars().all(|c| c.is_ascii_digit()) || !frac.chars().all(|c| c.is_ascii_digit()) || frac.len() > 2 {
        return None;
    }
    let whole: i64 = if whole.is_empty() { 0 } else { whole.parse().ok()? };
    let frac: i64 = match frac.len() {
        0 => 0,
        1 => frac.parse::<i64>().ok()? * 10,
        _ => frac.parse().ok()?,
    };
    whole.checked_mul(100)?.checked_add(frac)
}

/// 2550 → „25.50“
pub fn format_price(cents: i64) -> String {
    format!("{}.{:02}", cents / 100, cents % 100)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn preise_werden_umgerechnet() {
        assert_eq!(parse_price("25"), Some(2500));
        assert_eq!(parse_price("25.5"), Some(2550));
        assert_eq!(parse_price("25,50"), Some(2550));
        assert_eq!(parse_price(" 0.05 "), Some(5));
        assert_eq!(parse_price(".5"), Some(50));
        assert_eq!(parse_price("1.234"), None);
        assert_eq!(parse_price("-3"), None);
        assert_eq!(parse_price(""), None);
        assert_eq!(parse_price("."), None);
        assert_eq!(parse_price("abc"), None);
        assert_eq!(format_price(2550), "25.50");
        assert_eq!(format_price(5), "0.05");
    }
}
