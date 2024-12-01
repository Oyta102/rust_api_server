use std::env;
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation,errors::Error};
use serde::{Deserialize, Serialize};
use crate::models::users::UserName;

#[derive(Serialize, Deserialize, Debug)]
pub struct Claims{
    pub sub:u32,
    exp:i64
}

pub fn create_jwt(users: &UserName) ->Result<String,Error>{
    let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let jwt_hours = env::var("JWT_HOURS").expect("JWT_HOURS must be set");
    let claims = Claims{
        sub:users.id,
        exp:(Utc::now()+Duration::hours(jwt_hours.parse().unwrap())).timestamp()
    };
    encode(&Header::default(),&claims,&EncodingKey::from_secret(jwt_secret.as_ref()))
}

pub fn verify_jwt(token: &str) -> Result<Claims,String> {
    let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let decoding_key = DecodingKey::from_secret(jwt_secret.as_ref());
    let validation = Validation::new(Algorithm::HS256);
    let time = Utc::now().timestamp();

    match decode::<Claims>(token, &decoding_key, &validation) {
        Ok(data) =>{
            if data.claims.exp > time{
                Ok(data.claims)
            }else {
                Err("Expired".parse().unwrap())
            }
        }
        Err(_) => Err("Invalid".parse().unwrap()),
    }
}