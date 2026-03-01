# Environment Configuration

`TornClient::from_env("spec/capabilities.json")` loads `.env` (via `dotenvy`) and process env vars.

## Required

- `TORN_API_KEYS` (comma-separated), or
- `TORN_API_KEY` (single key fallback)

## Optional Runtime Settings

- `TORN_API_BASE_URL_V2` (default: `https://api.torn.com/v2`)
- `TORN_API_BASE_URL_V1` (default: `https://api.torn.com`)
- `TORN_HTTP_TIMEOUT_SECS` (default: `30`)
- `TORN_USER_AGENT` (default: `torn-sdk-rust/0.1`)
- `TORN_MAX_ATTEMPTS` (default: `3`)
- `TORN_NETWORK_RETRY_BACKOFF_MS` (default: `250`)
- `TORN_RATE_LIMIT_PER_KEY_PER_MIN` (default: `100`)
- `TORN_RATE_LIMIT_PER_IP_PER_MIN` (default: `1000`)
- `TORN_MAX_IN_FLIGHT` (default: `8`)

## Optional Live Test Settings

Ignored live tests are in `tests/live_torn_smoke.rs`.

- `TORN_LIVE_TEST_API_KEY`
  - optional explicit key for live tests.
  - fallback lookup order is `TORN_LIVE_TEST_API_KEY`, then `TORN_API_KEY`, then first key in `TORN_API_KEYS`.
- `TORN_LIVE_TEST_USER_ID` (default: `3637232`)
- `TORN_LIVE_TEST_ITEM_ID` (default: `1`)
- `TORN_LIVE_TEST_REQUIRE_RUNTIME_FALLBACK`
  - set to `true`/`1`/`yes` to hard-fail if runtime v2->v1 fallback is not observed in the fallback smoke test.

## Live Test Command

```powershell
cargo test --test live_torn_smoke -- --ignored
```

## Notes

- Keep `.env` local and never commit real keys.
- Keep live calls bounded (`limit`, narrow ranges) when validating endpoint behavior.
