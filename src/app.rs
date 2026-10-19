use leptos::prelude::*;
use leptos_meta::{provide_meta_context, Link, Meta, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{ParentRoute, Route, Router, Routes, A},
    hooks::{use_location, use_navigate},
    path, SsrMode,
};

use crate::{
    api::{get_site_data, current_user, Login, Logout},
    content::{use_site, Site},
    data::Category,
    pages::{
        about::AboutPage,
        admin::{AccountPage, AdminHome, AdminLayout, ContentEditor, PortfolioEditor, RolesAdmin, UsersAdmin},
        contact::ContactPage,
        home::HomePage,
        portfolio::{CategoryOverview, CategoryPage, PhotosPage, PortfolioOverview, PortfolioPage, VideosPage},
    },
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

    // Inhalte, Portfolio und angemeldeter Benutzer – einmal laden, überall per use_site()
    provide_context(Site {
        data: Resource::new(|| (), |_| get_site_data()),
        user: Resource::new(|| (), |_| current_user()),
    });

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
                // SsrMode::Async: HTML erst senden, wenn die Inhalte aus der DB da sind (SEO)
                <Routes fallback=NotFound>
                    <Route path=path!("") view=HomePage ssr=SsrMode::Async />
                    <Route path=path!("about") view=AboutPage ssr=SsrMode::Async />
                    <ParentRoute path=path!("portfolio") view=PortfolioPage ssr=SsrMode::Async>
                        <Route path=path!("") view=PortfolioOverview />
                        <ParentRoute path=path!(":kategorie") view=CategoryPage>
                            <Route path=path!("") view=CategoryOverview />
                            <Route path=path!("videos") view=VideosPage />
                            <Route path=path!("fotos") view=PhotosPage />
                        </ParentRoute>
                    </ParentRoute>
                    <Route path=path!("contact") view=ContactPage ssr=SsrMode::Async />
                    <ParentRoute path=path!("admin") view=AdminLayout ssr=SsrMode::Async>
                        <Route path=path!("") view=AdminHome />
                        <Route path=path!("inhalte") view=ContentEditor />
                        <Route path=path!("portfolio") view=PortfolioEditor />
                        <Route path=path!("benutzer") view=UsersAdmin />
                        <Route path=path!("rollen") view=RolesAdmin />
                        <Route path=path!("konto") view=AccountPage />
                    </ParentRoute>
                </Routes>
            </main>
            <Transition fallback=|| ()>
                <Footer />
            </Transition>
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
const FOOTER_INPUT: &str = "w-full rounded-lg border border-line bg-bg-2 px-3 py-2 text-[0.9rem] text-ink placeholder:text-[#5f5c57] focus:border-accent focus:outline-none sm:w-52";

#[component]
fn Footer() -> impl IntoView {
    let site = use_site();

    view! {
        <footer class="border-t border-line bg-bg pt-14">
            <div class="wrap grid gap-8 md:grid-cols-[2fr_1fr_1fr]">
                <div>
                    <p class="mb-2.5 font-display text-xl font-extrabold tracking-[0.08em]">
                        "NADER" <span class="text-accent">"BEN"</span>
                    </p>
                    <p class="text-muted">{move || site.text("footer.claim")}</p>
                </div>
                <nav class="flex flex-col gap-2">
                    <a class=FOOTER_LINK href="/about">"About"</a>
                    <a class=FOOTER_LINK href="/portfolio">"Portfolio"</a>
                    {Category::ALL
                        .map(|c| view! { <a class=format!("{FOOTER_LINK} pl-3") href=c.href()>{c.name()}</a> })
                        .collect_view()}
                    <a class=FOOTER_LINK href="/contact">"Contact"</a>
                </nav>
                <div class="flex flex-col gap-2">
                    <a class=FOOTER_LINK href=move || site.text("social.instagram") target="_blank" rel="noopener">"Instagram"</a>
                    <a class=FOOTER_LINK href=move || site.text("social.vimeo") target="_blank" rel="noopener">"Vimeo"</a>
                    <a class=FOOTER_LINK href=move || site.text("social.youtube") target="_blank" rel="noopener">"YouTube"</a>
                </div>
            </div>
            <div class="wrap mt-12 flex flex-col gap-4 border-t border-line py-6 text-[0.85rem] text-muted md:flex-row md:items-center md:justify-between">
                <span>"© 2026 Nader Ben. Alle Rechte vorbehalten."</span>
                <FooterLogin />
            </div>
        </footer>
    }
}

/// Anmeldung bzw. Konto-Links in der Fusszeile
#[component]
fn FooterLogin() -> impl IntoView {
    let site = use_site();
    let (open, set_open) = signal(false);
    let login = ServerAction::<Login>::new();
    let logout = ServerAction::<Logout>::new();
    let navigate = use_navigate();
    let location = use_location();

    // Nach erfolgreicher Anmeldung: Benutzer neu laden, Formular schliessen
    Effect::new(move |_| {
        if let Some(Ok(_)) = login.value().get() {
            set_open.set(false);
            site.user.refetch();
        }
    });
    // Nach dem Abmelden: Benutzer neu laden und die Verwaltung verlassen
    Effect::new(move |_| {
        if let Some(Ok(())) = logout.value().get() {
            site.user.refetch();
            if location.pathname.get_untracked().starts_with("/admin") {
                navigate("/", Default::default());
            }
        }
    });

    let error = move || match login.value().get() {
        Some(Err(ServerFnError::ServerError(msg))) => Some(msg),
        Some(Err(e)) => Some(e.to_string()),
        _ => None,
    };

    move || match site.current_user() {
        Some(user) => view! {
            <div class="flex flex-wrap items-center gap-x-4 gap-y-2">
                <span>
                    "Angemeldet als " <strong class="text-ink">{user.name.clone()}</strong>
                    " · " {user.role.clone()}
                </span>
                {if user.has_admin_access() {
                    view! { <A href="/admin" attr:class="text-accent hover:underline">"Verwaltung"</A> }.into_any()
                } else {
                    view! { <A href="/admin/konto" attr:class="text-accent hover:underline">"Mein Konto"</A> }.into_any()
                }}
                <button class="underline-offset-4 hover:text-ink hover:underline" on:click=move |_| { logout.dispatch(Logout {}); }>
                    "Abmelden"
                </button>
            </div>
        }
        .into_any(),
        None => view! {
            <div class="flex flex-col items-start gap-3 md:items-end">
                <Show
                    when=move || open.get()
                    fallback=move || view! {
                        <button class="underline-offset-4 hover:text-ink hover:underline" on:click=move |_| set_open.set(true)>
                            "Anmelden"
                        </button>
                    }
                >
                    <ActionForm action=login attr:class="flex w-full flex-col gap-2 sm:w-auto sm:flex-row sm:items-center">
                        <input class=FOOTER_INPUT type="email" name="email" required autocomplete="username" placeholder="E-Mail" aria-label="E-Mail" />
                        <input class=FOOTER_INPUT type="password" name="password" required autocomplete="current-password" placeholder="Passwort" aria-label="Passwort" />
                        <div class="flex gap-2">
                            <button type="submit" class="btn btn-primary px-4 py-2" disabled=move || login.pending().get()>
                                {move || if login.pending().get() { "…" } else { "Anmelden" }}
                            </button>
                            <button type="button" class="px-2 hover:text-ink" aria-label="Abbrechen" on:click=move |_| set_open.set(false)>
                                "✕"
                            </button>
                        </div>
                    </ActionForm>
                    {move || error().map(|msg| view! { <p class="mb-0 text-[#f4a49b]" role="alert">{msg}</p> })}
                </Show>
            </div>
        }
        .into_any(),
    }
}

#[component]
pub fn NotFound() -> impl IntoView {
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
