use std::fmt;
use std::fmt::{Formatter};
use actix_web::{HttpResponse, ResponseError,http::StatusCode};
use chrono::Utc;
use serde::Serialize;

#[derive(Serialize,Debug)]
pub enum AuthError{
    InvalidToken,
    MissingToken,
    TokenExpired,
    InvalidTokenFormat
}

#[derive(Serialize)]
struct ErrorResponse{
    code:u16,
    message:String,
    time:i64
}

impl fmt::Display for AuthError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            AuthError::InvalidToken=>write!(f,"Invalid token"), //无效的令牌
            AuthError::MissingToken=>write!(f,"Must have a token"), //缺少令牌
            AuthError::TokenExpired=>write!(f,"Token has expired"), //令牌已过期
            AuthError::InvalidTokenFormat=>write!(f,"Invalid token format"), //令牌格式无效
        }
    }
}

impl ResponseError for AuthError {
    fn error_response(&self) -> HttpResponse {
        let status_code = match self {
            AuthError::InvalidToken=>StatusCode::UNAUTHORIZED,
            AuthError::MissingToken=>StatusCode::UNAUTHORIZED,
            AuthError::TokenExpired=>StatusCode::UNAUTHORIZED,
            AuthError::InvalidTokenFormat=>StatusCode::UNAUTHORIZED
        };

        let error_response = ErrorResponse{
            code:status_code.as_u16(),
            message:self.to_string(),
            time:Utc::now().timestamp()
        };
        HttpResponse::build(status_code).json(error_response)
    }
}