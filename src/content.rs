//! Alle bearbeitbaren Texte und Bilder der Seiten.
//!
//! Jedes Feld hat einen festen Schlüssel und einen Standardwert. In der
//! Datenbank (Tabelle `content`) stehen nur geänderte Werte – fehlt ein
//! Schlüssel dort, wird der Standardwert angezeigt.

use leptos::prelude::*;

use crate::{data::SiteData, models::SessionUser};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FieldKind {
    /// Einzeilig
    Text,
    /// Mehrzeilig
    LongText,
    /// Bild-URL (`/media/…` nach Upload oder `https://…`)
    Image,
}

#[derive(Clone, Copy, Debug)]
pub struct Field {
    pub key: &'static str,
    pub label: &'static str,
    pub kind: FieldKind,
    pub default: &'static str,
}

#[derive(Clone, Copy, Debug)]
pub struct PageDef {
    pub slug: &'static str,
    pub name: &'static str,
    pub fields: &'static [Field],
}

const fn text(key: &'static str, label: &'static str, default: &'static str) -> Field {
    Field { key, label, kind: FieldKind::Text, default }
}
const fn long(key: &'static str, label: &'static str, default: &'static str) -> Field {
    Field { key, label, kind: FieldKind::LongText, default }
}
const fn image(key: &'static str, label: &'static str, default: &'static str) -> Field {
    Field { key, label, kind: FieldKind::Image, default }
}

pub const PAGES: &[PageDef] = &[
    PageDef {
        slug: "start",
        name: "Startseite",
        fields: &[
            image("home.hero.image", "Titelbild", "https://picsum.photos/seed/nb-hero/2000/1200"),
            text("home.hero.eyebrow", "Überzeile", "Fotografie · Videografie · St. Gallen"),
            text("home.hero.title1", "Titel Zeile 1", "Bilder, die bleiben."),
            text("home.hero.title2", "Titel Zeile 2 (Outline)", "Filme, die berühren."),
            long("home.hero.lead", "Einleitung", "Ich erzähle Geschichten mit Licht – für Paare, Marken und Menschen, die mehr wollen als nur schöne Bilder."),
            text("home.marquee", "Laufband (mit Komma getrennt)", "Hochzeiten, Portraits, Imagefilme, Musikvideos, Events, Drohnenaufnahmen"),
            text("home.featured.eyebrow", "Ausgewählte Arbeiten – Überzeile", "Ausgewählte Arbeiten"),
            text("home.featured.title", "Ausgewählte Arbeiten – Titel", "Ein Blick ins Portfolio"),
            text("home.showreel.eyebrow", "Showreel – Überzeile", "Showreel 2026"),
            text("home.showreel.title", "Showreel – Titel", "Bewegte Bilder"),
            text("home.showreel.text", "Showreel – Text", "Imagefilme, Hochzeitsfilme & Musikvideos"),
            text("home.stat1.value", "Kennzahl 1 – Wert", "8+"),
            text("home.stat1.label", "Kennzahl 1 – Bezeichnung", "Jahre Erfahrung"),
            text("home.stat2.value", "Kennzahl 2 – Wert", "240"),
            text("home.stat2.label", "Kennzahl 2 – Bezeichnung", "Projekte"),
            text("home.stat3.value", "Kennzahl 3 – Wert", "90"),
            text("home.stat3.label", "Kennzahl 3 – Bezeichnung", "Hochzeiten"),
            text("home.stat4.value", "Kennzahl 4 – Wert", "35"),
            text("home.stat4.label", "Kennzahl 4 – Bezeichnung", "Marken & Firmen"),
        ],
    },
    PageDef {
        slug: "about",
        name: "About",
        fields: &[
            text("about.title", "Titel", "Hinter der Kamera"),
            image("about.image", "Portrait", "https://picsum.photos/seed/nb-about/900/1150"),
            text("about.caption", "Bildunterschrift", "Nader Ben · Fotograf & Videograf"),
            long("about.lead", "Einleitung", "Hallo, ich bin Nader – visueller Geschichtenerzähler aus St. Gallen."),
            long("about.text1", "Text Absatz 1", "Seit über acht Jahren halte ich fest, was Menschen bewegt: den nervösen Blick vor dem Ja-Wort, die Energie eines Live-Events, die Leidenschaft hinter einer Marke. Mein Stil ist reduziert, cineastisch und immer echt."),
            long("about.text2", "Text Absatz 2", "Ob ein intimes Portrait oder eine mehrtägige Filmproduktion – ich arbeite mit modernem Equipment, einem Auge fürs Detail und viel Ruhe. Damit du dich wohlfühlst und das Ergebnis für sich spricht."),
            text("about.gear", "Equipment (mit Komma getrennt)", "Sony FX3, Sony A7 IV, DJI Mavic 3, DaVinci Resolve, Lightroom"),
            text("about.services.title", "Leistungen – Titel", "Was ich für dich mache"),
            text("about.service1.title", "Leistung 1 – Titel", "Fotografie"),
            long("about.service1.text", "Leistung 1 – Text", "Portraits, Hochzeiten, Business- und Eventfotografie – natürlich, ehrlich und mit Gefühl für Licht."),
            text("about.service2.title", "Leistung 2 – Titel", "Videografie"),
            long("about.service2.text", "Leistung 2 – Text", "Imagefilme, Hochzeitsfilme, Social-Media-Clips und Musikvideos in Kinoqualität."),
            text("about.service3.title", "Leistung 3 – Titel", "Postproduktion"),
            long("about.service3.text", "Leistung 3 – Text", "Color Grading, Schnitt, Sounddesign und Retusche – alles aus einer Hand."),
            text("about.service4.title", "Leistung 4 – Titel", "Drohne"),
            long("about.service4.text", "Leistung 4 – Text", "Zertifizierte Luftaufnahmen für Perspektiven, die sonst niemand zeigt."),
            text("about.steps.title", "Ablauf – Titel", "So arbeiten wir zusammen"),
            text("about.step1.title", "Schritt 1 – Titel", "Kennenlernen"),
            long("about.step1.text", "Schritt 1 – Text", "Wir besprechen deine Idee, den Stil und den Rahmen – unverbindlich bei einem Kaffee oder per Call."),
            text("about.step2.title", "Schritt 2 – Titel", "Konzept"),
            long("about.step2.text", "Schritt 2 – Text", "Ich erstelle ein Moodboard, plane Locations, Licht und Ablauf."),
            text("about.step3.title", "Schritt 3 – Titel", "Produktion"),
            long("about.step3.text", "Schritt 3 – Text", "Am Shooting-Tag kümmere ich mich um alles. Du kannst einfach du sein."),
            text("about.step4.title", "Schritt 4 – Titel", "Lieferung"),
            long("about.step4.text", "Schritt 4 – Text", "Fertig bearbeitete Bilder und Filme in einer persönlichen Online-Galerie."),
            long("about.quote", "Kundenstimme", "„Nader hat unseren Hochzeitstag so eingefangen, wie er sich angefühlt hat. Wir schauen den Film immer wieder – und weinen jedes Mal.“"),
            text("about.quote.author", "Kundenstimme – Name", "Lea & Marco, Hochzeit am Zürichsee"),
        ],
    },
    PageDef {
        slug: "portfolio",
        name: "Portfolio",
        fields: &[
            text("portfolio.title", "Titel", "Arbeiten"),
            long("portfolio.kuenstler.description", "Künstler – Beschreibung", "Musiker, Tänzer und Kreative – Promo-Shootings, Musikvideos und Backstage-Momente."),
            image("portfolio.kuenstler.cover", "Künstler – Titelbild (leer = erstes Foto)", ""),
            long("portfolio.objekte.description", "Objekte – Beschreibung", "Produkte, Architektur und Räume – präzise ausgeleuchtet und ins beste Licht gerückt."),
            image("portfolio.objekte.cover", "Objekte – Titelbild (leer = erstes Foto)", ""),
            long("portfolio.hochzeiten.description", "Hochzeiten – Beschreibung", "Vom Getting Ready bis zum letzten Tanz – ehrlich, emotional und zeitlos."),
            image("portfolio.hochzeiten.cover", "Hochzeiten – Titelbild (leer = erstes Foto)", ""),
            long("portfolio.firmenportrait.description", "Firmenportrait – Beschreibung", "Imagefilme, Teamportraits und Einblicke, die zeigen, wofür eine Marke steht."),
            image("portfolio.firmenportrait.cover", "Firmenportrait – Titelbild (leer = erstes Foto)", ""),
        ],
    },
    PageDef {
        slug: "contact",
        name: "Contact",
        fields: &[
            text("contact.title", "Titel", "Erzähl mir von deinem Projekt."),
            long("contact.lead", "Einleitung", "Hochzeit, Imagefilm oder Portrait-Shooting – schreib mir, ich freue mich auf deine Idee."),
            text("contact.email", "E-Mail", "hello@naderben.com"),
            text("contact.phone", "Telefon", "+41 79 000 00 00"),
            text("contact.location", "Studio – Ort", "Zürich, Schweiz"),
            text("contact.availability", "Studio – Zusatz", "Weltweit verfügbar"),
        ],
    },
    PageDef {
        slug: "allgemein",
        name: "Allgemein",
        fields: &[
            text("footer.claim", "Footer – Text", "Foto & Video aus Zürich – für Menschen, Marken und Momente."),
            text("cta.title", "Aufruf am Seitenende", "Lass uns etwas Grossartiges festhalten."),
            text("social.instagram", "Instagram-Link", "https://instagram.com"),
            text("social.vimeo", "Vimeo-Link", "https://vimeo.com"),
            text("social.youtube", "YouTube-Link", "https://youtube.com"),
        ],
    },
];

pub fn find_field(key: &str) -> Option<&'static Field> {
    PAGES.iter().flat_map(|p| p.fields.iter()).find(|f| f.key == key)
}

pub fn default_value(key: &str) -> &'static str {
    find_field(key).map(|f| f.default).unwrap_or("")
}

/// Zugriff auf Seiteninhalte und angemeldeten Benutzer – per Context überall verfügbar.
#[derive(Clone, Copy)]
pub struct Site {
    pub data: Resource<Result<SiteData, ServerFnError>>,
    pub user: Resource<Result<Option<SessionUser>, ServerFnError>>,
}

impl Site {
    /// Aktueller Text eines Feldes (Datenbank oder Standardwert). Reaktiv.
    pub fn text(&self, key: &str) -> String {
        self.data
            .with(|d| {
                d.as_ref()
                    .and_then(|r| r.as_ref().ok())
                    .and_then(|d| d.content.get(key).cloned())
            })
            .unwrap_or_else(|| default_value(key).to_string())
    }

    /// Liste aus einem kommagetrennten Feld
    pub fn list(&self, key: &str) -> Vec<String> {
        self.text(key)
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect()
    }

    /// Mit den geladenen Portfolio-Daten arbeiten (leer, solange nichts geladen ist). Reaktiv.
    pub fn with_data<T>(&self, f: impl FnOnce(&SiteData) -> T) -> T {
        let empty = SiteData::default();
        self.data.with(|d| match d.as_ref().and_then(|r| r.as_ref().ok()) {
            Some(data) => f(data),
            None => f(&empty),
        })
    }

    pub fn current_user(&self) -> Option<SessionUser> {
        self.user.get().and_then(|r| r.ok()).flatten()
    }
}

pub fn use_site() -> Site {
    expect_context::<Site>()
}
