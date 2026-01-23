use anyhow::Result;
use std::env;

mod client;
mod models;

use client::AppInsightsClient;

#[tokio::main]
async fn main() -> Result<()> {
    // Load .env file if it exists
    dotenvy::dotenv().ok();
    
    let app_id = env::var("APP_INSIGHTS_APP_ID")?;
    let api_key = env::var("APP_INSIGHTS_API_KEY")?;

    println!("Fetching recent exceptions from Application Insights...\n");

    let client = AppInsightsClient::new(app_id, api_key);
    let exceptions = client.get_recent_exceptions(24).await?;

    if exceptions.is_empty() {
        println!("No exceptions found in the last 24 hours. 🎉");
    } else {
        println!("Found {} exceptions:\n", exceptions.len());
        for (i, ex) in exceptions.iter().enumerate() {
            println!("{}. [{}]", i + 1, ex.timestamp);
            println!("   Type: {}", ex.exception_type);
            println!("   Operation: {}", ex.operation_name);
            println!("   Message: {}", ex.message);
            println!();
        }
    }

    Ok(())
}