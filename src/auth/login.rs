use reqwest::Client;
use serde::{Serialize, Deserialize};
use rpassword::read_password;
use crate::utils::config_path::get_config_path;
use std::fs;
use std::io::{self, Write};

#[derive(Serialize)]
struct LoginRequest<'a> {
    email: &'a str,
    password: &'a str,
}

#[derive(Serialize, Deserialize)]
pub struct LoginResponse {
    pub token: String,
}

pub async fn login() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let (email, password) = tokio::task::spawn_blocking(|| -> Result<(String, String), Box<dyn std::error::Error + Send + Sync>> {
        let mut email = String::new();
        print!("Email: ");
        io::stdout().flush()?;
        io::stdin().read_line(&mut email)?;
        let email = email.trim().to_string();

        print!("Password: ");
        io::stdout().flush()?;
        let password = read_password()?.trim().to_string();

        Ok((email, password))
    }).await??;

    let client = Client::new();
    let res = client
        .post("http://localhost:5000/api/auth/login")
        .json(&LoginRequest {
            email: &email,
            password: &password,
        })
        .send()
        .await?;

    if res.status().is_success() {
        let body: LoginResponse = res.json().await?;
        if let Some(path) = get_config_path() {
            fs::create_dir_all(path.parent().unwrap())?;
            fs::write(path, serde_json::to_string(&body)?)?;
            println!("☑️ Logged in successfully!");
        } else {
            println!("⚠️ Failed to locate config directory.");
        }
    } else {
        let et = res.text().await?;
        println!("❌ Login failed: {et}");
    }

    Ok(())
}
