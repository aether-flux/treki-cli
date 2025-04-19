use reqwest::{Response as AsyncResponse};

pub async fn print_response (res: AsyncResponse, verbose: bool) {
    // Print Status Code and Headers if verbose mode
    if verbose {
        println!("✅ Status: {}\n\n", res.status());

        println!("📦 Headers:\n");
        for (k, v) in res.headers().iter() {
            let val_str = v.to_str().unwrap_or("<non-UTF8>");
            println!("{}: {}", k, val_str);
        }
        println!("\n\n");
    }

    // Get value of Content-Type header (in response)
    let content_type = {
        let headers = res.headers();
        headers
            .get(reqwest::header::CONTENT_TYPE)
            .and_then(|v| v.to_str().ok())
            .unwrap_or("")
            .to_string()
    };
    let body = res.text().await.unwrap_or_else(|_| "❌ Failed to read response body.".into());

    // Pretty print JSON if Content-Type is application/json
    if content_type.contains("application/json") {
        match serde_json::from_str::<serde_json::Value>(&body) {
            Ok (json) => {
                let pretty = colored_json::to_colored_json_auto(&json).unwrap_or_else(|_| body.clone());
                println!("{}", pretty);
            },
            Err (_) => {
                eprintln!("⚠️ Response said JSON couldn't parse it. Raw body below:");
                println!("{}", body);  // Fall back to printing the raw body if any error occurs
            }
        }
    } else {
        println!("{}", body);  // Print the raw body if Content-Type is not application/json
    }
}
