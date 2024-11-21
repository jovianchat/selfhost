use crate::{utils::init::ENV_VARS, Error, Result};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // The user ID or username or identifier
    pub exp: usize,  // Expiry time of the token
    pub token_type: JWT,
}
#[derive(Serialize, Deserialize)]
pub enum JWT {
    AccessToken,
    RefreshToken,
}
impl JWT {
    pub fn encode(self, sub: String) -> Result<(String, usize)> {
        let env_vars = ENV_VARS.get().unwrap();
        let secret = &env_vars.jwt_secret;

        let now = chrono::Utc::now();
        let exp = match self {
            Self::AccessToken => (now + chrono::Duration::hours(1)).timestamp() as usize,
            Self::RefreshToken => (now + chrono::Duration::days(7)).timestamp() as usize,
        };
        let claims = Claims {
            sub,
            exp,
            token_type: self,
        };

        let token = jsonwebtoken::encode(
            &jsonwebtoken::Header::default(),
            &claims,
            &jsonwebtoken::EncodingKey::from_secret(secret.as_bytes()),
        )
        .map_err(|e| anyhow::anyhow!("failed to encode JWT: {}", e))?;
        Ok((token, exp))
    }
    pub fn decode(token: String) -> Result<String> {
        let env_vars = ENV_VARS.get().unwrap();
        let secret = &env_vars.jwt_secret;

        let token_data = jsonwebtoken::decode::<Claims>(
            &token,
            &jsonwebtoken::DecodingKey::from_secret(secret.as_bytes()),
            &jsonwebtoken::Validation::default(),
        )
        .map_err(|e| anyhow::anyhow!("failed to decode JWT: {}", e))?;
        let now = chrono::Utc::now();
        if token_data.claims.exp < now.timestamp() as usize {
            return Err(Error::Unauthorized);
        }
        Ok(token_data.claims.sub)
    }
}
