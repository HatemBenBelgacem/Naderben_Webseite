use leptos::prelude::*;
use leptos_meta::Title;
use leptos_router::components::A;

use super::zine::BgWords;
use crate::{content::use_site, data::Category};

/// Kachel-Positionen und leichte Drehung im Featured-Grid (Tailwind braucht vollständige Klassennamen)
const FEATURED_SPANS: [&str; 4] = [
    "col-span-2 row-span-2 md:col-span-5 md:row-span-5 -rotate-1",
    "col-span-2 row-span-2 md:col-span-7 md:row-span-3 rotate-1",
    "md:col-span-4 md:row-span-2 rotate-2",
    "md:col-span-3 md:row-span-2 -rotate-2",
];

#[component]
pub fn HomePage() -> impl IntoView {
    let site = use_site();
    // Je ein Bild aus den vier Portfolio-Bereichen
    let featured = move || {
        site.with_data(|d| Category::ALL.iter().filter_map(|&c| d.photos_in(c).next().cloned()).collect::<Vec<_>>())
    };
    // Zwei kleine Bilder für die Collage im Hero
    let collage = move || {
        site.with_data(|d| {
            [Category::Kuenstler, Category::Hochzeiten]
                .iter()
                .filter_map(|&c| d.photos_in(c).nth(1).or(d.photos_in(c).next()).map(|p| p.thumb().to_string()))
                .collect::<Vec<_>>()
        })
    };
    // Showreel: erstes Video (bevorzugt Firmenportrait)
    let video = move || {
        site.with_data(|d| d.videos_in(Category::Firmenportrait).next().or(d.videos.first()).cloned())
    };
    let stat = |n: u8| (format!("home.stat{n}.value"), format!("home.stat{n}.label"));
    let marquee = Signal::derive(move || site.list("home.marquee"));

    view! {
        <Transition fallback=|| ()>
        <Title text="" />

        // ───────────── Hero: Riesenschrift + Foto-Collage ─────────────
        <section class="relative flex min-h-svh items-center overflow-hidden pt-[110px] pb-[clamp(56px,8vw,100px)]">
            <BgWords words=marquee rows=5 />
            <div class="wrap relative grid items-center gap-12 md:grid-cols-[1.05fr_1fr]">
                <div>
                    <p class="eyebrow animate-reveal [animation-delay:100ms]">{move || site.text("home.hero.eyebrow")}</p>
                    <h1 class="animate-reveal text-[clamp(3.2rem,7.4vw,7.6rem)] leading-[0.97] [animation-delay:220ms]">
                        {move || site.text("home.hero.title1")}
                        <br />
                        <span class="highlight">{move || site.text("home.hero.title2")}</span>
                    </h1>
                    <p class="lead animate-reveal [animation-delay:360ms]">{move || site.text("home.hero.lead")}</p>
                    <div class="mt-8 flex animate-reveal flex-wrap gap-3 [animation-delay:480ms]">
                        <A href="/portfolio" attr:class="btn btn-primary">"Portfolio ansehen"</A>
                        <A href="/contact" attr:class="btn btn-ghost">"Projekt anfragen"</A>
                    </div>
                </div>

                <div class="relative mx-auto aspect-[4/5] w-full max-w-[540px] animate-reveal [animation-delay:300ms]">
                    <div class="group paper-shadow absolute inset-[7%_9%_9%_6%] -rotate-3 transition-transform duration-500 hover:rotate-0">
                        <img
                            class="torn photo-bw size-full object-cover"
                            src=move || site.text("home.hero.image")
                            alt="Titelbild"
                        />
                    </div>
                    {move || {
                        let imgs = collage();
                        view! {
                            {imgs.first().cloned().map(|src| view! {
                                <div class="group paper-shadow absolute top-0 right-0 w-[40%] rotate-6 transition-transform duration-500 hover:rotate-2">
                                    <img class="torn-alt photo-bw aspect-[4/5] w-full object-cover" src=src alt="" loading="lazy" />
                                </div>
                            })}
                            {imgs.get(1).cloned().map(|src| view! {
                                <div class="group paper-shadow absolute bottom-0 left-0 w-[44%] -rotate-6 transition-transform duration-500 hover:-rotate-2">
                                    <img class="torn photo-bw aspect-[5/4] w-full object-cover" src=src alt="" loading="lazy" />
                                </div>
                            })}
                        }
                    }}
                    <span class="tape top-[3%] left-[28%] -rotate-6"></span>
                    <span class="absolute right-[3%] bottom-[12%] rotate-3 border-2 border-ink bg-accent px-3 py-1.5 font-display text-[clamp(1.3rem,2.4vw,2rem)] leading-none font-bold uppercase">
                        "Foto & Video"
                    </span>
                </div>
            </div>
        </section>

        // ───────────── Laufband ─────────────
        <section class="overflow-hidden border-y-2 border-ink bg-ink py-4" aria-hidden="true">
            <div class="flex w-max animate-marquee gap-8 font-display text-[clamp(2rem,5vw,4rem)] leading-none font-bold tracking-[-0.03em] whitespace-nowrap text-bg uppercase">
                {move || {
                    let words = site.list("home.marquee");
                    [words.clone(), words]
                        .concat()
                        .into_iter()
                        .map(|word| {
                            view! {
                                <span>{word}</span>
                                <span class="text-accent">"✦"</span>
                            }
                        })
                        .collect_view()
                }}
            </div>
        </section>

        // ───────────── Ausgewählte Arbeiten ─────────────
        <section class="wrap section-y">
            <div class="mb-12 flex flex-wrap items-end justify-between gap-6">
                <div>
                    <p class="eyebrow">{move || site.text("home.featured.eyebrow")}</p>
                    <h2 class="mb-0">{move || site.text("home.featured.title")}</h2>
                </div>
                <A href="/portfolio" attr:class="btn btn-ghost">"Zum Portfolio →"</A>
            </div>
            <div class="grid auto-rows-[160px] grid-cols-2 gap-5 md:auto-rows-[120px] md:grid-cols-12">
                {move || featured()
                    .into_iter()
                    .zip(FEATURED_SPANS)
                    .map(|(p, span)| {
                        view! {
                            <div class=format!("group paper-shadow transition-transform duration-500 ease-soft hover:z-10 hover:rotate-0 hover:scale-[1.02] {span}")>
                                <A href=format!("{}/fotos", p.category.href()) attr:class="torn relative block size-full overflow-hidden bg-surface">
                                    <img class="photo-bw size-full object-cover" src=p.thumb().to_string() alt=p.title.clone() loading="lazy" />
                                    <span class="absolute bottom-[8%] left-4 flex -rotate-2 flex-col items-start gap-1">
                                        <span class="bg-ink px-1.5 py-0.5 text-[0.7rem] font-medium text-bg uppercase">{p.category.name()}</span>
                                        <strong class="border-2 border-ink bg-zine px-2 py-1 font-display text-[clamp(1.1rem,2vw,1.6rem)] leading-none uppercase">
                                            {p.title.clone()}
                                        </strong>
                                    </span>
                                </A>
                            </div>
                        }
                    })
                    .collect_view()}
            </div>
        </section>

        // ───────────── Showreel ─────────────
        {move || video().map(|video| view! {
        <section class="wrap pb-[clamp(64px,10vw,120px)]">
            <div class="group paper-shadow rotate-[-0.6deg]">
                <A
                    href=format!("{}/videos", video.category.href())
                    attr:class="torn-alt relative block aspect-[4/3] overflow-hidden bg-surface md:aspect-[21/9]"
                >
                    <img class="photo-bw size-full object-cover" src=video.thumb() alt="Showreel" loading="lazy" />
                    <div class="absolute inset-0 flex items-end gap-5 p-[clamp(20px,4vw,56px)]">
                        <span class="play-btn size-[76px] animate-pulse-ring transition-transform duration-400 ease-soft group-hover:scale-110" aria-hidden="true"></span>
                        <div class="max-w-md rotate-1 border-2 border-ink bg-bg px-5 pt-4 pb-3">
                            <p class="eyebrow mb-2">{move || site.text("home.showreel.eyebrow")}</p>
                            <h2 class="mb-1 text-[clamp(2rem,4.4vw,3.6rem)]">{move || site.text("home.showreel.title")}</h2>
                            <p class="mb-0 text-[0.95rem]">{move || site.text("home.showreel.text")}</p>
                        </div>
                    </div>
                </A>
            </div>
        </section>
        })}

        // ───────────── Kennzahlen ─────────────
        <section class="wrap grid grid-cols-2 gap-6 gap-y-12 border-t-2 border-ink pt-14 pb-[clamp(64px,10vw,120px)] md:grid-cols-4">
            {(1..=4u8)
                .map(stat)
                .map(|(num, label)| {
                    view! {
                        <div class="flex flex-col">
                            <strong class="font-display text-[clamp(4rem,9vw,7.5rem)] leading-[0.8] font-bold tracking-[-0.04em]">
                                {move || site.text(&num)}
                            </strong>
                            <span class="mt-3 text-[0.85rem] font-medium uppercase">{move || site.text(&label)}</span>
                        </div>
                    }
                })
                .collect_view()}
        </section>

        <CtaBand />
        </Transition>
    }
}

#[component]
pub fn CtaBand() -> impl IntoView {
    let site = use_site();
    view! {
        <section class="relative overflow-hidden border-t-2 border-ink bg-accent py-[clamp(64px,9vw,120px)]">
            <div class="wrap relative flex flex-col items-start gap-8 md:flex-row md:items-end md:justify-between">
                <h2 class="mb-0 max-w-[14ch] text-[clamp(2.8rem,7vw,6.5rem)]">{move || site.text("cta.title")}</h2>
                <A href="/contact" attr:class="btn btn-primary">"Jetzt anfragen"</A>
            </div>
        </section>
    }
}
