use diesel::internal::derives::multiconnection::chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use diesel::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};

use actix_web::{middleware, App, HttpServer};
use utoipa::OpenApi;
use utoipa_rapidoc::RapiDoc;
use utoipa_scalar::{Scalar, Servable};
use utoipa_swagger_ui::SwaggerUi;

mod connection;
mod schema;
mod query;
mod model;
mod controller;

#[derive(OpenApi)]
#[openapi(
    paths(
        controller::create_otp::create_otp_key,
        controller::validate_otp::validate_otp_key,
        controller::delete_by_date::delete_otp_by_date
    ),
    components(
        schemas(model::otp_keys::NewOtpKey, model::otp_keys::OtpKeyResponse, model::otp_keys::OtpMessageResponse, model::otp_keys::OtpKeyRequest, model::otp_keys::DeleteByDateRequest )
    ),
    tags(
        (name = "todo", description = "Todo management endpoints.")
    )
)]
pub struct ApiDoc();

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    log::info!("Starting HTTP server");

    let d = NaiveDate::from_ymd_opt(2015, 6, 3).unwrap();
    let t = NaiveTime::from_hms_milli_opt(12, 34, 56, 789).unwrap();
    let dt = NaiveDateTime::new(d, t);

    println!("{:?}", dt);

    let openapi = ApiDoc::openapi();

    HttpServer::new(move|| {
        App::new()
            .wrap(middleware::Logger::default())
            .service(controller::create_otp::create_otp_key)
            .service(controller::validate_otp::validate_otp_key)
            .service(controller::delete_by_date::delete_otp_by_date)
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", openapi.clone()),
                //SwaggerUi::new("/swagger-ui/{_:.*}")
            )
            .service(RapiDoc::new("/api-docs/openapi.json").path("/rapidoc"))
            .service(Scalar::with_url("/scalar", openapi.clone()))

    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
