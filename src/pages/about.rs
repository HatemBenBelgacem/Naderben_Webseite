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
            <figure class="group relative md:sticky md:top-[100px]">
                <span class="tape -top-3 left-1/3 z-10 rotate-3"></span>
                <div class="paper-shadow -rotate-2 transition-transform duration-500 group-hover:rotate-0">
                <img
                    class="torn photo-bw aspect-[4/5] w-full object-cover"
                    src=t("about.image")
                    alt=t("about.caption")
                />
                </div>
                <figcaption class="mt-5 inline-block rotate-1 bg-ink px-2 py-1 text-[0.8rem] font-medium text-bg uppercase">{t("about.caption")}</figcaption>
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
                                    <span class="border-2 border-ink bg-zine px-3 py-1 text-[0.8rem] font-medium uppercase">
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
                            <article class="border-2 border-ink bg-bg-2 p-7 transition duration-300 ease-soft hover:-translate-x-1 hover:-translate-y-1 hover:shadow-[6px_6px_0_var(--color-ink)]">
                                <span class="mb-10 inline-block bg-accent px-2 font-display text-2xl font-bold">{format!("{n:02}")}</span>
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
                            <li class="border-t-2 border-ink pt-6">
                                <span class="mb-3 block font-display text-[4rem] leading-none font-bold text-transparent [-webkit-text-stroke:1.5px_var(--color-ink)]">
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
                <p class="mx-auto mb-8 max-w-[24ch] font-display text-[clamp(2rem,4.6vw,3.8rem)] leading-[0.95] font-bold tracking-[-0.03em] uppercase">
                    {t("about.quote")}
                </p>
                <cite class="highlight text-[0.9rem] font-medium uppercase not-italic">{t("about.quote.author")}</cite>
            </blockquote>
        </section>

        <CtaBand />
        </Transition>
    }
}
