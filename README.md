# Application Insights Exception Fetcher

A lightweight Rust command-line tool for querying Azure Application Insights telemetry data.

## Current Features

- Fetches recent exceptions from Application Insights
- **Command-line arguments** to configure time range, limit, and exception type filter
- Displays exception details including timestamp, type, message, and operation name
- Simple, fast native executable with no runtime dependencies

## Prerequisites

- Rust toolchain (install from [rustup.rs](https://rustup.rs))
- Azure Application Insights resource
- Application Insights API credentials (App ID and API Key)

## Installation

1. Clone or download this project
2. Navigate to the project root directory
3. Install globally:

```bash
cargo install --path .
```

This installs the binary to `~/.cargo/bin/` (already in your PATH), so you can run `app-insights-fetcher` from anywhere.

**Alternative:** Build without installing globally:
```bash
cargo build --release
```
The compiled executable will be in `target/release/app-insights-fetcher.exe` (Windows) or `target/release/app-insights-fetcher` (Linux/Mac).

## Configuration

### Getting Your Credentials

1. Go to Azure Portal → Your Application Insights resource
2. Navigate to **API Access** (under "Configure")
3. Copy your **Application ID**
4. Click **Create API key**:
   - Name: "Exception Fetcher" (or your choice)
   - Permissions: Check "Read telemetry"
   - Copy the generated key immediately (you can't see it again!)

### Setting Up Environment Variables

**Option 1: Environment Variables (Quick Start)**

PowerShell:
```powershell
$env:APP_INSIGHTS_APP_ID = "your-app-id-here"
$env:APP_INSIGHTS_API_KEY = "your-api-key-here"
cargo run
```

CMD:
```cmd
set APP_INSIGHTS_APP_ID=your-app-id-here
set APP_INSIGHTS_API_KEY=your-api-key-here
cargo run
```

**Option 2: .env File (Recommended)**

1. Create a `.env` file in the project root:
```
APP_INSIGHTS_APP_ID=your-app-id-here
APP_INSIGHTS_API_KEY=your-api-key-here
```

2. Add `dotenvy = "0.15"` to `Cargo.toml` dependencies

3. Add to top of `main()` function:
```rust
dotenvy::dotenv().ok();
```

4. Run the app:
```bash
cargo run
```

**Important:** Add `.env` to your `.gitignore` to avoid committing secrets!

## Usage

```bash
# Basic usage (defaults: last 24 hours, limit 50)
app-insights-fetcher

# Specify time range
app-insights-fetcher --hours 48

# Limit number of results
app-insights-fetcher --limit 10

# Filter by exception type
app-insights-fetcher --type SqlException

# Filter by exception message
# Works with full and partial matches
app-insights-fetcher --message "Resolving Dispute for payment intent Id."

# Combine options
app-insights-fetcher --hours 12 --limit 25 --type NullReferenceException

# Show help
app-insights-fetcher --help
```

**Development mode** (if not installed globally):
```bash
cargo run -- --hours 48 --limit 10
```

### Sample Output

```
Fetching recent exceptions from Application Insights...

Found 3 exceptions:

1. [2026-01-23T14:32:15.234Z]
   Type: System.Data.SqlClient.SqlException
   Operation: GET /api/payments
   Message: Timeout expired. The timeout period elapsed...

2. [2026-01-23T13:45:22.123Z]
   Type: System.NullReferenceException
   Operation: POST /api/reconciliation
   Message: Object reference not set to an instance...

3. [2026-01-23T12:10:05.456Z]
   Type: Microsoft.Azure.ServiceBus.TimeoutException
   Operation: ProcessMessageAsync
   Message: The operation did not complete within...
```

## Project Structure

```
app-insights-fetcher/
├── Cargo.toml          # Project dependencies and metadata
├── .env               # Local environment variables (gitignored)
├── src/
│   ├── main.rs        # Entry point and CLI logic
│   ├── client.rs      # Application Insights API client
│   └── models.rs      # Data structures and response models
└── target/            # Compiled binaries (created by cargo)
```

## Future Enhancements

### Completed
 
- [✅] **Command-line arguments**: Configure time range, limit, and filters
  ```bash
  app-insights-fetcher --hours 48 --limit 100 --type SqlException
  ```

### Planned Features

- **Multiple query types**: Support for traces, requests, dependencies, and custom metrics
  ```bash
  app-insights-fetcher query traces --severity Error
  app-insights-fetcher query requests --duration ">5000ms"
  ```

- **Export capabilities**: Output results to JSON, CSV, or other formats
  ```bash
  app-insights-fetcher --output exceptions.json --format json
  ```

- **Scheduled execution**: Run as a background service or scheduled task

- **Alerting system**: Send notifications (email, Slack, Teams) when specific patterns are detected
  ```bash
  app-insights-fetcher monitor --alert-on "SqlException" --webhook https://...
  ```

- **Web API mode**: Serve telemetry data via REST endpoints
  ```bash
  app-insights-fetcher serve --port 8080
  ```

- **Aggregation and analytics**: Group, count, and analyze exception patterns
  ```bash
  app-insights-fetcher analyze --group-by type --time-bucket 1h
  ```

- **Multi-resource support**: Query multiple Application Insights resources simultaneously

- **Interactive mode**: TUI (Terminal User Interface) for browsing telemetry data

- **Caching**: Cache recent queries to reduce API calls and improve performance

## Development

### Building

```bash
# Debug build (faster compilation, slower runtime)
cargo build

# Release build (slower compilation, optimized runtime)
cargo build --release

# Check code without building
cargo check
```

### Running Tests

```bash
cargo test
```

### Code Formatting

```bash
cargo fmt
```

### Linting

```bash
cargo clippy
```

## Dependencies

- **tokio**: Async runtime for concurrent API calls
- **reqwest**: HTTP client for API requests
- **serde/serde_json**: JSON serialization/deserialization
- **anyhow**: Error handling
- **chrono**: Date/time handling

## Security Notes

- API keys grant read access to all telemetry data - treat them as secrets
- Never commit `.env` files or hardcode credentials
- For production deployments, use Azure Key Vault or secure configuration providers
- Consider using managed identities when running in Azure

## Contributing

This is a learning project for exploring Rust development. Contributions and suggestions welcome!

## License

MIT License - feel free to use and modify as needed.

## Troubleshooting

**"Environment variable not set" error:**
- Ensure `APP_INSIGHTS_APP_ID` and `APP_INSIGHTS_API_KEY` are set
- Check that you're running from the project root directory

**"API request failed" error:**
- Verify your API key has "Read telemetry" permissions
- Check that your Application Insights resource is accessible
- Ensure the App ID matches your Application Insights resource

**Compilation errors:**
- Run `cargo clean` and try building again
- Ensure you have the latest stable Rust: `rustup update`

**No exceptions found:**
- Your application may not have had exceptions in the last 24 hours (good news!)
- Verify the correct Application Insights resource is configured
- Try generating a test exception in your application

## 📋 Changelog

### v1.2.0 (2026-02-15)
- Added command-line-argument (`--message`)

### v1.1.0 (2026-01-26)
- Added command-line arguments (`--hours`, `--limit`, `--type`)
- Added `clap` dependency for CLI parsing

### v1.0.0 (2026-01-23)
- Initial release

---

Made with ❤️ by John McKillip | Ice Nine Media