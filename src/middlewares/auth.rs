use std::future::{ready, Ready};

use actix_web::{dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform}, Error, HttpMessage};
use actix_web::http::header::AUTHORIZATION;
use futures_util::future::LocalBoxFuture;
use crate::utils::errors::AuthError;
use crate::utils::jwt::verify_jwt;

pub struct JwtAuth;

// Middleware factory is `Transform` trait
// `S` - type of the next service
// `B` - type of response's body
impl<S, B> Transform<S, ServiceRequest> for JwtAuth
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = JwtAuthMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(JwtAuthMiddleware { service }))
    }
}

pub struct JwtAuthMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for JwtAuthMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        static SKIP_AUTH_ROUTES:&[&str] = &["signin","signup"];
        let path = req.path();
        let path_segments: Vec<&str> = path.split('/').collect();
        let last_segment = path_segments.last().unwrap_or(&"");
        if SKIP_AUTH_ROUTES.contains(&last_segment){
            let fut = self.service.call(req);
            return Box::pin(async move {
                let res = fut.await?;
                Ok(res)
            })
        }

        let auth_header = req.headers().get(AUTHORIZATION);
        if let Some(auth_header) = auth_header{
            let auth_header = auth_header.to_str().unwrap_or("");
            if auth_header.starts_with("Bearer "){
                let token = auth_header.trim_start_matches("Bearer ");
                match verify_jwt(&token) {
                    Ok(claims)=>{
                        req.extensions_mut().insert(claims.sub);
                        let fut = self.service.call(req);
                        Box::pin(async move {
                            let res = fut.await?;
                            Ok(res)
                        })
                    }
                    Err(err)=>{
                        if err == "Invalid"{
                            Box::pin(async move {
                                Err(AuthError::InvalidToken.into())
                            })
                        }else {
                            Box::pin(async move {
                                Err(AuthError::TokenExpired.into())
                            })
                        }
                    }
                }
            }else {
                Box::pin(async move {
                    Err(AuthError::InvalidTokenFormat.into())
                })
            }
        }else {
            Box::pin(async move {
                Err(AuthError::MissingToken.into())
            })
        }
    }
}