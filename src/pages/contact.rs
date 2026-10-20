use leptos::prelude::*;
use leptos_meta::Title;

use crate::content::use_site;

/// Nimmt Kontaktanfragen entgegen. Aktuell werden sie validiert und ins
/// Server-Log geschrieben (sichtbar in den Railway-Logs). Hier lässt sich
/// später E-Mail-Versand (z. B. lettre/SMTP) oder ein Webhook anschliessen.
#[server]
pub async fn send_contact(
    name: String,
    email: String,
    project_type: String,
    message: String,
    // Honeypot gegen Spam-Bots – muss leer bleiben
    website: String,
) -> Result<(), ServerFnError> {
    if !website.is_empty() {
        return Ok(());
    }
    let (name, email, message) = (name.trim(), email.trim(), message.trim());

    if name.len() < 2 {
        return Err(ServerFnError::new("Bitte gib deinen Namen an."));
    }
    let valid_email = email
        .split_once('@')
        .is_some_and(|(user, domain)| !user.is_empty() && domain.contains('.') && !domain.ends_with('.'));
    if !valid_email {
        return Err(ServerFnError::new("Bitte gib eine gültige E-Mail-Adresse an."));
    }
    if message.len() < 10 {
        return Err(ServerFnError::new("Erzähl mir etwas mehr über dein Projekt (mind. 10 Zeichen)."));
    }
    if name.len() > 200 || email.len() > 200 || message.len() > 5000 {
        return Err(ServerFnError::new("Die Eingabe ist zu lang."));
    }

    println!(
        "[kontakt] Neue Anfrage\n  Name: {name}\n  E-Mail: {email}\n  Projekt: {project_type}\n  Nachricht: {message}"
    );
    Ok(())
}

const LABEL: &str = "flex flex-col gap-2";
const LABEL_TEXT: &str = "text-[0.85rem] text-muted";
const FIELD: &str = "rounded-xl border border-line bg-white px-4 py-[0.85em] text-ink transition placeholder:text-black/35 focus:border-accent focus:ring-[3px] focus:ring-accent/20 focus:outline-none";
const INFO_LABEL: &str = "eyebrow mb-1 block";
const INFO_LINK: &str = "text-[1.2rem] font-medium underline decoration-2 decoration-transparent underline-offset-4 transition-colors hover:decoration-accent";
const STATUS: &str = "rounded-xl px-[1.1em] py-[0.9em] text-[0.95rem]";

#[component]
pub fn ContactPage() -> impl IntoView {
    let site = use_site();
    let t = move |key: &'static str| move || site.text(key);
    let action = ServerAction::<SendContact>::new();
    let pending = action.pending();

    let result = move || {
        action.value().get().map(|r| match r {
            Ok(()) => view! {
                <p class=format!("{STATUS} bg-success/10 text-success") role="status">
                    "Danke für deine Nachricht! Ich melde mich innerhalb von 48 Stunden."
                </p>
            }
            .into_any(),
            Err(e) => {
                let msg = match e {
                    ServerFnError::ServerError(m) => m,
                    other => other.to_string(),
                };
                view! {
                    <p class=format!("{STATUS} bg-danger/10 text-danger") role="alert">
                        {msg}
                    </p>
                }
                .into_any()
            }
        })
    };

    view! {
        <Transition fallback=|| ()>
        <Title text="Contact" />
        <section class="wrap page-hero">
            <p class="eyebrow">"Contact"</p>
            <h1 class="max-w-[18ch]">{t("contact.title")}</h1>
            <p class="lead">{t("contact.lead")}</p>
        </section>

        <section class="wrap grid gap-[clamp(32px,6vw,96px)] pb-[clamp(64px,10vw,120px)] md:grid-cols-[4fr_7fr]">
            <aside class="flex flex-col gap-7">
                <div>
                    <small class=INFO_LABEL>"E-Mail"</small>
                    <a class=INFO_LINK href=move || format!("mailto:{}", site.text("contact.email"))>{t("contact.email")}</a>
                </div>
                <div>
                    <small class=INFO_LABEL>"Telefon"</small>
                    <a
                        class=INFO_LINK
                        href=move || format!("tel:{}", site.text("contact.phone").chars().filter(|c| c.is_ascii_digit() || *c == '+').collect::<String>())
                    >
                        {t("contact.phone")}
                    </a>
                </div>
                <div>
                    <small class=INFO_LABEL>"Studio"</small>
                    <p class="mb-0 text-[1.1rem]">{t("contact.location")} <br /> {t("contact.availability")}</p>
                </div>
                <div>
                    <small class=INFO_LABEL>"Social"</small>
                    <p class="mb-0 flex gap-5">
                        <a class=INFO_LINK href=t("social.instagram") target="_blank" rel="noopener">"Instagram"</a>
                        <a class=INFO_LINK href=t("social.vimeo") target="_blank" rel="noopener">"Vimeo"</a>
                    </p>
                </div>
            </aside>

            <ActionForm
                action=action
                attr:class="flex flex-col gap-5 border-2 border-ink bg-bg-2 p-[clamp(24px,4vw,44px)] shadow-[8px_8px_0_var(--color-ink)]"
            >
                <div class="grid gap-5 sm:grid-cols-2">
                    <label class=LABEL>
                        <span class=LABEL_TEXT>"Name"</span>
                        <input class=FIELD type="text" name="name" required minlength="2" autocomplete="name" placeholder="Vor- und Nachname" />
                    </label>
                    <label class=LABEL>
                        <span class=LABEL_TEXT>"E-Mail"</span>
                        <input class=FIELD type="email" name="email" required autocomplete="email" placeholder="du@beispiel.ch" />
                    </label>
                </div>
                <label class=LABEL>
                    <span class=LABEL_TEXT>"Art des Projekts"</span>
                    <select class=FIELD name="project_type">
                        <option>"Hochzeit (Foto)"</option>
                        <option>"Hochzeit (Video)"</option>
                        <option>"Portrait-Shooting"</option>
                        <option>"Imagefilm / Werbung"</option>
                        <option>"Event"</option>
                        <option>"Musikvideo"</option>
                        <option>"Etwas anderes"</option>
                    </select>
                </label>
                <label class=LABEL>
                    <span class=LABEL_TEXT>"Nachricht"</span>
                    <textarea class=format!("{FIELD} resize-y") name="message" rows="6" required minlength="10" placeholder="Datum, Ort, Idee, Budget …"></textarea>
                </label>
                // Honeypot – für Menschen unsichtbar
                <label class="absolute -left-[9999px] size-px overflow-hidden" aria-hidden="true">
                    "Website"
                    <input type="text" name="website" tabindex="-1" autocomplete="off" />
                </label>
                <button type="submit" class="btn btn-primary self-start" disabled=move || pending.get()>
                    {move || if pending.get() { "Wird gesendet …" } else { "Nachricht senden" }}
                </button>
                {result}
            </ActionForm>
        </section>
        </Transition>
    }
}
