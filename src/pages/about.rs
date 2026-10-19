use leptos::prelude::*;
use leptos_meta::Title;

use super::home::CtaBand;
use crate::content::use_site;

#[component]
pub fn AboutPage() -> impl IntoView {
    let site = use_site();
    let t = move |key: &'static str| move || site.text(key);

    view! {
        <Transition fallback=|| ()>
        <Title text="About" />
        <section class="wrap page-hero">
            <p class="eyebrow">"About"</p>
            <h1 class="max-w-[18ch]">{t("about.title")}</h1>
        </section>

        <section class="wrap grid items-start gap-[clamp(32px,6vw,96px)] md:grid-cols-[5fr_6fr]">
            <figure class="md:sticky md:top-[100px]">
                <img
                    class="aspect-[4/5] w-full rounded-card object-cover"
                    src=t("about.image")
                    alt=t("about.caption")
                />
                <figcaption class="mt-3 text-[0.85rem] text-muted">{t("about.caption")}</figcaption>
            </figure>
            <div class="text-body">
                <p class="lead">{t("about.lead")}</p>
                <p>{t("about.text1")}</p>
                <p>{t("about.text2")}</p>
                <div class="mt-8 flex flex-wrap gap-2">
                    {move || {
                        site.list("about.gear")
                            .into_iter()
                            .map(|g| {
                                view! {
                                    <span class="rounded-full border border-line px-4 py-1.5 text-[0.85rem] text-muted">
                                        {g}
                                    </span>
                                }
                            })
                            .collect_view()
                    }}
                </div>
            </div>
        </section>

        <section class="wrap section-y">
            <p class="eyebrow">"Leistungen"</p>
            <h2>{t("about.services.title")}</h2>
            <div class="mt-10 grid gap-4 sm:grid-cols-2 lg:grid-cols-4">
                {(1..=4)
                    .map(|n| {
                        let (title, text) = (format!("about.service{n}.title"), format!("about.service{n}.text"));
                        view! {
                            <article class="rounded-card border border-line bg-bg-2 p-7 transition duration-300 ease-soft hover:-translate-y-1 hover:border-accent/50">
                                <span class="mb-10 block font-display text-accent">{format!("{n:02}")}</span>
                                <h3>{move || site.text(&title)}</h3>
                                <p class="mb-0 text-[0.95rem] text-muted">{move || site.text(&text)}</p>
                            </article>
                        }
                    })
                    .collect_view()}
            </div>
        </section>

        <section class="wrap section-y pt-0">
            <p class="eyebrow">"Ablauf"</p>
            <h2>{t("about.steps.title")}</h2>
            <ol class="mt-10 grid gap-8 sm:grid-cols-2 lg:grid-cols-4">
                {(1..=4)
                    .map(|n| {
                        let (title, text) = (format!("about.step{n}.title"), format!("about.step{n}.text"));
                        view! {
                            <li class="border-t border-line pt-6">
                                <span class="mb-2.5 block font-display text-[2.6rem] leading-none font-bold text-transparent [-webkit-text-stroke:1px_var(--color-muted)]">
                                    {format!("{n:02}")}
                                </span>
                                <h3>{move || site.text(&title)}</h3>
                                <p class="text-[0.95rem] text-muted">{move || site.text(&text)}</p>
                            </li>
                        }
                    })
                    .collect_view()}
            </ol>
        </section>

        <section class="wrap section-y pt-0">
            <blockquote class="text-center">
                <p class="mx-auto mb-6 max-w-[30ch] font-display text-[clamp(1.5rem,3.2vw,2.4rem)] leading-tight">
                    {t("about.quote")}
                </p>
                <cite class="text-accent not-italic">{t("about.quote.author")}</cite>
            </blockquote>
        </section>

        <CtaBand />
        </Transition>
    }
}
