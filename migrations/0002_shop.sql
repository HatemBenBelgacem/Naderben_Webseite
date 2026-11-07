-- Online-Shop: Produkte, Warenkörbe und Bestellungen

-- Produkte. Preise in Rappen/Cent (ganze Zahlen), damit es keine Rundungsfehler gibt.
CREATE TABLE products (
    id           INTEGER PRIMARY KEY AUTOINCREMENT,
    name         TEXT    NOT NULL,
    description  TEXT    NOT NULL DEFAULT '',
    price_cents  INTEGER NOT NULL CHECK (price_cents >= 0),
    stock        INTEGER NOT NULL DEFAULT 0 CHECK (stock >= 0),
    created_at   TEXT    NOT NULL DEFAULT (datetime('now'))
);

-- Warenkörbe
CREATE TABLE carts (
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    created_at  TEXT    NOT NULL DEFAULT (datetime('now'))
);

-- Verbindung Warenkorb <-> Produkt (n:m).
-- Ein Produkt kann pro Warenkorb nur einmal vorkommen; die Menge steht in quantity.
CREATE TABLE cart_items (
    cart_id     INTEGER NOT NULL REFERENCES carts(id) ON DELETE CASCADE,
    product_id  INTEGER NOT NULL REFERENCES products(id) ON DELETE CASCADE,
    quantity    INTEGER NOT NULL DEFAULT 1 CHECK (quantity > 0),
    PRIMARY KEY (cart_id, product_id)
);
CREATE INDEX cart_items_product ON cart_items(product_id);

-- Bestellungen
CREATE TABLE orders (
    id              INTEGER PRIMARY KEY AUTOINCREMENT,
    customer_name   TEXT    NOT NULL,
    customer_email  TEXT    NOT NULL COLLATE NOCASE,
    status          TEXT    NOT NULL DEFAULT 'offen'
                    CHECK (status IN ('offen', 'bezahlt', 'versendet', 'storniert')),
    total_cents     INTEGER NOT NULL CHECK (total_cents >= 0),
    created_at      TEXT    NOT NULL DEFAULT (datetime('now'))
);

-- Positionen einer Bestellung.
-- product_id darf leer werden (ON DELETE SET NULL), damit alte Bestellungen erhalten bleiben.
-- Name und Preis werden kopiert: Sie gelten für die Bestellung, auch wenn sich das Produkt ändert.
CREATE TABLE order_items (
    id               INTEGER PRIMARY KEY AUTOINCREMENT,
    order_id         INTEGER NOT NULL REFERENCES orders(id) ON DELETE CASCADE,
    product_id       INTEGER REFERENCES products(id) ON DELETE SET NULL,
    product_name     TEXT    NOT NULL,
    unit_price_cents INTEGER NOT NULL CHECK (unit_price_cents >= 0),
    quantity         INTEGER NOT NULL CHECK (quantity > 0)
);
CREATE INDEX order_items_order ON order_items(order_id);
CREATE INDEX order_items_product ON order_items(product_id);
