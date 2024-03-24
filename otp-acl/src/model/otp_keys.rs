use diesel::delete;
use diesel::internal::derives::multiconnection::chrono;
use diesel::prelude::*;

use super::schema::otp_keys;
use super::schema::otp_keys::dsl::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::model::schema::otp_keys)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct OtpKey {
    pub id: i32,
    pub otp_public_key: String,
    pub otp_private_key: String,
    pub otp_user: String,
    pub retry: i32,
    pub expiration_date: chrono::NaiveDateTime, 
    pub otp_key_enable: bool, 
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::model::schema::otp_keys)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct OtpKeyRequest {
    pub otp_public_key: String,
    pub otp_private_key: String,
    pub otp_user: String,
}

#[derive(Insertable)]
#[diesel(table_name = crate::model::schema::otp_keys)]
pub struct NewOtpKey<'a> {
    pub otp_public_key: &'a str,
    pub otp_private_key: &'a str,
    pub otp_user: &'a str,
    pub retry: &'a i32,
    pub expiration_date: &'a chrono::NaiveDateTime,
    pub otp_key_enable: &'a bool, 
}

impl OtpKey {
    pub fn create_otp_key<'a>(conn: &mut PgConnection, new_otp_key: &NewOtpKey) -> Result<OtpKey, diesel::result::Error> {
        diesel::insert_into(otp_keys::table)
            .values(new_otp_key)
            .get_result::<OtpKey>(conn)
    }

    pub fn select_otp_key<'a>(conn: &mut PgConnection, input_otp_key: OtpKeyRequest) -> Result<Vec<OtpKey>, diesel::result::Error> {
        otp_keys
            .filter(otp_keys::otp_public_key.eq(input_otp_key.otp_public_key))
            .filter(otp_keys::otp_private_key.eq(input_otp_key.otp_private_key))
            .filter(otp_keys::otp_user.eq(input_otp_key.otp_user))
            .load(conn)
    }

    pub fn delete_by_id<'a>(conn: &mut PgConnection, delete_id: i32) -> u32 {
        diesel::delete(otp_keys.filter(otp_keys::id.eq(delete_id)))
            .execute(conn)
            .unwrap()
            .try_into()
            .unwrap()
    }

    pub fn delete_by_date<'a>(conn: &mut PgConnection, input_expiration_date: chrono::NaiveDateTime) -> QueryResult<usize> {
        delete(otp_keys)
            .filter(otp_keys::expiration_date.le(input_expiration_date))
            .execute(conn)
    }
}