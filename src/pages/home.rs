use actix_session::Session;
use actix_web::{Error, get, HttpResponse, post, web};
use serde_derive::Deserialize;
use tera::Tera;

use crate::database::index::{Database, DatabaseConnection};
use crate::pages::helper::index::{add_absence_entry_for_user, add_time_entry_for_user, delete_absence_entry_for_user, delete_time_entry_for_user, edit_absence_entry_for_user, edit_time_entry_for_user};
use crate::pages::index::{AbsenceEntry, prepare_context, render_template, TimeEntry, UserInfo};

#[get("/home")]
pub async fn home(session: Session, tmpl: web::Data<Tera>, db: web::Data<DatabaseConnection>) -> Result<HttpResponse, Error> {
    // Get the existing context
    let mut context = prepare_context(&session)?;

    // Get user_id from session
    let user_id = match session.get::<UserInfo>("user_info")? {
        Some(user_info) => user_info.id,
        None => return Ok(HttpResponse::Unauthorized().finish()),
    };

    // Get time entries from the database
    let entries = match db.get_time_entries(user_id).await {
        Ok(entries) => entries,
        Err(_) => return Ok(HttpResponse::InternalServerError().finish()),
    };

    // Calculate total time
    let total_minutes: i64 = entries.iter().map(|entry| entry.spent_time.unwrap_or(0)).sum();
    let total_hours: f64 = (total_minutes as f64) / 60.0;
    let rounded_total_hours: f64 = (total_hours * 100.0).round() / 100.0;

    // Add the time entries to the context
    context.insert("time_entries", &entries);
    context.insert("total_minutes", &total_minutes);
    context.insert("total_hours", &rounded_total_hours);

    // Get absence entries from the database
    let absence_entries = match db.get_absence_entries(user_id).await {
        Ok(absence_entries) => absence_entries,
        Err(_) => return Ok(HttpResponse::InternalServerError().finish()),
    };

    // Calculate total absence days
    let total_absence_days: usize = absence_entries.len();

    // Add the absence entries to the context
    context.insert("absence_entries", &absence_entries);
    context.insert("total_absence_days", &total_absence_days);

    render_template(&tmpl, "home", &context).await
}

#[post("/add_time_entry")]
async fn add_time_entry(
    db: web::Data<DatabaseConnection>,
    web::Json(entry): web::Json<TimeEntry>,
    session: Session,
) -> Result<HttpResponse, Error> {
    let user_id = match session.get::<UserInfo>("user_info")? {
        Some(user_info) => user_info.id,
        None => return Ok(HttpResponse::Unauthorized().finish()),
    };

    add_time_entry_for_user(&db, user_id, entry).await
}

#[post("/edit_time_entry")]
async fn edit_time_entry(
    db: web::Data<DatabaseConnection>,
    web::Json(entry): web::Json<TimeEntry>,
) -> Result<HttpResponse, Error> {
    edit_time_entry_for_user(&db, entry).await
}

#[derive(Deserialize)]
pub struct TimeDeleteRequest {
    pub id: i64,
}

#[post("/delete_time_entry")]
async fn delete_time_entry(
    db: web::Data<DatabaseConnection>,
    web::Json(entry_delete_request): web::Json<TimeDeleteRequest>,
    session: Session,
) -> Result<HttpResponse, Error> {
    let user_id = match session.get::<UserInfo>("user_info")? {
        Some(user_info) => user_info.id,
        None => return Ok(HttpResponse::Unauthorized().finish()),
    };

    delete_time_entry_for_user(&db, user_id, entry_delete_request.id).await
}

#[post("/add_absence_entry")]
async fn add_absence_entry(
    db: web::Data<DatabaseConnection>,
    web::Json(entry): web::Json<AbsenceEntry>,
    session: Session,
) -> Result<HttpResponse, Error> {
    let user_id = match session.get::<UserInfo>("user_info")? {
        Some(user_info) => user_info.id,
        None => return Ok(HttpResponse::Unauthorized().finish()),
    };

    add_absence_entry_for_user(&db, user_id, entry).await
}

#[post("/edit_absence_entry")]
async fn edit_absence_entry(
    db: web::Data<DatabaseConnection>,
    web::Json(entry): web::Json<AbsenceEntry>,
) -> Result<HttpResponse, Error> {
    edit_absence_entry_for_user(&db, entry).await
}

#[derive(Deserialize)]
pub struct AbsenceDeleteRequest {
    pub id: i64,
}

#[post("/delete_absence_entry")]
async fn delete_absence_entry(
    db: web::Data<DatabaseConnection>,
    web::Json(absence_delete_request): web::Json<AbsenceDeleteRequest>,
    session: Session,
) -> Result<HttpResponse, Error> {
    let user_id = match session.get::<UserInfo>("user_info")? {
        Some(user_info) => user_info.id,
        None => return Ok(HttpResponse::Unauthorized().finish()),
    };

    delete_absence_entry_for_user(&db, user_id, absence_delete_request.id).await
}
