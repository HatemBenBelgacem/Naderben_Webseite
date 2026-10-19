//! Portfolio-Inhalte. Bilder kommen aktuell als Platzhalter von picsum.photos,
//! Videos sind YouTube-Embeds (Blender-Open-Movies) – einfach durch eigene Werke ersetzen.

/// Die vier Bereiche des Portfolios, jeweils mit Videos und Fotos.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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

    /// URL-Segment, z. B. `/portfolio/kuenstler/fotos`
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

    pub fn description(self) -> &'static str {
        match self {
            Category::Kuenstler => "Musiker, Tänzer und Kreative – Promo-Shootings, Musikvideos und Backstage-Momente.",
            Category::Objekte => "Produkte, Architektur und Räume – präzise ausgeleuchtet und ins beste Licht gerückt.",
            Category::Hochzeiten => "Vom Getting Ready bis zum letzten Tanz – ehrlich, emotional und zeitlos.",
            Category::Firmenportrait => "Imagefilme, Teamportraits und Einblicke, die zeigen, wofür eine Marke steht.",
        }
    }

    pub fn from_slug(slug: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|c| c.slug() == slug)
    }

    pub fn href(self) -> String {
        format!("/portfolio/{}", self.slug())
    }

    /// Indizes in `PHOTOS` (die Lightbox blättert über diese Liste)
    pub fn photo_indices(self) -> Vec<usize> {
        PHOTOS
            .iter()
            .enumerate()
            .filter(|(_, p)| p.category == self)
            .map(|(i, _)| i)
            .collect()
    }

    pub fn photos(self) -> impl Iterator<Item = &'static Photo> {
        PHOTOS.iter().filter(move |p| p.category == self)
    }

    pub fn videos(self) -> impl Iterator<Item = &'static Video> {
        VIDEOS.iter().filter(move |v| v.category == self)
    }

    /// Titelbild für Übersichtskarten: erstes Foto der Kategorie
    pub fn cover(self) -> String {
        self.photos().next().map(|p| p.thumb()).unwrap_or_default()
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Photo {
    pub title: &'static str,
    pub category: Category,
    /// Seed für das Platzhalterbild bzw. Pfad, sobald eigene Bilder unter /public liegen
    pub seed: &'static str,
    pub width: u32,
    pub height: u32,
}

impl Photo {
    pub fn thumb(&self) -> String {
        self.src(800)
    }

    pub fn full(&self) -> String {
        self.src(1800)
    }

    fn src(&self, w: u32) -> String {
        let h = w * self.height / self.width;
        format!("https://picsum.photos/seed/{}/{w}/{h}", self.seed)
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Video {
    pub title: &'static str,
    pub category: Category,
    pub client: &'static str,
    pub duration: &'static str,
    pub youtube_id: &'static str,
}

impl Video {
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

use Category::*;

pub const PHOTOS: &[Photo] = &[
    // Künstler
    Photo { title: "Golden Hour", category: Kuenstler, seed: "nb-portrait-1", width: 4, height: 5 },
    Photo { title: "Studio Noir", category: Kuenstler, seed: "nb-portrait-3", width: 4, height: 5 },
    Photo { title: "Backstage", category: Kuenstler, seed: "nb-event-1", width: 2, height: 3 },
    Photo { title: "Urban", category: Kuenstler, seed: "nb-portrait-4", width: 3, height: 2 },
    // Objekte
    Photo { title: "Nebelmeer", category: Objekte, seed: "nb-land-2", width: 16, height: 9 },
    Photo { title: "Stille", category: Objekte, seed: "nb-portrait-2", width: 2, height: 3 },
    Photo { title: "Seeufer", category: Objekte, seed: "nb-land-3", width: 4, height: 5 },
    Photo { title: "Gipfel", category: Objekte, seed: "nb-land-4", width: 3, height: 2 },
    // Hochzeiten
    Photo { title: "Ja, ich will", category: Hochzeiten, seed: "nb-wedding-1", width: 3, height: 2 },
    Photo { title: "Erster Tanz", category: Hochzeiten, seed: "nb-wedding-2", width: 4, height: 5 },
    Photo { title: "Konfetti", category: Hochzeiten, seed: "nb-wedding-3", width: 2, height: 3 },
    Photo { title: "Ringe", category: Hochzeiten, seed: "nb-wedding-4", width: 1, height: 1 },
    // Firmenportrait
    Photo { title: "Keynote", category: Firmenportrait, seed: "nb-event-2", width: 3, height: 2 },
    Photo { title: "Gala", category: Firmenportrait, seed: "nb-event-4", width: 4, height: 5 },
    Photo { title: "Teamwork", category: Firmenportrait, seed: "nb-event-3", width: 16, height: 9 },
    Photo { title: "Alpenglühen", category: Firmenportrait, seed: "nb-land-1", width: 3, height: 2 },
];

pub const VIDEOS: &[Video] = &[
    // Künstler
    Video { title: "Sintel", category: Kuenstler, client: "Blender Studio", duration: "14:48", youtube_id: "eRsGyueVLvQ" },
    Video { title: "Elephants Dream", category: Kuenstler, client: "Blender Studio", duration: "10:54", youtube_id: "TLkA0RELQ1g" },
    // Objekte
    Video { title: "Cosmos Laundromat", category: Objekte, client: "Blender Studio", duration: "12:10", youtube_id: "Y-rmzh0PI3c" },
    // Hochzeiten
    Video { title: "Spring", category: Hochzeiten, client: "Blender Studio", duration: "7:44", youtube_id: "WhWc3b3KhnY" },
    // Firmenportrait
    Video { title: "Big Buck Bunny", category: Firmenportrait, client: "Blender Studio", duration: "10:34", youtube_id: "aqz-KE-bpKQ" },
    Video { title: "Tears of Steel", category: Firmenportrait, client: "Blender Studio", duration: "12:14", youtube_id: "R6MlUcmOul8" },
];
