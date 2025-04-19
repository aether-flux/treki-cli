use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm};
use serde::Deserialize;
use serde_json::json;
use reqwest::Client;
use crate::utils::config_path::get_config_path;
use crate::auth::login::LoginResponse;
use std::fs;

#[derive(Debug, Deserialize)]
struct Claims {
    userId: usize,
    exp: usize,
    iat: Option<usize>,
}

#[derive(Debug, Deserialize)]
pub struct UserDetails {
    pub username: String,
    pub email: String,
}

pub fn get_decoded_token () -> Result<jsonwebtoken::TokenData<Claims>, Box<dyn std::error::Error>> {
    let path = get_config_path().ok_or("⚠️ No config path found.")?;
    let data = fs::read_to_string(path)?;
    let token: LoginResponse = serde_json::from_str(&data)?;

    let decoded = decode::<Claims>(
        &token.token,
        &DecodingKey::from_secret("your_secret_key".as_bytes()),
        &Validation::new(Algorithm::HS256)
    )?;

    Ok(decoded)
}

async fn get_user_details (id: usize) -> Result<UserDetails, Box<dyn std::error::Error>> {
    let client = Client::new();
    let res = client
        .post("http://localhost:5000/api/auth/user")
        .json(&json!({ "id": id }))
        .send()
        .await?;

    if res.status().is_success() {
        let user = res.json::<UserDetails>().await?;
        Ok(user)
    } else {
        Err(format!("Failed to fetch user details. Status: {}", res.status()).into())
    }
}

pub async fn whoami () -> Result<(), Box<dyn std::error::Error>> {
    let decoded = get_decoded_token()?;
    let user = get_user_details(decoded.claims.userId).await?;

    println!("👤 Logged in as: {}", user.username);
    println!("📧 Email: {}", user.email);

    Ok(())
}
