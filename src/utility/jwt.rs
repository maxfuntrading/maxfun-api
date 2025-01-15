use serde::{Deserialize, Serialize};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};

use crate::core::consts;
use crate::utility::{LibResult, error::LibError};


#[derive(Serialize, Deserialize, Debug)]
pub struct Claims {
    pub id: String,
    pub iat: i64,
    pub exp: i64,
}

pub struct DecodeResult {
    pub id: String,
    pub is_exp: bool,
}

pub fn encode_token(id: String) -> LibResult<String> {
    let iat = chrono::Local::now().timestamp();
    let exp = iat + consts::JWT_LIVE;
    let claims = Claims { id, iat, exp };
    let header = Header {
        kid: Some(consts::JWT_KID.to_string()),
        alg: Algorithm::HS512,
        ..Default::default()
    };
    let token = encode(
        &header,
        &claims,
        &EncodingKey::from_secret(&consts::JWT_SECRET.as_bytes()),
    )?;
    Ok(token)
}

pub fn decode_token(token: String) -> LibResult<DecodeResult> {
    let token_data = match decode::<Claims>(
        &token,
        &DecodingKey::from_secret(&consts::JWT_SECRET.as_bytes()),
        &Validation::new(Algorithm::HS512),
    ) {
        Ok(c) => c,
        Err(err) => return Err(LibError::JWTokenErr(err)),
    };

    let now = chrono::Local::now().timestamp();
    let claims = token_data.claims;
    let is_exp = if claims.exp - now < consts::JWT_EXPT { true } else { false };
    let result = DecodeResult {
        id: claims.id,
        is_exp,
    };
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gen_token() {
        dotenvy::dotenv().ok();
        let id = "0xea16dbf5ee69637d094d707dc02435828f19b705".to_string();
        let result = encode_token(id);
        println!("result: {:?}", result);
    }

    #[test]
    fn test_decode_token() {
        dotenvy::dotenv().ok();
        let token = "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzUxMiIsImtpZCI6ImFyNGI3ZE1nd25FcjhHMXUwRzZkT242bnFqNHgwdXEzMldHczJ0N0FLYyJ9.eyJpZCI6IjB4ZWExNmRiZjVlZTY5NjM3ZDA5NGQ3MDdkYzAyNDM1ODI4ZjE5YjcwNSIsImlhdCI6MTcyNTk1NzE5NiwiZXhwIjoxNzI2NTYxOTk2fQ.pWcKVK-vWsO4cV67w04BhhL5LNgECb3lA3503_okJjCBZg2EvZdMtDLo6OCfJ43bUMafYlMX830IxugZ5kZbiw";
        decode_token(token.into()).unwrap();
    }
}