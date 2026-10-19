use leptos::prelude::*;

use super::{
    guard,
    ui::{confirm, watch_action, ImageUpload, Status, StatusLine, BTN_DANGER, BTN_SAVE, BTN_SMALL, CARD, INPUT, LABEL, LABEL_TEXT},
};
use crate::{
    api::{AddPhoto, DeletePhoto, DeleteVideo, MovePhoto, MoveVideo, SaveVideo, UpdatePhoto},
    content::use_site,
    data::{Category, PhotoItem, VideoItem},
    models::MediaInfo,
};

const CAT_TAB: &str = "rounded-lg px-3 py-1.5 text-[0.9rem] transition-colors";

/// Alle Portfolio-Aktionen an einem Ort, damit Unterkomponenten sie nutzen können
#[derive(Clone, Copy)]
struct Actions {
    add_photo: ServerAction<AddPhoto>,
    update_photo: ServerAction<UpdatePhoto>,
    delete_photo: ServerAction<DeletePhoto>,
    move_photo: ServerAction<MovePhoto>,
    save_video: ServerAction<SaveVideo>,
    delete_video: ServerAction<DeleteVideo>,
    move_video: ServerAction<MoveVideo>,
    status: Status,
}

/// `/admin/portfolio` – Fotos und Videos je Bereich
#[component]
pub fn PortfolioEditor() -> impl IntoView {
    guard(|u| u.can_edit_content, || view! { <Editor /> }.into_any())
}

#[component]
fn Editor() -> impl IntoView {
    let site = use_site();
    let (category, set_category) = signal(Category::Kuenstler);
    let actions = Actions {
        add_photo: ServerAction::new(),
        update_photo: ServerAction::new(),
        delete_photo: ServerAction::new(),
        move_photo: ServerAction::new(),
        save_video: ServerAction::new(),
        delete_video: ServerAction::new(),
        move_video: ServerAction::new(),
        status: RwSignal::new(None),
    };
    let reload = move || site.data.refetch();
    watch_action!(actions.add_photo, actions.status, "Foto hinzugefügt.", reload);
    watch_action!(actions.update_photo, actions.status, "Foto gespeichert.", reload);
    watch_action!(actions.delete_photo, actions.status, "Foto gelöscht.", reload);
    watch_action!(actions.move_photo, actions.status, "Reihenfolge geändert.", reload);
    watch_action!(actions.save_video, actions.status, "Video gespeichert.", reload);
    watch_action!(actions.delete_video, actions.status, "Video gelöscht.", reload);
    watch_action!(actions.move_video, actions.status, "Reihenfolge geändert.", reload);

    let photos = move || site.with_data(|d| d.photos_in(category.get()).cloned().collect::<Vec<_>>());
    let videos = move || site.with_data(|d| d.videos_in(category.get()).cloned().collect::<Vec<_>>());

    view! {
        <div class="flex flex-col gap-8">
            <div class="flex flex-wrap items-center gap-2">
                {Category::ALL
                    .map(|c| {
                        view! {
                            <button
                                class=move || format!(
                                    "{CAT_TAB} {}",
                                    if category.get() == c { "bg-accent text-accent-ink" } else { "border border-line text-muted hover:text-ink" },
                                )
                                on:click=move |_| {
                                    actions.status.set(None);
                                    set_category.set(c);
                                }
                            >
                                {c.name()}
                            </button>
                        }
                    })
                    .collect_view()}
                <a class="ml-auto text-[0.9rem] text-accent hover:underline" href=move || format!("{}", category.get().href()) target="_blank">
                    "Bereich ansehen ↗"
                </a>
            </div>

            <div class="sticky top-[90px] z-10 empty:hidden">
                <StatusLine status=actions.status />
            </div>

            <section class="flex flex-col gap-4">
                <h3 class="mb-0">"Fotos"</h3>
                <NewPhoto category actions />
                <div class="grid gap-4 sm:grid-cols-2 xl:grid-cols-3">
                    <For
                        each=photos
                        key=|p| (p.id, p.title.clone(), p.image_url.clone())
                        children=move |p| view! { <PhotoCard photo=p actions /> }
                    />
                </div>
                {move || photos().is_empty().then_some(view! { <p class="text-muted">"Noch keine Fotos in diesem Bereich."</p> })}
            </section>

            <section class="flex flex-col gap-4">
                <h3 class="mb-0">"Videos"</h3>
                <VideoForm video=None category actions />
                <For
                    each=videos
                    key=|v| (v.id, v.title.clone(), v.client.clone(), v.duration.clone(), v.youtube_id.clone())
                    children=move |v| view! { <VideoForm video=Some(v) category actions /> }
                />
                {move || videos().is_empty().then_some(view! { <p class="text-muted">"Noch keine Videos in diesem Bereich."</p> })}
            </section>
        </div>
    }
}

#[component]
fn NewPhoto(category: ReadSignal<Category>, actions: Actions) -> impl IntoView {
    let (title, set_title) = signal(String::new());
    let (image, set_image) = signal(None::<MediaInfo>);
    let (url, set_url) = signal(String::new());

    // Nach dem Hinzufügen Formular leeren
    Effect::new(move |_| {
        if let Some(Ok(())) = actions.add_photo.value().get() {
            set_title.set(String::new());
            set_image.set(None);
            set_url.set(String::new());
        }
    });

    let preview = move || image.get().map(|i| i.url).unwrap_or_else(|| url.get());
    let add = move |_| {
        let info = image.get_untracked();
        actions.add_photo.dispatch(AddPhoto {
            category: category.get_untracked().slug().to_string(),
            title: title.get_untracked(),
            image_url: info.as_ref().map(|i| i.url.clone()).unwrap_or_else(|| url.get_untracked()),
            width: info.as_ref().and_then(|i| i.width),
            height: info.as_ref().and_then(|i| i.height),
        });
    };

    view! {
        <div class=format!("{CARD} flex flex-col gap-4 sm:flex-row")>
            <div class="flex aspect-[4/3] w-full shrink-0 items-center justify-center overflow-hidden rounded-xl border border-dashed border-line bg-bg sm:w-44">
                {move || {
                    let src = preview();
                    if src.is_empty() {
                        view! { <span class="text-[0.8rem] text-muted">"Neues Foto"</span> }.into_any()
                    } else {
                        view! { <img class="size-full object-cover" src=src alt="Vorschau" /> }.into_any()
                    }
                }}
            </div>
            <div class="flex w-full flex-col gap-3">
                <label class=LABEL>
                    <span class=LABEL_TEXT>"Titel"</span>
                    <input class=INPUT type="text" placeholder="z. B. Golden Hour" prop:value=title on:input=move |e| set_title.set(event_target_value(&e)) />
                </label>
                <div class="flex flex-wrap items-center gap-3">
                    <ImageUpload on_uploaded=move |info: MediaInfo| set_image.set(Some(info)) />
                    <span class="text-[0.8rem] text-muted">"oder"</span>
                    <input
                        class=format!("{INPUT} sm:max-w-xs")
                        type="url"
                        placeholder="https://… (externe Bildadresse)"
                        prop:value=url
                        on:input=move |e| {
                            set_image.set(None);
                            set_url.set(event_target_value(&e));
                        }
                    />
                </div>
                <div>
                    <button class=BTN_SAVE on:click=add disabled=move || actions.add_photo.pending().get() || preview().is_empty()>
                        "Foto hinzufügen"
                    </button>
                </div>
            </div>
        </div>
    }
}

#[component]
fn CategorySelect(value: ReadSignal<Category>, set_value: WriteSignal<Category>) -> impl IntoView {
    view! {
        <select
            class=INPUT
            on:change=move |e| {
                if let Some(c) = Category::from_slug(&event_target_value(&e)) {
                    set_value.set(c);
                }
            }
        >
            {Category::ALL
                .map(|c| view! { <option value=c.slug() selected=move || value.get() == c>{c.name()}</option> })
                .collect_view()}
        </select>
    }
}

#[component]
fn PhotoCard(photo: PhotoItem, actions: Actions) -> impl IntoView {
    let id = photo.id;
    let (title, set_title) = signal(photo.title.clone());
    let (category, set_category) = signal(photo.category);
    let (orig_title, orig_category) = (photo.title.clone(), photo.category);
    let changed = move || title.get() != orig_title || category.get() != orig_category;

    view! {
        <div class=format!("{CARD} flex flex-col gap-3 p-4 sm:p-4")>
            <img class="aspect-[4/3] w-full rounded-xl object-cover" src=photo.thumb().to_string() alt=photo.title.clone() loading="lazy" />
            <input class=INPUT type="text" prop:value=title on:input=move |e| set_title.set(event_target_value(&e)) aria-label="Titel" />
            <CategorySelect value=category set_value=set_category />
            <div class="flex flex-wrap gap-2">
                <button class=BTN_SMALL title="Nach vorne" on:click=move |_| { actions.move_photo.dispatch(MovePhoto { id, direction: -1 }); }>"←"</button>
                <button class=BTN_SMALL title="Nach hinten" on:click=move |_| { actions.move_photo.dispatch(MovePhoto { id, direction: 1 }); }>"→"</button>
                <button
                    class=BTN_SMALL
                    disabled=move || !changed()
                    on:click=move |_| {
                        actions.update_photo.dispatch(UpdatePhoto {
                            id,
                            category: category.get_untracked().slug().to_string(),
                            title: title.get_untracked(),
                        });
                    }
                >
                    "Speichern"
                </button>
                <button
                    class=format!("{BTN_DANGER} ml-auto")
                    on:click=move |_| {
                        if confirm("Dieses Foto wirklich löschen?") {
                            actions.delete_photo.dispatch(DeletePhoto { id });
                        }
                    }
                >
                    "Löschen"
                </button>
            </div>
        </div>
    }
}

/// Neues Video (video = None) oder bestehendes bearbeiten
#[component]
fn VideoForm(video: Option<VideoItem>, category: ReadSignal<Category>, actions: Actions) -> impl IntoView {
    let id = video.as_ref().map(|v| v.id);
    let initial = video.clone();
    let field = move |f: fn(&VideoItem) -> String| initial.as_ref().map(f).unwrap_or_default();
    let (title, set_title) = signal(field(|v| v.title.clone()));
    let (client, set_client) = signal(field(|v| v.client.clone()));
    let (duration, set_duration) = signal(field(|v| v.duration.clone()));
    let (youtube, set_youtube) = signal(field(|v| v.youtube_id.clone()));
    let (cat, set_cat) = signal(video.as_ref().map(|v| v.category).unwrap_or(category.get_untracked()));

    // Neues Video: Bereich folgt der Auswahl oben; nach dem Speichern leeren
    if id.is_none() {
        Effect::new(move |_| set_cat.set(category.get()));
        Effect::new(move |_| {
            if let Some(Ok(())) = actions.save_video.value().get() {
                for s in [set_title, set_client, set_duration, set_youtube] {
                    s.set(String::new());
                }
            }
        });
    }

    let thumb = move || {
        let y = youtube.get();
        (y.len() == 11).then(|| format!("https://i.ytimg.com/vi/{y}/hqdefault.jpg"))
    };
    let save = move |_| {
        actions.save_video.dispatch(SaveVideo {
            id,
            category: cat.get_untracked().slug().to_string(),
            title: title.get_untracked(),
            client: client.get_untracked(),
            duration: duration.get_untracked(),
            youtube: youtube.get_untracked(),
        });
    };

    view! {
        <div class=format!("{CARD} flex flex-col gap-4 md:flex-row {}", if id.is_none() { "border-dashed" } else { "" })>
            <div class="flex aspect-video w-full shrink-0 items-center justify-center overflow-hidden rounded-xl border border-line bg-bg md:w-56">
                {move || match thumb() {
                    Some(src) => view! { <img class="size-full object-cover" src=src alt="Vorschau" /> }.into_any(),
                    None => view! { <span class="text-[0.8rem] text-muted">{if id.is_none() { "Neues Video" } else { "Vorschau" }}</span> }.into_any(),
                }}
            </div>
            <div class="grid w-full gap-3 sm:grid-cols-2">
                <label class=LABEL>
                    <span class=LABEL_TEXT>"Titel"</span>
                    <input class=INPUT type="text" prop:value=title on:input=move |e| set_title.set(event_target_value(&e)) />
                </label>
                <label class=LABEL>
                    <span class=LABEL_TEXT>"YouTube-Link oder Video-ID"</span>
                    <input class=INPUT type="text" placeholder="https://youtu.be/…" prop:value=youtube on:input=move |e| set_youtube.set(event_target_value(&e)) />
                </label>
                <label class=LABEL>
                    <span class=LABEL_TEXT>"Kunde"</span>
                    <input class=INPUT type="text" prop:value=client on:input=move |e| set_client.set(event_target_value(&e)) />
                </label>
                <label class=LABEL>
                    <span class=LABEL_TEXT>"Dauer"</span>
                    <input class=INPUT type="text" placeholder="3:45" prop:value=duration on:input=move |e| set_duration.set(event_target_value(&e)) />
                </label>
                {id.map(|_| view! {
                    <label class=LABEL>
                        <span class=LABEL_TEXT>"Bereich"</span>
                        <CategorySelect value=cat set_value=set_cat />
                    </label>
                })}
                <div class="flex flex-wrap items-end gap-2 sm:col-span-2">
                    {match id {
                        None => view! {
                            <button class=BTN_SAVE on:click=save disabled=move || actions.save_video.pending().get()>"Video hinzufügen"</button>
                        }
                        .into_any(),
                        Some(id) => view! {
                            <button class=BTN_SMALL title="Nach oben" on:click=move |_| { actions.move_video.dispatch(MoveVideo { id, direction: -1 }); }>"↑"</button>
                            <button class=BTN_SMALL title="Nach unten" on:click=move |_| { actions.move_video.dispatch(MoveVideo { id, direction: 1 }); }>"↓"</button>
                            <button class=BTN_SMALL on:click=save>"Speichern"</button>
                            <button
                                class=format!("{BTN_DANGER} ml-auto")
                                on:click=move |_| {
                                    if confirm("Dieses Video wirklich löschen?") {
                                        actions.delete_video.dispatch(DeleteVideo { id });
                                    }
                                }
                            >
                                "Löschen"
                            </button>
                        }
                        .into_any(),
                    }}
                </div>
            </div>
        </div>
    }
}
