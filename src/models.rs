use egg_mode::Token;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct APICredentials {
    pub api_key: String,
    pub api_secret: String,
}

#[derive(Serialize, Deserialize)]
pub struct OAuthCredentials {
    pub access_token: Token,
}
