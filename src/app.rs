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
                    href="https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600&family=Roboto+Condensed:wght@300;400;700;800&display=swap"
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

const NAV_LINK: &str = "relative font-medium uppercase text-ink/70 transition-colors hover:text-ink aria-[current=page]:text-ink md:after:absolute md:after:inset-x-0 md:after:-bottom-1.5 md:after:h-[3px] md:after:origin-left md:after:scale-x-0 md:after:bg-accent md:after:transition-transform md:after:duration-350 md:after:ease-soft md:hover:after:scale-x-100 md:aria-[current=page]:after:scale-x-100";

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
            class="group fixed inset-x-0 top-0 z-50 h-[76px] border-b-2 border-ink bg-bg/90 backdrop-blur-[10px]"
            class:menu-open=open
        >
            <div class="wrap flex h-full items-center justify-between">
                <A href="/" attr:class="font-display text-[1.6rem] leading-none font-extrabold tracking-[-0.02em] uppercase">
                    "NADER"
                    <span class="ml-0.5 bg-accent px-1">"BEN"</span>
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
                        attr:class="border-2 border-ink bg-accent px-[1.1em] py-[0.45em] font-body text-base font-medium text-accent-ink uppercase transition-colors hover:bg-ink hover:text-bg aria-[current=page]:bg-ink aria-[current=page]:text-bg md:text-[0.9rem]"
                    >
                        "Contact"
                    </A>
                </nav>
            </div>
        </header>
    }
}

const FOOTER_LINK: &str = "text-white/70 transition-colors hover:text-bg";
const DIALOG_INPUT: &str = "w-full rounded-xl border border-line bg-white px-3.5 py-2.5 text-ink placeholder:text-black/35 transition focus:border-accent focus:ring-[3px] focus:ring-accent/20 focus:outline-none";

#[component]
fn Footer() -> impl IntoView {
    let site = use_site();

    view! {
        <footer class="bg-ink pt-16 text-white">
            <div class="wrap grid gap-8 sm:grid-cols-2 md:grid-cols-[2fr_1fr_1fr_1fr]">
                <div class="sm:col-span-2 md:col-span-1">
                    <p class="mb-3 font-display text-[clamp(3rem,7vw,5.5rem)] leading-[0.8] font-extrabold tracking-[-0.03em] text-bg uppercase">
                        "NADER" <br /> <span class="text-accent">"BEN"</span>
                    </p>
                    <p class="max-w-[34ch] text-white/60">{move || site.text("footer.claim")}</p>
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
                <FooterAccount />
            </div>
            <div class="wrap mt-14 border-t border-white/15 py-6 text-[0.8rem] text-white/50 uppercase">
                "© 2026 Nader Ben. Alle Rechte vorbehalten."
            </div>
        </footer>
    }
}

/// Eigene Footer-Spalte: „Anmelden“ öffnet den Login-Dialog, angemeldet folgen Konto-Links
#[component]
fn FooterAccount() -> impl IntoView {
    let site = use_site();
    let open = RwSignal::new(false);
    let login = ServerAction::<Login>::new();
    let logout = ServerAction::<Logout>::new();
    let navigate = use_navigate();
    let location = use_location();

    // Nach erfolgreicher Anmeldung: Dialog schliessen, Benutzer neu laden
    Effect::new(move |_| {
        if let Some(Ok(_)) = login.value().get() {
            open.set(false);
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

    let open_dialog = move |_| {
        login.value().set(None);
        open.set(true);
    };

    view! {
        <div class="flex flex-col gap-2">
            {move || match site.current_user() {
                Some(user) => view! {
                    <span class="text-[0.85rem] text-white/50">
                        "Angemeldet als " <strong class="font-medium text-white">{user.name.clone()}</strong>
                    </span>
                    {if user.has_admin_access() {
                        view! { <A href="/admin" attr:class=FOOTER_LINK>"Verwaltung"</A> }.into_any()
                    } else {
                        view! { <A href="/admin/konto" attr:class=FOOTER_LINK>"Mein Konto"</A> }.into_any()
                    }}
                    <button class=format!("{FOOTER_LINK} text-left") on:click=move |_| { logout.dispatch(Logout {}); }>
                        "Abmelden"
                    </button>
                }
                .into_any(),
                None => view! {
                    <button class=format!("{FOOTER_LINK} text-left") aria-haspopup="dialog" on:click=open_dialog>
                        "Anmelden"
                    </button>
                }
                .into_any(),
            }}
        </div>
        <LoginDialog open login />
    }
}

/// Anmeldemaske als eigenes Fenster über der Seite (schliessen mit ✕, Esc oder Klick daneben)
#[component]
fn LoginDialog(open: RwSignal<bool>, login: ServerAction<Login>) -> impl IntoView {
    let email = NodeRef::<leptos::html::Input>::new();

    let handle = window_event_listener(leptos::ev::keydown, move |e| {
        if e.key() == "Escape" && open.get_untracked() {
            open.set(false);
        }
    });
    on_cleanup(move || handle.remove());

    // Beim Öffnen: E-Mail-Feld fokussieren und Seite dahinter nicht scrollen lassen
    Effect::new(move |_| {
        let is_open = open.get();
        if let Some(body) = document().body() {
            let _ = body.class_list().toggle_with_force("overflow-hidden", is_open);
        }
        if is_open {
            request_animation_frame(move || {
                if let Some(input) = email.get_untracked() {
                    let _ = input.focus();
                }
            });
        }
    });

    let error = move || match login.value().get() {
        Some(Err(ServerFnError::ServerError(msg))) => Some(msg),
        Some(Err(e)) => Some(e.to_string()),
        _ => None,
    };

    view! {
        <Show when=move || open.get()>
            <div
                class="fixed inset-0 z-[100] flex animate-fade items-center justify-center bg-[rgb(5_5_6/0.82)] px-4 backdrop-blur-sm"
                on:click=move |_| open.set(false)
            >
                <div
                    role="dialog"
                    aria-modal="true"
                    aria-labelledby="login-title"
                    class="relative w-full max-w-sm animate-zoom-in rounded-card border border-line bg-bg-2 p-7 shadow-2xl shadow-black/60"
                    on:click=|e| e.stop_propagation()
                >
                    <button
                        class="absolute top-4 right-4 size-9 rounded-full text-muted transition-colors hover:bg-white/10 hover:text-ink"
                        aria-label="Schliessen"
                        on:click=move |_| open.set(false)
                    >
                        "✕"
                    </button>
                    <p class="eyebrow">"Verwaltung"</p>
                    <h2 id="login-title" class="mb-6 text-[1.9rem]">"Anmelden"</h2>
                    <ActionForm action=login attr:class="flex flex-col gap-4">
                        <label class="flex flex-col gap-1.5">
                            <span class="text-[0.85rem] text-muted">"E-Mail"</span>
                            <input node_ref=email class=DIALOG_INPUT type="email" name="email" required autocomplete="username" />
                        </label>
                        <label class="flex flex-col gap-1.5">
                            <span class="text-[0.85rem] text-muted">"Passwort"</span>
                            <input class=DIALOG_INPUT type="password" name="password" required autocomplete="current-password" />
                        </label>
                        {move || error().map(|msg| view! {
                            <p class="mb-0 rounded-xl bg-danger/10 px-3.5 py-2.5 text-[0.9rem] text-danger" role="alert">{msg}</p>
                        })}
                        <button type="submit" class="btn btn-primary mt-1 justify-center" disabled=move || login.pending().get()>
                            {move || if login.pending().get() { "Wird geprüft …" } else { "Anmelden" }}
                        </button>
                    </ActionForm>
                </div>
            </div>
        </Show>
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
