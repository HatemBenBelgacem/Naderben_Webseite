use leptos::prelude::*;
use leptos_meta::{provide_meta_context, Link, Meta, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{ParentRoute, Route, Router, Routes, A},
    hooks::use_location,
    path,
};

use crate::pages::{
    about::AboutPage,
    contact::ContactPage,
    home::HomePage,
    portfolio::{PhotosPage, PortfolioOverview, PortfolioPage, VideosPage},
};

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="de">
            <head>
                <meta charset="utf-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1" />
                <link rel="preconnect" href="https://fonts.googleapis.com" />
                <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin="" />
                <link
                    rel="stylesheet"
                    href="https://fonts.googleapis.com/css2?family=Inter:wght@300;400;500;600&family=Syne:wght@500;600;700;800&display=swap"
                />
                <AutoReload options=options.clone() />
                <HydrationScripts options />
                <MetaTags />
            </head>
            <body>
                <App />
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/naderben.css" />
        <Link rel="icon" type_="image/svg+xml" href="/favicon.svg" />
        <Meta
            name="description"
            content="Nader Ben – Fotograf & Videograf. Hochzeiten, Portraits, Imagefilme und Events."
        />
        <Title formatter=|t: String| {
            if t.is_empty() { "Nader Ben · Foto & Video".to_string() } else { format!("{t} · Nader Ben") }
        } />

        <Router>
            <Header />
            <main class="min-h-[70vh]">
                <Routes fallback=NotFound>
                    <Route path=path!("") view=HomePage />
                    <Route path=path!("about") view=AboutPage />
                    <ParentRoute path=path!("portfolio") view=PortfolioPage>
                        <Route path=path!("") view=PortfolioOverview />
                        <Route path=path!("videos") view=VideosPage />
                        <Route path=path!("fotos") view=PhotosPage />
                    </ParentRoute>
                    <Route path=path!("contact") view=ContactPage />
                </Routes>
            </main>
            <Footer />
        </Router>
    }
}

const NAV_LINK: &str = "relative text-muted transition-colors hover:text-ink aria-[current=page]:text-ink md:after:absolute md:after:inset-x-0 md:after:-bottom-1.5 md:after:h-px md:after:origin-left md:after:scale-x-0 md:after:bg-accent md:after:transition-transform md:after:duration-350 md:after:ease-soft md:hover:after:scale-x-100 md:aria-[current=page]:after:scale-x-100";

#[component]
fn Header() -> impl IntoView {
    let (open, set_open) = signal(false);
    let location = use_location();

    // Mobile-Menü bei Navigation schliessen
    Effect::new(move |_| {
        location.pathname.track();
        set_open.set(false);
    });

    view! {
        <header
            class="group fixed inset-x-0 top-0 z-50 h-[76px] border-b border-line bg-linear-to-b from-bg/85 to-bg/55 backdrop-blur-[14px]"
            class:menu-open=open
        >
            <div class="wrap flex h-full items-center justify-between">
                <A href="/" attr:class="font-display text-xl font-extrabold tracking-[0.08em]">
                    "NADER"
                    <span class="text-accent">"BEN"</span>
                </A>
                <button
                    class="relative z-10 size-10 md:hidden"
                    aria-label="Menü öffnen"
                    aria-expanded=move || open.get().to_string()
                    on:click=move |_| set_open.update(|o| *o = !*o)
                >
                    <span class="absolute inset-x-[9px] top-[15px] h-0.5 bg-ink transition-all duration-300 ease-soft group-[.menu-open]:top-[19px] group-[.menu-open]:rotate-45"></span>
                    <span class="absolute inset-x-[9px] top-[23px] h-0.5 bg-ink transition-all duration-300 ease-soft group-[.menu-open]:top-[19px] group-[.menu-open]:-rotate-45"></span>
                </button>
                <nav class="pointer-events-none fixed inset-x-0 top-[76px] flex -translate-y-[120%] flex-col items-start gap-[1.4rem] border-b border-line bg-bg px-6 pt-8 pb-10 font-display text-[1.8rem] opacity-0 transition-all duration-450 ease-soft group-[.menu-open]:pointer-events-auto group-[.menu-open]:translate-y-0 group-[.menu-open]:opacity-100 md:pointer-events-auto md:static md:translate-y-0 md:flex-row md:items-center md:gap-[2.2rem] md:border-0 md:bg-transparent md:p-0 md:font-body md:text-[0.95rem] md:opacity-100">
                    <A href="/about" attr:class=NAV_LINK>"About"</A>
                    <A href="/portfolio" attr:class=NAV_LINK>"Portfolio"</A>
                    <A
                        href="/contact"
                        attr:class="rounded-full bg-accent px-[1.2em] py-[0.55em] font-body text-base text-accent-ink transition-colors hover:bg-accent-hover aria-[current=page]:bg-accent-hover md:text-[0.95rem]"
                    >
                        "Contact"
                    </A>
                </nav>
            </div>
        </header>
    }
}

const FOOTER_LINK: &str = "text-muted transition-colors hover:text-accent";

#[component]
fn Footer() -> impl IntoView {
    view! {
        <footer class="border-t border-line bg-bg pt-14">
            <div class="wrap grid gap-8 md:grid-cols-[2fr_1fr_1fr]">
                <div>
                    <p class="mb-2.5 font-display text-xl font-extrabold tracking-[0.08em]">
                        "NADER" <span class="text-accent">"BEN"</span>
                    </p>
                    <p class="text-muted">"Foto & Video aus Zürich – für Menschen, Marken und Momente."</p>
                </div>
                <nav class="flex flex-col gap-2">
                    <a class=FOOTER_LINK href="/about">"About"</a>
                    <a class=FOOTER_LINK href="/portfolio/fotos">"Fotos"</a>
                    <a class=FOOTER_LINK href="/portfolio/videos">"Videos"</a>
                    <a class=FOOTER_LINK href="/contact">"Contact"</a>
                </nav>
                <div class="flex flex-col gap-2">
                    <a class=FOOTER_LINK href="https://instagram.com" target="_blank" rel="noopener">"Instagram"</a>
                    <a class=FOOTER_LINK href="https://vimeo.com" target="_blank" rel="noopener">"Vimeo"</a>
                    <a class=FOOTER_LINK href="https://youtube.com" target="_blank" rel="noopener">"YouTube"</a>
                </div>
            </div>
            <div class="wrap mt-12 border-t border-line py-6 text-[0.85rem] text-muted">
                "© 2026 Nader Ben. Alle Rechte vorbehalten."
            </div>
        </footer>
    }
}

#[component]
fn NotFound() -> impl IntoView {
    #[cfg(feature = "ssr")]
    {
        if let Some(resp) = use_context::<leptos_axum::ResponseOptions>() {
            resp.set_status(axum::http::StatusCode::NOT_FOUND);
        }
    }

    view! {
        <Title text="404" />
        <section class="wrap page-hero min-h-[70vh]">
            <p class="eyebrow">"404"</p>
            <h1 class="max-w-[18ch]">"Dieses Bild ist nicht im Kasten."</h1>
            <p class="lead">"Die Seite existiert nicht – aber im Portfolio gibt es genug zu sehen."</p>
            <A href="/portfolio" attr:class="btn btn-primary">"Zum Portfolio"</A>
        </section>
    }
}
