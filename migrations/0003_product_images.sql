-- Bilder je Produkt: Liste von Bild-Adressen in der gewünschten Reihenfolge.
-- Uploads stehen als /media/{id}, externe Bilder als https-Link.
ALTER TABLE products ADD COLUMN image_urls TEXT[] NOT NULL DEFAULT '{}';
