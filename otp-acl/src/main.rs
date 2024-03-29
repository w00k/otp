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

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {name}!")
}

#[get("/examples")]
async fn examples(otp: web::Json<NewOtpKeyRequest>) -> impl Responder {



    let d = NaiveDate::from_ymd_opt(2015, 6, 3).unwrap();
    let t = NaiveTime::from_hms_milli_opt(12, 34, 56, 789).unwrap();
    let dt = NaiveDateTime::new(d, t);

    let key_doo = NewOtpKey {
        otp_public_key: "oweirndfpb$jslkdfj&".to_string(),
        otp_private_key: "34567".to_string(),
        otp_user: "JohnDoe123".to_string(),
        retry: 5,
        expiration_date: dt,
        otp_key_enable: true
    };

    let otp_key_doo = OtpKeyRequest {
        otp_public_key: "oweirndfpb$jslkdfj&".parse().unwrap(),
        otp_private_key: "34567".parse().unwrap(),
        otp_user: "JohnDoe123".parse().unwrap(),
    };

    let d1 = NaiveDate::from_ymd_opt(2018, 6, 3).unwrap();
    let t1 = NaiveTime::from_hms_milli_opt(12, 34, 56, 789).unwrap();
    let dt1 = NaiveDateTime::new(d1, t1);

    let key_jess = NewOtpKey {
        otp_public_key: "piuwerwerjnsdfsdf&".to_string(),
        otp_private_key: "12345".to_string(),
        otp_user: "Jesse123".to_string(),
        retry: 3,
        expiration_date: dt1,
        otp_key_enable: true
    };

    //let mut my_pool =  pool.clone().get_ref();
    //let new_pool = *my_pool;
    //create new otp key
    //let mut conn = pool.get_ref();
    //let mut conn = pool.get()?;
    //new_otp_key(&mut conn, key_doo);
    log::info!("create otp {:?}", otp);
    let otp_key_request: NewOtpKeyRequest = otp.into_inner();

    let otp_key: NewOtpKey = NewOtpKey {
        otp_public_key: otp_key_request.otp_public_key,
        otp_private_key: otp_key_request.otp_private_key, 
        otp_user: otp_key_request.otp_user, 
        retry: otp_key_request.retry, 
        expiration_date: otp_key_request.expiration_date, 
        otp_key_enable: otp_key_request.otp_key_enable, 
    };

    let mut pool = connection::connection::establish_connection();
    //new_otp_key(pool, key_jess);

    let otp = new_otp_key(pool, otp_key);

    if otp.is_ok() {
        let otp_key = otp.unwrap();
        let response = OtpKeyResponse {
                otp_public_key: otp_key.otp_public_key,
                otp_user: otp_key.otp_user,
                retry: otp_key.retry,
                otp_key_enable: otp_key.otp_key_enable,
            };
            HttpResponse::Ok().json(response)
        } else {
            HttpResponse::NotFound().json("ERROR")
        }
    
        

    /* 
    match otp {
        Ok(value) => {
            let response = OtpKeyResponse {
                otp_public_key: value.otp_public_key,
                otp_user: value.otp_user,
                retry: value.retry,
                otp_key_enable: value.otp_key_enable,
            };
            HttpResponse::Ok().json(response)
        },
        Error(_) => HttpResponse::NotFound().json("ERROR"),
    }
    */

    /*
    //find otp key
    //pool = connection::connection::establish_connection();
    let otp_key_result = find_otp_key(pool, otp_key_doo);

    //if otp key is found update retry
    if otp_key_result.is_ok() {
        let otp_key = otp_key_result.unwrap();
        //pool = connection::connection::establish_connection();
        let retry = otp_key.retry - 1;
        update_retry(my_pool, otp_key.id, retry); //update retry

        let otp_key_doo = OtpKeyRequest {
            otp_public_key: otp_key.otp_public_key,
            otp_private_key: otp_key.otp_private_key,
            otp_user: otp_key.otp_user,
        };
        //pool = connection::connection::establish_connection();
        _ = find_otp_key(my_pool, otp_key_doo); //verify otp key update
    }

    //delete otp key with id 11
    //pool = connection::connection::establish_connection();
    delete_by_id(my_pool, 11);

    //delete otp keys by date
    let d2 = NaiveDate::from_ymd_opt(2024, 6, 3).unwrap();
    let t2 = NaiveTime::from_hms_milli_opt(12, 34, 56, 789).unwrap();
    let dt2 = NaiveDateTime::new(d2, t2);

    //pool = connection::connection::establish_connection();
    delete_by_date(my_pool, dt2);
     */
}



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
            .service(greet)
            //.service(examples)
            //.service(controller::create_otp::create_otp_key)
            .route("/create", web::put().to(controller::create_otp::create_otp_key))
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
