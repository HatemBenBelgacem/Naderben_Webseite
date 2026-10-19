use leptos::{ev, prelude::*};
use leptos_meta::Title;
use leptos_router::components::{Outlet, A};

use crate::data::{Photo, Video, PHOTOS, PHOTO_CATEGORIES, VIDEOS, VIDEO_CATEGORIES};

const TAB: &str = "rounded-full px-[1.3em] py-[0.55em] text-[0.95rem] text-muted transition-colors hover:text-ink aria-[current=page]:bg-ink aria-[current=page]:text-bg";
const CHIP: &str = "rounded-full border px-[1.15em] py-[0.5em] text-[0.9rem] transition-all duration-250";
const CHIP_ON: &str = "border-accent bg-accent text-accent-ink";
const CHIP_OFF: &str = "border-line text-muted hover:border-ink/35 hover:text-ink";
const MEDIA_IMG: &str = "size-full object-cover transition-transform duration-900 ease-soft group-hover:scale-105";
const SECTION_END: &str = "pb-[clamp(64px,10vw,120px)]";

const LIGHTBOX: &str = "fixed inset-0 z-[100] flex animate-fade items-center justify-center bg-[rgb(5_5_6/0.94)] px-3 py-16 backdrop-blur-sm sm:px-[72px]";
const LB_CLOSE: &str = "absolute top-5 right-6 size-11 rounded-full text-[1.4rem] transition-colors hover:bg-white/10";
const LB_NAV: &str = "absolute bottom-3 size-[52px] rounded-full border border-line text-[2rem] leading-none transition-colors hover:bg-white/10 sm:top-1/2 sm:bottom-auto sm:-translate-y-1/2";

/// Rahmen für alle Portfolio-Unterseiten (Übersicht, Videos, Fotos)
#[component]
pub fn PortfolioPage() -> impl IntoView {
    view! {
        <section class="wrap page-hero">
            <p class="eyebrow">"Portfolio"</p>
            <h1 class="max-w-[18ch]">"Arbeiten"</h1>
            <nav class="mt-6 inline-flex gap-1 rounded-full border border-line bg-bg-2 p-[5px]" aria-label="Portfolio-Bereiche">
                <A href="/portfolio" exact=true attr:class=TAB>"Übersicht"</A>
                <A href="/portfolio/videos" attr:class=TAB>"Videos"</A>
                <A href="/portfolio/fotos" attr:class=TAB>"Fotos"</A>
            </nav>
        </section>
        <Outlet />
    }
}

#[component]
fn ProductCard(href: &'static str, image: String, count: String, title: &'static str, text: &'static str, cta: &'static str) -> impl IntoView {
    view! {
        <A
            href=href
            attr:class="group flex flex-col overflow-hidden rounded-card border border-line bg-surface transition-colors hover:border-accent/50"
        >
            <img class="aspect-[16/10] w-full object-cover transition-transform duration-900 ease-soft group-hover:scale-105" src=image alt=title loading="lazy" />
            <div class="px-[30px] pt-7 pb-8">
                <span class="text-[0.8rem] tracking-[0.15em] text-accent uppercase">{count}</span>
                <h2 class="mb-[0.3em]">{title}</h2>
                <p class="text-muted">{text}</p>
                <span class="font-medium text-accent">{cta}</span>
            </div>
        </A>
    }
}

#[component]
pub fn PortfolioOverview() -> impl IntoView {
    view! {
        <Title text="Portfolio" />
        <section class=format!("wrap grid gap-6 md:grid-cols-2 {SECTION_END}")>
            <ProductCard
                href="/portfolio/videos"
                image=VIDEOS[1].thumb()
                count=format!("{:02} Filme", VIDEOS.len())
                title="Videos"
                text="Imagefilme, Hochzeitsfilme, Musikvideos und Event-Recaps – vom Konzept bis zum Color Grading."
                cta="Filme ansehen →"
            />
            <ProductCard
                href="/portfolio/fotos"
                image=PHOTOS[2].thumb()
                count=format!("{:02} Bilder", PHOTOS.len())
                title="Fotos"
                text="Portraits, Hochzeiten, Landschaften und Events – zeitlos bearbeitet, mit Liebe zum Licht."
                cta="Galerie öffnen →"
            />
        </section>
    }
}

#[component]
fn CategoryFilter(
    categories: &'static [&'static str],
    active: ReadSignal<&'static str>,
    set_active: WriteSignal<&'static str>,
) -> impl IntoView {
    view! {
        <div class="mb-8 flex flex-wrap gap-2" role="group" aria-label="Kategorie filtern">
            {categories
                .iter()
                .map(|&c| {
                    view! {
                        <button
                            class=move || format!("{CHIP} {}", if active.get() == c { CHIP_ON } else { CHIP_OFF })
                            aria-pressed=move || (active.get() == c).to_string()
                            on:click=move |_| set_active.set(c)
                        >
                            {c}
                        </button>
                    }
                })
                .collect_view()}
        </div>
    }
}

/// Schliesst ein Overlay mit Escape und blättert mit den Pfeiltasten.
fn use_overlay_keys(on_close: impl Fn() + 'static, on_step: impl Fn(isize) + 'static) {
    let handle = window_event_listener(ev::keydown, move |e| match e.key().as_str() {
        "Escape" => on_close(),
        "ArrowRight" => on_step(1),
        "ArrowLeft" => on_step(-1),
        _ => {}
    });
    on_cleanup(move || handle.remove());
}

#[component]
pub fn PhotosPage() -> impl IntoView {
    let (filter, set_filter) = signal("Alle");
    let (selected, set_selected) = signal(None::<usize>);

    let visible = Memo::new(move |_| {
        let f = filter.get();
        PHOTOS
            .iter()
            .enumerate()
            .filter(|(_, p)| f == "Alle" || p.category == f)
            .map(|(i, _)| i)
            .collect::<Vec<_>>()
    });

    let step = move |dir: isize| {
        let list = visible.get_untracked();
        if let Some(cur) = selected.get_untracked() {
            if let Some(pos) = list.iter().position(|&i| i == cur) {
                let len = list.len() as isize;
                let next = (pos as isize + dir).rem_euclid(len) as usize;
                set_selected.set(Some(list[next]));
            }
        }
    };
    use_overlay_keys(move || set_selected.set(None), step);

    view! {
        <Title text="Fotos" />
        <section class="wrap">
            <CategoryFilter categories=&PHOTO_CATEGORIES active=filter set_active=set_filter />
            <div class=format!("columns-[3_320px] gap-4 {SECTION_END}")>
                <For
                    each=move || visible.get()
                    key=|i| *i
                    children=move |i| {
                        let p: Photo = PHOTOS[i];
                        view! {
                            <button
                                class="group relative mb-4 block w-full animate-reveal break-inside-avoid overflow-hidden rounded-card bg-surface text-left [animation-duration:0.6s]"
                                on:click=move |_| set_selected.set(Some(i))
                            >
                                <img
                                    class="h-auto w-full transition-transform duration-900 ease-soft group-hover:scale-105"
                                    src=p.thumb()
                                    alt=p.title
                                    loading="lazy"
                                    width=p.width * 200
                                    height=p.height * 200
                                />
                                <span class="absolute inset-x-0 bottom-0 flex flex-col bg-linear-to-t from-black/75 to-transparent px-[18px] py-4 font-display font-semibold opacity-0 transition-opacity duration-350 group-hover:opacity-100 group-focus-visible:opacity-100">
                                    <small class="font-body text-[0.72rem] font-medium tracking-[0.15em] text-accent uppercase">
                                        {p.category}
                                    </small>
                                    {p.title}
                                </span>
                            </button>
                        }
                    }
                />
            </div>
        </section>

        <Show when=move || selected.get().is_some()>
            {move || {
                selected
                    .get()
                    .map(|i| {
                        let p = PHOTOS[i];
                        view! {
                            <div class=LIGHTBOX on:click=move |_| set_selected.set(None)>
                                <button class=LB_CLOSE aria-label="Schliessen">"✕"</button>
                                <button
                                    class=format!("{LB_NAV} left-3")
                                    aria-label="Vorheriges Bild"
                                    on:click=move |e| {
                                        e.stop_propagation();
                                        step(-1);
                                    }
                                >
                                    "‹"
                                </button>
                                <figure class="flex max-h-full max-w-full flex-col items-center" on:click=|e| e.stop_propagation()>
                                    <img
                                        class="max-h-[calc(100vh-160px)] w-auto animate-zoom-in rounded-[10px]"
                                        src=p.full()
                                        alt=p.title
                                    />
                                    <figcaption class="mt-3.5 text-muted">
                                        <strong class="text-ink">{p.title}</strong>
                                        " · "
                                        {p.category}
                                    </figcaption>
                                </figure>
                                <button
                                    class=format!("{LB_NAV} right-3")
                                    aria-label="Nächstes Bild"
                                    on:click=move |e| {
                                        e.stop_propagation();
                                        step(1);
                                    }
                                >
                                    "›"
                                </button>
                            </div>
                        }
                    })
            }}
        </Show>
    }
}

#[component]
pub fn VideosPage() -> impl IntoView {
    let (filter, set_filter) = signal("Alle");
    let (playing, set_playing) = signal(None::<usize>);

    let visible = Memo::new(move |_| {
        let f = filter.get();
        VIDEOS
            .iter()
            .enumerate()
            .filter(|(_, v)| f == "Alle" || v.category == f)
            .map(|(i, _)| i)
            .collect::<Vec<_>>()
    });

    use_overlay_keys(move || set_playing.set(None), |_| {});

    view! {
        <Title text="Videos" />
        <section class="wrap">
            <CategoryFilter categories=&VIDEO_CATEGORIES active=filter set_active=set_filter />
            <div class=format!("grid grid-cols-1 gap-x-6 gap-y-8 sm:grid-cols-[repeat(auto-fill,minmax(340px,1fr))] {SECTION_END}")>
                <For
                    each=move || visible.get()
                    key=|i| *i
                    children=move |i| {
                        let v: Video = VIDEOS[i];
                        view! {
                            <button
                                class="group animate-reveal text-left [animation-duration:0.6s]"
                                on:click=move |_| set_playing.set(Some(i))
                            >
                                <div class="relative aspect-video overflow-hidden rounded-card bg-surface">
                                    <img class=format!("{MEDIA_IMG} brightness-80") src=v.thumb() alt=v.title loading="lazy" />
                                    <span
                                        class="play-btn absolute top-1/2 left-1/2 size-16 -translate-1/2 opacity-90 transition-transform duration-400 ease-soft group-hover:scale-112"
                                        aria-hidden="true"
                                    ></span>
                                    <span class="absolute right-3 bottom-3 rounded-md bg-black/70 px-[0.6em] py-[0.15em] text-[0.8rem]">
                                        {v.duration}
                                    </span>
                                </div>
                                <div class="pt-3.5">
                                    <small class="text-xs tracking-[0.12em] text-muted uppercase">
                                        {v.category} " · " {v.client}
                                    </small>
                                    <h3 class="mt-[0.2em] mb-0">{v.title}</h3>
                                </div>
                            </button>
                        }
                    }
                />
            </div>
        </section>

        <Show when=move || playing.get().is_some()>
            {move || {
                playing
                    .get()
                    .map(|i| {
                        let v = VIDEOS[i];
                        view! {
                            <div class=LIGHTBOX on:click=move |_| set_playing.set(None)>
                                <button class=LB_CLOSE aria-label="Schliessen">"✕"</button>
                                <div
                                    class="aspect-video w-[min(1200px,100%)] animate-zoom-in overflow-hidden rounded-xl bg-black"
                                    on:click=|e| e.stop_propagation()
                                >
                                    <iframe
                                        class="size-full border-0"
                                        src=v.embed()
                                        title=v.title
                                        allow="autoplay; encrypted-media; picture-in-picture; fullscreen"
                                        allowfullscreen=true
                                    ></iframe>
                                </div>
                            </div>
                        }
                    })
            }}
        </Show>
    }
}
