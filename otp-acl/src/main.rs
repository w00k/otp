use diesel::internal::derives::multiconnection::chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use diesel::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};

use actix_web::{middleware, App, HttpServer};

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
            .service(controller::create_otp::create_otp_key)
            .service(controller::validate_otp::validate_otp_key)
            .service(controller::delete_by_date::delete_otp_by_date)

    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
