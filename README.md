# Naderben_Webseite

Portfolio-Website für Foto & Video – Fullstack in Rust mit [Leptos 0.8](https://leptos.dev) (SSR + Hydration), Axum und SQLite.

## Seiten

| Route                               | Inhalt                                                    |
| ----------------------------------- | --------------------------------------------------------- |
| `/`                                 | Landing mit Hero, ausgewählten Arbeiten, Showreel         |
| `/about`                            | Über mich, Leistungen, Ablauf, Testimonial                |
| `/portfolio`                        | Die vier Bereiche Künstler, Objekte, Hochzeiten, Firmenportrait |
| `/portfolio/{bereich}/videos`       | Videos eines Bereichs mit Player-Overlay                  |
| `/portfolio/{bereich}/fotos`        | Fotos eines Bereichs mit Lightbox (← → Esc)               |
| `/contact`                          | Kontaktformular (Leptos Server-Funktion)                  |
| `/admin`                            | Verwaltung (nur nach Anmeldung, siehe unten)              |

## Verwaltung

Die **Anmeldung** befindet sich in der Fusszeile jeder Seite. Danach erscheint dort der Link zur Verwaltung:

| Bereich            | Recht                       | Inhalt                                               |
| ------------------ | --------------------------- | ---------------------------------------------------- |
| `/admin/inhalte`   | Inhalte bearbeiten          | Alle Texte und Bilder pro Seite (Upload oder URL)    |
| `/admin/portfolio` | Inhalte bearbeiten          | Fotos/Videos je Bereich hinzufügen, ordnen, löschen  |
| `/admin/benutzer`  | Benutzer verwalten          | Konten anlegen, Rolle zuweisen, sperren, Passwort    |
| `/admin/rollen`    | Rollen verwalten            | Rollen und ihre Berechtigungen                       |
| `/admin/konto`     | jeder angemeldete Benutzer  | Eigenes Passwort ändern                              |

- **Rollen:** *Admin* (alles), *Redaktor* (Inhalte & Portfolio), *Benutzer* (keine Verwaltungsrechte). Weitere Rollen lassen sich anlegen. Niemand kann Rechte vergeben, die er selbst nicht hat.
- **Inhaber:** `hatem.benbelgacem@outlook.com` ist immer Admin – das Konto kann nicht gelöscht, gesperrt oder herabgestuft werden.
- **Erstes Passwort:** Beim ersten Start wird das Inhaber-Konto angelegt. Ist `ADMIN_PASSWORD` gesetzt, gilt dieses Passwort, sonst wird ein zufälliges erzeugt und einmalig ins Server-Log geschrieben. Nach der ersten Anmeldung unter *Mein Konto* ändern.
- Es gibt **keine öffentliche Registrierung** – neue Konten legt ein Admin an.
- Passwörter werden mit Argon2 gespeichert, Sessions als HttpOnly-Cookie (30 Tage, in der DB nur als SHA-256-Hash).

## Daten

Alles liegt in einer SQLite-Datei (`DATABASE_URL`, Standard `sqlite://data/naderben.db`):
Benutzer, Rollen, Sessions, geänderte Texte, Portfolio-Einträge und hochgeladene Bilder (als BLOB, ausgeliefert unter `/media/{id}`, max. 15 MB, JPG/PNG/WebP/GIF).

- Schema: `migrations/` – wird beim Start automatisch angewendet.
- Standardtexte und Bildfelder: `src/content.rs`. In der DB stehen nur Abweichungen; „Standard“ im Editor stellt den Originaltext wieder her.
- Beim allerersten Start werden Platzhalter-Galerien aus `src/data.rs` eingespielt.

## Entwicklung

```bash
rustup target add wasm32-unknown-unknown
cargo install cargo-leptos --locked
cargo leptos watch
```

Dann <http://127.0.0.1:3080> öffnen. Die lokale Datenbank liegt in `data/` (nicht im Git). Das Startpasswort steht im Terminal-Log, oder vorher setzen:

```bash
ADMIN_PASSWORD='ein-sicheres-passwort' cargo leptos watch
```

## Design

Tailwind CSS v4 – Utility-Klassen direkt in den `view!`-Makros, Design-Tokens und Bausteine wie `btn`, `eyebrow`, `wrap` in `style/tailwind.css`. cargo-leptos lädt Tailwind automatisch, Node/npm ist nicht nötig.

Kontaktanfragen werden validiert und ins Server-Log geschrieben (`src/pages/contact.rs`). Für E-Mail-Versand dort z. B. `lettre` (SMTP) oder einen Webhook anschliessen.

## Deployment auf Railway

1. Repo auf GitHub pushen und in Railway als neuen Service verbinden – Railway baut mit dem `Dockerfile` (`railway.json`).
2. **Volume anlegen** (*Service → Settings → Volumes*) mit Mount-Pfad **`/app/data`**. Ohne Volume gehen Benutzer, Texte und Bilder bei jedem Deploy verloren.
3. Variable **`ADMIN_PASSWORD`** setzen (nur für den allerersten Start relevant).
4. Der Server bindet automatisch an `0.0.0.0:$PORT`.
5. Unter *Settings → Networking* eine Domain generieren bzw. `naderben.com` verbinden.

Lokal testen:

```bash
docker build -t naderben .
docker run --rm -p 8080:8080 -v naderben-data:/app/data -e ADMIN_PASSWORD='ein-sicheres-passwort' naderben
```

> Hinweis: `.cargo/config.toml` setzt `--cfg erase_components` für alle Targets (Server + WASM). Ohne das Flag scheitert der macOS-Linker an den langen Leptos-Typnamen; es muss auf beiden Seiten gleich sein, sonst schlägt die Hydration fehl.
