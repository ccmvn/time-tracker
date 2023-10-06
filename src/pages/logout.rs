use actix_web::{Error, get, HttpRequest, HttpResponse};
use actix_web::cookie::{Cookie, time};
use actix_web::http::header::{self};

#[get("/logout")]
pub async fn logout(req: HttpRequest) -> Result<HttpResponse, Error> {
    let mut res = HttpResponse::Found()
        .append_header((header::LOCATION, "/login"))
        .finish();

    let cookie = req.cookie("token");
    if let Some(_cookie) = cookie {
        let cookie = Cookie::build("token", "")
            .path("/")
            .http_only(true)
            .max_age(time::Duration::seconds(0))
            .finish();
        res.add_cookie(&cookie)?;
    }

    Ok(res)
}
