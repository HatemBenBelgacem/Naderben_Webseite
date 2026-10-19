-- Rollen mit einzelnen Berechtigungen
CREATE TABLE roles (
    id                INTEGER PRIMARY KEY AUTOINCREMENT,
    name              TEXT    NOT NULL UNIQUE COLLATE NOCASE,
    description       TEXT    NOT NULL DEFAULT '',
    can_edit_content  INTEGER NOT NULL DEFAULT 0,
    can_manage_users  INTEGER NOT NULL DEFAULT 0,
    can_manage_roles  INTEGER NOT NULL DEFAULT 0,
    -- System-Rollen können nicht gelöscht werden
    is_system         INTEGER NOT NULL DEFAULT 0
);

INSERT INTO roles (id, name, description, can_edit_content, can_manage_users, can_manage_roles, is_system) VALUES
    (1, 'Admin',    'Voller Zugriff inkl. Benutzer- und Rollenverwaltung', 1, 1, 1, 1),
    (2, 'Redaktor', 'Darf Texte, Bilder und das Portfolio bearbeiten',     1, 0, 0, 1),
    (3, 'Benutzer', 'Angemeldet, ohne Verwaltungsrechte',                  0, 0, 0, 1);

CREATE TABLE users (
    id             INTEGER PRIMARY KEY AUTOINCREMENT,
    email          TEXT    NOT NULL UNIQUE COLLATE NOCASE,
    name           TEXT    NOT NULL,
    password_hash  TEXT    NOT NULL,
    role_id        INTEGER NOT NULL REFERENCES roles(id),
    active         INTEGER NOT NULL DEFAULT 1,
    created_at     TEXT    NOT NULL DEFAULT (datetime('now'))
);

-- Sessions: gespeichert wird nur der SHA-256-Hash des Cookie-Tokens
CREATE TABLE sessions (
    token_hash  TEXT    PRIMARY KEY,
    user_id     INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    expires_at  INTEGER NOT NULL
);
CREATE INDEX sessions_user ON sessions(user_id);

-- Überschriebene Texte/Bild-URLs der Seiten (Standardwerte stehen im Code)
CREATE TABLE content (
    key         TEXT PRIMARY KEY,
    value       TEXT NOT NULL,
    updated_at  TEXT NOT NULL DEFAULT (datetime('now')),
    updated_by  INTEGER REFERENCES users(id) ON DELETE SET NULL
);

-- Hochgeladene Bilder
CREATE TABLE media (
    id            INTEGER PRIMARY KEY AUTOINCREMENT,
    filename      TEXT    NOT NULL,
    content_type  TEXT    NOT NULL,
    data          BLOB    NOT NULL,
    width         INTEGER,
    height        INTEGER,
    created_at    TEXT    NOT NULL DEFAULT (datetime('now'))
);

-- Portfolio
CREATE TABLE photos (
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    category    TEXT    NOT NULL,
    title       TEXT    NOT NULL,
    image_url   TEXT    NOT NULL,
    thumb_url   TEXT,
    width       INTEGER,
    height      INTEGER,
    sort_order  INTEGER NOT NULL DEFAULT 0
);
CREATE INDEX photos_category ON photos(category, sort_order);

CREATE TABLE videos (
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    category    TEXT    NOT NULL,
    title       TEXT    NOT NULL,
    client      TEXT    NOT NULL DEFAULT '',
    duration    TEXT    NOT NULL DEFAULT '',
    youtube_id  TEXT    NOT NULL,
    sort_order  INTEGER NOT NULL DEFAULT 0
);
CREATE INDEX videos_category ON videos(category, sort_order);

-- Interne Zustände, z. B. ob die Startdaten schon eingespielt wurden
CREATE TABLE app_meta (
    key    TEXT PRIMARY KEY,
    value  TEXT NOT NULL
);
