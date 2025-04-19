use std::collections::HashMap;
use serde::Deserialize;
use crate::auth::login;
use crate::auth::whoami;
use crate::cli::Commands;
use crate::utils::printer::print_response;
use crate::utils::parser::parse_headers;
use reqwest::{Client as AsyncClient, Response as AsyncResponse};

#[derive(Debug, Deserialize)]
struct SavedReq {
    id: usize,
    method: String,
    url: String,
    headers: Option<HashMap<String, serde_json::Value>>,
    body: Option<HashMap<String, serde_json::Value>>,
    status: u16,
    time: String,
    #[serde(flatten)]
    extra_fields: Option<HashMap<String, serde_json::Value>>,
}

// Function to handle repetitive logic for method handlers
async fn req_handler (mut req:reqwest::RequestBuilder, headers: Option<String>, body: Option<String>, verbose: bool) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Create a hashmap to store headers
    let mut header_map = if let Some(hstr) = headers {
        parse_headers(&hstr)
    } else {
        HashMap::new()
    };

    // Set Content-Type to application/json by default
    if !header_map.contains_key("Content-Type") {
        header_map.insert("Content-Type".to_string(), "application/json".to_string());
    }

    // Attach headers to the request
    for (k, v) in &header_map {
        req = req.header(k, v);
    }

    // Attach body to the request
    let req = if let Some(body_str) = body {
        let json_body: serde_json::Value = serde_json::from_str(&body_str)?;
        req.json(&json_body)
    } else {
        req
    };

     let res = req.send().await?;
     print_response(res, verbose).await;  // Print the response
    
//     let res = req.send().await;
//     match res {
//         Ok(resp) => print_response(resp, verbose).await,
//         Err(e) => {
//             eprintln!("❌ Send failed: {}", e);
//             return Err(Box::new(e));
//         }
//     }
    
    Ok(())
}

async fn get_handler (mut req:reqwest::RequestBuilder, headers: Option<String>, verbose: bool) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Create a hashmap to store headers
    let header_map = if let Some(hstr) = headers {
        parse_headers(&hstr)
    } else {
        HashMap::new()
    };

    // Attach headers to the request
    for (k, v) in &header_map {
        req = req.header(k, v);
    }

    let res = req.send().await?;
    print_response(res, verbose).await;

    Ok(())
}

async fn run_req_by_id(id: usize, verbose: bool) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let tc = AsyncClient::new();
    let url = format!("http://localhost:5000/api/requests/{}", id);

    let res = tc.get(&url).send().await?;

    // Check the status first, before consuming the body
    if !res.status().is_success() {
        return Err(format!("❌ Failed to fetch request. Status: {}", res.status()).into());
    }

    // Since the body will be consumed, let's handle raw body logging
    let raw_body = res.text().await?;

    // Deserialize the response body to SavedReq struct
    let saved_req: SavedReq = serde_json::from_str(&raw_body)?;

    // Convert headers and body to strings, if present
    let headers_string: Option<String> = match saved_req.headers {
        Some(ref map) => Some(serde_json::to_string(map).map_err(|e| format!("Failed to serialize headers: {}", e))?),
        None => None,
    };

    let body_string = match saved_req.body {
        Some(ref b) => {
            let map: serde_json::Map<String, serde_json::Value> = b.clone().into_iter().collect();
            Some(serde_json::Value::Object(map).to_string())
        },
        None => None,
    };

    println!("🌐 Making request: {} {}\n\n", saved_req.method, saved_req.url);

    // Handle request based on the method in saved_req
    match saved_req.method.to_ascii_uppercase().as_str() {
        "GET" => {
            get_handler(tc.get(&saved_req.url), headers_string, verbose).await?;
        },
        "POST" => {
            req_handler(tc.post(&saved_req.url), headers_string, body_string, verbose).await?;
        },
        "PUT" => {
            req_handler(tc.put(&saved_req.url), headers_string, body_string, verbose).await?;
        },
        "DELETE" => {
            req_handler(tc.delete(&saved_req.url), headers_string, body_string, verbose).await?;
        },
        "PATCH" => {
            req_handler(tc.patch(&saved_req.url), headers_string, body_string, verbose).await?;
        },
        _ => {
            return Err(format!("❌ Unsupported HTTP method: {}", saved_req.method).into());
        }
    }

    Ok(())
}

pub async fn handle_command (cmd: crate::cli::CliArgs) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let client = AsyncClient::new();  // Create a new client
    let verbose = cmd.verbose;

    // Check which command is provided
    match cmd.command {
        Commands::Get { url, headers } => {
            get_handler(client.get(&url), headers, verbose).await?;
        }
        Commands::Post { url, body, headers } => {
            req_handler(client.post(&url), headers, body, verbose).await?;
        }
        Commands::Put { url, body, headers } => {
            req_handler(client.put(&url), headers, body, verbose).await?;
        }
        Commands::Patch { url, body, headers } => {
            req_handler(client.patch(&url), headers, body, verbose).await?;
        }
        Commands::Delete { url, body, headers } => {
            req_handler(client.delete(&url), headers, body, verbose).await?;
        },

        // Auth commands
        Commands::Login => {
            login::login().await?;
        },
        Commands::Whoami => {
            if let Err(err) = whoami::whoami().await {
                eprintln!("❌ Error: {}", err);
            }
        },

        // Request API by ID in database
        Commands::Run { id } => {
            run_req_by_id(id, verbose).await?;
        }
    }

    Ok(())
}
