-- Online-Shop: Produkte, Warenkörbe und Bestellungen

-- Produkte. Preise in Rappen (ganze Zahlen), damit es keine Rundungsfehler gibt.
CREATE TABLE products (
    id           BIGSERIAL PRIMARY KEY,
    name         TEXT   NOT NULL,
    description  TEXT   NOT NULL DEFAULT '',
    price_cents  BIGINT NOT NULL CHECK (price_cents >= 0),
    stock        BIGINT NOT NULL DEFAULT 0 CHECK (stock >= 0),
    created_at   TEXT   NOT NULL DEFAULT to_char(now() AT TIME ZONE 'UTC', 'YYYY-MM-DD HH24:MI:SS')
);

-- Warenkörbe. Im Cookie des Besuchers steht das Token, hier nur dessen SHA-256-Hash.
CREATE TABLE carts (
    id          BIGSERIAL PRIMARY KEY,
    token_hash  TEXT NOT NULL UNIQUE,
    created_at  TEXT NOT NULL DEFAULT to_char(now() AT TIME ZONE 'UTC', 'YYYY-MM-DD HH24:MI:SS')
);

-- Verbindung Warenkorb <-> Produkt (n:m).
-- Ein Produkt kann pro Warenkorb nur einmal vorkommen; die Menge steht in quantity.
CREATE TABLE cart_items (
    cart_id     BIGINT NOT NULL REFERENCES carts(id) ON DELETE CASCADE,
    product_id  BIGINT NOT NULL REFERENCES products(id) ON DELETE CASCADE,
    quantity    BIGINT NOT NULL DEFAULT 1 CHECK (quantity > 0),
    PRIMARY KEY (cart_id, product_id)
);
CREATE INDEX cart_items_product ON cart_items(product_id);

-- Bestellungen
CREATE TABLE orders (
    id              BIGSERIAL PRIMARY KEY,
    customer_name   TEXT   NOT NULL,
    customer_email  TEXT   NOT NULL,
    status          TEXT   NOT NULL DEFAULT 'offen'
                    CHECK (status IN ('offen', 'bezahlt', 'versendet', 'storniert')),
    total_cents     BIGINT NOT NULL CHECK (total_cents >= 0),
    created_at      TEXT   NOT NULL DEFAULT to_char(now() AT TIME ZONE 'UTC', 'YYYY-MM-DD HH24:MI:SS')
);

-- Positionen einer Bestellung.
-- product_id darf leer werden (ON DELETE SET NULL), damit alte Bestellungen erhalten bleiben.
-- Name und Preis werden kopiert: Sie gelten für die Bestellung, auch wenn sich das Produkt ändert.
CREATE TABLE order_items (
    id               BIGSERIAL PRIMARY KEY,
    order_id         BIGINT NOT NULL REFERENCES orders(id) ON DELETE CASCADE,
    product_id       BIGINT REFERENCES products(id) ON DELETE SET NULL,
    product_name     TEXT   NOT NULL,
    unit_price_cents BIGINT NOT NULL CHECK (unit_price_cents >= 0),
    quantity         BIGINT NOT NULL CHECK (quantity > 0)
);
CREATE INDEX order_items_order ON order_items(order_id);
CREATE INDEX order_items_product ON order_items(product_id);
