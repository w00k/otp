// @generated automatically by Diesel CLI.

diesel::table! {
    otp_keys (id) {
        id -> Int4,
        otp_public_key -> Varchar,
        otp_private_key -> Varchar,
        otp_user -> Varchar,
        retry -> Int4,
        expiration_date -> Timestamp,
        otp_key_enable -> Bool,
    }
}
