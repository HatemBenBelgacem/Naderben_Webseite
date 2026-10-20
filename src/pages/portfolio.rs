use leptos::{ev, prelude::*};
use leptos_meta::Title;
use leptos_router::{
    components::{Outlet, A},
    hooks::use_params_map,
};

use super::zine::BgWords;
use crate::app::NotFound;
use crate::content::{use_site, Site};
use crate::data::{Category, PhotoItem, VideoItem};

const TAB: &str = "shrink-0 px-[1.2em] py-[0.5em] text-[0.9rem] font-medium whitespace-nowrap uppercase text-ink/70 transition-colors hover:text-ink aria-[current=page]:bg-ink aria-[current=page]:text-bg";
const SUB_TAB: &str = "-mb-[2px] border-b-4 border-transparent pb-2 font-medium uppercase text-ink/60 transition-colors hover:text-ink aria-[current=page]:border-accent aria-[current=page]:text-ink";
const SECTION_END: &str = "pb-[clamp(64px,10vw,120px)]";

const LIGHTBOX: &str = "fixed inset-0 z-[100] flex animate-fade items-center justify-center bg-[rgb(5_5_6/0.94)] text-white px-3 py-16 backdrop-blur-sm sm:px-[72px]";
const LB_CLOSE: &str = "absolute top-5 right-6 size-11 rounded-full text-[1.4rem] transition-colors hover:bg-white/10";
const LB_NAV: &str = "absolute bottom-3 size-[52px] rounded-full border border-line text-[2rem] leading-none transition-colors hover:bg-white/10 sm:top-1/2 sm:bottom-auto sm:-translate-y-1/2";

/// "01 Video" / "04 Videos"
fn count_label(n: usize, singular: &str, plural: &str) -> String {
    format!("{n:02} {}", if n == 1 { singular } else { plural })
}

/// Kategorie aus dem URL-Segment `/portfolio/:kategorie`
fn use_category() -> Memo<Option<Category>> {
    let params = use_params_map();
    Memo::new(move |_| params.read().get("kategorie").as_deref().and_then(Category::from_slug))
}

fn description(site: Site, c: Category) -> String {
    site.text(&c.description_key())
}

/// Rahmen für das ganze Portfolio: Titel + Navigation über die vier Bereiche
#[component]
pub fn PortfolioPage() -> impl IntoView {
    let site = use_site();
    view! {
        <Transition fallback=|| ()>
        <section class="relative overflow-hidden">
            <BgWords words=Signal::derive(|| Category::ALL.map(|c| c.name().to_string()).to_vec()) rows=3 />
            <div class="wrap page-hero relative">
            <p class="eyebrow">"Portfolio"</p>
            <h1 class="max-w-[18ch]">{move || site.text("portfolio.title")}</h1>
            <nav
                class="mt-6 inline-flex max-w-full gap-1 overflow-x-auto border-2 border-ink bg-bg-2 p-1 [scrollbar-width:none]"
                aria-label="Portfolio-Bereiche"
            >
                <A href="/portfolio" exact=true attr:class=TAB>"Übersicht"</A>
                {Category::ALL
                    .map(|c| view! { <A href=c.href() attr:class=TAB>{c.name()}</A> })
                    .collect_view()}
            </nav>
            </div>
        </section>
        <Outlet />
        </Transition>
    }
}

/// `/portfolio` – die vier Bereiche als Karten
#[component]
pub fn PortfolioOverview() -> impl IntoView {
    view! {
        <Title text="Portfolio" />
        <section class=format!("wrap grid gap-6 md:grid-cols-2 {SECTION_END}")>
            {Category::ALL.map(|c| view! { <CategoryCard category=c /> }).collect_view()}
        </section>
    }
}

#[component]
fn CategoryCard(category: Category) -> impl IntoView {
    let site = use_site();
    let counts = move || {
        site.with_data(|d| {
            format!(
                "{} · {}",
                count_label(d.videos_in(category).count(), "Video", "Videos"),
                count_label(d.photos_in(category).count(), "Foto", "Fotos")
            )
        })
    };

    view! {
        <article class="group flex flex-col border-2 border-ink bg-bg-2 transition-shadow duration-300 hover:shadow-[8px_8px_0_var(--color-ink)]">
            <A href=category.href() attr:class="block overflow-hidden border-b-2 border-ink">
                <img
                    class="photo-bw aspect-[16/10] w-full object-cover group-hover:scale-105"
                    src=move || site.with_data(|d| d.cover(category))
                    alt=category.name()
                    loading="lazy"
                />
            </A>
            <div class="flex flex-1 flex-col px-[30px] pt-7 pb-8">
                <span class="self-start bg-ink px-1.5 py-0.5 text-[0.75rem] font-medium text-bg uppercase">{counts}</span>
                <h2 class="mt-4 mb-[0.3em]">
                    <A href=category.href() attr:class="hover:text-accent">{category.name()}</A>
                </h2>
                <p class="text-muted">{move || description(site, category)}</p>
                <div class="mt-auto flex flex-wrap gap-3">
                    <A href=format!("{}/videos", category.href()) attr:class="btn btn-ghost">"Videos"</A>
                    <A href=format!("{}/fotos", category.href()) attr:class="btn btn-primary">"Fotos"</A>
                </div>
            </div>
        </article>
    }
}

/// Rahmen für einen Bereich: Name, Beschreibung und Unterteilung Videos / Fotos
#[component]
pub fn CategoryPage() -> impl IntoView {
    let site = use_site();
    let category = use_category();

    move || match category.get() {
        None => view! { <NotFound /> }.into_any(),
        Some(c) => view! {
            <section class="wrap">
                <h2 class="mb-2">{c.name()}</h2>
                <p class="lead">{move || description(site, c)}</p>
                <nav class="mt-6 mb-8 flex gap-6 border-b border-line" aria-label=format!("{} – Bereiche", c.name())>
                    <A href=c.href() exact=true attr:class=SUB_TAB>"Übersicht"</A>
                    <A href=format!("{}/videos", c.href()) attr:class=SUB_TAB>"Videos"</A>
                    <A href=format!("{}/fotos", c.href()) attr:class=SUB_TAB>"Fotos"</A>
                </nav>
            </section>
            <Outlet />
        }
        .into_any(),
    }
}

#[component]
fn ProductCard(href: String, image: String, count: String, title: &'static str, text: &'static str, cta: &'static str) -> impl IntoView {
    view! {
        <A
            href=href
            attr:class="group flex flex-col overflow-hidden border-2 border-ink bg-bg-2 transition-shadow duration-300 hover:shadow-[8px_8px_0_var(--color-ink)]"
        >
            <img class="photo-bw aspect-[16/10] w-full border-b-2 border-ink object-cover group-hover:scale-105" src=image alt=title loading="lazy" />
            <div class="px-[30px] pt-7 pb-8">
                <span class="bg-ink px-1.5 py-0.5 text-[0.75rem] font-medium text-bg uppercase">{count}</span>
                <h3 class="mt-4 mb-[0.3em] text-[clamp(2rem,4vw,3rem)]">{title}</h3>
                <p class="text-muted">{text}</p>
                <span class="font-medium uppercase underline decoration-accent decoration-4 underline-offset-4">{cta}</span>
            </div>
        </A>
    }
}

/// `/portfolio/:kategorie` – Einstieg in Videos und Fotos des Bereichs
#[component]
pub fn CategoryOverview() -> impl IntoView {
    let site = use_site();
    let category = use_category();

    move || {
        category.get().map(|c| {
            let (video_cover, photo_cover, n_videos, n_photos) = site.with_data(|d| {
                let photos: Vec<_> = d.photos_in(c).collect();
                (
                    d.videos_in(c).next().map(|v| v.thumb()).unwrap_or_else(|| d.cover(c)),
                    photos.get(1).or(photos.first()).map(|p| p.thumb().to_string()).unwrap_or_default(),
                    d.videos_in(c).count(),
                    photos.len(),
                )
            });
            view! {
                <Title text=c.name() />
                <section class=format!("wrap grid gap-6 md:grid-cols-2 {SECTION_END}")>
                    <ProductCard
                        href=format!("{}/videos", c.href())
                        image=video_cover
                        count=count_label(n_videos, "Film", "Filme")
                        title="Videos"
                        text="Bewegte Bilder mit Ton, Rhythmus und Emotion – vom Konzept bis zum Color Grading."
                        cta="Filme ansehen →"
                    />
                    <ProductCard
                        href=format!("{}/fotos", c.href())
                        image=photo_cover
                        count=count_label(n_photos, "Bild", "Bilder")
                        title="Fotos"
                        text="Ausgewählte Bilder – zeitlos bearbeitet, mit Liebe zum Licht."
                        cta="Galerie öffnen →"
                    />
                </section>
            }
        })
    }
}

#[component]
fn EmptyState(text: &'static str) -> impl IntoView {
    view! {
        <p class=format!("rounded-card border border-dashed border-line px-6 py-10 text-center text-muted {SECTION_END}")>
            {text}
        </p>
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

/// `/portfolio/:kategorie/fotos`
#[component]
pub fn PhotosPage() -> impl IntoView {
    let site = use_site();
    let category = use_category();
    let (selected, set_selected) = signal(None::<i64>);

    let visible = Memo::new(move |_| {
        let Some(c) = category.get() else { return Vec::new() };
        site.with_data(|d| d.photos_in(c).cloned().collect::<Vec<PhotoItem>>())
    });

    // Beim Wechsel des Bereichs eine offene Lightbox schliessen
    Effect::new(move |_| {
        category.track();
        set_selected.set(None);
    });

    let step = move |dir: isize| {
        let list = visible.get_untracked();
        if let Some(cur) = selected.get_untracked() {
            if let Some(pos) = list.iter().position(|p| p.id == cur) {
                let len = list.len() as isize;
                let next = (pos as isize + dir).rem_euclid(len) as usize;
                set_selected.set(Some(list[next].id));
            }
        }
    };
    use_overlay_keys(move || set_selected.set(None), step);

    let current = move || {
        let id = selected.get()?;
        visible.with(|list| list.iter().find(|p| p.id == id).cloned())
    };

    view! {
        <Title text=move || category.get().map(|c| format!("Fotos · {}", c.name())).unwrap_or_default() />
        <section class="wrap">
            <Show
                when=move || !visible.with(Vec::is_empty)
                fallback=|| view! { <EmptyState text="In diesem Bereich gibt es noch keine Fotos." /> }
            >
                <div class=format!("columns-[3_320px] gap-4 {SECTION_END}")>
                    <For
                        each=move || visible.get()
                        key=|p| (p.id, p.title.clone(), p.image_url.clone())
                        children=move |p: PhotoItem| {
                            let id = p.id;
                            view! {
                                <button
                                    class="group paper-shadow relative mb-7 block w-full animate-reveal break-inside-avoid text-left transition-transform duration-500 ease-soft [animation-duration:0.6s] odd:-rotate-1 even:rotate-1 hover:z-10 hover:rotate-0 hover:scale-[1.02]"
                                    on:click=move |_| set_selected.set(Some(id))
                                >
                                    <span class="torn relative block overflow-hidden bg-surface">
                                    <img
                                        class="photo-bw h-auto w-full"
                                        src=p.thumb().to_string()
                                        alt=p.title.clone()
                                        loading="lazy"
                                        width=p.width
                                        height=p.height
                                    />
                                    <span class="absolute bottom-[7%] left-4 -rotate-2 border-2 border-ink bg-zine px-2 py-1 font-display text-xl leading-none font-bold uppercase opacity-0 transition-opacity duration-300 group-hover:opacity-100 group-focus-visible:opacity-100">
                                        {p.title.clone()}
                                    </span>
                                    </span>
                                </button>
                            }
                        }
                    />
                </div>
            </Show>
        </section>

        {move || {
            current()
                .map(|p| {
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
                                    src=p.image_url.clone()
                                    alt=p.title.clone()
                                />
                                <figcaption class="mt-3.5 text-muted">
                                    <strong class="text-ink">{p.title.clone()}</strong>
                                    " · "
                                    {p.category.name()}
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
    }
}

/// `/portfolio/:kategorie/videos`
#[component]
pub fn VideosPage() -> impl IntoView {
    let site = use_site();
    let category = use_category();
    let (playing, set_playing) = signal(None::<i64>);

    let visible = Memo::new(move |_| {
        let Some(c) = category.get() else { return Vec::new() };
        site.with_data(|d| d.videos_in(c).cloned().collect::<Vec<VideoItem>>())
    });

    Effect::new(move |_| {
        category.track();
        set_playing.set(None);
    });

    use_overlay_keys(move || set_playing.set(None), |_| {});

    let current = move || {
        let id = playing.get()?;
        visible.with(|list| list.iter().find(|v| v.id == id).cloned())
    };

    view! {
        <Title text=move || category.get().map(|c| format!("Videos · {}", c.name())).unwrap_or_default() />
        <section class="wrap">
            <Show
                when=move || !visible.with(Vec::is_empty)
                fallback=|| view! { <EmptyState text="In diesem Bereich gibt es noch keine Videos." /> }
            >
                <div class=format!("grid grid-cols-1 gap-x-6 gap-y-8 sm:grid-cols-[repeat(auto-fill,minmax(340px,1fr))] {SECTION_END}")>
                    <For
                        each=move || visible.get()
                        key=|v| (v.id, v.title.clone(), v.youtube_id.clone(), v.client.clone(), v.duration.clone())
                        children=move |v: VideoItem| {
                            let id = v.id;
                            view! {
                                <button
                                    class="group animate-reveal text-left [animation-duration:0.6s]"
                                    on:click=move |_| set_playing.set(Some(id))
                                >
                                    <div class="paper-shadow transition-transform duration-500 ease-soft group-hover:-rotate-1">
                                    <div class="torn-alt relative aspect-video overflow-hidden bg-surface">
                                        <img
                                            class="photo-bw size-full object-cover group-hover:scale-105"
                                            src=v.thumb()
                                            alt=v.title.clone()
                                            loading="lazy"
                                        />
                                        <span
                                            class="play-btn absolute top-1/2 left-1/2 size-16 -translate-1/2 opacity-90 transition-transform duration-400 ease-soft group-hover:scale-112"
                                            aria-hidden="true"
                                        ></span>
                                        <span class="absolute right-3 bottom-4 bg-ink px-[0.6em] py-[0.15em] text-[0.8rem] font-medium text-bg">
                                            {v.duration.clone()}
                                        </span>
                                    </div>
                                    </div>
                                    <div class="pt-3.5">
                                        <small class="text-xs font-medium text-ink/60 uppercase">
                                            {v.category.name()} " · " {v.client.clone()}
                                        </small>
                                        <h3 class="mt-1 mb-0 text-[1.9rem]">{v.title.clone()}</h3>
                                    </div>
                                </button>
                            }
                        }
                    />
                </div>
            </Show>
        </section>

        {move || {
            current()
                .map(|v| {
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
                                    title=v.title.clone()
                                    allow="autoplay; encrypted-media; picture-in-picture; fullscreen"
                                    allowfullscreen=true
                                ></iframe>
                            </div>
                        </div>
                    }
                })
        }}
    }
}
