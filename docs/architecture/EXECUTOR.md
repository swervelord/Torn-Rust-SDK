# Executor Layer

The executor runs `RequestPlan` output with production concerns:

- Per-key rate limit: 100 calls/minute.
- Per-IP rate limit: 1000 calls/minute.
- Minute-boundary reset behavior (`HH:MM:00`).
- API key rotation across configured keys.
- Fallback split execution when combined selection requests fail.
- Automatic v2 -> v1 downgrade when Torn indicates a selection is not available in v2.
- Async-first execution (Tokio runtime).
- Global in-flight concurrency cap (`max_in_flight`).
- Per-request runtime overrides (`ExecutionOptions`) for attempts and timeout.

## Usage

```rust
use torn_sdk_planner::{
    ExecutionOptions, ExecutorConfig, PlanRequest, RequestExecutor, RequestPlanner,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let planner = RequestPlanner::from_capabilities_file("spec/capabilities.json")?;

    let executor = RequestExecutor::with_default_transport(
        vec![
            "key_one".to_string(),
            "key_two".to_string(),
        ],
        ExecutorConfig::default(),
    )?;

    let report = executor.plan_and_execute(
        &planner,
        &PlanRequest::new("user", vec!["profile", "discord", "hof"])
            .with_id("3637232")
            .with_filter("striptags", "true"),
    )
    .await?;

    let _report_with_overrides = executor
        .plan_and_execute_with_options(
            &planner,
            &PlanRequest::new("user", vec!["profile"]).with_id("3637232"),
            ExecutionOptions::default()
                .with_max_attempts(2)
                .with_request_timeout(std::time::Duration::from_secs(10)),
        )
        .await?;

    println!("calls executed: {}", report.calls.len());
    println!("merged json: {}", report.merged_json);
    Ok(())
}
```

For higher-level intent-style calls, use `TornClient` in:

- `../guides/CLIENT_API.md`
- `../guides/ENVIRONMENT.md`

## Notes

- Rate limiting is local to this SDK process. If other processes use the same key(s), Torn may still return rate-limit errors. The executor handles this by waiting for the next minute boundary and retrying.
- If a combined request fails with a selection-combination error, fallback split calls are executed automatically.
- If v2 returns a migration-style error (for example "not available in API v2"), the executor retries the request against v1 automatically.
- Cancellation follows async future cancellation semantics: dropping the future cancels in-flight execution.
- `max_in_flight` caps concurrent HTTP sends across all async tasks sharing the same executor instance.
