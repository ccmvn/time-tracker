use actix_web::cookie::Key;
use actix_web::web;

use crate::CONFIG;
use crate::middleware::authority::authority_middleware;
use crate::middleware::login::login_middleware;
use crate::middleware::session::session_middleware;
use crate::pages::admin::user_details::{admin_add_absence_entry, admin_add_time_entry, admin_delete_absence_entry, admin_delete_time_entry, admin_edit_absence_entry, admin_edit_time_entry, admin_update_authority, user_details};
use crate::pages::admin::users::users;
use crate::pages::home::{add_absence_entry, add_time_entry, delete_absence_entry, delete_time_entry, edit_absence_entry, edit_time_entry, home};
use crate::pages::index::index;
use crate::pages::login::{login, login_post};
use crate::pages::logout::logout;
use crate::pages::settings::{change_email, change_password, settings};

pub fn main_config(config: &mut web::ServiceConfig) {
    // Derive the secret key from the configuration
    let secret_key = Key::derive_from(CONFIG.get_secret_key().as_bytes());

    // Configure the routes
    config
        .service(
            web::scope("")
                // Middleware
                .wrap(session_middleware(secret_key.clone()))
                .wrap(login_middleware())

                // Routes
                .service(index)
                .service(login)
                .service(login_post)
                .service(logout)
                .service(home)
                .service(add_time_entry)
                .service(edit_time_entry)
                .service(delete_time_entry)
                .service(add_absence_entry)
                .service(edit_absence_entry)
                .service(delete_absence_entry)
                .service(settings)
                .service(change_password)
                .service(change_email)
                .service(
                    web::scope("/admin")
                        // Middleware
                        .wrap(authority_middleware("ADMINISTRATOR"))

                        // Routes
                        .service(users)
                        .service(user_details)
                        .service(admin_update_authority)
                        .service(admin_add_time_entry)
                        .service(admin_edit_time_entry)
                        .service(admin_delete_time_entry)
                        .service(admin_add_absence_entry)
                        .service(admin_edit_absence_entry)
                        .service(admin_delete_absence_entry)
                )
        );
}
