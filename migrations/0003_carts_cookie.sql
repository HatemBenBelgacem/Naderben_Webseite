-- Warenkörbe über ein Cookie zuordnen.
-- Der Browser speichert ein zufälliges Token. In der DB steht nur dessen SHA-256-Hash
-- (wie bei den Sitzungen). Die Tabelle ist neu und wird noch von keinem Code beschrieben,
-- deshalb wird sie hier einfach neu aufgebaut. Zuerst die Positionen, dann der Korb.
DELETE FROM cart_items;
DROP TABLE carts;

CREATE TABLE carts (
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    token_hash  TEXT    NOT NULL UNIQUE,
    created_at  TEXT    NOT NULL DEFAULT (datetime('now'))
);
