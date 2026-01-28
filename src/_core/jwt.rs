use actix_web::{FromRequest, HttpRequest, dev::Payload};
use chrono::{Duration, Utc};
use futures_util::future::{Ready, ready};
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use surrealdb::RecordId;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,  // 用户 ID (例如 users:admin)
    pub role: String, // 用户角色 (maintainer/manager)
    pub exp: usize,   // 过期时间
}

// 后续通过环境变量引入
const SECRET: &[u8] = b"your_very_safe_secret";

impl Claims {
    pub fn is_manager(&self) -> crate::_core::error::Result<()> {
        if self.role == "manager" {
            Ok(())
        } else {
            Err(crate::_core::error::Error::PermissionError)
        }
    }
    pub fn get_user_id(&self) -> Option<RecordId> {
        let sub = self.sub.as_str();
        let sub = sub.parse::<RecordId>().ok()?;
        Some(sub)
    }
}
// 生成 Token
pub fn create_jwt(user_id: &RecordId, role: &str) -> crate::_core::error::Result<String> {
    let expiration = Utc::now()
        .checked_add_signed(Duration::hours(6))
        .expect("valid timestamp")
        .timestamp() as usize;

    let claims = Claims {
        sub: user_id.to_string(),
        role: role.to_owned(),
        exp: expiration,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(SECRET),
    )
    .map_err(|e| e.into())
}

// 验证 Token
pub fn verify_jwt(token: &str) -> crate::_core::error::Result<Claims> {
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(SECRET),
        &Validation::new(Algorithm::HS256),
    )?;

    Ok(token_data.claims)
}

impl FromRequest for Claims {
    type Error = actix_web::Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        // 1. 从 Header 提取 Authorization: Bearer <token>

        let header = req
            .headers()
            .get("Authorization")
            .and_then(|v| v.to_str().ok());

        if let Some(auth) = header {
            if let Some(token) = auth.strip_prefix("Bearer ") {
                match verify_jwt(token) {
                    Ok(claims) => {
                        return ready(Ok(claims));
                    }
                    Err(e) => {
                        tracing::debug!("Invalid token: {:?}", e);
                        return ready(Err(actix_web::error::ErrorUnauthorized("Invalid token")));
                    }
                }
            }
        }

        ready(Err(actix_web::error::ErrorUnauthorized("Missing token")))
    }
}
