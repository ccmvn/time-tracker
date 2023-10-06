use std::pin::Pin;
use std::task::{Context, Poll};

use actix_session::SessionExt;
use actix_web::{
    body::EitherBody,
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpResponse,
};
use actix_web::http::header;
use futures::future::{ok, Ready};
use futures::Future;
use log::warn;

use crate::pages::helper::authority::check_authority;
use crate::pages::index::UserInfo;

pub struct AuthorityService<S> {
    service: S,
    authority: &'static str,
}

pub struct AuthorityMiddleware {
    authority: &'static str,
}

impl AuthorityMiddleware {
    pub fn new(authority: &'static str) -> Self {
        Self { authority }
    }
}

impl<S, B> Transform<S, ServiceRequest> for AuthorityMiddleware
    where
        S: Service<ServiceRequest, Response=ServiceResponse<B>, Error=Error>,
        S::Future: 'static,
        B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Transform = AuthorityService<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(AuthorityService {
            service,
            authority: self.authority,
        })
    }
}

impl<S, B> Service<ServiceRequest> for AuthorityService<S>
    where
        S: Service<ServiceRequest, Response=ServiceResponse<B>, Error=Error>,
        S::Future: 'static,
        B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output=Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&self, ctx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(ctx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        if let Some(session) = req.get_session().get::<UserInfo>("user_info").unwrap_or(None) {
            let user_authority = session.authority.unwrap_or_else(|| "EMPLOYEE".to_string());
            let required_authority = self.authority.to_string();

            warn!("Passed through authority middleware. User authority: {}. Required authority: {}", user_authority, required_authority);

            if let Err(e) = check_authority(&user_authority, &required_authority) {
                warn!("User does not have sufficient authority. Current authority: {}. Required authority: {}. Error: {}", user_authority, required_authority, e.to_string());

                let (req, _pl) = req.into_parts();

                let response = HttpResponse::Found()
                    .insert_header((header::LOCATION, "/home"))
                    .finish()
                    .map_into_right_body();

                return Box::pin(async { Ok(ServiceResponse::new(req, response)) });
            }
        } else {
            warn!("No user_info found in session");
        }

        let fut = self.service.call(req);
        Box::pin(async move {
            let res = fut.await?;
            Ok(res.map_into_left_body())
        })
    }
}

pub fn authority_middleware(authority: &'static str) -> AuthorityMiddleware {
    AuthorityMiddleware::new(authority)
}
