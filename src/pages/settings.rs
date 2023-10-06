use actix_session::Session;
use actix_web::{Error, get, HttpResponse, post, web};
use bcrypt::verify;
use regex::Regex;
use serde_derive::Deserialize;
use tera::Tera;

use crate::database::index::{Database, DatabaseConnection};
use crate::pages::index::{prepare_context, render_template, UserInfo};

#[get("/settings")]
pub async fn settings(session: Session, tmpl: web::Data<Tera>) -> Result<HttpResponse, Error> {
    let context = prepare_context(&session)?;
    render_template(&tmpl, "settings", &context).await
}

#[derive(Deserialize)]
pub struct PasswordChangeRequest {
    current_password: String,
    new_password: String,
    confirm_password: String,
}

#[derive(Deserialize)]
pub struct EmailChangeRequest {
    new_email: String,
}

#[post("/change_password")]
pub async fn change_password(
    db: web::Data<DatabaseConnection>,
    req: web::Json<PasswordChangeRequest>,
    session: Session,
) -> Result<HttpResponse, Error> {
    if let Some(user_info) = session.get::<UserInfo>("user_info")? {
        // Get the current stored password hash from the database
        let current_stored_password = db.get_password(user_info.id).await.unwrap();

        // Verify the current password
        if !verify(&req.current_password, &current_stored_password).unwrap() {
            return Ok(HttpResponse::BadRequest().body("Aktuelles Passwort ist falsch."));
        }

        if req.new_password != req.confirm_password {
            return Ok(HttpResponse::BadRequest().body("Neues Passwort und Bestätigungspasswort stimmen nicht überein."));
        }

        if req.new_password == req.current_password {
            return Ok(HttpResponse::BadRequest().body("Neues Passwort muss sich vom aktuellen Passwort unterscheiden."));
        }

        if !is_strong_password(&req.new_password) {
            return Ok(HttpResponse::BadRequest().body("Das neue Passwort muss mindestens 8 Zeichen lang sein und mindestens einen Großbuchstaben, einen Kleinbuchstaben und eine Zahl enthalten."));
        }

        // Update the password in the database
        let result = db.update_password(user_info.id, &req.new_password).await;
        match result {
            Ok(_) => Ok(HttpResponse::Ok().body("Passwort erfolgreich geändert")),
            Err(_) => Ok(HttpResponse::InternalServerError().body("Fehler beim Aktualisieren des Passworts")),
        }
    } else {
        Ok(HttpResponse::Unauthorized().body("Nicht autorisiert"))
    }
}

pub fn is_strong_password(password: &str) -> bool {
    let min_length = 8;
    let has_lowercase = Regex::new(r"[a-z]+").unwrap();
    let has_uppercase = Regex::new(r"[A-Z]+").unwrap();
    let has_digit = Regex::new(r"\d+").unwrap();

    password.len() >= min_length
        && has_lowercase.is_match(password)
        && has_uppercase.is_match(password)
        && has_digit.is_match(password)
}


#[post("/change_email")]
pub async fn change_email(
    db: web::Data<DatabaseConnection>,
    req: web::Json<EmailChangeRequest>,
    session: Session,
) -> Result<HttpResponse, Error> {
    if let Some(user_info) = session.get::<UserInfo>("user_info")? {
        // Fetch the current email from the database.
        let current_email = db.get_email(user_info.id).await.unwrap();

        // Check if the new email is the same as the current email.
        if current_email == req.new_email {
            return Ok(HttpResponse::BadRequest().body("Neue E-Mail muss sich von der aktuellen E-Mail unterscheiden."));
        }

        // Validate the email format.
        if !is_valid_email(&req.new_email) {
            return Ok(HttpResponse::BadRequest().body("Ungültiges E-Mail-Format."));
        }

        // Update the email in the database
        let result = db.update_email(user_info.id, &req.new_email).await;
        match result {
            Ok(_) => {
                // Update email in the current session
                if let Some(mut user_info) = session.get::<UserInfo>("user_info")? {
                    user_info.email = Option::from(req.new_email.clone());
                    session.insert("user_info", user_info)?;

                    Ok(HttpResponse::Ok().body("E-Mail erfolgreich geändert"))
                } else {
                    Ok(HttpResponse::InternalServerError().body("Fehler beim Aktualisieren der E-Mail"))
                }
            }
            Err(_) => Ok(HttpResponse::InternalServerError().body("Fehler beim Aktualisieren der E-Mail")),
        }
    } else {
        Ok(HttpResponse::Unauthorized().body("Nicht autorisiert"))
    }
}

pub fn is_valid_email(email: &str) -> bool {
    let email_regex = Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap();
    email_regex.is_match(email)
}
