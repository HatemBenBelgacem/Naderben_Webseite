# Naderben_Webseite

Portfolio-Website für Foto & Video – Fullstack in Rust mit [Leptos 0.8](https://leptos.dev) (SSR + Hydration), Axum und PostgreSQL.

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
- **Erstes Passwort:** Beim ersten Start wird das Inhaber-Konto angelegt. Ist `ADMIN_PASSWORD` gesetzt, gilt dieses Passwort, sonst wird ein zufälliges erzeugt und einmalig ins Server-Log geschrieben (Zeile `Startpasswort: …`). Nach der ersten Anmeldung unter *Mein Konto* ändern.
- **Passwort vergessen:** Mit `ADMIN_PASSWORD=<neu>` **und** `ADMIN_PASSWORD_RESET=1` starten – das Inhaber-Passwort wird beim Start ersetzt und alle seine Sessions beendet. Danach `ADMIN_PASSWORD_RESET` wieder entfernen (auf Railway: Variable löschen und neu deployen).

  ```bash
  ADMIN_PASSWORD='neues-passwort' ADMIN_PASSWORD_RESET=1 cargo leptos watch
  ```
- Es gibt **keine öffentliche Registrierung** – neue Konten legt ein Admin an.
- Passwörter werden mit Argon2 gespeichert, Sessions als HttpOnly-Cookie (30 Tage, in der DB nur als SHA-256-Hash).

## Daten

Alles liegt in einer **PostgreSQL**-Datenbank (`DATABASE_URL`, lokal Standard `postgres://naderben:naderben@localhost:5432/naderben`):
Benutzer, Rollen, Sessions, geänderte Texte, Portfolio, Shop (Produkte, Warenkörbe, Bestellungen) und hochgeladene Bilder (als BYTEA, ausgeliefert unter `/media/{id}`, max. 15 MB, JPG/PNG/WebP/GIF).

- Schema: `migrations/` – wird beim Start automatisch angewendet (leere Datenbank genügt).
- Standardtexte und Bildfelder: `src/content.rs`. In der DB stehen nur Abweichungen; „Standard“ im Editor stellt den Originaltext wieder her.
- Beim allerersten Start werden Platzhalter-Galerien aus `src/data.rs` eingespielt.

## Entwicklung

PostgreSQL lokal starten (braucht Docker):

```bash
docker compose up -d
```

Die Datenbank läuft dann auf `localhost:5432` (Benutzer und Passwort `naderben`, siehe `docker-compose.yml`). Die Daten bleiben im Docker-Volume `naderben-pg` erhalten; zum Zurücksetzen `docker compose down -v`.

```bash
rustup target add wasm32-unknown-unknown
cargo install cargo-leptos --locked
cargo leptos watch
```

Dann <http://127.0.0.1:3080> öffnen. Das Startpasswort steht im Terminal-Log, oder vorher setzen:

```bash
ADMIN_PASSWORD='ein-sicheres-passwort' cargo leptos watch
```

## Design

Tailwind CSS v4 – Utility-Klassen direkt in den `view!`-Makros, Design-Tokens und Bausteine wie `btn`, `eyebrow`, `wrap` in `style/tailwind.css`. cargo-leptos lädt Tailwind automatisch, Node/npm ist nicht nötig.

Kontaktanfragen werden validiert und ins Server-Log geschrieben (`src/pages/contact.rs`). Für E-Mail-Versand dort z. B. `lettre` (SMTP) oder einen Webhook anschliessen.

## Deployment auf Railway

1. Repo auf GitHub pushen und in Railway als neuen Service verbinden – Railway baut mit dem `Dockerfile` (`railway.json`).
2. **PostgreSQL-Dienst hinzufügen** (*New → Database → PostgreSQL*). Im App-Dienst unter *Variables* die Variable **`DATABASE_URL`** auf den Dienst referenzieren, z. B. `${{Postgres.DATABASE_URL}}`. Kein Volume nötig: die Daten liegen in der Datenbank.
3. Variable **`ADMIN_PASSWORD`** setzen (nur für den allerersten Start relevant).
4. Der Server bindet automatisch an `0.0.0.0:$PORT`.
5. Unter *Settings → Networking* eine Domain generieren bzw. `naderben.com` verbinden.

Lokal testen (Container gegen die lokale PostgreSQL-Datenbank):

```bash
docker build -t naderben .
docker run --rm -p 8080:8080 \
  -e DATABASE_URL='postgres://naderben:naderben@host.docker.internal:5432/naderben' \
  -e ADMIN_PASSWORD='ein-sicheres-passwort' naderben
```

> Hinweis: `.cargo/config.toml` setzt `--cfg erase_components` für alle Targets (Server + WASM). Ohne das Flag scheitert der macOS-Linker an den langen Leptos-Typnamen; es muss auf beiden Seiten gleich sein, sonst schlägt die Hydration fehl.
