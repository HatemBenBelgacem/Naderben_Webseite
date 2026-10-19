//! Server-Funktionen. Jede schreibende Funktion prüft auf dem Server die Rechte
//! des angemeldeten Benutzers – die Oberfläche blendet nur zusätzlich aus.

use leptos::prelude::*;
use leptos::server_fn::codec::{Json, MultipartData, MultipartFormData};

use crate::{
    data::SiteData,
    models::{ContentEntry, MediaInfo, RoleRow, SessionUser, UserRow},
};

#[cfg(feature = "ssr")]
use crate::{
    content::{find_field, FieldKind},
    data::Category,
    models::ADMIN_ROLE_ID,
    server::{
        auth::{self, Perm},
        db::{self, pool},
        db_err, media, user_err,
    },
};

// ───────────── Öffentlich ─────────────

#[server]
pub async fn get_site_data() -> Result<SiteData, ServerFnError> {
    let content: Vec<(String, String)> = sqlx::query_as("SELECT key, value FROM content")
        .fetch_all(pool())
        .await
        .map_err(db_err)?;
    Ok(SiteData {
        content: content.into_iter().collect(),
        photos: db::load_photos().await?,
        videos: db::load_videos().await?,
    })
}

#[server]
pub async fn current_user() -> Result<Option<SessionUser>, ServerFnError> {
    auth::current_user().await
}

// ───────────── Anmeldung & eigenes Konto ─────────────

#[server]
pub async fn login(email: String, password: String) -> Result<SessionUser, ServerFnError> {
    let email = email.trim().to_lowercase();
    let row: Option<(i64, String, bool)> = sqlx::query_as("SELECT id, password_hash, active FROM users WHERE email = ?")
        .bind(&email)
        .fetch_optional(pool())
        .await
        .map_err(db_err)?;

    let user_id = match row {
        Some((id, hash, true)) if auth::verify_password(&password, &hash).await => id,
        _ => {
            auth::failed_login_delay().await;
            return Err(user_err("E-Mail oder Passwort ist falsch."));
        }
    };

    // Das Inhaber-Konto ist immer Admin
    if auth::is_owner(&email) {
        sqlx::query("UPDATE users SET role_id = ? WHERE id = ?")
            .bind(ADMIN_ROLE_ID)
            .bind(user_id)
            .execute(pool())
            .await
            .map_err(db_err)?;
    }

    auth::start_session(user_id).await?;
    auth::user_by_id(user_id).await?.ok_or_else(|| user_err("Anmeldung fehlgeschlagen."))
}

#[server]
pub async fn logout() -> Result<(), ServerFnError> {
    auth::end_session().await
}

#[server]
pub async fn change_password(current_password: String, new_password: String) -> Result<(), ServerFnError> {
    let me = auth::require(Perm::LoggedIn).await?;
    let (hash,): (String,) = sqlx::query_as("SELECT password_hash FROM users WHERE id = ?")
        .bind(me.id)
        .fetch_one(pool())
        .await
        .map_err(db_err)?;
    if !auth::verify_password(&current_password, &hash).await {
        auth::failed_login_delay().await;
        return Err(user_err("Das aktuelle Passwort stimmt nicht."));
    }
    auth::validate_password(&new_password)?;
    let hash = auth::hash_password(&new_password).await?;
    sqlx::query("UPDATE users SET password_hash = ? WHERE id = ?")
        .bind(hash)
        .bind(me.id)
        .execute(pool())
        .await
        .map_err(db_err)?;
    // Andere Geräte abmelden
    auth::end_sessions_of(me.id, true).await
}

// ───────────── Inhalte & Bilder ─────────────

#[server(input = Json)]
pub async fn save_content(entries: Vec<ContentEntry>) -> Result<(), ServerFnError> {
    let me = auth::require(Perm::EditContent).await?;
    let mut replaced_images = Vec::new();

    for entry in &entries {
        let field = find_field(&entry.key).ok_or_else(|| user_err(format!("Unbekanntes Feld: {}", entry.key)))?;
        let value = entry.value.trim();
        if value.chars().count() > 5000 {
            return Err(user_err(format!("«{}» ist zu lang.", field.label)));
        }
        if field.kind == FieldKind::Image {
            media::validate_image_url(value)?;
            let old: Option<(String,)> = sqlx::query_as("SELECT value FROM content WHERE key = ?")
                .bind(field.key)
                .fetch_optional(pool())
                .await
                .map_err(db_err)?;
            if let Some((old,)) = old.filter(|(o,)| o != value) {
                replaced_images.push(old);
            }
        }

        if value == field.default {
            // Standardwert → Eintrag entfernen, damit spätere Änderungen am Standard greifen
            sqlx::query("DELETE FROM content WHERE key = ?").bind(field.key).execute(pool()).await.map_err(db_err)?;
        } else {
            sqlx::query(
                "INSERT INTO content (key, value, updated_by) VALUES (?, ?, ?)
                 ON CONFLICT(key) DO UPDATE SET value = excluded.value, updated_at = datetime('now'), updated_by = excluded.updated_by",
            )
            .bind(field.key)
            .bind(value)
            .bind(me.id)
            .execute(pool())
            .await
            .map_err(db_err)?;
        }
    }

    for url in replaced_images {
        media::cleanup(&url).await?;
    }
    Ok(())
}

/// Bild hochladen (Formularfeld `file`). Gibt die URL `/media/{id}` zurück.
#[server(input = MultipartFormData)]
pub async fn upload_image(data: MultipartData) -> Result<MediaInfo, ServerFnError> {
    auth::require(Perm::EditContent).await?;
    let mut data = data.into_inner().ok_or_else(|| user_err("Ungültiger Upload."))?;

    while let Ok(Some(mut field)) = data.next_field().await {
        if field.name() != Some("file") {
            continue;
        }
        let filename = field.file_name().unwrap_or("bild").to_string();
        let mut bytes = Vec::new();
        while let Ok(Some(chunk)) = field.chunk().await {
            bytes.extend_from_slice(&chunk);
            if bytes.len() > media::MAX_UPLOAD_BYTES {
                return Err(user_err("Das Bild ist grösser als 15 MB."));
            }
        }
        if bytes.is_empty() {
            return Err(user_err("Bitte eine Bilddatei auswählen."));
        }
        return media::store(&filename, bytes).await;
    }
    Err(user_err("Bitte eine Bilddatei auswählen."))
}

// ───────────── Portfolio ─────────────

#[cfg(feature = "ssr")]
fn parse_category(slug: &str) -> Result<Category, ServerFnError> {
    Category::from_slug(slug).ok_or_else(|| user_err("Unbekannter Bereich."))
}

#[cfg(feature = "ssr")]
fn clean_title(title: &str) -> Result<String, ServerFnError> {
    let title = title.trim();
    if title.is_empty() || title.chars().count() > 120 {
        return Err(user_err("Bitte einen Titel mit höchstens 120 Zeichen angeben."));
    }
    Ok(title.to_string())
}

/// Akzeptiert YouTube-Links (watch, youtu.be, embed, shorts) oder die reine Video-ID
#[cfg(feature = "ssr")]
fn parse_youtube_id(input: &str) -> Result<String, ServerFnError> {
    let input = input.trim();
    let candidate = if let Some(pos) = input.find("v=") {
        &input[pos + 2..]
    } else if let Some(pos) = ["youtu.be/", "/embed/", "/shorts/"].iter().find_map(|p| input.find(p).map(|i| i + p.len())) {
        &input[pos..]
    } else {
        input
    };
    let id: String = candidate.chars().take_while(|c| c.is_ascii_alphanumeric() || *c == '-' || *c == '_').collect();
    if id.len() == 11 {
        Ok(id)
    } else {
        Err(user_err("Bitte einen gültigen YouTube-Link oder eine Video-ID angeben."))
    }
}

#[server(input = Json)]
pub async fn add_photo(category: String, title: String, image_url: String, width: Option<u32>, height: Option<u32>) -> Result<(), ServerFnError> {
    auth::require(Perm::EditContent).await?;
    let category = parse_category(&category)?;
    let title = clean_title(&title)?;
    let image_url = image_url.trim().to_string();
    if image_url.is_empty() {
        return Err(user_err("Bitte ein Bild hochladen oder eine Adresse angeben."));
    }
    media::validate_image_url(&image_url)?;
    let order = db::next_sort_order("photos", category.slug()).await?;
    sqlx::query("INSERT INTO photos (category, title, image_url, width, height, sort_order) VALUES (?, ?, ?, ?, ?, ?)")
        .bind(category.slug())
        .bind(title)
        .bind(image_url)
        .bind(width)
        .bind(height)
        .bind(order)
        .execute(pool())
        .await
        .map_err(db_err)?;
    Ok(())
}

#[server(input = Json)]
pub async fn update_photo(id: i64, category: String, title: String) -> Result<(), ServerFnError> {
    auth::require(Perm::EditContent).await?;
    let category = parse_category(&category)?;
    let title = clean_title(&title)?;
    sqlx::query("UPDATE photos SET category = ?, title = ? WHERE id = ?")
        .bind(category.slug())
        .bind(title)
        .bind(id)
        .execute(pool())
        .await
        .map_err(db_err)?;
    Ok(())
}

#[server(input = Json)]
pub async fn delete_photo(id: i64) -> Result<(), ServerFnError> {
    auth::require(Perm::EditContent).await?;
    let row: Option<(String, Option<String>)> = sqlx::query_as("DELETE FROM photos WHERE id = ? RETURNING image_url, thumb_url")
        .bind(id)
        .fetch_optional(pool())
        .await
        .map_err(db_err)?;
    if let Some((image, thumb)) = row {
        media::cleanup(&image).await?;
        if let Some(thumb) = thumb {
            media::cleanup(&thumb).await?;
        }
    }
    Ok(())
}

#[server(input = Json)]
pub async fn move_photo(id: i64, direction: i32) -> Result<(), ServerFnError> {
    auth::require(Perm::EditContent).await?;
    db::move_item("photos", id, direction).await
}

#[server(input = Json)]
pub async fn save_video(id: Option<i64>, category: String, title: String, client: String, duration: String, youtube: String) -> Result<(), ServerFnError> {
    auth::require(Perm::EditContent).await?;
    let category = parse_category(&category)?;
    let title = clean_title(&title)?;
    let youtube_id = parse_youtube_id(&youtube)?;
    let (client, duration) = (client.trim().chars().take(120).collect::<String>(), duration.trim().chars().take(20).collect::<String>());

    match id {
        Some(id) => {
            sqlx::query("UPDATE videos SET category = ?, title = ?, client = ?, duration = ?, youtube_id = ? WHERE id = ?")
                .bind(category.slug())
                .bind(title)
                .bind(client)
                .bind(duration)
                .bind(youtube_id)
                .bind(id)
                .execute(pool())
                .await
                .map_err(db_err)?;
        }
        None => {
            let order = db::next_sort_order("videos", category.slug()).await?;
            sqlx::query("INSERT INTO videos (category, title, client, duration, youtube_id, sort_order) VALUES (?, ?, ?, ?, ?, ?)")
                .bind(category.slug())
                .bind(title)
                .bind(client)
                .bind(duration)
                .bind(youtube_id)
                .bind(order)
                .execute(pool())
                .await
                .map_err(db_err)?;
        }
    }
    Ok(())
}

#[server(input = Json)]
pub async fn delete_video(id: i64) -> Result<(), ServerFnError> {
    auth::require(Perm::EditContent).await?;
    sqlx::query("DELETE FROM videos WHERE id = ?").bind(id).execute(pool()).await.map_err(db_err)?;
    Ok(())
}

#[server(input = Json)]
pub async fn move_video(id: i64, direction: i32) -> Result<(), ServerFnError> {
    auth::require(Perm::EditContent).await?;
    db::move_item("videos", id, direction).await
}

// ───────────── Rollen ─────────────

#[cfg(feature = "ssr")]
type RoleTuple = (i64, String, String, bool, bool, bool, bool, i64);

#[cfg(feature = "ssr")]
async fn load_role(id: i64) -> Result<RoleRow, ServerFnError> {
    let row: Option<RoleTuple> = sqlx::query_as(
        "SELECT r.id, r.name, r.description, r.can_edit_content, r.can_manage_users, r.can_manage_roles, r.is_system,
                (SELECT COUNT(*) FROM users u WHERE u.role_id = r.id)
         FROM roles r WHERE r.id = ?",
    )
    .bind(id)
    .fetch_optional(pool())
    .await
    .map_err(db_err)?;
    row.map(to_role).ok_or_else(|| user_err("Rolle nicht gefunden."))
}

#[cfg(feature = "ssr")]
fn to_role((id, name, description, edit, users, roles, is_system, user_count): RoleTuple) -> RoleRow {
    RoleRow { id, name, description, can_edit_content: edit, can_manage_users: users, can_manage_roles: roles, is_system, user_count }
}

/// Niemand darf Rechte vergeben, die er selbst nicht hat
#[cfg(feature = "ssr")]
fn ensure_no_escalation(me: &SessionUser, edit: bool, users: bool, roles: bool) -> Result<(), ServerFnError> {
    if (edit && !me.can_edit_content) || (users && !me.can_manage_users) || (roles && !me.can_manage_roles) {
        return Err(user_err("Du kannst keine Rechte vergeben, die du selbst nicht hast."));
    }
    Ok(())
}

#[cfg(feature = "ssr")]
fn clean_role_name(name: &str) -> Result<String, ServerFnError> {
    let name = name.trim();
    if name.is_empty() || name.chars().count() > 40 {
        return Err(user_err("Bitte einen Rollennamen mit höchstens 40 Zeichen angeben."));
    }
    Ok(name.to_string())
}

#[cfg(feature = "ssr")]
fn unique_violation(e: sqlx::Error, msg: &str) -> ServerFnError {
    match &e {
        sqlx::Error::Database(d) if d.is_unique_violation() => user_err(msg),
        _ => db_err(e),
    }
}

#[server]
pub async fn list_roles() -> Result<Vec<RoleRow>, ServerFnError> {
    auth::require(Perm::UsersOrRoles).await?;
    let rows: Vec<RoleTuple> = sqlx::query_as(
        "SELECT r.id, r.name, r.description, r.can_edit_content, r.can_manage_users, r.can_manage_roles, r.is_system,
                (SELECT COUNT(*) FROM users u WHERE u.role_id = r.id)
         FROM roles r ORDER BY r.id",
    )
    .fetch_all(pool())
    .await
    .map_err(db_err)?;
    Ok(rows.into_iter().map(to_role).collect())
}

#[server(input = Json)]
pub async fn save_role(
    id: Option<i64>,
    name: String,
    description: String,
    can_edit_content: bool,
    can_manage_users: bool,
    can_manage_roles: bool,
) -> Result<(), ServerFnError> {
    let me = auth::require(Perm::ManageRoles).await?;
    let name = clean_role_name(&name)?;
    let description: String = description.trim().chars().take(200).collect();

    match id {
        Some(ADMIN_ROLE_ID) => {
            // Admin behält Namen und alle Rechte – nur die Beschreibung ist änderbar
            sqlx::query("UPDATE roles SET description = ? WHERE id = ?")
                .bind(description)
                .bind(ADMIN_ROLE_ID)
                .execute(pool())
                .await
                .map_err(db_err)?;
        }
        Some(id) => {
            let current = load_role(id).await?;
            // Nur neu hinzukommende Rechte prüfen
            ensure_no_escalation(
                &me,
                can_edit_content && !current.can_edit_content,
                can_manage_users && !current.can_manage_users,
                can_manage_roles && !current.can_manage_roles,
            )?;
            sqlx::query(
                "UPDATE roles SET name = ?, description = ?, can_edit_content = ?, can_manage_users = ?, can_manage_roles = ? WHERE id = ?",
            )
            .bind(name)
            .bind(description)
            .bind(can_edit_content)
            .bind(can_manage_users)
            .bind(can_manage_roles)
            .bind(id)
            .execute(pool())
            .await
            .map_err(|e| unique_violation(e, "Eine Rolle mit diesem Namen existiert bereits."))?;
        }
        None => {
            ensure_no_escalation(&me, can_edit_content, can_manage_users, can_manage_roles)?;
            sqlx::query(
                "INSERT INTO roles (name, description, can_edit_content, can_manage_users, can_manage_roles) VALUES (?, ?, ?, ?, ?)",
            )
            .bind(name)
            .bind(description)
            .bind(can_edit_content)
            .bind(can_manage_users)
            .bind(can_manage_roles)
            .execute(pool())
            .await
            .map_err(|e| unique_violation(e, "Eine Rolle mit diesem Namen existiert bereits."))?;
        }
    }
    Ok(())
}

#[server(input = Json)]
pub async fn delete_role(id: i64) -> Result<(), ServerFnError> {
    auth::require(Perm::ManageRoles).await?;
    let role = load_role(id).await?;
    if role.is_system {
        return Err(user_err("System-Rollen können nicht gelöscht werden."));
    }
    if role.user_count > 0 {
        return Err(user_err("Die Rolle ist noch Benutzern zugewiesen."));
    }
    sqlx::query("DELETE FROM roles WHERE id = ?").bind(id).execute(pool()).await.map_err(db_err)?;
    Ok(())
}

// ───────────── Benutzer ─────────────

#[cfg(feature = "ssr")]
type UserTuple = (i64, String, String, i64, String, bool, String);

#[cfg(feature = "ssr")]
const USER_LIST_SELECT: &str = "SELECT u.id, u.email, u.name, u.role_id, r.name, u.active, u.created_at
     FROM users u JOIN roles r ON r.id = u.role_id";

#[cfg(feature = "ssr")]
fn to_user_row((id, email, name, role_id, role_name, active, created_at): UserTuple) -> UserRow {
    UserRow { id, email, name, role_id, role_name, active, created_at }
}

#[cfg(feature = "ssr")]
async fn load_user(id: i64) -> Result<UserRow, ServerFnError> {
    let row: Option<UserTuple> = sqlx::query_as(&format!("{USER_LIST_SELECT} WHERE u.id = ?"))
        .bind(id)
        .fetch_optional(pool())
        .await
        .map_err(db_err)?;
    row.map(to_user_row).ok_or_else(|| user_err("Benutzer nicht gefunden."))
}

/// Rolle darf nur vergeben werden, wenn sie keine Rechte über die eigenen hinaus hat
#[cfg(feature = "ssr")]
async fn check_assignable_role(me: &SessionUser, role_id: i64) -> Result<(), ServerFnError> {
    let role = load_role(role_id).await?;
    ensure_no_escalation(me, role.can_edit_content, role.can_manage_users, role.can_manage_roles)
}

#[cfg(feature = "ssr")]
fn clean_name(name: &str) -> Result<String, ServerFnError> {
    let name = name.trim();
    if name.is_empty() || name.chars().count() > 80 {
        return Err(user_err("Bitte einen Namen mit höchstens 80 Zeichen angeben."));
    }
    Ok(name.to_string())
}

#[cfg(feature = "ssr")]
fn clean_email(email: &str) -> Result<String, ServerFnError> {
    let email = email.trim().to_lowercase();
    let valid = email.len() <= 200
        && email
            .split_once('@')
            .is_some_and(|(user, domain)| !user.is_empty() && domain.contains('.') && !domain.starts_with('.') && !domain.ends_with('.'));
    if valid {
        Ok(email)
    } else {
        Err(user_err("Bitte eine gültige E-Mail-Adresse angeben."))
    }
}

#[server]
pub async fn list_users() -> Result<Vec<UserRow>, ServerFnError> {
    auth::require(Perm::ManageUsers).await?;
    let rows: Vec<UserTuple> = sqlx::query_as(&format!("{USER_LIST_SELECT} ORDER BY u.name COLLATE NOCASE"))
        .fetch_all(pool())
        .await
        .map_err(db_err)?;
    Ok(rows.into_iter().map(to_user_row).collect())
}

#[server(input = Json)]
pub async fn create_user(name: String, email: String, role_id: i64, password: String) -> Result<(), ServerFnError> {
    let me = auth::require(Perm::ManageUsers).await?;
    let name = clean_name(&name)?;
    let email = clean_email(&email)?;
    auth::validate_password(&password)?;
    let role_id = if auth::is_owner(&email) { ADMIN_ROLE_ID } else { role_id };
    check_assignable_role(&me, role_id).await?;

    let hash = auth::hash_password(&password).await?;
    sqlx::query("INSERT INTO users (email, name, password_hash, role_id) VALUES (?, ?, ?, ?)")
        .bind(email)
        .bind(name)
        .bind(hash)
        .bind(role_id)
        .execute(pool())
        .await
        .map_err(|e| unique_violation(e, "Ein Benutzer mit dieser E-Mail existiert bereits."))?;
    Ok(())
}

#[server(input = Json)]
pub async fn update_user(id: i64, name: String, role_id: i64, active: bool) -> Result<(), ServerFnError> {
    let me = auth::require(Perm::ManageUsers).await?;
    let user = load_user(id).await?;
    let name = clean_name(&name)?;

    if user.is_owner() {
        if me.id != user.id && name != user.name {
            return Err(user_err("Nur der Inhaber selbst kann seinen Namen ändern."));
        }
        if role_id != ADMIN_ROLE_ID || !active {
            return Err(user_err("Das Inhaber-Konto bleibt immer aktiver Admin."));
        }
    }
    if me.id == id && !active {
        return Err(user_err("Du kannst dich nicht selbst deaktivieren."));
    }
    if role_id != user.role_id {
        check_assignable_role(&me, role_id).await?;
        // Auch wer bereits mehr Rechte hat, darf nur von Gleichberechtigten herabgestuft werden
        check_assignable_role(&me, user.role_id).await?;
    }

    sqlx::query("UPDATE users SET name = ?, role_id = ?, active = ? WHERE id = ?")
        .bind(name)
        .bind(role_id)
        .bind(active)
        .bind(id)
        .execute(pool())
        .await
        .map_err(db_err)?;
    if !active {
        auth::end_sessions_of(id, false).await?;
    }
    Ok(())
}

#[server(input = Json)]
pub async fn set_user_password(id: i64, password: String) -> Result<(), ServerFnError> {
    let me = auth::require(Perm::ManageUsers).await?;
    let user = load_user(id).await?;
    if user.is_owner() && me.id != user.id {
        return Err(user_err("Das Passwort des Inhabers kann nur er selbst ändern."));
    }
    check_assignable_role(&me, user.role_id).await?;
    auth::validate_password(&password)?;
    let hash = auth::hash_password(&password).await?;
    sqlx::query("UPDATE users SET password_hash = ? WHERE id = ?")
        .bind(hash)
        .bind(id)
        .execute(pool())
        .await
        .map_err(db_err)?;
    auth::end_sessions_of(id, me.id == id).await
}

#[server(input = Json)]
pub async fn delete_user(id: i64) -> Result<(), ServerFnError> {
    let me = auth::require(Perm::ManageUsers).await?;
    let user = load_user(id).await?;
    if user.is_owner() {
        return Err(user_err("Das Inhaber-Konto kann nicht gelöscht werden."));
    }
    if me.id == id {
        return Err(user_err("Du kannst dich nicht selbst löschen."));
    }
    check_assignable_role(&me, user.role_id).await?;
    sqlx::query("DELETE FROM users WHERE id = ?").bind(id).execute(pool()).await.map_err(db_err)?;
    Ok(())
}
