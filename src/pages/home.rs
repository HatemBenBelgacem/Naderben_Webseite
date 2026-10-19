use leptos::prelude::*;
use leptos_meta::Title;
use leptos_router::components::A;

use crate::{content::use_site, data::Category};

/// Kachel-Positionen im Featured-Grid (Tailwind braucht vollständige Klassennamen)
const FEATURED_SPANS: [&str; 4] = [
    "col-span-2 row-span-2 md:col-span-5 md:row-span-5",
    "col-span-2 row-span-2 md:col-span-7 md:row-span-3",
    "md:col-span-4 md:row-span-2",
    "md:col-span-3 md:row-span-2",
];

#[component]
pub fn HomePage() -> impl IntoView {
    let site = use_site();
    // Je ein Bild aus den vier Portfolio-Bereichen
    let featured = move || {
        site.with_data(|d| Category::ALL.iter().filter_map(|&c| d.photos_in(c).next().cloned()).collect::<Vec<_>>())
    };
    // Showreel: erstes Video (bevorzugt Firmenportrait)
    let video = move || {
        site.with_data(|d| d.videos_in(Category::Firmenportrait).next().or(d.videos.first()).cloned())
    };
    let stat = |n: u8| (format!("home.stat{n}.value"), format!("home.stat{n}.label"));

    view! {
        <Transition fallback=|| ()>
        <Title text="" />
        <section class="relative flex min-h-svh items-end overflow-hidden pb-[clamp(80px,12vh,140px)]">
            <div
                class="absolute inset-0 animate-hero-zoom bg-cover bg-center"
                style=move || format!("background-image:url('{}')", site.text("home.hero.image"))
            ></div>
            <div class="hero-overlay absolute inset-0"></div>
            <div class="wrap relative">
                <p class="eyebrow animate-reveal [animation-delay:150ms]">{move || site.text("home.hero.eyebrow")}</p>
                <h1 class="animate-reveal text-[clamp(2.8rem,8.5vw,7.2rem)] [animation-delay:300ms]">
                    {move || site.text("home.hero.title1")}
                    <br />
                    <em class="text-outline not-italic">{move || site.text("home.hero.title2")}</em>
                </h1>
                <p class="lead animate-reveal [animation-delay:450ms]">
                    {move || site.text("home.hero.lead")}
                </p>
                <div class="mt-8 flex animate-reveal flex-wrap gap-3 [animation-delay:600ms]">
                    <A href="/portfolio" attr:class="btn btn-primary">"Portfolio ansehen"</A>
                    <A href="/contact" attr:class="btn btn-ghost">"Projekt anfragen"</A>
                </div>
            </div>
            <div class="absolute right-8 bottom-10 hidden text-[0.72rem] tracking-[0.3em] text-muted uppercase [writing-mode:vertical-rl] after:mx-auto after:mt-3 after:block after:h-14 after:w-px after:bg-linear-to-b after:from-accent after:to-transparent md:block">
                "Scroll"
            </div>
        </section>

        <section class="overflow-hidden border-y border-line bg-bg-2 py-[22px]" aria-hidden="true">
            <div class="flex w-max animate-marquee gap-10 font-display text-[clamp(1.4rem,3vw,2.2rem)] font-semibold whitespace-nowrap">
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

        <section class="wrap section-y">
            <div class="mb-10 flex items-end justify-between gap-6">
                <div>
                    <p class="eyebrow">{move || site.text("home.featured.eyebrow")}</p>
                    <h2 class="mb-0">{move || site.text("home.featured.title")}</h2>
                </div>
                <A href="/portfolio" attr:class="font-medium whitespace-nowrap text-accent">"Zum Portfolio →"</A>
            </div>
            <div class="grid auto-rows-[160px] grid-cols-2 gap-4 md:auto-rows-[120px] md:grid-cols-12">
                {move || featured()
                    .into_iter()
                    .zip(FEATURED_SPANS)
                    .map(|(p, span)| {
                        view! {
                            <A
                                href=format!("{}/fotos", p.category.href())
                                attr:class=format!("group relative overflow-hidden rounded-card bg-surface {span}")
                            >
                                <img
                                    class="size-full object-cover transition-transform duration-900 ease-soft group-hover:scale-105"
                                    src=p.thumb().to_string()
                                    alt=p.title.clone()
                                    loading="lazy"
                                />
                                <div class="absolute inset-x-0 bottom-0 flex translate-y-2.5 flex-col bg-linear-to-t from-black/70 to-transparent px-5 py-[18px] opacity-0 transition duration-400 ease-soft group-hover:translate-y-0 group-hover:opacity-100 group-focus-visible:translate-y-0 group-focus-visible:opacity-100">
                                    <span class="text-xs tracking-[0.15em] text-accent uppercase">{p.category.name()}</span>
                                    <strong>{p.title}</strong>
                                </div>
                            </A>
                        }
                    })
                    .collect_view()}
            </div>
        </section>

        {move || video().map(|video| view! {
        <section class="wrap pb-[clamp(64px,10vw,120px)]">
            <A
                href=format!("{}/videos", video.category.href())
                attr:class="group relative block aspect-[4/3] overflow-hidden rounded-card bg-surface md:aspect-[21/9]"
            >
                <img
                    class="size-full object-cover brightness-55 transition-transform duration-900 ease-soft group-hover:scale-105"
                    src=video.thumb()
                    alt="Showreel"
                    loading="lazy"
                />
                <div class="absolute inset-0 flex items-end gap-6 p-[clamp(24px,4vw,56px)]">
                    <span
                        class="play-btn size-[76px] animate-pulse-ring transition-transform duration-400 ease-soft group-hover:scale-110"
                        aria-hidden="true"
                    ></span>
                    <div>
                        <p class="eyebrow">{move || site.text("home.showreel.eyebrow")}</p>
                        <h2 class="mb-[0.2em]">{move || site.text("home.showreel.title")}</h2>
                        <p class="mb-0 text-muted">{move || site.text("home.showreel.text")}</p>
                    </div>
                </div>
            </A>
        </section>
        })}

        <section class="wrap grid grid-cols-2 gap-6 gap-y-10 border-t border-line pt-14 pb-[clamp(64px,10vw,120px)] md:grid-cols-4">
            {(1..=4u8)
                .map(stat)
                .map(|(num, label)| {
                    view! {
                        <div class="flex flex-col">
                            <strong class="font-display text-[clamp(2.4rem,5vw,4rem)] leading-none text-accent">
                                {move || site.text(&num)}
                            </strong>
                            <span class="mt-2 text-muted">{move || site.text(&label)}</span>
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
        <section class="bg-accent py-[clamp(56px,8vw,96px)] text-accent-ink">
            <div class="wrap flex flex-col items-start gap-8 md:flex-row md:items-center md:justify-between">
                <h2 class="mb-0 max-w-[16ch]">{move || site.text("cta.title")}</h2>
                <A href="/contact" attr:class="btn bg-accent-ink text-ink hover:bg-black">"Jetzt anfragen"</A>
            </div>
        </section>
    }
}
