use leptos::prelude::*;

use super::{
    guard,
    ui::{confirm, err_text, watch_action, ImageUpload, Status, StatusLine, BTN_DANGER, BTN_SAVE, CARD, INPUT, LABEL, LABEL_TEXT},
};
use crate::{
    api::{DeleteProduct, ListProducts, SaveProduct},
    models::{format_price, parse_price, MediaInfo, Product},
};

/// `/admin/shop` – Produkte, Preise und Lagerbestand
#[component]
pub fn ShopEditor() -> impl IntoView {
    guard(|u| u.can_edit_content, || view! { <Editor /> }.into_any())
}

#[component]
fn Editor() -> impl IntoView {
    let status: Status = RwSignal::new(None);
    // Liste erst im Browser laden (nach dem Aufbau der Seite). Eine Action, keine Resource:
    // so wartet das Server-Rendering der Verwaltung nicht auf die Liste.
    let products = ServerAction::<ListProducts>::new();
    let reload = Callback::new(move |_: ()| {
        products.dispatch(ListProducts {});
    });
    Effect::new(move |_| {
        products.dispatch(ListProducts {});
    });

    view! {
        <div class="flex flex-col gap-8">
            <div class="sticky top-[90px] z-10 empty:hidden">
                <StatusLine status=status />
            </div>

            <section class="flex flex-col gap-4">
                <h3 class="mb-0">"Neues Produkt"</h3>
                <ProductForm product=None status on_change=reload />
            </section>

            <section class="flex flex-col gap-4">
                <h3 class="mb-0">"Produkte"</h3>
                {move || match products.value().get() {
                    None => view! { <p class="text-muted">"Lädt …"</p> }.into_any(),
                    Some(Err(e)) => view! { <p class="text-danger">{err_text(e)}</p> }.into_any(),
                    Some(Ok(list)) if list.is_empty() => view! { <p class="text-muted">"Noch keine Produkte."</p> }.into_any(),
                    Some(Ok(list)) => view! {
                        <div class="flex flex-col gap-4">
                            {list
                                .into_iter()
                                .map(|p| view! { <ProductForm product=Some(p) status on_change=reload /> })
                                .collect_view()}
                        </div>
                    }
                    .into_any(),
                }}
            </section>
        </div>
    }
}

/// Neues Produkt (product = None) oder bestehendes bearbeiten
#[component]
fn ProductForm(product: Option<Product>, status: Status, on_change: Callback<()>) -> impl IntoView {
    let id = product.as_ref().map(|p| p.id);
    let is_new = id.is_none();
    let p = product.unwrap_or_default();

    let name = RwSignal::new(p.name.clone());
    let description = RwSignal::new(p.description.clone());
    let price = RwSignal::new(if is_new { String::new() } else { format_price(p.price_cents) });
    let stock = RwSignal::new(if is_new { String::new() } else { p.stock.to_string() });
    let images = RwSignal::new(p.image_urls.clone());

    let save = ServerAction::<SaveProduct>::new();
    let delete = ServerAction::<DeleteProduct>::new();
    let reload = move || on_change.run(());
    watch_action!(save, status, "Produkt gespeichert.", reload);
    watch_action!(delete, status, "Produkt gelöscht.", reload);

    // Neues Produkt: Formular nach dem Speichern leeren
    if is_new {
        Effect::new(move |_| {
            if let Some(Ok(())) = save.value().get() {
                name.set(String::new());
                description.set(String::new());
                price.set(String::new());
                stock.set(String::new());
                images.set(Vec::new());
            }
        });
    }

    let on_save = move |_| {
        let Some(price_cents) = parse_price(&price.get_untracked()) else {
            status.set(Some(Err("Bitte einen gültigen Preis angeben, zum Beispiel 25.50.".to_string())));
            return;
        };
        let Ok(stock_value) = stock.get_untracked().trim().parse::<i64>() else {
            status.set(Some(Err("Der Lagerbestand muss eine ganze Zahl sein.".to_string())));
            return;
        };
        save.dispatch(SaveProduct {
            id,
            name: name.get_untracked(),
            description: description.get_untracked(),
            price_cents,
            stock: stock_value,
            image_urls: images.get_untracked(),
        });
    };

    let on_delete = move |_| {
        if let Some(id) = id {
            if confirm("Dieses Produkt wirklich löschen? Es verschwindet auch aus allen Warenkörben.") {
                delete.dispatch(DeleteProduct { id });
            }
        }
    };

    view! {
        <div class=format!("{CARD} flex flex-col gap-4 {}", if is_new { "border-dashed" } else { "" })>
            <div class="grid gap-3 sm:grid-cols-2">
                <label class=format!("{LABEL} sm:col-span-2")>
                    <span class=LABEL_TEXT>"Name"</span>
                    <input class=INPUT type="text" placeholder="z. B. Fotoabzug A4" prop:value=name on:input=move |e| name.set(event_target_value(&e)) />
                </label>
                <label class=format!("{LABEL} sm:col-span-2")>
                    <span class=LABEL_TEXT>"Beschreibung"</span>
                    <textarea class=INPUT rows="2" prop:value=description on:input=move |e| description.set(event_target_value(&e))></textarea>
                </label>
                <label class=LABEL>
                    <span class=LABEL_TEXT>"Preis in Franken"</span>
                    <input class=INPUT type="text" inputmode="decimal" placeholder="25.00" prop:value=price on:input=move |e| price.set(event_target_value(&e)) />
                </label>
                <label class=LABEL>
                    <span class=LABEL_TEXT>"Lagerbestand"</span>
                    <input class=INPUT type="number" min="0" step="1" prop:value=stock on:input=move |e| stock.set(event_target_value(&e)) />
                </label>
                <div class="flex flex-col gap-3 sm:col-span-2">
                    <span class=LABEL_TEXT>"Bilder"</span>
                    <div class="flex flex-wrap gap-3">
                        {move || {
                            images
                                .get()
                                .into_iter()
                                .enumerate()
                                .map(|(i, url)| {
                                    view! {
                                        <div class="flex flex-col gap-2">
                                            <img class="size-24 rounded-xl border border-line object-cover" src=url alt="Produktbild" loading="lazy" />
                                            <button
                                                class=BTN_DANGER
                                                type="button"
                                                on:click=move |_| images.update(|v| {
                                                    v.remove(i);
                                                })
                                            >
                                                "Entfernen"
                                            </button>
                                        </div>
                                    }
                                })
                                .collect_view()
                        }}
                    </div>
                    {move || {
                        images.get().is_empty().then(|| view! { <p class="mb-0 text-[0.9rem] text-muted">"Noch keine Bilder."</p> })
                    }}
                    <ImageUpload
                        on_uploaded=move |info: MediaInfo| images.update(|v| v.push(info.url))
                        label="Bild hinzufügen"
                    />
                </div>
                <div class="flex flex-wrap items-end gap-2 sm:col-span-2">
                    <button class=BTN_SAVE on:click=on_save disabled=move || save.pending().get()>
                        {if is_new { "Produkt hinzufügen" } else { "Speichern" }}
                    </button>
                    {(!is_new).then(|| view! {
                        <button class=format!("{BTN_DANGER} ml-auto") on:click=on_delete>"Löschen"</button>
                    })}
                </div>
            </div>
        </div>
    }
}
