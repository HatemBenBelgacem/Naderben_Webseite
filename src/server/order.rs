//! Bestellung aufgeben. Lager, Preise und Warenkorb werden in einer Transaktion
//! geändert. Schlägt ein Schritt fehl, wird alles zurückgerollt.

use leptos::prelude::ServerFnError;

use super::{cart, db::pool, db_err, user_err};
use crate::models::OrderConfirmation;

/// Warenkorb des Besuchers in eine Bestellung umwandeln
pub async fn place(customer_name: &str, customer_email: &str) -> Result<OrderConfirmation, ServerFnError> {
    let cart_id = cart::cart_id().await?.ok_or_else(|| user_err("Dein Warenkorb ist leer."))?;
    let mut tx = pool().begin().await.map_err(db_err)?;

    // Positionen mit den aktuellen Preisen aus der Produkttabelle
    let rows: Vec<(i64, String, i64, i64)> = sqlx::query_as(
        "SELECT p.id, p.name, p.price_cents, ci.quantity
         FROM cart_items ci JOIN products p ON p.id = ci.product_id
         WHERE ci.cart_id = $1 ORDER BY p.id",
    )
    .bind(cart_id)
    .fetch_all(&mut *tx)
    .await
    .map_err(db_err)?;
    if rows.is_empty() {
        return Err(user_err("Dein Warenkorb ist leer."));
    }

    // Lager abziehen. Nur wenn genug da ist, sonst bricht die ganze Bestellung ab.
    let mut total: i64 = 0;
    for (product_id, name, price_cents, quantity) in &rows {
        let done = sqlx::query("UPDATE products SET stock = stock - $1 WHERE id = $2 AND stock >= $3")
            .bind(*quantity)
            .bind(*product_id)
            .bind(*quantity)
            .execute(&mut *tx)
            .await
            .map_err(db_err)?;
        if done.rows_affected() == 0 {
            return Err(user_err(format!(
                "Von «{name}» sind nicht mehr genug auf Lager. Bitte passe deinen Warenkorb an."
            )));
        }
        total += price_cents * quantity;
    }

    let (order_id,): (i64,) =
        sqlx::query_as("INSERT INTO orders (customer_name, customer_email, total_cents) VALUES ($1, $2, $3) RETURNING id")
            .bind(customer_name)
            .bind(customer_email)
            .bind(total)
            .fetch_one(&mut *tx)
            .await
            .map_err(db_err)?;

    // Name und Preis werden kopiert, damit die Bestellung auch nach Änderungen stimmt
    for (product_id, name, price_cents, quantity) in &rows {
        sqlx::query(
            "INSERT INTO order_items (order_id, product_id, product_name, unit_price_cents, quantity)
             VALUES ($1, $2, $3, $4, $5)",
        )
        .bind(order_id)
        .bind(*product_id)
        .bind(name.as_str())
        .bind(*price_cents)
        .bind(*quantity)
        .execute(&mut *tx)
        .await
        .map_err(db_err)?;
    }

    sqlx::query("DELETE FROM cart_items WHERE cart_id = $1")
        .bind(cart_id)
        .execute(&mut *tx)
        .await
        .map_err(db_err)?;
    tx.commit().await.map_err(db_err)?;

    Ok(OrderConfirmation {
        id: order_id,
        total_cents: total,
        item_count: rows.iter().map(|(_, _, _, quantity)| quantity).sum(),
    })
}
