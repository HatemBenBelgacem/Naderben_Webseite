//! Warenkorb des Besuchers. Zuordnung über das Cookie `nb_cart`:
//! Im Cookie steht ein zufälliges Token, in der Tabelle `carts` nur dessen Hash.

use leptos::prelude::ServerFnError;

use super::{auth, db::pool, db_err, user_err};
use crate::models::{Cart, CartLine};

pub const CART_COOKIE: &str = "nb_cart";
const CART_DAYS: i64 = 30;
/// Höchstmenge pro Produkt im Warenkorb
pub const MAX_QUANTITY: i64 = 99;

/// Warenkorb-ID aus dem Cookie. Ohne Cookie (oder bei unbekanntem Token) gibt es keinen Korb.
pub async fn cart_id() -> Result<Option<i64>, ServerFnError> {
    let Some(token) = auth::request_cookie(CART_COOKIE).await else { return Ok(None) };
    let row: Option<(i64,)> = sqlx::query_as("SELECT id FROM carts WHERE token_hash = ?")
        .bind(auth::hash_token(&token))
        .fetch_optional(pool())
        .await
        .map_err(db_err)?;
    Ok(row.map(|(id,)| id))
}

/// Warenkorb des Besuchers. Fehlt er, wird ein neuer angelegt und das Cookie gesetzt.
pub async fn ensure_cart_id() -> Result<i64, ServerFnError> {
    if let Some(id) = cart_id().await? {
        return Ok(id);
    }
    let token = auth::new_token();
    let (id,): (i64,) = sqlx::query_as("INSERT INTO carts (token_hash) VALUES (?) RETURNING id")
        .bind(auth::hash_token(&token))
        .fetch_one(pool())
        .await
        .map_err(db_err)?;
    auth::set_cookie(auth::cookie(CART_COOKIE, &token, CART_DAYS * 24 * 3600));
    Ok(id)
}

/// Alle Positionen eines Warenkorbs, mit aktuellem Namen und Preis aus der Produkttabelle
pub async fn load(cart_id: i64) -> Result<Cart, ServerFnError> {
    let rows: Vec<(i64, String, i64, i64, i64)> = sqlx::query_as(
        "SELECT p.id, p.name, p.price_cents, p.stock, ci.quantity
         FROM cart_items ci JOIN products p ON p.id = ci.product_id
         WHERE ci.cart_id = ? ORDER BY ci.rowid",
    )
    .bind(cart_id)
    .fetch_all(pool())
    .await
    .map_err(db_err)?;

    let lines = rows
        .into_iter()
        .map(|(product_id, name, price_cents, stock, quantity)| CartLine { product_id, name, price_cents, stock, quantity })
        .collect();
    Ok(Cart { lines })
}

/// Lagerbestand eines Produkts. `None`, wenn es das Produkt nicht gibt.
async fn stock_of(product_id: i64) -> Result<Option<i64>, ServerFnError> {
    let row: Option<(i64,)> = sqlx::query_as("SELECT stock FROM products WHERE id = ?")
        .bind(product_id)
        .fetch_optional(pool())
        .await
        .map_err(db_err)?;
    Ok(row.map(|(stock,)| stock))
}

/// Menge eines Produkts im Warenkorb (0, wenn es nicht darin liegt)
async fn quantity_in(cart_id: i64, product_id: i64) -> Result<i64, ServerFnError> {
    let row: Option<(i64,)> = sqlx::query_as("SELECT quantity FROM cart_items WHERE cart_id = ? AND product_id = ?")
        .bind(cart_id)
        .bind(product_id)
        .fetch_optional(pool())
        .await
        .map_err(db_err)?;
    Ok(row.map_or(0, |(quantity,)| quantity))
}

/// Menge gegen Höchstmenge und Lagerbestand prüfen
fn check_quantity(quantity: i64, stock: i64) -> Result<(), ServerFnError> {
    if stock <= 0 {
        return Err(user_err("Dieses Produkt ist derzeit nicht auf Lager."));
    }
    if quantity > MAX_QUANTITY {
        return Err(user_err(format!("Höchstens {MAX_QUANTITY} Stück pro Produkt.")));
    }
    if quantity > stock {
        return Err(user_err(format!("Leider sind nur {stock} Stück auf Lager.")));
    }
    Ok(())
}

async fn write_quantity(cart_id: i64, product_id: i64, quantity: i64) -> Result<(), ServerFnError> {
    sqlx::query(
        "INSERT INTO cart_items (cart_id, product_id, quantity) VALUES (?, ?, ?)
         ON CONFLICT(cart_id, product_id) DO UPDATE SET quantity = excluded.quantity",
    )
    .bind(cart_id)
    .bind(product_id)
    .bind(quantity)
    .execute(pool())
    .await
    .map_err(db_err)?;
    Ok(())
}

/// Produkt in den Warenkorb legen oder die Menge um `quantity` erhöhen. Gibt den Korb zurück.
pub async fn add(product_id: i64, quantity: i64) -> Result<Cart, ServerFnError> {
    if quantity < 1 {
        return Err(user_err("Die Menge muss mindestens 1 sein."));
    }
    let stock = stock_of(product_id).await?.ok_or_else(|| user_err("Produkt nicht gefunden."))?;
    let cart = ensure_cart_id().await?;
    let total = quantity_in(cart, product_id).await? + quantity;
    check_quantity(total, stock)?;
    write_quantity(cart, product_id, total).await?;
    load(cart).await
}

/// Menge auf `quantity` setzen. Bei 0 oder weniger wird das Produkt entfernt. Gibt den Korb zurück.
pub async fn set_quantity(product_id: i64, quantity: i64) -> Result<Cart, ServerFnError> {
    if quantity <= 0 {
        return remove(product_id).await;
    }
    let stock = stock_of(product_id).await?.ok_or_else(|| user_err("Produkt nicht gefunden."))?;
    check_quantity(quantity, stock)?;
    let cart = ensure_cart_id().await?;
    write_quantity(cart, product_id, quantity).await?;
    load(cart).await
}

/// Produkt aus dem Warenkorb entfernen. Ohne Warenkorb ist das Ergebnis ein leerer Korb.
pub async fn remove(product_id: i64) -> Result<Cart, ServerFnError> {
    let Some(cart) = cart_id().await? else { return Ok(Cart::default()) };
    sqlx::query("DELETE FROM cart_items WHERE cart_id = ? AND product_id = ?")
        .bind(cart)
        .bind(product_id)
        .execute(pool())
        .await
        .map_err(db_err)?;
    load(cart).await
}
