use actix_session::Session;
use actix_web::{Error, get, HttpResponse, post, web};
use log::{error, info};
use serde::Deserialize;
use tera::{Context, Tera};

use crate::database::index::{Database, DatabaseConnection};
use crate::pages::index::render_template;

#[get("/login")]
pub async fn login(tmpl: web::Data<Tera>) -> Result<HttpResponse, Error> {
    let context = Context::new();
    render_template(&tmpl, "login", &context).await
}

#[post("/login")]
pub async fn login_post(
    session: Session,
    form: web::Form<LoginRequest>,
    tmpl: web::Data<Tera>,
    db: web::Data<DatabaseConnection>,
) -> Result<HttpResponse, Error> {
    let username = &form.username;
    let password = &form.password;

    match db.check_login(username, password).await {
        Ok(Some(user_id)) => {
            info!("Login success");

            // Fetch user info with the ID
            let user_info = db.get_user(user_id).await.map_err(|e| {
                error!("Failed to get user info: {}", e);
                actix_web::error::ErrorInternalServerError("Internal server error")
            })?;

            // Store the user data in the session
            session.insert("user_info", user_info).map_err(|e| {
                error!("Failed to insert user info into session: {}", e);
                actix_web::error::ErrorInternalServerError("Internal server error")
            })?;

            // Redirect to home page
            Ok(HttpResponse::Found().append_header(("location", "/home")).finish())
        }
        Ok(None) => {
            info!("Login failed: Password incorrect or user not found");
            let mut context = Context::new();
            context.insert("show_toast", &true);
            render_template(&tmpl, "login", &context).await
        }
        Err(_) => {
            info!("Login failed: Username not found or other database error");
            let mut context = Context::new();
            context.insert("show_toast", &true);
            render_template(&tmpl, "login", &context).await
        }
    }
}

#[derive(Deserialize)]
pub struct LoginRequest {
    username: String,
    password: String,
}
