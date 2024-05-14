use utoipa::OpenApi;
use crate::controller;
use crate::model;

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
        (name = "OTP ACL", description = "Data layer for access to data")
    )
)]
pub struct ApiDoc();