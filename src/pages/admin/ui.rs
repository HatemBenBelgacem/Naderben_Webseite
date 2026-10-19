//! Gemeinsame Bausteine der Verwaltung

use leptos::{html, prelude::*};

use crate::{api::upload_image, models::MediaInfo};

pub const INPUT: &str = "w-full rounded-xl border border-line bg-bg px-3.5 py-2.5 text-[0.95rem] text-ink transition placeholder:text-[#5f5c57] focus:border-accent focus:ring-[3px] focus:ring-accent/20 focus:outline-none disabled:opacity-50";
pub const LABEL: &str = "flex flex-col gap-1.5";
pub const LABEL_TEXT: &str = "text-[0.8rem] text-muted";
pub const CARD: &str = "rounded-card border border-line bg-bg-2 p-5 sm:p-6";
pub const BTN_SMALL: &str = "rounded-lg border border-line px-3 py-1.5 text-[0.85rem] transition-colors hover:border-ink/40 hover:text-ink disabled:opacity-40";
pub const BTN_DANGER: &str = "rounded-lg border border-[#f4a49b]/40 px-3 py-1.5 text-[0.85rem] text-[#f4a49b] transition-colors hover:bg-[#f4a49b]/10 disabled:opacity-40";
pub const BTN_SAVE: &str = "btn btn-primary px-4 py-2";
pub const CHECKBOX: &str = "size-4 accent-[#e9b44c]";

/// Fehlermeldung eines Server-Aufrufs ohne technisches Präfix
pub fn err_text(e: ServerFnError) -> String {
    match e {
        ServerFnError::ServerError(msg) => msg,
        other => other.to_string(),
    }
}

/// Rückmeldung nach einer Aktion: `Ok(text)` grün, `Err(text)` rot
pub type Status = RwSignal<Option<Result<String, String>>>;

#[component]
pub fn StatusLine(status: Status) -> impl IntoView {
    move || {
        status.get().map(|s| match s {
            Ok(msg) => view! { <p class="mb-0 text-[0.9rem] text-[#9fe0b0]" role="status">{msg}</p> }.into_any(),
            Err(msg) => view! { <p class="mb-0 text-[0.9rem] text-[#f4a49b]" role="alert">{msg}</p> }.into_any(),
        })
    }
}

/// Nach jeder abgeschlossenen Server-Aktion den Status setzen und bei Erfolg `on_ok` ausführen
macro_rules! watch_action {
    ($action:expr, $status:expr, $ok:expr, $on_ok:expr) => {{
        let action = $action;
        let status = $status;
        let on_ok = $on_ok;
        Effect::new(move |_| match action.value().get() {
            Some(Ok(_)) => {
                status.set(Some(Ok($ok.to_string())));
                on_ok();
            }
            Some(Err(e)) => status.set(Some(Err($crate::pages::admin::ui::err_text(e)))),
            None => {}
        });
    }};
}
pub(crate) use watch_action;

/// Sicherheitsabfrage im Browser
pub fn confirm(message: &str) -> bool {
    window().confirm_with_message(message).unwrap_or(false)
}

/// Datei-Auswahl, die das Bild sofort hochlädt und die neue URL meldet
#[component]
pub fn ImageUpload(#[prop(into)] on_uploaded: Callback<MediaInfo>, #[prop(optional)] label: Option<&'static str>) -> impl IntoView {
    let form = NodeRef::<html::Form>::new();
    let upload = Action::new_local(|data: &web_sys::FormData| upload_image(data.clone().into()));

    Effect::new(move |_| {
        if let Some(Ok(info)) = upload.value().get() {
            on_uploaded.run(info);
            if let Some(form) = form.get_untracked() {
                form.reset();
            }
        }
    });

    let on_change = move |_| {
        if let Some(form) = form.get_untracked() {
            if let Ok(data) = web_sys::FormData::new_with_form(&form) {
                upload.dispatch_local(data);
            }
        }
    };

    view! {
        <form node_ref=form class="flex flex-wrap items-center gap-3" on:submit=|e| e.prevent_default()>
            <label class=format!("{BTN_SMALL} cursor-pointer")>
                <input
                    class="sr-only"
                    type="file"
                    name="file"
                    accept="image/jpeg,image/png,image/webp,image/gif"
                    on:change=on_change
                    disabled=move || upload.pending().get()
                />
                {move || if upload.pending().get() { "Wird hochgeladen …" } else { label.unwrap_or("Bild hochladen") }}
            </label>
            {move || match upload.value().get() {
                Some(Err(e)) => Some(view! { <span class="text-[0.85rem] text-[#f4a49b]">{err_text(e)}</span> }),
                _ => None,
            }}
        </form>
    }
}
