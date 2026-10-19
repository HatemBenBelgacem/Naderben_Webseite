#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use axum::{extract::DefaultBodyLimit, routing::get, Router};
    use leptos::logging::log;
    use leptos::prelude::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use naderben::{app::*, server};
    use std::net::SocketAddr;
    use tower_http::compression::CompressionLayer;

    // SQLite öffnen, Migrationen und Admin-Konto – ohne Datenbank kein Start
    if let Err(e) = server::db::init().await {
        eprintln!("[db] Start fehlgeschlagen: {e}");
        std::process::exit(1);
    }

    let conf = get_configuration(None).unwrap();
    let mut leptos_options = conf.leptos_options;

    // Railway (und andere PaaS) geben den Port über $PORT vor.
    if let Some(port) = std::env::var("PORT").ok().and_then(|p| p.parse::<u16>().ok()) {
        leptos_options.site_addr = SocketAddr::from(([0, 0, 0, 0], port));
    }
    let addr = leptos_options.site_addr;
    let routes = generate_route_list(App);

    let app = Router::new()
        // Hochgeladene Bilder aus der Datenbank
        .route("/media/{id}", get(server::media::serve))
        .leptos_routes(&leptos_options, routes, {
            let leptos_options = leptos_options.clone();
            move || shell(leptos_options.clone())
        })
        .fallback(leptos_axum::file_and_error_handler(shell))
        // Bild-Uploads bis 15 MB (+ Reserve für Formular-Overhead)
        .layer(DefaultBodyLimit::max(server::media::MAX_UPLOAD_BYTES + 1024 * 1024))
        .layer(CompressionLayer::new())
        .with_state(leptos_options);

    log!("listening on http://{}", &addr);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .with_graceful_shutdown(async {
            let _ = tokio::signal::ctrl_c().await;
        })
        .await
        .unwrap();
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // Kein clientseitiges main – Hydration startet über lib.rs
}
