use leptos::prelude::*;
use leptos_meta::Title;

use super::home::CtaBand;

const SERVICES: [(&str, &str, &str); 4] = [
    ("01", "Fotografie", "Portraits, Hochzeiten, Business- und Eventfotografie – natürlich, ehrlich und mit Gefühl für Licht."),
    ("02", "Videografie", "Imagefilme, Hochzeitsfilme, Social-Media-Clips und Musikvideos in Kinoqualität."),
    ("03", "Postproduktion", "Color Grading, Schnitt, Sounddesign und Retusche – alles aus einer Hand."),
    ("04", "Drohne", "Zertifizierte Luftaufnahmen für Perspektiven, die sonst niemand zeigt."),
];

const STEPS: [(&str, &str); 4] = [
    ("Kennenlernen", "Wir besprechen deine Idee, den Stil und den Rahmen – unverbindlich bei einem Kaffee oder per Call."),
    ("Konzept", "Ich erstelle ein Moodboard, plane Locations, Licht und Ablauf."),
    ("Produktion", "Am Shooting-Tag kümmere ich mich um alles. Du kannst einfach du sein."),
    ("Lieferung", "Fertig bearbeitete Bilder und Filme in einer persönlichen Online-Galerie."),
];

const GEAR: [&str; 5] = ["Sony FX3", "Sony A7 IV", "DJI Mavic 3", "DaVinci Resolve", "Lightroom"];

#[component]
pub fn AboutPage() -> impl IntoView {
    view! {
        <Title text="About" />
        <section class="wrap page-hero">
            <p class="eyebrow">"About"</p>
            <h1 class="max-w-[18ch]">"Hinter der Kamera"</h1>
        </section>

        <section class="wrap grid items-start gap-[clamp(32px,6vw,96px)] md:grid-cols-[5fr_6fr]">
            <figure class="md:sticky md:top-[100px]">
                <img
                    class="aspect-[4/5] w-full rounded-card object-cover"
                    src="https://picsum.photos/seed/nb-about/900/1150"
                    alt="Nader Ben mit Kamera"
                />
                <figcaption class="mt-3 text-[0.85rem] text-muted">"Nader Ben · Fotograf & Videograf"</figcaption>
            </figure>
            <div class="text-body">
                <p class="lead">
                    "Hallo, ich bin Nader – visueller Geschichtenerzähler aus St. Gallen."
                </p>
                <p>
                    "Seit über acht Jahren halte ich fest, was Menschen bewegt: den nervösen Blick vor dem Ja-Wort, die Energie eines Live-Events, die Leidenschaft hinter einer Marke. Mein Stil ist reduziert, cineastisch und immer echt."
                </p>
                <p>
                    "Ob ein intimes Portrait oder eine mehrtägige Filmproduktion – ich arbeite mit modernem Equipment, einem Auge fürs Detail und viel Ruhe. Damit du dich wohlfühlst und das Ergebnis für sich spricht."
                </p>
                <div class="mt-8 flex flex-wrap gap-2">
                    {GEAR
                        .map(|g| {
                            view! {
                                <span class="rounded-full border border-line px-4 py-1.5 text-[0.85rem] text-muted">
                                    {g}
                                </span>
                            }
                        })
                        .collect_view()}
                </div>
            </div>
        </section>

        <section class="wrap section-y">
            <p class="eyebrow">"Leistungen"</p>
            <h2>"Was ich für dich mache"</h2>
            <div class="mt-10 grid gap-4 sm:grid-cols-2 lg:grid-cols-4">
                {SERVICES
                    .iter()
                    .map(|(n, title, text)| {
                        view! {
                            <article class="rounded-card border border-line bg-bg-2 p-7 transition duration-300 ease-soft hover:-translate-y-1 hover:border-accent/50">
                                <span class="mb-10 block font-display text-accent">{*n}</span>
                                <h3>{*title}</h3>
                                <p class="mb-0 text-[0.95rem] text-muted">{*text}</p>
                            </article>
                        }
                    })
                    .collect_view()}
            </div>
        </section>

        <section class="wrap section-y pt-0">
            <p class="eyebrow">"Ablauf"</p>
            <h2>"So arbeiten wir zusammen"</h2>
            <ol class="mt-10 grid gap-8 sm:grid-cols-2 lg:grid-cols-4">
                {STEPS
                    .iter()
                    .enumerate()
                    .map(|(i, (title, text))| {
                        view! {
                            <li class="border-t border-line pt-6">
                                <span class="mb-2.5 block font-display text-[2.6rem] leading-none font-bold text-transparent [-webkit-text-stroke:1px_var(--color-muted)]">
                                    {format!("{:02}", i + 1)}
                                </span>
                                <h3>{*title}</h3>
                                <p class="text-[0.95rem] text-muted">{*text}</p>
                            </li>
                        }
                    })
                    .collect_view()}
            </ol>
        </section>

        <section class="wrap section-y pt-0">
            <blockquote class="text-center">
                <p class="mx-auto mb-6 max-w-[30ch] font-display text-[clamp(1.5rem,3.2vw,2.4rem)] leading-tight">
                    "„Nader hat unseren Hochzeitstag so eingefangen, wie er sich angefühlt hat. Wir schauen den Film immer wieder – und weinen jedes Mal.“"
                </p>
                <cite class="text-accent not-italic">"Lea & Marco, Hochzeit am Zürichsee"</cite>
            </blockquote>
        </section>

        <CtaBand />
    }
}
