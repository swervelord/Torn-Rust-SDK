# Request Planner

`RequestPlanner` builds efficient Torn API call plans from requested selections.

## What it does

- Merges compatible selections into a single generic endpoint call when possible.
- Forces standalone selections into their own request.
- Routes direct-only selections to dedicated endpoints.
- Routes v1-only or v2-fallback selections to v1 generic endpoints.
- Builds fallback split requests for combined generic calls.

## Example usage

```rust
use torn_sdk_planner::{PlanRequest, RequestPlanner};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let planner = RequestPlanner::from_capabilities_file("spec/capabilities.json")?;

    let plan = planner.plan(
        &PlanRequest::new("user", vec!["profile", "discord", "hof"])
            .with_id("3637232")
            .with_filter("striptags", "true"),
    )?;

    for request in plan.requests {
        println!("{} {:?}", request.path, request.query);
        if !request.fallback_split.is_empty() {
            println!("  fallback splits: {}", request.fallback_split.len());
        }
    }

    Ok(())
}
```

Expected combined query (single call):

`/user?selections=profile,discord,hof&id=3637232&striptags=true`

Executor integration and rate-limited execution details are in:

- `EXECUTOR.md`
- `../guides/ENVIRONMENT.md`
