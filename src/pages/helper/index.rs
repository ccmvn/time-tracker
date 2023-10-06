use actix_web::{Error, HttpResponse};

use crate::database::index::{Database, DatabaseConnection};
use crate::pages::index::{AbsenceEntry, TimeEntry};

pub async fn add_time_entry_for_user(db: &DatabaseConnection, user_id: i64, entry: TimeEntry) -> Result<HttpResponse, Error> {
    match db.add_time_entry(user_id, entry.task.as_deref(), entry.spent_time, entry.date.as_deref()).await {
        Ok(_) => Ok(HttpResponse::Ok().json(serde_json::json!({"status": "ok"}))),
        Err(_) => Ok(HttpResponse::InternalServerError().json(serde_json::json!({"status": "error"}))),
    }
}

pub async fn edit_time_entry_for_user(db: &DatabaseConnection, entry: TimeEntry) -> Result<HttpResponse, Error> {
    let entry_id = match entry.id {
        Some(id) => id,
        None => return Ok(HttpResponse::BadRequest().json(serde_json::json!({"status": "entry id missing"}))),
    };

    let spent_time = match entry.spent_time {
        Some(time) => time,
        None => return Ok(HttpResponse::BadRequest().json(serde_json::json!({"status": "spent time missing"}))),
    };

    let task_str = entry.task.as_deref().unwrap_or_default();
    let date_str = entry.date.as_deref().unwrap_or_default();

    match db.update_time_entry(entry_id, task_str, spent_time, date_str).await {
        Ok(_) => Ok(HttpResponse::Ok().json(serde_json::json!({"status": "ok"}))),
        Err(_) => Ok(HttpResponse::InternalServerError().json(serde_json::json!({"status": "error"}))),
    }
}

pub async fn delete_time_entry_for_user(db: &DatabaseConnection, user_id: i64, entry_id: i64) -> Result<HttpResponse, Error> {
    match db.delete_time_entry(user_id, entry_id.into()).await {
        Ok(_) => Ok(HttpResponse::Ok().json(serde_json::json!({"status": "ok"}))),
        Err(_) => Ok(HttpResponse::InternalServerError().json(serde_json::json!({"status": "error"}))),
    }
}

pub async fn add_absence_entry_for_user(db: &DatabaseConnection, user_id: i64, entry: AbsenceEntry) -> Result<HttpResponse, Error> {
    match db.add_absence_entry(user_id, entry.absence_date.as_str(), entry.reason.as_deref()).await {
        Ok(_) => Ok(HttpResponse::Ok().json(serde_json::json!({"status": "ok"}))),
        Err(_) => Ok(HttpResponse::InternalServerError().json(serde_json::json!({"status": "error"}))),
    }
}

pub async fn edit_absence_entry_for_user(db: &DatabaseConnection, entry: AbsenceEntry) -> Result<HttpResponse, Error> {
    let absence_id = match entry.id {
        Some(id) => id,
        None => return Ok(HttpResponse::BadRequest().json(serde_json::json!({"status": "entry id missing"}))),
    };

    let absence_date_str = entry.absence_date.as_str();
    let reason_str = entry.reason.as_deref().unwrap_or_default();

    match db.update_absence_entry(absence_id, absence_date_str, reason_str).await {
        Ok(_) => Ok(HttpResponse::Ok().json(serde_json::json!({"status": "ok"}))),
        Err(_) => Ok(HttpResponse::InternalServerError().json(serde_json::json!({"status": "error"}))),
    }
}

pub async fn delete_absence_entry_for_user(db: &DatabaseConnection, user_id: i64, absence_id: i64) -> Result<HttpResponse, Error> {
    match db.delete_absence_entry(user_id, absence_id).await {
        Ok(_) => Ok(HttpResponse::Ok().json(serde_json::json!({"status": "ok"}))),
        Err(_) => Ok(HttpResponse::InternalServerError().json(serde_json::json!({"status": "error"}))),
    }
}
