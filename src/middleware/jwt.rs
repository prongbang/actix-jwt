use actix_web::error::ErrorUnauthorized;
use actix_web::{Error, FromRequest, HttpRequest};
use actix_web::dev::Payload;
use futures::future::{err, ok, Ready};
use jsonwebtoken::{Algorithm, decode, DecodingKey, Validation};
use crate::common::jwt::Claims;

pub struct Authorization;

impl FromRequest for Authorization {
    type Error = Error;
    type Future = Ready<Result<Authorization, Error>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        let _auth = req.headers().get("Authorization");
        match _auth {
            Some(_) => {
                let _split: Vec<&str> = _auth.unwrap().to_str().unwrap().split("Bearer").collect();
                let token = _split[1].trim();
                let secret_key = "secret69".as_bytes();
                match decode::<Claims>(
                    token,
                    &DecodingKey::from_secret(secret_key),
                    &Validation::new(Algorithm::HS256),
                ) {
                    Ok(_token) => ok(Authorization),
                    Err(_e) => err(ErrorUnauthorized("invalid token!")),
                }
            }
            None => err(ErrorUnauthorized("401 Unauthorized")),
        }
    }
}