//! Verwaltung unter `/admin`: Inhalte, Portfolio, Benutzer, Rollen und eigenes Konto.

mod account;
mod content_editor;
mod portfolio_editor;
mod roles;
pub mod ui;
mod users;

pub use account::AccountPage;
pub use content_editor::ContentEditor;
pub use portfolio_editor::PortfolioEditor;
pub use roles::RolesAdmin;
pub use users::UsersAdmin;

use leptos::prelude::*;
use leptos_meta::{Meta, Title};
use leptos_router::components::{Outlet, A};

use crate::{content::use_site, models::SessionUser};

const TAB: &str = "shrink-0 rounded-full px-[1.1em] py-[0.5em] text-[0.9rem] whitespace-nowrap text-muted transition-colors hover:text-ink aria-[current=page]:bg-ink aria-[current=page]:text-bg";

struct Section {
    href: &'static str,
    name: &'static str,
    text: &'static str,
    allowed: fn(&SessionUser) -> bool,
}

const SECTIONS: [Section; 5] = [
    Section { href: "/admin/inhalte", name: "Inhalte", text: "Texte und Bilder der Seiten bearbeiten.", allowed: |u| u.can_edit_content },
    Section { href: "/admin/portfolio", name: "Portfolio", text: "Fotos und Videos der vier Bereiche verwalten.", allowed: |u| u.can_edit_content },
    Section { href: "/admin/benutzer", name: "Benutzer", text: "Konten anlegen, Rollen zuweisen, sperren.", allowed: |u| u.can_manage_users },
    Section { href: "/admin/rollen", name: "Rollen", text: "Rollen und ihre Berechtigungen festlegen.", allowed: |u| u.can_manage_roles },
    Section { href: "/admin/konto", name: "Mein Konto", text: "Eigenes Passwort ändern.", allowed: |_| true },
];

/// Rahmen der Verwaltung – nur für angemeldete Benutzer
#[component]
pub fn AdminLayout() -> impl IntoView {
    let site = use_site();

    view! {
        <Transition fallback=|| ()>
        <Title text="Verwaltung" />
        <Meta name="robots" content="noindex, nofollow" />
        {move || match site.current_user() {
            None => view! {
                <section class="wrap page-hero min-h-[60vh]">
                    <p class="eyebrow">"Verwaltung"</p>
                    <h1 class="max-w-[18ch]">"Bitte anmelden"</h1>
                    <p class="lead">"Die Anmeldung findest du ganz unten in der Fusszeile."</p>
                </section>
            }
            .into_any(),
            Some(user) => view! {
                <section class="wrap page-hero pb-6">
                    <p class="eyebrow">"Verwaltung"</p>
                    <h1 class="mb-2 max-w-[18ch] text-[clamp(2.2rem,5vw,3.6rem)]">{format!("Hallo {}", user.name)}</h1>
                    <p class="text-muted">{format!("Rolle: {}", user.role)}</p>
                    <nav class="mt-4 inline-flex max-w-full gap-1 overflow-x-auto rounded-full border border-line bg-bg-2 p-[5px] [scrollbar-width:none]">
                        <A href="/admin" exact=true attr:class=TAB>"Übersicht"</A>
                        {SECTIONS
                            .iter()
                            .filter(|s| (s.allowed)(&user))
                            .map(|s| view! { <A href=s.href attr:class=TAB>{s.name}</A> })
                            .collect_view()}
                    </nav>
                </section>
                <div class="wrap pb-[clamp(64px,10vw,120px)]">
                    <Outlet />
                </div>
            }
            .into_any(),
        }}
        </Transition>
    }
}

#[component]
pub fn AdminHome() -> impl IntoView {
    let site = use_site();
    move || {
        site.current_user().map(|user| {
            view! {
                <div class="grid gap-4 sm:grid-cols-2 lg:grid-cols-3">
                    {SECTIONS
                        .iter()
                        .filter(|s| (s.allowed)(&user))
                        .map(|s| {
                            view! {
                                <A href=s.href attr:class="rounded-card border border-line bg-bg-2 p-6 transition-colors hover:border-accent/50">
                                    <h3 class="mb-1">{s.name}</h3>
                                    <p class="mb-0 text-[0.95rem] text-muted">{s.text}</p>
                                </A>
                            }
                        })
                        .collect_view()}
                </div>
            }
        })
    }
}

/// Hinweis, wenn eine Unterseite ohne passendes Recht aufgerufen wird
#[component]
pub fn NoPermission() -> impl IntoView {
    view! { <p class="text-muted">"Für diesen Bereich fehlt dir die Berechtigung."</p> }
}

/// Zeigt `children` nur, wenn der angemeldete Benutzer das Recht hat
pub fn guard(check: fn(&SessionUser) -> bool, view: impl Fn() -> AnyView + Send + Sync + 'static) -> impl IntoView {
    let site = use_site();
    move || match site.current_user() {
        Some(u) if check(&u) => view(),
        _ => view! { <NoPermission /> }.into_any(),
    }
}
