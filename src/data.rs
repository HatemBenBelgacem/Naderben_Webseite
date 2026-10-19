//! Gemeinsame Datentypen für Portfolio und Seiteninhalte.
//!
//! Die eigentlichen Inhalte liegen in der SQLite-Datenbank. Die `SEED_*`-Listen
//! werden nur beim ersten Start in die leere Datenbank geschrieben
//! (Platzhalter von picsum.photos bzw. Blender-Open-Movies auf YouTube).

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Die vier Bereiche des Portfolios, jeweils mit Videos und Fotos.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Category {
    Kuenstler,
    Objekte,
    Hochzeiten,
    Firmenportrait,
}

impl Category {
    pub const ALL: [Category; 4] = [
        Category::Kuenstler,
        Category::Objekte,
        Category::Hochzeiten,
        Category::Firmenportrait,
    ];

    /// URL-Segment und Wert in der Datenbank, z. B. `kuenstler`
    pub fn slug(self) -> &'static str {
        match self {
            Category::Kuenstler => "kuenstler",
            Category::Objekte => "objekte",
            Category::Hochzeiten => "hochzeiten",
            Category::Firmenportrait => "firmenportrait",
        }
    }

    pub fn name(self) -> &'static str {
        match self {
            Category::Kuenstler => "Künstler",
            Category::Objekte => "Objekte",
            Category::Hochzeiten => "Hochzeiten",
            Category::Firmenportrait => "Firmenportrait",
        }
    }

    pub fn from_slug(slug: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|c| c.slug() == slug)
    }

    pub fn href(self) -> String {
        format!("/portfolio/{}", self.slug())
    }

    /// Inhalts-Schlüssel für die Beschreibung des Bereichs
    pub fn description_key(self) -> String {
        format!("portfolio.{}.description", self.slug())
    }

    /// Inhalts-Schlüssel für ein eigenes Titelbild des Bereichs
    pub fn cover_key(self) -> String {
        format!("portfolio.{}.cover", self.slug())
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PhotoItem {
    pub id: i64,
    pub category: Category,
    pub title: String,
    pub image_url: String,
    pub thumb_url: Option<String>,
    pub width: Option<u32>,
    pub height: Option<u32>,
}

impl PhotoItem {
    pub fn thumb(&self) -> &str {
        self.thumb_url.as_deref().unwrap_or(&self.image_url)
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VideoItem {
    pub id: i64,
    pub category: Category,
    pub title: String,
    pub client: String,
    pub duration: String,
    pub youtube_id: String,
}

impl VideoItem {
    pub fn thumb(&self) -> String {
        format!("https://i.ytimg.com/vi/{}/hqdefault.jpg", self.youtube_id)
    }

    pub fn embed(&self) -> String {
        format!(
            "https://www.youtube-nocookie.com/embed/{}?autoplay=1&rel=0&modestbranding=1",
            self.youtube_id
        )
    }
}

/// Alles, was die öffentlichen Seiten brauchen – wird einmal pro Seitenaufruf geladen.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct SiteData {
    /// Überschriebene Texte/Bilder; fehlende Schlüssel fallen auf die Standardwerte zurück
    pub content: HashMap<String, String>,
    pub photos: Vec<PhotoItem>,
    pub videos: Vec<VideoItem>,
}

impl SiteData {
    pub fn photos_in(&self, c: Category) -> impl Iterator<Item = &PhotoItem> {
        self.photos.iter().filter(move |p| p.category == c)
    }

    pub fn videos_in(&self, c: Category) -> impl Iterator<Item = &VideoItem> {
        self.videos.iter().filter(move |v| v.category == c)
    }

    /// Eigenes Titelbild des Bereichs, sonst das erste Foto
    pub fn cover(&self, c: Category) -> String {
        match self.content.get(&c.cover_key()) {
            Some(url) if !url.is_empty() => url.clone(),
            _ => self.photos_in(c).next().map(|p| p.thumb().to_string()).unwrap_or_default(),
        }
    }
}

// ───────────── Startdaten für die leere Datenbank ─────────────

pub struct SeedPhoto {
    pub title: &'static str,
    pub category: Category,
    pub seed: &'static str,
    pub width: u32,
    pub height: u32,
}

impl SeedPhoto {
    pub fn url(&self, w: u32) -> String {
        let h = w * self.height / self.width;
        format!("https://picsum.photos/seed/{}/{w}/{h}", self.seed)
    }
}

pub struct SeedVideo {
    pub title: &'static str,
    pub category: Category,
    pub client: &'static str,
    pub duration: &'static str,
    pub youtube_id: &'static str,
}

use Category::*;

pub const SEED_PHOTOS: &[SeedPhoto] = &[
    SeedPhoto { title: "Golden Hour", category: Kuenstler, seed: "nb-portrait-1", width: 4, height: 5 },
    SeedPhoto { title: "Studio Noir", category: Kuenstler, seed: "nb-portrait-3", width: 4, height: 5 },
    SeedPhoto { title: "Backstage", category: Kuenstler, seed: "nb-event-1", width: 2, height: 3 },
    SeedPhoto { title: "Urban", category: Kuenstler, seed: "nb-portrait-4", width: 3, height: 2 },
    SeedPhoto { title: "Nebelmeer", category: Objekte, seed: "nb-land-2", width: 16, height: 9 },
    SeedPhoto { title: "Stille", category: Objekte, seed: "nb-portrait-2", width: 2, height: 3 },
    SeedPhoto { title: "Seeufer", category: Objekte, seed: "nb-land-3", width: 4, height: 5 },
    SeedPhoto { title: "Gipfel", category: Objekte, seed: "nb-land-4", width: 3, height: 2 },
    SeedPhoto { title: "Ja, ich will", category: Hochzeiten, seed: "nb-wedding-1", width: 3, height: 2 },
    SeedPhoto { title: "Erster Tanz", category: Hochzeiten, seed: "nb-wedding-2", width: 4, height: 5 },
    SeedPhoto { title: "Konfetti", category: Hochzeiten, seed: "nb-wedding-3", width: 2, height: 3 },
    SeedPhoto { title: "Ringe", category: Hochzeiten, seed: "nb-wedding-4", width: 1, height: 1 },
    SeedPhoto { title: "Keynote", category: Firmenportrait, seed: "nb-event-2", width: 3, height: 2 },
    SeedPhoto { title: "Gala", category: Firmenportrait, seed: "nb-event-4", width: 4, height: 5 },
    SeedPhoto { title: "Teamwork", category: Firmenportrait, seed: "nb-event-3", width: 16, height: 9 },
    SeedPhoto { title: "Alpenglühen", category: Firmenportrait, seed: "nb-land-1", width: 3, height: 2 },
];

pub const SEED_VIDEOS: &[SeedVideo] = &[
    SeedVideo { title: "Sintel", category: Kuenstler, client: "Blender Studio", duration: "14:48", youtube_id: "eRsGyueVLvQ" },
    SeedVideo { title: "Elephants Dream", category: Kuenstler, client: "Blender Studio", duration: "10:54", youtube_id: "TLkA0RELQ1g" },
    SeedVideo { title: "Cosmos Laundromat", category: Objekte, client: "Blender Studio", duration: "12:10", youtube_id: "Y-rmzh0PI3c" },
    SeedVideo { title: "Spring", category: Hochzeiten, client: "Blender Studio", duration: "7:44", youtube_id: "WhWc3b3KhnY" },
    SeedVideo { title: "Big Buck Bunny", category: Firmenportrait, client: "Blender Studio", duration: "10:34", youtube_id: "aqz-KE-bpKQ" },
    SeedVideo { title: "Tears of Steel", category: Firmenportrait, client: "Blender Studio", duration: "12:14", youtube_id: "R6MlUcmOul8" },
];
