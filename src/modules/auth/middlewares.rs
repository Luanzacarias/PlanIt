use axum::{
    body::Body,
    extract::Request,
    http::{self, Response},
    middleware::Next,
};

use super::jwt::JwtConfig;
use crate::helpers::api_response::ApiResponse;

pub async fn authorize(mut req: Request, next: Next) -> Result<Response<Body>, ApiResponse> {
    let auth_header = req.headers_mut().get(http::header::AUTHORIZATION);

    let auth_header = match auth_header {
        Some(header) => header
            .to_str()
            .map_err(|_| ApiResponse::unauthorized("Empty header is not allowed"))?,
        None => {
            return Err(ApiResponse::unauthorized(
                "Please add the token to the header",
            ))
        }
    };

    let mut header = auth_header.split_whitespace();
    let (_, token) = (header.next(), header.next());

    let jwt = JwtConfig::new();
    let token_data = match jwt.decode_token(token.unwrap()) {
        Ok(data) => data,
        Err(err) => {
            eprintln!("Error decoding token: {:?}", err);
            return Err(ApiResponse::unauthorized("Invalid token"));
        }
    };

    req.extensions_mut().insert(token_data);
    Ok(next.run(req).await)
}
