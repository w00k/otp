use diesel::internal::derives::multiconnection::chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use crate::model::otp_keys::{NewOtpKey, OtpKeyRequest};
use crate::query::delete::{delete_by_date, delete_by_id};
use crate::query::insert::new_otp_key;
use crate::query::select::find_otp_key;

mod connection;
mod schema;
mod query;
mod model;

fn main() {
    println!("Hello, world!");
    let mut pool = connection::connection::establish_connection();

    let d = NaiveDate::from_ymd_opt(2015, 6, 3).unwrap();
    let t = NaiveTime::from_hms_milli_opt(12, 34, 56, 789).unwrap();
    let dt = NaiveDateTime::new(d, t);

    let key_doo = NewOtpKey {
        otp_public_key: "oweirndfpb$jslkdfj&",
        otp_private_key: "34567",
        otp_user: "JohnDoe123",
        retry: &5,
        expiration_date: &dt,
        otp_key_enable: &true
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
        otp_public_key: "piuwerwerjnsdfsdf&",
        otp_private_key: "12345",
        otp_user: "Jesse123",
        retry: &3,
        expiration_date: &dt1,
        otp_key_enable: &true
    };

    new_otp_key(pool, key_doo);

    pool = connection::connection::establish_connection();
    new_otp_key(pool, key_jess);

    pool = connection::connection::establish_connection();
    find_otp_key(pool, otp_key_doo);

    pool = connection::connection::establish_connection();
    delete_by_id(pool, 11);

    let d2 = NaiveDate::from_ymd_opt(2024, 6, 3).unwrap();
    let t2 = NaiveTime::from_hms_milli_opt(12, 34, 56, 789).unwrap();
    let dt2 = NaiveDateTime::new(d2, t2);

    pool = connection::connection::establish_connection();
    delete_by_date(pool, dt2);

}
