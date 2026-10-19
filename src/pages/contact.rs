use leptos::prelude::*;
use leptos_meta::Title;

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
const FIELD: &str = "rounded-xl border border-line bg-bg px-4 py-[0.85em] text-ink transition placeholder:text-[#5f5c57] focus:border-accent focus:ring-[3px] focus:ring-accent/20 focus:outline-none";
const INFO_LABEL: &str = "eyebrow mb-1 block";
const INFO_LINK: &str = "text-[1.1rem] transition-colors hover:text-accent";
const STATUS: &str = "rounded-xl px-[1.1em] py-[0.9em] text-[0.95rem]";

#[component]
pub fn ContactPage() -> impl IntoView {
    let action = ServerAction::<SendContact>::new();
    let pending = action.pending();

    let result = move || {
        action.value().get().map(|r| match r {
            Ok(()) => view! {
                <p class=format!("{STATUS} bg-[rgb(120_200_140/0.12)] text-[#9fe0b0]") role="status">
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
                    <p class=format!("{STATUS} bg-[rgb(230_100_90/0.12)] text-[#f4a49b]") role="alert">
                        {msg}
                    </p>
                }
                .into_any()
            }
        })
    };

    view! {
        <Title text="Contact" />
        <section class="wrap page-hero">
            <p class="eyebrow">"Contact"</p>
            <h1 class="max-w-[18ch]">"Erzähl mir von deinem Projekt."</h1>
            <p class="lead">"Hochzeit, Imagefilm oder Portrait-Shooting – schreib mir, ich freue mich auf deine Idee."</p>
        </section>

        <section class="wrap grid gap-[clamp(32px,6vw,96px)] pb-[clamp(64px,10vw,120px)] md:grid-cols-[4fr_7fr]">
            <aside class="flex flex-col gap-7">
                <div>
                    <small class=INFO_LABEL>"E-Mail"</small>
                    <a class=INFO_LINK href="mailto:hello@naderben.com">"hello@naderben.com"</a>
                </div>
                <div>
                    <small class=INFO_LABEL>"Telefon"</small>
                    <a class=INFO_LINK href="tel:+41790000000">"+41 79 000 00 00"</a>
                </div>
                <div>
                    <small class=INFO_LABEL>"Studio"</small>
                    <p class="mb-0 text-[1.1rem]">"Zürich, Schweiz" <br /> "Weltweit verfügbar"</p>
                </div>
                <div>
                    <small class=INFO_LABEL>"Social"</small>
                    <p class="mb-0 flex gap-5">
                        <a class=INFO_LINK href="https://instagram.com" target="_blank" rel="noopener">"Instagram"</a>
                        <a class=INFO_LINK href="https://vimeo.com" target="_blank" rel="noopener">"Vimeo"</a>
                    </p>
                </div>
            </aside>

            <ActionForm
                action=action
                attr:class="flex flex-col gap-5 rounded-card border border-line bg-bg-2 p-[clamp(24px,4vw,44px)]"
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
    }
}
