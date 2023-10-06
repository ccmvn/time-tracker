use actix_session::config::{BrowserSession, CookieContentSecurity};
use actix_session::SessionMiddleware;
use actix_session::storage::CookieSessionStore;
use actix_web::cookie::{Key, SameSite};

pub fn session_middleware(secret_key: Key) -> SessionMiddleware<CookieSessionStore> {
    // Build the session middleware
    SessionMiddleware::builder(
        CookieSessionStore::default(),
        secret_key,
    )
        .cookie_name("token".to_string())
        .cookie_secure(false)  // Set this to true in production
        .session_lifecycle(BrowserSession::default())
        .cookie_same_site(SameSite::Strict)
        .cookie_content_security(CookieContentSecurity::Private)
        .cookie_http_only(true)
        .build()
}
