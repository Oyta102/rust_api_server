use actix_web::{HttpResponse, Responder};
use crate::utils::response::ApiResp;

pub async fn routers_handler()->impl Responder{
    HttpResponse::NotFound().json(ApiResp::not_full("Not Found",()))
}