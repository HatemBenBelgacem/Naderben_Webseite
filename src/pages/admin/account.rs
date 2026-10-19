use leptos::prelude::*;

use super::ui::{watch_action, StatusLine, Status, BTN_SAVE, CARD, INPUT, LABEL, LABEL_TEXT};
use crate::{api::ChangePassword, content::use_site, models::MIN_PASSWORD_LEN};

#[component]
pub fn AccountPage() -> impl IntoView {
    let site = use_site();
    let (current, set_current) = signal(String::new());
    let (new, set_new) = signal(String::new());
    let (repeat, set_repeat) = signal(String::new());
    let status: Status = RwSignal::new(None);
    let change = ServerAction::<ChangePassword>::new();

    watch_action!(change, status, "Passwort geändert. Andere Geräte wurden abgemeldet.", move || {
        set_current.set(String::new());
        set_new.set(String::new());
        set_repeat.set(String::new());
    });

    let submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        if new.get_untracked() != repeat.get_untracked() {
            status.set(Some(Err("Die neuen Passwörter stimmen nicht überein.".into())));
            return;
        }
        change.dispatch(ChangePassword { current_password: current.get_untracked(), new_password: new.get_untracked() });
    };

    view! {
        <div class="grid gap-6 lg:grid-cols-2">
            <div class=CARD>
                <h3>"Mein Konto"</h3>
                {move || site.current_user().map(|u| view! {
                    <dl class="mb-0 grid grid-cols-[auto_1fr] gap-x-6 gap-y-2 text-[0.95rem]">
                        <dt class="text-muted">"Name"</dt><dd>{u.name}</dd>
                        <dt class="text-muted">"E-Mail"</dt><dd>{u.email}</dd>
                        <dt class="text-muted">"Rolle"</dt><dd>{u.role}</dd>
                    </dl>
                })}
            </div>
            <form class=format!("{CARD} flex flex-col gap-4") on:submit=submit>
                <h3 class="mb-0">"Passwort ändern"</h3>
                <label class=LABEL>
                    <span class=LABEL_TEXT>"Aktuelles Passwort"</span>
                    <input class=INPUT type="password" autocomplete="current-password" required
                        prop:value=current on:input=move |e| set_current.set(event_target_value(&e)) />
                </label>
                <label class=LABEL>
                    <span class=LABEL_TEXT>{format!("Neues Passwort (mind. {MIN_PASSWORD_LEN} Zeichen)")}</span>
                    <input class=INPUT type="password" autocomplete="new-password" required minlength=MIN_PASSWORD_LEN.to_string()
                        prop:value=new on:input=move |e| set_new.set(event_target_value(&e)) />
                </label>
                <label class=LABEL>
                    <span class=LABEL_TEXT>"Neues Passwort wiederholen"</span>
                    <input class=INPUT type="password" autocomplete="new-password" required
                        prop:value=repeat on:input=move |e| set_repeat.set(event_target_value(&e)) />
                </label>
                <div class="flex flex-wrap items-center gap-4">
                    <button type="submit" class=BTN_SAVE disabled=move || change.pending().get()>"Passwort speichern"</button>
                    <StatusLine status />
                </div>
            </form>
        </div>
    }
}
