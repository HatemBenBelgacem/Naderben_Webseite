use leptos::prelude::*;

use super::{
    guard,
    ui::{confirm, err_text, watch_action, Status, StatusLine, BTN_DANGER, BTN_SAVE, BTN_SMALL, CARD, CHECKBOX, INPUT, LABEL, LABEL_TEXT},
};
use crate::{
    api::{list_roles, DeleteRole, SaveRole},
    content::use_site,
    models::RoleRow,
};

#[derive(Clone, Copy)]
struct Actions {
    save: ServerAction<SaveRole>,
    delete: ServerAction<DeleteRole>,
    status: Status,
}

/// `/admin/rollen`
#[component]
pub fn RolesAdmin() -> impl IntoView {
    guard(|u| u.can_manage_roles, || view! { <Roles /> }.into_any())
}

#[component]
fn Roles() -> impl IntoView {
    let site = use_site();
    let roles = Resource::new(|| (), |_| list_roles());
    let actions = Actions { save: ServerAction::new(), delete: ServerAction::new(), status: RwSignal::new(None) };
    let reload = move || {
        roles.refetch();
        // Rechte der eigenen Rolle könnten sich geändert haben
        site.user.refetch();
    };
    watch_action!(actions.save, actions.status, "Rolle gespeichert.", reload);
    watch_action!(actions.delete, actions.status, "Rolle gelöscht.", reload);

    view! {
        <div class="flex flex-col gap-6">
            <p class="mb-0 max-w-[70ch] text-[0.95rem] text-muted">
                "Jede Rolle bündelt Berechtigungen. Die Admin-Rolle hat immer alle Rechte; System-Rollen lassen sich nicht löschen. "
                "Du kannst nur Rechte vergeben, die du selbst besitzt."
            </p>
            <RoleCard role=None actions />
            <div class="sticky top-[90px] z-10"><StatusLine status=actions.status /></div>
            <Transition fallback=|| view! { <p class="text-muted">"Lädt …"</p> }>
                {move || Suspend::new(async move {
                    match roles.await {
                        Ok(list) => view! {
                            <div class="grid gap-4 lg:grid-cols-2">
                                {list.into_iter().map(|r| view! { <RoleCard role=Some(r) actions /> }).collect_view()}
                            </div>
                        }
                        .into_any(),
                        Err(e) => view! { <p class="text-danger">{err_text(e)}</p> }.into_any(),
                    }
                })}
            </Transition>
        </div>
    }
}

/// Neue Rolle (role = None) oder bestehende bearbeiten
#[component]
fn RoleCard(role: Option<RoleRow>, actions: Actions) -> impl IntoView {
    let id = role.as_ref().map(|r| r.id);
    let locked = role.as_ref().is_some_and(RoleRow::is_admin);
    let (name, set_name) = signal(role.as_ref().map(|r| r.name.clone()).unwrap_or_default());
    let (description, set_description) = signal(role.as_ref().map(|r| r.description.clone()).unwrap_or_default());
    let (edit, set_edit) = signal(role.as_ref().is_some_and(|r| r.can_edit_content));
    let (users, set_users) = signal(role.as_ref().is_some_and(|r| r.can_manage_users));
    let (roles, set_roles) = signal(role.as_ref().is_some_and(|r| r.can_manage_roles));

    if id.is_none() {
        Effect::new(move |_| {
            if let Some(Ok(())) = actions.save.value().get() {
                set_name.set(String::new());
                set_description.set(String::new());
                for s in [set_edit, set_users, set_roles] {
                    s.set(false);
                }
            }
        });
    }

    let save = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        actions.save.dispatch(SaveRole {
            id,
            name: name.get_untracked(),
            description: description.get_untracked(),
            can_edit_content: edit.get_untracked(),
            can_manage_users: users.get_untracked(),
            can_manage_roles: roles.get_untracked(),
        });
    };

    let checkbox = move |label: &'static str, value: ReadSignal<bool>, set: WriteSignal<bool>| {
        view! {
            <label class="flex items-center gap-2 text-[0.9rem]">
                <input class=CHECKBOX type="checkbox" prop:checked=value disabled=locked on:change=move |e| set.set(event_target_checked(&e)) />
                {label}
            </label>
        }
    };

    let deletable = role.as_ref().is_some_and(|r| !r.is_system && r.user_count == 0);
    let badge = role.as_ref().map(|r| {
        let mut parts = vec![format!("{} Benutzer", r.user_count)];
        if r.is_system {
            parts.push("System-Rolle".into());
        }
        parts.join(" · ")
    });

    view! {
        <form class=format!("{CARD} flex flex-col gap-3 {}", if id.is_none() { "border-dashed" } else { "" }) on:submit=save>
            <div class="flex items-baseline justify-between gap-2">
                <h3 class="mb-0">{if id.is_none() { "Neue Rolle" } else { "Rolle" }}</h3>
                {badge.map(|b| view! { <span class="text-[0.8rem] text-muted">{b}</span> })}
            </div>
            <div class="grid gap-3 sm:grid-cols-[1fr_2fr]">
                <label class=LABEL>
                    <span class=LABEL_TEXT>"Name"</span>
                    <input class=INPUT type="text" required disabled=locked prop:value=name on:input=move |e| set_name.set(event_target_value(&e)) />
                </label>
                <label class=LABEL>
                    <span class=LABEL_TEXT>"Beschreibung"</span>
                    <input class=INPUT type="text" prop:value=description on:input=move |e| set_description.set(event_target_value(&e)) />
                </label>
            </div>
            <div class="flex flex-wrap gap-x-6 gap-y-2">
                {checkbox("Inhalte & Portfolio bearbeiten", edit, set_edit)}
                {checkbox("Benutzer verwalten", users, set_users)}
                {checkbox("Rollen verwalten", roles, set_roles)}
            </div>
            <div class="flex flex-wrap items-center gap-2">
                <button type="submit" class=if id.is_none() { BTN_SAVE } else { BTN_SMALL } disabled=move || actions.save.pending().get()>
                    {if id.is_none() { "Rolle anlegen" } else { "Speichern" }}
                </button>
                {id.filter(|_| deletable).map(|id| view! {
                    <button
                        type="button"
                        class=format!("{BTN_DANGER} ml-auto")
                        on:click=move |_| {
                            if confirm("Diese Rolle löschen?") {
                                actions.delete.dispatch(DeleteRole { id });
                            }
                        }
                    >
                        "Löschen"
                    </button>
                })}
            </div>
        </form>
    }
}
