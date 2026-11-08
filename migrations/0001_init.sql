-- Rollen mit einzelnen Berechtigungen
CREATE TABLE roles (
    id                BIGSERIAL PRIMARY KEY,
    name              TEXT    NOT NULL,
    description       TEXT    NOT NULL DEFAULT '',
    can_edit_content  BOOLEAN NOT NULL DEFAULT FALSE,
    can_manage_users  BOOLEAN NOT NULL DEFAULT FALSE,
    can_manage_roles  BOOLEAN NOT NULL DEFAULT FALSE,
    -- System-Rollen können nicht gelöscht werden
    is_system         BOOLEAN NOT NULL DEFAULT FALSE
);
-- Rollennamen sind ohne Groß-/Kleinschreibung eindeutig
CREATE UNIQUE INDEX roles_name_key ON roles (lower(name));

INSERT INTO roles (id, name, description, can_edit_content, can_manage_users, can_manage_roles, is_system) VALUES
    (1, 'Admin',    'Voller Zugriff inkl. Benutzer- und Rollenverwaltung', TRUE,  TRUE,  TRUE,  TRUE),
    (2, 'Redaktor', 'Darf Texte, Bilder und das Portfolio bearbeiten',     TRUE,  FALSE, FALSE, TRUE),
    (3, 'Benutzer', 'Angemeldet, ohne Verwaltungsrechte',                  FALSE, FALSE, FALSE, TRUE);
-- Die ID-Zähler hinter den festen IDs 1–3 weiterzählen lassen
SELECT setval(pg_get_serial_sequence('roles', 'id'), (SELECT MAX(id) FROM roles));

CREATE TABLE users (
    id             BIGSERIAL PRIMARY KEY,
    -- E-Mails werden immer klein geschrieben gespeichert
    email          TEXT    NOT NULL UNIQUE,
    name           TEXT    NOT NULL,
    password_hash  TEXT    NOT NULL,
    role_id        BIGINT  NOT NULL REFERENCES roles(id),
    active         BOOLEAN NOT NULL DEFAULT TRUE,
    created_at     TEXT    NOT NULL DEFAULT to_char(now() AT TIME ZONE 'UTC', 'YYYY-MM-DD HH24:MI:SS')
);

-- Sessions: gespeichert wird nur der SHA-256-Hash des Cookie-Tokens
CREATE TABLE sessions (
    token_hash  TEXT   PRIMARY KEY,
    user_id     BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    expires_at  BIGINT NOT NULL
);
CREATE INDEX sessions_user ON sessions(user_id);

-- Überschriebene Texte/Bild-URLs der Seiten (Standardwerte stehen im Code)
CREATE TABLE content (
    key         TEXT PRIMARY KEY,
    value       TEXT NOT NULL,
    updated_at  TEXT NOT NULL DEFAULT to_char(now() AT TIME ZONE 'UTC', 'YYYY-MM-DD HH24:MI:SS'),
    updated_by  BIGINT REFERENCES users(id) ON DELETE SET NULL
);

-- Hochgeladene Bilder
CREATE TABLE media (
    id            BIGSERIAL PRIMARY KEY,
    filename      TEXT   NOT NULL,
    content_type  TEXT   NOT NULL,
    data          BYTEA  NOT NULL,
    width         BIGINT,
    height        BIGINT,
    created_at    TEXT   NOT NULL DEFAULT to_char(now() AT TIME ZONE 'UTC', 'YYYY-MM-DD HH24:MI:SS')
);

-- Portfolio
CREATE TABLE photos (
    id          BIGSERIAL PRIMARY KEY,
    category    TEXT   NOT NULL,
    title       TEXT   NOT NULL,
    image_url   TEXT   NOT NULL,
    thumb_url   TEXT,
    width       BIGINT,
    height      BIGINT,
    sort_order  BIGINT NOT NULL DEFAULT 0
);
CREATE INDEX photos_category ON photos(category, sort_order);

CREATE TABLE videos (
    id          BIGSERIAL PRIMARY KEY,
    category    TEXT   NOT NULL,
    title       TEXT   NOT NULL,
    client      TEXT   NOT NULL DEFAULT '',
    duration    TEXT   NOT NULL DEFAULT '',
    youtube_id  TEXT   NOT NULL,
    sort_order  BIGINT NOT NULL DEFAULT 0
);
CREATE INDEX videos_category ON videos(category, sort_order);

-- Interne Zustände, z. B. ob die Startdaten schon eingespielt wurden
CREATE TABLE app_meta (
    key    TEXT PRIMARY KEY,
    value  TEXT NOT NULL
);
