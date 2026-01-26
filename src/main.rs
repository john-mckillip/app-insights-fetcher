use anyhow::Result;
use clap::Parser;
use std::env;

mod client;
mod models;

use client::AppInsightsClient;

#[derive(Parser, Debug)]
#[command(name = "app-insights-fetcher")]
#[command(about = "Fetches exception telemetry from Azure Application Insights")]
struct Cli {
    /// Number of hours to look back
    #[arg(short = 'H', long, default_value_t = 24)]
    hours: u32,

    /// Maximum number of exceptions to return
    #[arg(short, long, default_value_t = 50)]
    limit: u32,

    /// Filter by exception type (e.g., SqlException)
    #[arg(short = 't', long = "type")]
    exception_type: Option<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Load .env file if it exists
    dotenvy::dotenv().ok();

    // Parse command line arguments
    let cli = Cli::parse();

    let app_id = env::var("APP_INSIGHTS_APP_ID")?;
    let api_key = env::var("APP_INSIGHTS_API_KEY")?;

    println!(
        "Fetching recent exceptions from the last {} hours(limit: {})...\n",
        cli.hours, cli.limit
    );

    let client = AppInsightsClient::new(app_id, api_key);
    let exceptions = client
        .get_recent_exceptions(cli.hours, cli.limit, cli.exception_type.as_deref())
        .await?;

    if exceptions.is_empty() {
        println!("No exceptions found in the last {} hours. 🎉", cli.hours);
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
