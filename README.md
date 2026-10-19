# Naderben_Webseite

Portfolio-Website für Foto & Video – Fullstack in Rust mit [Leptos 0.8](https://leptos.dev) (SSR + Hydration) und Axum.

## Seiten

| Route                | Inhalt                                             |
| -------------------- | -------------------------------------------------- |
| `/`                  | Landing mit Hero, ausgewählten Arbeiten, Showreel  |
| `/about`             | Über mich, Leistungen, Ablauf, Testimonial         |
| `/portfolio`         | Übersicht der Produkte **Videos** und **Fotos**    |
| `/portfolio/videos`  | Video-Grid mit Filter und Player-Overlay           |
| `/portfolio/fotos`   | Masonry-Galerie mit Filter und Lightbox (← → Esc)  |
| `/contact`           | Kontaktformular (Leptos Server-Funktion)           |

## Entwicklung

```bash
rustup target add wasm32-unknown-unknown
cargo install cargo-leptos --locked
cargo leptos watch
```

Dann <http://127.0.0.1:3080> öffnen.

## Inhalte anpassen

- **Fotos & Videos:** `src/data.rs` – aktuell Platzhalter (picsum.photos bzw. Blender-Open-Movies auf YouTube). Eigene Bilder in `public/` legen und `Photo::src` darauf umstellen.
- **Texte:** `src/pages/*.rs`
- **Design:** Tailwind CSS v4 – Utility-Klassen direkt in den `view!`-Makros, Design-Tokens (Farben, Fonts, Animationen) und Bausteine wie `btn`, `eyebrow`, `wrap` in `style/tailwind.css`. cargo-leptos lädt Tailwind automatisch, Node/npm ist nicht nötig.
- **Kontaktanfragen** werden validiert und ins Server-Log geschrieben (`src/pages/contact.rs`). Für E-Mail-Versand dort z. B. `lettre` (SMTP) oder einen Webhook anschliessen.

## Deployment auf Railway

1. Repo auf GitHub pushen und in Railway als neuen Service verbinden.
2. Railway erkennt `railway.json` und baut mit dem `Dockerfile`.
3. Der Server bindet automatisch an `0.0.0.0:$PORT` – keine weiteren Variablen nötig.
4. Unter *Settings → Networking* eine Domain generieren bzw. `naderben.com` verbinden.

Lokal testen:

```bash
docker build -t naderben .
docker run --rm -p 8080:8080 naderben
```

> Hinweis: `.cargo/config.toml` setzt `--cfg erase_components` für alle Targets (Server + WASM). Ohne das Flag scheitert der macOS-Linker an den langen Leptos-Typnamen; es muss auf beiden Seiten gleich sein, sonst schlägt die Hydration fehl.
