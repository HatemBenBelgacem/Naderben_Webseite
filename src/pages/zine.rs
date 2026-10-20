//! Gestaltungs-Bausteine im Collage-/Zine-Stil

use leptos::prelude::*;

/// Riesige Ton-in-Ton-Schrift im Hintergrund, Zeilen versetzt (statisch – sehr breite
/// animierte Ebenen überfordern das Compositing im Browser).
/// `words` wird pro Zeile wiederholt; rein dekorativ (aria-hidden).
#[component]
pub fn BgWords(#[prop(into)] words: Signal<Vec<String>>, #[prop(default = 4)] rows: usize, #[prop(optional)] class: &'static str) -> impl IntoView {
    view! {
        <div
            class=format!("pointer-events-none absolute inset-0 flex flex-col justify-center overflow-hidden select-none {class}")
            aria-hidden="true"
        >
            {(0..rows)
                .map(|row| {
                    // Jede Zeile beginnt bei einem anderen Wort
                    let line = move || {
                        let w = words.get();
                        if w.is_empty() {
                            return String::new();
                        }
                        let n = w.len();
                        let rotated: Vec<&String> = (0..n).map(|i| &w[(i + row * 2) % n]).collect();
                        let one = rotated.iter().map(|s| s.as_str()).collect::<Vec<_>>().join(" ");
                        format!("{one} {one}")
                    };
                    view! {
                        <div
                            class="w-max font-display text-[clamp(4.5rem,13vw,13rem)] leading-[0.8] font-bold tracking-[-0.045em] whitespace-nowrap text-deep uppercase"
                            style=format!("margin-left: -{}%", 8 + row * 13)
                        >
                            {line}
                        </div>
                    }
                })
                .collect_view()}
        </div>
    }
}
