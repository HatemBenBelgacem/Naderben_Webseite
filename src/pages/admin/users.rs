use leptos::prelude::*;

use super::{
    guard,
    ui::{confirm, err_text, watch_action, Status, StatusLine, BTN_DANGER, BTN_SAVE, BTN_SMALL, CARD, CHECKBOX, INPUT, LABEL, LABEL_TEXT},
};
use crate::{
    api::{list_roles, list_users, CreateUser, DeleteUser, SetUserPassword, UpdateUser},
    content::use_site,
    models::{RoleRow, UserRow, MIN_PASSWORD_LEN, USER_ROLE_ID},
};

#[derive(Clone, Copy)]
struct Actions {
    create: ServerAction<CreateUser>,
    update: ServerAction<UpdateUser>,
    password: ServerAction<SetUserPassword>,
    delete: ServerAction<DeleteUser>,
    status: Status,
}

/// `/admin/benutzer`
#[component]
pub fn UsersAdmin() -> impl IntoView {
    guard(|u| u.can_manage_users, || view! { <Users /> }.into_any())
}

#[component]
fn Users() -> impl IntoView {
    let site = use_site();
    let users = Resource::new(|| (), |_| list_users());
    let roles = Resource::new(|| (), |_| list_roles());
    let actions = Actions {
        create: ServerAction::new(),
        update: ServerAction::new(),
        password: ServerAction::new(),
        delete: ServerAction::new(),
        status: RwSignal::new(None),
    };
    let reload = move || {
        users.refetch();
        // Eigener Name/Rolle könnte sich geändert haben
        site.user.refetch();
    };
    watch_action!(actions.create, actions.status, "Benutzer angelegt.", reload);
    watch_action!(actions.update, actions.status, "Benutzer gespeichert.", reload);
    watch_action!(actions.password, actions.status, "Passwort gesetzt – der Benutzer wurde überall abgemeldet.", || ());
    watch_action!(actions.delete, actions.status, "Benutzer gelöscht.", reload);

    let me = move || site.current_user().map(|u| u.id);

    view! {
        <Transition fallback=|| view! { <p class="text-muted">"Lädt …"</p> }>
            {move || Suspend::new(async move {
                let users = users.await;
                let roles = roles.await;
                match (users, roles) {
                    (Ok(users), Ok(roles)) => {
                        let roles = StoredValue::new(roles);
                        view! {
                            <div class="flex flex-col gap-6">
                                <NewUser roles actions />
                                <div class="sticky top-[90px] z-10"><StatusLine status=actions.status /></div>
                                <div class="grid gap-4">
                                    {users
                                        .into_iter()
                                        .map(|u| {
                                            let is_me = me() == Some(u.id);
                                            view! { <UserCard user=u roles actions is_me /> }
                                        })
                                        .collect_view()}
                                </div>
                            </div>
                        }
                        .into_any()
                    }
                    (Err(e), _) | (_, Err(e)) => view! { <p class="text-danger">{err_text(e)}</p> }.into_any(),
                }
            })}
        </Transition>
    }
}

#[component]
fn RoleSelect(roles: StoredValue<Vec<RoleRow>>, value: ReadSignal<i64>, set_value: WriteSignal<i64>, #[prop(optional)] disabled: bool) -> impl IntoView {
    view! {
        <select
            class=INPUT
            disabled=disabled
            on:change=move |e| {
                if let Ok(id) = event_target_value(&e).parse() {
                    set_value.set(id);
                }
            }
        >
            {roles
                .get_value()
                .into_iter()
                .map(|r| {
                    let id = r.id;
                    view! { <option value=id.to_string() selected=move || value.get() == id>{r.name}</option> }
                })
                .collect_view()}
        </select>
    }
}

#[component]
fn NewUser(roles: StoredValue<Vec<RoleRow>>, actions: Actions) -> impl IntoView {
    let (name, set_name) = signal(String::new());
    let (email, set_email) = signal(String::new());
    let (password, set_password) = signal(String::new());
    let (role, set_role) = signal(USER_ROLE_ID);

    Effect::new(move |_| {
        if let Some(Ok(())) = actions.create.value().get() {
            set_name.set(String::new());
            set_email.set(String::new());
            set_password.set(String::new());
            set_role.set(USER_ROLE_ID);
        }
    });

    let submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        actions.create.dispatch(CreateUser {
            name: name.get_untracked(),
            email: email.get_untracked(),
            role_id: role.get_untracked(),
            password: password.get_untracked(),
        });
    };

    view! {
        <form class=format!("{CARD} grid gap-3 sm:grid-cols-2 lg:grid-cols-4") on:submit=submit>
            <h3 class="mb-0 sm:col-span-2 lg:col-span-4">"Neuer Benutzer"</h3>
            <label class=LABEL>
                <span class=LABEL_TEXT>"Name"</span>
                <input class=INPUT type="text" required prop:value=name on:input=move |e| set_name.set(event_target_value(&e)) />
            </label>
            <label class=LABEL>
                <span class=LABEL_TEXT>"E-Mail"</span>
                <input class=INPUT type="email" required autocomplete="off" prop:value=email on:input=move |e| set_email.set(event_target_value(&e)) />
            </label>
            <label class=LABEL>
                <span class=LABEL_TEXT>"Rolle"</span>
                <RoleSelect roles value=role set_value=set_role />
            </label>
            <label class=LABEL>
                <span class=LABEL_TEXT>{format!("Startpasswort (mind. {MIN_PASSWORD_LEN} Zeichen)")}</span>
                <input class=INPUT type="password" required autocomplete="new-password" minlength=MIN_PASSWORD_LEN.to_string()
                    prop:value=password on:input=move |e| set_password.set(event_target_value(&e)) />
            </label>
            <div class="sm:col-span-2 lg:col-span-4">
                <button type="submit" class=BTN_SAVE disabled=move || actions.create.pending().get()>"Benutzer anlegen"</button>
            </div>
        </form>
    }
}

#[component]
fn UserCard(user: UserRow, roles: StoredValue<Vec<RoleRow>>, actions: Actions, is_me: bool) -> impl IntoView {
    let id = user.id;
    let owner = user.is_owner();
    let (name, set_name) = signal(user.name.clone());
    let (role, set_role) = signal(user.role_id);
    let (active, set_active) = signal(user.active);
    let (new_password, set_new_password) = signal(String::new());
    let original = user.clone();
    let changed = move || name.get() != original.name || role.get() != original.role_id || active.get() != original.active;

    view! {
        <div class=format!("{CARD} flex flex-col gap-4 {}", if user.active { "" } else { "opacity-70" })>
            <div class="flex flex-wrap items-baseline justify-between gap-2">
                <div>
                    <strong class="text-ink">{user.email.clone()}</strong>
                    {owner.then_some(view! { <span class="ml-2 rounded-full bg-accent/15 px-2 py-0.5 text-[0.75rem] text-accent">"Inhaber"</span> })}
                    {is_me.then_some(view! { <span class="ml-2 rounded-full border border-line px-2 py-0.5 text-[0.75rem] text-muted">"Du"</span> })}
                    {(!user.active).then_some(view! { <span class="ml-2 rounded-full border border-danger/40 px-2 py-0.5 text-[0.75rem] text-danger">"Gesperrt"</span> })}
                </div>
                <span class="text-[0.8rem] text-muted">{format!("seit {}", user.created_at.get(..10).unwrap_or(&user.created_at))}</span>
            </div>
            <div class="grid gap-3 sm:grid-cols-[2fr_1.5fr_auto] sm:items-end">
                <label class=LABEL>
                    <span class=LABEL_TEXT>"Name"</span>
                    <input class=INPUT type="text" prop:value=name on:input=move |e| set_name.set(event_target_value(&e)) />
                </label>
                <label class=LABEL>
                    <span class=LABEL_TEXT>"Rolle"</span>
                    <RoleSelect roles value=role set_value=set_role disabled=owner />
                </label>
                <label class="flex items-center gap-2 pb-2.5 text-[0.9rem]">
                    <input class=CHECKBOX type="checkbox" prop:checked=active disabled=owner || is_me
                        on:change=move |e| set_active.set(event_target_checked(&e)) />
                    "Aktiv"
                </label>
            </div>
            <div class="flex flex-wrap items-center gap-2">
                <button
                    class=BTN_SMALL
                    disabled=move || !changed()
                    on:click=move |_| {
                        actions.update.dispatch(UpdateUser { id, name: name.get_untracked(), role_id: role.get_untracked(), active: active.get_untracked() });
                    }
                >
                    "Speichern"
                </button>
                {(!owner || is_me).then_some(view! {
                    <details class="group">
                        <summary class=format!("{BTN_SMALL} cursor-pointer list-none")>"Passwort setzen"</summary>
                        <form
                            class="mt-2 flex flex-wrap gap-2"
                            on:submit=move |ev| {
                                ev.prevent_default();
                                actions.password.dispatch(SetUserPassword { id, password: new_password.get_untracked() });
                                set_new_password.set(String::new());
                            }
                        >
                            <input class=format!("{INPUT} max-w-xs") type="password" autocomplete="new-password" required
                                minlength=MIN_PASSWORD_LEN.to_string() placeholder="Neues Passwort"
                                prop:value=new_password on:input=move |e| set_new_password.set(event_target_value(&e)) />
                            <button type="submit" class=BTN_SMALL>"Setzen"</button>
                        </form>
                    </details>
                })}
                {(!owner && !is_me).then_some(view! {
                    <button
                        class=format!("{BTN_DANGER} ml-auto")
                        on:click=move |_| {
                            if confirm("Diesen Benutzer endgültig löschen?") {
                                actions.delete.dispatch(DeleteUser { id });
                            }
                        }
                    >
                        "Löschen"
                    </button>
                })}
            </div>
        </div>
    }
}
