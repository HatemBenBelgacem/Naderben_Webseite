use leptos::prelude::*;

use super::{
    guard,
    ui::{err_text, watch_action, Status, StatusLine, CARD, INPUT},
};
use crate::{
    api::{ListOrders, UpdateOrderStatus},
    models::{format_price, OrderSummary, ORDER_STATUSES},
};

/// `/admin/bestellungen` – Bestellungen ansehen und ihren Status ändern
#[component]
pub fn OrdersAdmin() -> impl IntoView {
    guard(|u| u.can_edit_content, || view! { <Orders /> }.into_any())
}

#[component]
fn Orders() -> impl IntoView {
    let status: Status = RwSignal::new(None);
    // Liste erst im Browser laden (wie in der Produktverwaltung), damit das Server-Rendering nicht wartet
    let list = ServerAction::<ListOrders>::new();
    let reload = Callback::new(move |_: ()| {
        list.dispatch(ListOrders {});
    });
    Effect::new(move |_| {
        list.dispatch(ListOrders {});
    });

    view! {
        <div class="flex flex-col gap-6">
            <div class="sticky top-[90px] z-10 empty:hidden">
                <StatusLine status=status />
            </div>
            {move || match list.value().get() {
                None => view! { <p class="text-muted">"Lädt …"</p> }.into_any(),
                Some(Err(e)) => view! { <p class="text-danger">{err_text(e)}</p> }.into_any(),
                Some(Ok(orders)) if orders.is_empty() => {
                    view! { <p class="text-muted">"Noch keine Bestellungen."</p> }.into_any()
                }
                Some(Ok(orders)) => view! {
                    <div class="flex flex-col gap-4">
                        {orders
                            .into_iter()
                            .map(|order| view! { <OrderCard order=order status=status on_change=reload /> })
                            .collect_view()}
                    </div>
                }
                .into_any(),
            }}
        </div>
    }
}

#[component]
fn OrderCard(order: OrderSummary, status: Status, on_change: Callback<()>) -> impl IntoView {
    let id = order.id;
    let update = ServerAction::<UpdateOrderStatus>::new();
    let reload = move || on_change.run(());
    watch_action!(update, status, "Status geändert.", reload);

    let lines: Vec<String> = order
        .items
        .iter()
        .map(|i| format!("{} × {} (CHF {} pro Stück)", i.quantity, i.product_name, format_price(i.unit_price_cents)))
        .collect();
    let current = order.status.clone();

    view! {
        <article class=format!("{CARD} flex flex-col gap-3")>
            <div class="flex flex-wrap items-baseline justify-between gap-2">
                <h3 class="mb-0">{format!("Bestellung Nr. {}", order.id)}</h3>
                <span class="text-[0.85rem] text-muted">{order.created_at}</span>
            </div>
            <p class="mb-0">{format!("{} · {}", order.customer_name, order.customer_email)}</p>
            <ul class="mb-0 flex flex-col gap-1 text-[0.95rem]">
                {lines.into_iter().map(|line| view! { <li>{line}</li> }).collect_view()}
            </ul>
            <div class="flex flex-wrap items-center justify-between gap-3 border-t border-line pt-3">
                <span class="font-medium">{format!("Total CHF {}", format_price(order.total_cents))}</span>
                <label class="flex items-center gap-2">
                    <span class="text-[0.85rem] text-muted">"Status"</span>
                    <select
                        class=INPUT
                        disabled=move || update.pending().get()
                        on:change=move |ev| {
                            update.dispatch(UpdateOrderStatus { id, status: event_target_value(&ev) });
                        }
                    >
                        {ORDER_STATUSES
                            .into_iter()
                            .map(|s| view! { <option value=s selected=s == current.as_str()>{s}</option> })
                            .collect_view()}
                    </select>
                </label>
            </div>
        </article>
    }
}
