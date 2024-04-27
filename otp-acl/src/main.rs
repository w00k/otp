use std::ops::{Deref, DerefMut};
use std::pin::Pin;
use std::sync::Arc;
use actix_web::http::Error;
use diesel::internal::derives::multiconnection::chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use diesel::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};

use actix_web::{get, middleware, web, App, HttpServer, Responder, error, HttpResponse};
use diesel::row::NamedRow;

use crate::model::otp_keys::{NewOtpKey, NewOtpKeyRequest, OtpKeyRequest, OtpKeyResponse};
use crate::query::delete::{delete_by_date, delete_by_id};
use crate::query::insert::new_otp_key;
use crate::query::select::find_otp_key;
use crate::query::update::update_retry;

mod connection;
mod schema;
mod query;
mod model;
mod controller;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    log::info!("Starting HTTP server");

    let d = NaiveDate::from_ymd_opt(2015, 6, 3).unwrap();
    let t = NaiveTime::from_hms_milli_opt(12, 34, 56, 789).unwrap();
    let dt = NaiveDateTime::new(d, t);

    println!("{:?}", dt);


    HttpServer::new(move|| {
        App::new()
            .wrap(middleware::Logger::default())
            .route("/create", web::put().to(controller::create_otp::create_otp_key))
            .route("/validate", web::post().to(controller::validate_otp::validate_otp_key))
            .route("/delete-by-date", web::post().to(controller::delete_by_date::delete_otp_by_date))

    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
