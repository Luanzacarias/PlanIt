use thiserror::Error;

use super::{
    dto::{AuthState, UserLoginResponse},
    jwt::JwtConfig,
};
use crate::modules::user::service::{UserService, UserServiceError};
use chrono::{Duration, Utc};

#[derive(Error, Debug)]
pub enum AuthServiceError {
    #[error("Invalid email or password")]
    Unauthorized,

    #[error("User error: {0}")]
    UserService(#[from] UserServiceError),
}

pub struct AuthService {
    jwt_config: JwtConfig,
    user_service: UserService,
}

impl AuthService {
    pub fn new(user_service: UserService) -> Self {
        Self {
            jwt_config: JwtConfig::new(),
            user_service,
        }
    }

    pub async fn login(
        &self,
        email: &str,
        password: &str,
    ) -> Result<UserLoginResponse, AuthServiceError> {
        let user = self
            .user_service
            .find_user_by_email(email)
            .await
            .map_err(AuthServiceError::from)?;
        match user {
            Some(user) => {
                if user.password != password {
                    return Err(AuthServiceError::Unauthorized);
                }

                let now = Utc::now();
                let expire: chrono::TimeDelta = Duration::hours(24);
                let exp: usize = (now + expire).timestamp() as usize;
                let token = self.jwt_config.encode_token(AuthState {
                    id: user.id.unwrap(),
                    email: user.email.clone(),
                    exp,
                });
                Ok(UserLoginResponse {
                    id: user.id.unwrap(),
                    email: user.email,
                    token: token.unwrap(),
                })
            }
            None => Err(AuthServiceError::Unauthorized),
        }
    }
}
