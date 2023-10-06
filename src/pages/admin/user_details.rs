use actix_session::Session;
use actix_web::{Error, get, HttpResponse, post, web};
use actix_web::web::Path;
use serde_derive::Deserialize;
use serde_json::json;
use tera::Tera;

use crate::database::index::{Database, DatabaseConnection};
use crate::pages::helper::index::{add_absence_entry_for_user, add_time_entry_for_user, delete_absence_entry_for_user, delete_time_entry_for_user, edit_absence_entry_for_user, edit_time_entry_for_user};
use crate::pages::home::{AbsenceDeleteRequest, TimeDeleteRequest};
use crate::pages::index::{AbsenceEntry, prepare_context, render_template, TimeEntry, UserInfo};

#[get("/user_details/{user_id}")]
pub async fn user_details(
    session: Session,
    tmpl: web::Data<Tera>,
    user_id: Path<i64>,
    db: web::Data<DatabaseConnection>,
) -> Result<HttpResponse, Error> {
    let user_id_inner = user_id.into_inner();

    match db.get_user(user_id_inner).await {
        Ok(user_details) => {
            let mut context = prepare_context(&session)?;
            context.insert("user_details", &user_details);

            // Get time entries from the database
            let entries = match db.get_time_entries(user_id_inner).await {
                Ok(entries) => entries,
                Err(_) => return Ok(HttpResponse::InternalServerError().finish()),
            };

            // Calculate total time
            let total_minutes: i64 = entries.iter().map(|entry| entry.spent_time.unwrap_or(0)).sum();
            let total_hours: f64 = (total_minutes as f64) / 60.0;
            let rounded_total_hours: f64 = (total_hours * 100.0).round() / 100.0;

            // Add the time entries and total time to the context
            context.insert("time_entries", &entries);
            context.insert("total_minutes", &total_minutes);
            context.insert("total_hours", &rounded_total_hours);

            // Get absence entries from the database
            let absence_entries = match db.get_absence_entries(user_id_inner).await {
                Ok(absence_entries) => absence_entries,
                Err(_) => return Ok(HttpResponse::InternalServerError().finish()),
            };

            // Calculate total absence days
            let total_absence_days: usize = absence_entries.len();

            // Add the absence entries and total absence days to the context
            context.insert("absence_entries", &absence_entries);
            context.insert("total_absence_days", &total_absence_days);

            render_template(&tmpl, "user_details", &context).await
        }
        Err(_) => Ok(HttpResponse::InternalServerError().into())
    }
}

#[derive(Deserialize)]
pub struct UpdateAuthority {
    user_id: i64,
    new_authority: String,
}

#[post("/user_details/{user_id}/update_authority")]
pub async fn admin_update_authority(
    form: web::Json<UpdateAuthority>,
    db: web::Data<DatabaseConnection>,
    user_id: Path<i64>,
    session: Session,
) -> Result<HttpResponse, Error> {
    let user_id_inner = user_id.into_inner();
    if form.user_id != user_id_inner {
        return Ok(HttpResponse::BadRequest().json(json!({"status": "User ID mismatch"})));
    }

    let result = db.update_authority(form.user_id, &form.new_authority).await;

    match result {
        Ok(_) => {
            if let Some(mut user_info) = session.get::<UserInfo>("user_info")? {
                user_info.authority = Some(form.new_authority.clone());
                session.insert("user_info", user_info)?;
            }

            Ok(HttpResponse::Ok().json(json!({"status": "success"})))
        }
        Err(_) => Ok(HttpResponse::InternalServerError().json(json!({"status": "error"}))),
    }
}


#[post("/user_details/{user_id}/add_time_entry")]
pub async fn admin_add_time_entry(
    db: web::Data<DatabaseConnection>,
    web::Json(entry): web::Json<TimeEntry>,
    user_id: Path<i64>,
) -> Result<HttpResponse, Error> {
    let user_id_inner = user_id.into_inner();

    add_time_entry_for_user(&db, user_id_inner, entry).await
}

#[post("/user_details/{user_id}/edit_time_entry")]
pub async fn admin_edit_time_entry(
    db: web::Data<DatabaseConnection>,
    web::Json(entry): web::Json<TimeEntry>,
) -> Result<HttpResponse, Error> {
    edit_time_entry_for_user(&db, entry).await
}

#[post("/user_details/{user_id}/delete_time_entry")]
pub async fn admin_delete_time_entry(
    db: web::Data<DatabaseConnection>,
    web::Json(entry_delete_request): web::Json<TimeDeleteRequest>,
    user_id: Path<i64>,
) -> Result<HttpResponse, Error> {
    let user_id_inner = user_id.into_inner();

    delete_time_entry_for_user(&db, user_id_inner, entry_delete_request.id).await
}

#[post("/user_details/{user_id}/add_absence_entry")]
pub async fn admin_add_absence_entry(
    db: web::Data<DatabaseConnection>,
    web::Json(entry): web::Json<AbsenceEntry>,
    user_id: Path<i64>,
) -> Result<HttpResponse, Error> {
    let user_id_inner = user_id.into_inner();

    add_absence_entry_for_user(&db, user_id_inner, entry).await
}

#[post("/user_details/{user_id}/edit_absence_entry")]
pub async fn admin_edit_absence_entry(
    db: web::Data<DatabaseConnection>,
    web::Json(entry): web::Json<AbsenceEntry>,
) -> Result<HttpResponse, Error> {
    edit_absence_entry_for_user(&db, entry).await
}

#[post("/user_details/{user_id}/delete_absence_entry")]
pub async fn admin_delete_absence_entry(
    db: web::Data<DatabaseConnection>,
    web::Json(absence_delete_request): web::Json<AbsenceDeleteRequest>,
    user_id: Path<i64>,
) -> Result<HttpResponse, Error> {
    let user_id_inner = user_id.into_inner();

    delete_absence_entry_for_user(&db, user_id_inner, absence_delete_request.id).await
}
