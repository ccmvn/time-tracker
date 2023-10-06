use std::future::Ready;

use actix_web::{body::EitherBody, dev::{self, Service, ServiceRequest, ServiceResponse, Transform}, Error, http, HttpResponse};
use futures_util::future::LocalBoxFuture;
use log::info;

pub struct Login;

impl<S, B> Transform<S, ServiceRequest> for Login
    where
        S: Service<ServiceRequest, Response=ServiceResponse<B>, Error=Error>,
        S::Future: 'static,
        B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Transform = LoginMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        std::future::ready(Ok(LoginMiddleware { service }))
    }
}

pub struct LoginMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for LoginMiddleware<S>
    where
        S: Service<ServiceRequest, Response=ServiceResponse<B>, Error=Error>,
        S::Future: 'static,
        B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    dev::forward_ready!(service);

    fn call(&self, request: ServiceRequest) -> Self::Future {
        let cookies = request.headers().get(http::header::COOKIE);
        let mut is_logged_in = false;

        if let Some(cookie_str) = cookies {
            let cookies_str = cookie_str.to_str().unwrap_or_default();
            for cookie in cookies_str.split(';') {
                let mut split = cookie.split('=');
                if let (Some(name), Some(value)) = (split.next(), split.next()) {
                    if name.trim() == "token" && !value.trim().is_empty() {
                        is_logged_in = true;
                        break;
                    }
                }
            }
        }

        if is_logged_in && request.path() == "/login" {
            info!("Already logged in. Redirecting to /home");

            let (req, _pl) = request.into_parts();
            let response = HttpResponse::Found()
                .insert_header((http::header::LOCATION, "/home"))
                .finish()
                .map_into_right_body();

            Box::pin(async { Ok(ServiceResponse::new(req, response)) })
        } else if !is_logged_in && request.path() != "/login" {
            info!("Not logged in. Redirecting to /login");

            let (req, _pl) = request.into_parts();
            let response = HttpResponse::Found()
                .insert_header((http::header::LOCATION, "/login"))
                .finish()
                .map_into_right_body();

            Box::pin(async { Ok(ServiceResponse::new(req, response)) })
        } else {
            let fut = self.service.call(request);
            Box::pin(async move {
                let res = fut.await?;
                Ok(res.map_into_left_body())
            })
        }
    }
}

pub fn login_middleware() -> Login {
    Login
}
