use super::dto::AuthState;
use jsonwebtoken::{
    decode, encode, errors::Result as JwtResult, DecodingKey, EncodingKey, Header, Validation,
};
use std::env;

pub struct JwtConfig {
    pub secret: String,
    pub algorithm: jsonwebtoken::Algorithm,
}

impl JwtConfig {
    pub fn new() -> Self {
        Self {
            secret: env::var("JWT_SECRET")
                .unwrap_or_else(|_| "fd183e2e-4d6c-47cd-89c8-619e8c0e9694 ​​".to_string()),
            algorithm: jsonwebtoken::Algorithm::HS256,
        }
    }

    pub fn encode_token(&self, claims: AuthState) -> JwtResult<String> {
        let encoding_key = EncodingKey::from_secret(self.secret.as_ref());
        encode(&Header::default(), &claims, &encoding_key)
    }

    pub fn decode_token(&self, token: &str) -> JwtResult<AuthState> {
        let decoding_key = DecodingKey::from_secret(self.secret.as_ref());

        decode::<AuthState>(token, &decoding_key, &Validation::new(self.algorithm))
            .map(|data| data.claims)
    }
}
