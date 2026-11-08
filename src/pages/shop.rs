//! Shop für Besucher: Produktliste, Warenkorb-Seite und der Warenkorb-Stand im Menü.

use leptos::prelude::*;
use leptos_meta::Title;
use leptos_router::components::A;

use crate::{
    api::{get_cart, get_shop_products, AddToCart, PlaceOrder, RemoveFromCart, UpdateCartItem},
    models::{format_price, Cart, CartLine, OrderConfirmation, Product},
    pages::admin::ui::{watch_action, Status, StatusLine, BTN_DANGER, BTN_SMALL, INPUT, LABEL, LABEL_TEXT},
};

/// Produkte und Warenkorb – einmal geladen, überall per `use_shop()` verfügbar
#[derive(Clone, Copy)]
pub struct Shop {
    products: Resource<Result<Vec<Product>, ServerFnError>>,
    cart_loaded: Resource<Result<Cart, ServerFnError>>,
    /// Stand nach der letzten Warenkorb-Aktion; ersetzt den geladenen Stand
    cart_latest: RwSignal<Option<Cart>>,
}

/// In `App` aufrufen, bevor die Seiten gerendert werden
pub fn provide_shop() {
    provide_context(Shop {
        products: Resource::new(|| (), |_| get_shop_products()),
        cart_loaded: Resource::new(|| (), |_| get_cart()),
        cart_latest: RwSignal::new(None),
    });
}

pub fn use_shop() -> Shop {
    expect_context::<Shop>()
}

impl Shop {
    /// Aktueller Warenkorb. `None`, solange er noch lädt. Reaktiv.
    pub fn cart(&self) -> Option<Cart> {
        self.cart_latest.get().or_else(|| self.cart_loaded.get().map(|r| r.unwrap_or_default()))
    }

    fn set_cart(&self, cart: Cart) {
        self.cart_latest.set(Some(cart));
    }
}

/// `/shop` – alle Produkte mit „In den Warenkorb“
#[component]
pub fn ShopPage() -> impl IntoView {
    let shop = use_shop();
    let status: Status = RwSignal::new(None);
    let add = ServerAction::<AddToCart>::new();
    Effect::new(move |_| {
        if let Some(Ok(cart)) = add.value().get() {
            shop.set_cart(cart);
        }
    });
    watch_action!(add, status, "In den Warenkorb gelegt.", || {});

    view! {
        <Transition fallback=|| ()>
        <Title text="Shop" />
        <section class="wrap page-hero">
            <p class="eyebrow">"Shop"</p>
            <h1 class="max-w-[18ch]">"Produkte"</h1>
            <p class="lead">"Wähle ein Produkt und lege es in deinen Warenkorb."</p>
        </section>
        <section class="wrap flex flex-col gap-6 pb-[clamp(64px,10vw,120px)]">
            <div class="sticky top-[90px] z-10 empty:hidden">
                <StatusLine status=status />
            </div>
            {move || match shop.products.get() {
                None => view! { <p class="text-muted">"Lädt …"</p> }.into_any(),
                Some(Err(_)) => view! { <p class="text-danger">"Die Produkte konnten nicht geladen werden."</p> }.into_any(),
                Some(Ok(list)) if list.is_empty() => {
                    view! { <p class="text-muted">"Im Moment gibt es noch keine Produkte."</p> }.into_any()
                }
                Some(Ok(list)) => view! {
                    <div class="grid gap-6 sm:grid-cols-2 lg:grid-cols-3">
                        {list.into_iter().map(|p| view! { <ProductCard product=p add=add /> }).collect_view()}
                    </div>
                }
                .into_any(),
            }}
        </section>
        </Transition>
    }
}

#[component]
fn ProductCard(product: Product, add: ServerAction<AddToCart>) -> impl IntoView {
    let id = product.id;
    let sold_out = product.stock <= 0;
    view! {
        <article class="flex flex-col gap-4 rounded-card border border-line bg-bg-2 p-6">
            <h3 class="mb-0">{product.name}</h3>
            <p class="mb-0 flex-1 text-muted">{product.description}</p>
            <div class="flex flex-wrap items-center justify-between gap-3">
                <span class="font-display text-[1.4rem]">{format!("CHF {}", format_price(product.price_cents))}</span>
                <button
                    class="btn btn-primary"
                    disabled=move || sold_out || add.pending().get()
                    on:click=move |_| {
                        add.dispatch(AddToCart { product_id: id, quantity: 1 });
                    }
                >
                    {if sold_out { "Ausverkauft" } else { "In den Warenkorb" }}
                </button>
            </div>
        </article>
    }
}

/// `/kasse` – Name und E-Mail eingeben, Bestellung abschicken
#[component]
pub fn CheckoutPage() -> impl IntoView {
    let shop = use_shop();
    let status: Status = RwSignal::new(None);
    let name = RwSignal::new(String::new());
    let email = RwSignal::new(String::new());
    let done = RwSignal::new(None::<OrderConfirmation>);
    let place = ServerAction::<PlaceOrder>::new();
    Effect::new(move |_| {
        if let Some(Ok(confirmation)) = place.value().get() {
            shop.set_cart(Cart::default());
            done.set(Some(confirmation));
        }
    });
    watch_action!(place, status, "Bestellung aufgegeben.", || {});

    view! {
        <Transition fallback=|| ()>
        <Title text="Kasse" />
        <section class="wrap page-hero">
            <p class="eyebrow">"Kasse"</p>
            <h1 class="max-w-[18ch]">"Bestellung abschliessen"</h1>
        </section>
        <section class="wrap flex flex-col gap-6 pb-[clamp(64px,10vw,120px)]">
            <div class="sticky top-[90px] z-10 empty:hidden">
                <StatusLine status=status />
            </div>
            {move || match done.get() {
                Some(confirmation) => view! { <Confirmation confirmation=confirmation /> }.into_any(),
                None => match shop.cart() {
                    None => view! { <p class="text-muted">"Lädt …"</p> }.into_any(),
                    Some(cart) if cart.lines.is_empty() => view! {
                        <p class="text-muted">
                            "Dein Warenkorb ist leer. "
                            <A href="/shop" attr:class="text-accent underline underline-offset-4">"Zum Shop"</A>
                        </p>
                    }
                    .into_any(),
                    Some(cart) => view! { <CheckoutForm cart=cart name=name email=email place=place /> }.into_any(),
                },
            }}
        </section>
        </Transition>
    }
}

#[component]
fn Confirmation(confirmation: OrderConfirmation) -> impl IntoView {
    view! {
        <div class="flex flex-col gap-4 rounded-card border-2 border-ink bg-bg-2 p-8">
            <p class="eyebrow">"Danke"</p>
            <h2 class="mb-0">{format!("Bestellung Nr. {}", confirmation.id)}</h2>
            <p class="mb-0 text-muted">
                {format!("{} Artikel, Gesamtsumme CHF {}.", confirmation.item_count, format_price(confirmation.total_cents))}
            </p>
            <div>
                <A href="/shop" attr:class="btn btn-primary">"Weiter einkaufen"</A>
            </div>
        </div>
    }
}

#[component]
fn CheckoutForm(cart: Cart, name: RwSignal<String>, email: RwSignal<String>, place: ServerAction<PlaceOrder>) -> impl IntoView {
    let total = cart.total_cents();
    let summary: Vec<(String, String)> = cart
        .lines
        .iter()
        .map(|l| (format!("{} × {}", l.quantity, l.name), format!("CHF {}", format_price(l.line_total_cents()))))
        .collect();

    view! {
        <div class="grid gap-8 md:grid-cols-[4fr_7fr]">
            <aside class="flex flex-col gap-3 self-start rounded-card border border-line bg-bg-2 p-6">
                <h3 class="mb-1">"Deine Bestellung"</h3>
                <ul class="flex flex-col gap-2">
                    {summary
                        .into_iter()
                        .map(|(what, price)| {
                            view! {
                                <li class="flex justify-between gap-4">
                                    <span>{what}</span>
                                    <span class="font-medium">{price}</span>
                                </li>
                            }
                        })
                        .collect_view()}
                </ul>
                <p class="mb-0 border-t border-line pt-3 font-display text-[1.3rem]">{format!("Total CHF {}", format_price(total))}</p>
            </aside>

            <form
                class="flex flex-col gap-5 border-2 border-ink bg-bg-2 p-[clamp(24px,4vw,44px)] shadow-[8px_8px_0_var(--color-ink)]"
                on:submit=move |ev| {
                    ev.prevent_default();
                    place.dispatch(PlaceOrder { name: name.get_untracked(), email: email.get_untracked() });
                }
            >
                <label class=LABEL>
                    <span class=LABEL_TEXT>"Name"</span>
                    <input
                        class=INPUT
                        type="text"
                        required
                        minlength="2"
                        autocomplete="name"
                        placeholder="Vor- und Nachname"
                        prop:value=name
                        on:input=move |e| name.set(event_target_value(&e))
                    />
                </label>
                <label class=LABEL>
                    <span class=LABEL_TEXT>"E-Mail"</span>
                    <input
                        class=INPUT
                        type="email"
                        required
                        autocomplete="email"
                        placeholder="du@beispiel.ch"
                        prop:value=email
                        on:input=move |e| email.set(event_target_value(&e))
                    />
                </label>
                <p class="mb-0 text-[0.9rem] text-muted">"Die Zahlung und der Versand werden separat mit dir vereinbart."</p>
                <button type="submit" class="btn btn-primary self-start" disabled=move || place.pending().get()>
                    {move || if place.pending().get() { "Wird gesendet …" } else { "Bestellung abschicken" }}
                </button>
            </form>
        </div>
    }
}

/// `/warenkorb` – Mengen ändern, Produkte entfernen, Summe
#[component]
pub fn CartPage() -> impl IntoView {
    let shop = use_shop();
    let status: Status = RwSignal::new(None);
    let update = ServerAction::<UpdateCartItem>::new();
    let remove = ServerAction::<RemoveFromCart>::new();
    Effect::new(move |_| {
        if let Some(Ok(cart)) = update.value().get() {
            shop.set_cart(cart);
        }
    });
    Effect::new(move |_| {
        if let Some(Ok(cart)) = remove.value().get() {
            shop.set_cart(cart);
        }
    });
    watch_action!(update, status, "Menge geändert.", || {});
    watch_action!(remove, status, "Produkt entfernt.", || {});

    view! {
        <Transition fallback=|| ()>
        <Title text="Warenkorb" />
        <section class="wrap page-hero">
            <p class="eyebrow">"Warenkorb"</p>
            <h1 class="max-w-[18ch]">"Dein Warenkorb"</h1>
        </section>
        <section class="wrap flex flex-col gap-6 pb-[clamp(64px,10vw,120px)]">
            <div class="sticky top-[90px] z-10 empty:hidden">
                <StatusLine status=status />
            </div>
            {move || match shop.cart() {
                None => view! { <p class="text-muted">"Lädt …"</p> }.into_any(),
                Some(cart) if cart.lines.is_empty() => view! {
                    <p class="text-muted">
                        "Dein Warenkorb ist leer. "
                        <A href="/shop" attr:class="text-accent underline underline-offset-4">"Zum Shop"</A>
                    </p>
                }
                .into_any(),
                Some(cart) => {
                    let total = cart.total_cents();
                    view! {
                        <ul class="flex flex-col gap-4">
                            {cart
                                .lines
                                .into_iter()
                                .map(|line| view! { <CartRow line=line update=update remove=remove /> })
                                .collect_view()}
                        </ul>
                        <div class="flex flex-col items-end gap-2 border-t border-line pt-6">
                            <p class="mb-0 font-display text-[1.6rem]">{format!("Total CHF {}", format_price(total))}</p>
                            <A href="/kasse" attr:class="btn btn-primary">"Zur Kasse"</A>
                        </div>
                    }
                    .into_any()
                }
            }}
        </section>
        </Transition>
    }
}

#[component]
fn CartRow(line: CartLine, update: ServerAction<UpdateCartItem>, remove: ServerAction<RemoveFromCart>) -> impl IntoView {
    let id = line.product_id;
    let quantity = line.quantity;
    let at_max = quantity >= line.stock;
    let unit = format!("CHF {} pro Stück", format_price(line.price_cents));
    let line_total = format!("CHF {}", format_price(line.line_total_cents()));
    view! {
        <li class="flex flex-col gap-4 rounded-card border border-line bg-bg-2 p-5 sm:flex-row sm:items-center sm:justify-between">
            <div>
                <h3 class="mb-1">{line.name}</h3>
                <p class="mb-0 text-[0.9rem] text-muted">{unit}</p>
            </div>
            <div class="flex flex-wrap items-center gap-3">
                <button
                    class=BTN_SMALL
                    aria-label="Eins weniger"
                    disabled=move || update.pending().get()
                    on:click=move |_| {
                        update.dispatch(UpdateCartItem { product_id: id, quantity: quantity - 1 });
                    }
                >
                    "−"
                </button>
                <span class="w-8 text-center font-medium">{quantity}</span>
                <button
                    class=BTN_SMALL
                    aria-label="Eins mehr"
                    disabled=move || at_max || update.pending().get()
                    on:click=move |_| {
                        update.dispatch(UpdateCartItem { product_id: id, quantity: quantity + 1 });
                    }
                >
                    "+"
                </button>
                <span class="w-28 text-right font-medium">{line_total}</span>
                <button
                    class=BTN_DANGER
                    on:click=move |_| {
                        remove.dispatch(RemoveFromCart { product_id: id });
                    }
                >
                    "Entfernen"
                </button>
            </div>
        </li>
    }
}
