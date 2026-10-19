//! Portfolio-Inhalte. Bilder kommen aktuell als Platzhalter von picsum.photos,
//! Videos sind YouTube-Embeds – einfach durch eigene Werke ersetzen.

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Photo {
    pub title: &'static str,
    pub category: &'static str,
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
    pub category: &'static str,
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

pub const PHOTO_CATEGORIES: [&str; 5] = ["Alle", "Portrait", "Hochzeit", "Landschaft", "Event"];
pub const VIDEO_CATEGORIES: [&str; 5] = ["Alle", "Imagefilm", "Hochzeit", "Musikvideo", "Event"];

pub const PHOTOS: &[Photo] = &[
    Photo { title: "Golden Hour", category: "Portrait", seed: "nb-portrait-1", width: 4, height: 5 },
    Photo { title: "Ja, ich will", category: "Hochzeit", seed: "nb-wedding-1", width: 3, height: 2 },
    Photo { title: "Alpenglühen", category: "Landschaft", seed: "nb-land-1", width: 3, height: 2 },
    Photo { title: "Backstage", category: "Event", seed: "nb-event-1", width: 4, height: 5 },
    Photo { title: "Stille", category: "Portrait", seed: "nb-portrait-2", width: 2, height: 3 },
    Photo { title: "Erster Tanz", category: "Hochzeit", seed: "nb-wedding-2", width: 4, height: 5 },
    Photo { title: "Nebelmeer", category: "Landschaft", seed: "nb-land-2", width: 16, height: 9 },
    Photo { title: "Keynote", category: "Event", seed: "nb-event-2", width: 3, height: 2 },
    Photo { title: "Studio Noir", category: "Portrait", seed: "nb-portrait-3", width: 4, height: 5 },
    Photo { title: "Konfetti", category: "Hochzeit", seed: "nb-wedding-3", width: 2, height: 3 },
    Photo { title: "Seeufer", category: "Landschaft", seed: "nb-land-3", width: 4, height: 5 },
    Photo { title: "Afterparty", category: "Event", seed: "nb-event-3", width: 16, height: 9 },
    Photo { title: "Urban", category: "Portrait", seed: "nb-portrait-4", width: 3, height: 2 },
    Photo { title: "Ringe", category: "Hochzeit", seed: "nb-wedding-4", width: 1, height: 1 },
    Photo { title: "Gipfel", category: "Landschaft", seed: "nb-land-4", width: 2, height: 3 },
    Photo { title: "Gala", category: "Event", seed: "nb-event-4", width: 4, height: 5 },
];

pub const VIDEOS: &[Video] = &[
    Video { title: "Big Buck Bunny", category: "Imagefilm", client: "Blender Studio", duration: "10:34", youtube_id: "aqz-KE-bpKQ" },
    Video { title: "Sintel", category: "Musikvideo", client: "Blender Studio", duration: "14:48", youtube_id: "eRsGyueVLvQ" },
    Video { title: "Tears of Steel", category: "Event", client: "Blender Studio", duration: "12:14", youtube_id: "R6MlUcmOul8" },
    Video { title: "Spring", category: "Hochzeit", client: "Blender Studio", duration: "7:44", youtube_id: "WhWc3b3KhnY" },
    Video { title: "Cosmos Laundromat", category: "Imagefilm", client: "Blender Studio", duration: "12:10", youtube_id: "Y-rmzh0PI3c" },
    Video { title: "Elephants Dream", category: "Musikvideo", client: "Blender Studio", duration: "10:54", youtube_id: "TLkA0RELQ1g" },
];
