use actix_web::{web};
use crate::handlers::routeshandlers;
use crate::controllers::user_controller;

pub fn init(cfg:&mut web::ServiceConfig){
    cfg.service(
        web::scope("/api")
        .service(
            web::scope("/v1")
                .service(user_controller::signin)
                .service(user_controller::signup)
                .service(user_controller::info)
                .service(user_controller::edit_info)
        ).service(
            web::scope("/v2").service(
                web::scope("/auth")
                    .route("/test",web::get().to(routeshandlers::routers_handler))
            )
        )

    )
    .default_service(web::to(routeshandlers::routers_handler));
}