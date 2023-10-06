use actix_session::Session;
use actix_web::{Error, get, HttpResponse, web};
use tera::Tera;

use crate::database::index::{Database, DatabaseConnection};
use crate::pages::index::{prepare_context, render_template};

#[get("/users")]
pub async fn users(session: Session, tmpl: web::Data<Tera>, db: web::Data<DatabaseConnection>) -> Result<HttpResponse, Error> {
    let mut context = prepare_context(&session)?;

    match db.get_all_users().await {
        Ok(all_users) => {
            context.insert("users", &all_users);
            render_template(&tmpl, "users", &context).await
        }
        Err(_) => Ok(HttpResponse::InternalServerError().into()),
    }
}
