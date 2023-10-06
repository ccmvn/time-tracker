use actix_session::Session;
use actix_web::{Error, get, HttpResponse};
use actix_web::error::ErrorInternalServerError;
use serde_derive::{Deserialize, Serialize};
use tera::{Context, Tera};

#[derive(Debug, Serialize, Deserialize)]
pub struct UserInfo {
    pub id: i64,
    pub username: Option<String>,
    pub email: Option<String>,
    pub authority: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TimeEntry {
    pub id: Option<i64>,
    pub task: Option<String>,
    pub spent_time: Option<i64>,
    pub date: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AbsenceEntry {
    pub id: Option<i64>,
    pub absence_date: String,
    pub reason: Option<String>,
}

pub async fn render_template(
    tmpl: &Tera,
    template: &str,
    context: &Context,
) -> Result<HttpResponse, Error> {
    tmpl.render(template, context)
        .map_err(ErrorInternalServerError)
        .map(|body| HttpResponse::Ok().content_type("text/html").body(body))
}

pub fn prepare_context(session: &Session) -> Result<Context, Error> {
    let mut context = Context::new();

    if let Some(user_info) = session.get::<UserInfo>("user_info")? {
        context.insert("username", &user_info.username);
        context.insert("email", &user_info.email);
        context.insert("authority", &user_info.authority);
    } else {
        context.insert("username", &"Unbekannter Benutzer".to_string());
        context.insert("email", &"Unbekannte E-Mail".to_string());
        context.insert("authority", &"Unbekannte Rolle".to_string());
    }

    Ok(context)
}

#[get("/")]
pub async fn index() -> Result<HttpResponse, Error> {
    // Redirect to /login
    Ok(HttpResponse::Found().append_header(("location", "/login")).finish())
}
