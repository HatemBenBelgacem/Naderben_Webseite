use std::collections::HashMap;

use leptos::prelude::*;

use super::{
    guard,
    ui::{watch_action, ImageUpload, Status, StatusLine, BTN_SAVE, BTN_SMALL, CARD, INPUT, LABEL_TEXT},
};
use crate::{
    api::SaveContent,
    content::{use_site, Field, FieldKind, Site, PAGES},
    models::ContentEntry,
};

const PAGE_TAB: &str = "rounded-lg px-3 py-1.5 text-[0.9rem] transition-colors";

/// `/admin/inhalte` – Texte und Bilder pro Seite bearbeiten
#[component]
pub fn ContentEditor() -> impl IntoView {
    guard(|u| u.can_edit_content, || view! { <Editor /> }.into_any())
}

#[component]
fn Editor() -> impl IntoView {
    let site = use_site();
    let (page, set_page) = signal(0usize);
    // Noch nicht gespeicherte Änderungen: Schlüssel → neuer Wert
    let drafts = RwSignal::new(HashMap::<String, String>::new());
    let status: Status = RwSignal::new(None);
    let save = ServerAction::<SaveContent>::new();

    watch_action!(save, status, "Gespeichert – die Seite zeigt die neuen Inhalte.", move || {
        drafts.update(|d| d.clear());
        site.data.refetch();
    });

    let dirty = move || drafts.with(|d| !d.is_empty());

    let switch_page = move |i: usize| {
        if dirty() && !super::ui::confirm("Ungespeicherte Änderungen verwerfen?") {
            return;
        }
        drafts.update(|d| d.clear());
        status.set(None);
        set_page.set(i);
    };

    let on_save = move |_| {
        let entries: Vec<ContentEntry> = drafts
            .get_untracked()
            .into_iter()
            .map(|(key, value)| ContentEntry { key, value })
            .collect();
        if entries.is_empty() {
            status.set(Some(Ok("Keine Änderungen.".into())));
            return;
        }
        save.dispatch(SaveContent { entries });
    };

    view! {
        <div class="flex flex-col gap-6">
            <div class="flex flex-wrap gap-2">
                {PAGES
                    .iter()
                    .enumerate()
                    .map(|(i, p)| {
                        view! {
                            <button
                                class=move || format!(
                                    "{PAGE_TAB} {}",
                                    if page.get() == i { "bg-accent text-accent-ink" } else { "border border-line text-muted hover:text-ink" },
                                )
                                on:click=move |_| switch_page(i)
                            >
                                {p.name}
                            </button>
                        }
                    })
                    .collect_view()}
            </div>

            <div class="grid gap-4">
                {move || {
                    PAGES[page.get()]
                        .fields
                        .iter()
                        .map(|f| view! { <FieldEditor field=*f drafts site /> })
                        .collect_view()
                }}
            </div>

            // Speichern-Leiste bleibt beim Scrollen sichtbar
            <div class="sticky bottom-4 z-10 flex flex-wrap items-center gap-4 rounded-card border border-line bg-bg-2/95 p-4 backdrop-blur">
                <button class=BTN_SAVE on:click=on_save disabled=move || save.pending().get() || !dirty()>
                    {move || if save.pending().get() { "Speichert …" } else { "Änderungen speichern" }}
                </button>
                <button class=BTN_SMALL disabled=move || !dirty() on:click=move |_| drafts.update(|d| d.clear())>
                    "Verwerfen"
                </button>
                <span class="text-[0.85rem] text-muted">
                    {move || {
                        let n = drafts.with(|d| d.len());
                        if n == 0 { String::new() } else { format!("{n} ungespeicherte Änderung(en)") }
                    }}
                </span>
                <StatusLine status />
            </div>
        </div>
    }
}

#[component]
fn FieldEditor(field: Field, drafts: RwSignal<HashMap<String, String>>, site: Site) -> impl IntoView {
    let key = field.key;
    let saved = move || site.text(key);
    let value = move || drafts.with(|d| d.get(key).cloned()).unwrap_or_else(saved);
    let set_value = move |v: String| {
        drafts.update(|d| {
            if v == site.text(key) {
                d.remove(key);
            } else {
                d.insert(key.to_string(), v);
            }
        })
    };
    let changed = move || drafts.with(|d| d.contains_key(key));
    let is_default = move || value() == field.default;

    let input = match field.kind {
        FieldKind::Text => view! {
            <input class=INPUT type="text" prop:value=value on:input=move |e| set_value(event_target_value(&e)) />
        }
        .into_any(),
        FieldKind::LongText => view! {
            <textarea class=format!("{INPUT} min-h-28 resize-y") rows="4" prop:value=value on:input=move |e| set_value(event_target_value(&e))></textarea>
        }
        .into_any(),
        FieldKind::Image => view! {
            <div class="flex flex-col gap-3 sm:flex-row sm:items-start">
                <div class="flex aspect-[4/3] w-full shrink-0 items-center justify-center overflow-hidden rounded-xl border border-line bg-bg sm:w-48">
                    {move || {
                        let url = value();
                        if url.is_empty() {
                            view! { <span class="px-3 text-center text-[0.8rem] text-muted">"Kein eigenes Bild"</span> }.into_any()
                        } else {
                            view! { <img class="size-full object-cover" src=url alt="Vorschau" /> }.into_any()
                        }
                    }}
                </div>
                <div class="flex w-full flex-col gap-2">
                    <input class=INPUT type="url" placeholder="https://… oder Bild hochladen" prop:value=value
                        on:input=move |e| set_value(event_target_value(&e)) />
                    <ImageUpload on_uploaded=move |info: crate::models::MediaInfo| set_value(info.url) />
                </div>
            </div>
        }
        .into_any(),
    };

    view! {
        <div class=move || format!("{CARD} flex flex-col gap-2 {}", if changed() { "border-accent/60" } else { "" })>
            <div class="flex items-center justify-between gap-3">
                <span class=LABEL_TEXT>
                    {field.label}
                    {move || changed().then_some(view! { <span class="ml-2 text-accent">"● geändert"</span> })}
                </span>
                <button
                    class="text-[0.8rem] text-muted underline-offset-4 hover:text-ink hover:underline disabled:invisible"
                    disabled=is_default
                    title="Standardtext wiederherstellen"
                    on:click=move |_| set_value(field.default.to_string())
                >
                    "Standard"
                </button>
            </div>
            {input}
        </div>
    }
}
