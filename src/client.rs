use anyhow::{Context, Result};
use reqwest::Client;

use crate::models::{Exception, QueryResponse};

pub struct AppInsightsClient {
    client: Client,
    app_id: String,
    api_key: String,
}

impl AppInsightsClient {
    pub fn new(app_id: String, api_key: String) -> Self {
        AppInsightsClient {
            client: Client::new(),
            app_id,
            api_key,
        }
    }

    fn escape_kql_string(s: &str) -> String {
    s.replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
        .replace('\r', "\\r")
        .replace('\t', "\\t")
    }

    async fn query(&self, kusto_query: &str) -> Result<QueryResponse> {
        let url = format!(
            "https://api.applicationinsights.io/v1/apps/{}/query",
            self.app_id
        );

        let response = self
            .client
            .get(&url)
            .header("x-api-key", &self.api_key)
            .query(&[("query", kusto_query)])
            .send()
            .await
            .context("Failed to send request")?;

        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();
            anyhow::bail!("API request failed with status {status}: {text}");
        }

        response
            .json::<QueryResponse>()
            .await
            .context("Failed to parse response")
    }

    pub async fn get_recent_exceptions(
        &self,
        hours: u32,
        limit: u32,
        exception_type: Option<&str>,
        exception_message: Option<&str>,
    ) -> Result<Vec<Exception>> {
        // Build the type filter if provided
        let type_filter = match exception_type {
            Some(t) => format!("| where type == \"{}\"", Self::escape_kql_string(t)),
            None => String::new(),
        };
        // Build the message filter if provided
        let message_filter: String = match exception_message {
            Some(m) => format!("| where outerMessage contains \"{}\"", Self::escape_kql_string(m)),
            None => String::new(),
        };

        let query = format!(
            r#"
        exceptions
        | where timestamp > ago({hours}h)
        {type_filter}
        {message_filter}
        | project timestamp, type, outerMessage, operation_Name
        | order by timestamp desc
        | limit {limit}
        "#
        );

        let response = self.query(&query).await?;

        let mut exceptions = Vec::new();

        if let Some(table) = response.tables.first() {
            for row in &table.rows {
                if row.len() >= 4 {
                    exceptions.push(Exception {
                        timestamp: row[0].as_str().unwrap_or("").to_string(),
                        exception_type: row[1].as_str().unwrap_or("Unknown").to_string(),
                        message: row[2].as_str().unwrap_or("").to_string(),
                        operation_name: row[3].as_str().unwrap_or("").to_string(),
                    });
                }
            }
        }

        Ok(exceptions)
    }
}
