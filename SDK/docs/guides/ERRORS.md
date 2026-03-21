# Error Taxonomy

## Core Errors

- `PlannerError`
  - capabilities load/parse failures,
  - unknown resource/selection,
  - missing endpoint/path parameter planning requirements.

- `ExecutorError`
  - configuration errors (`NoApiKeys`, invalid config),
  - transport errors (`Transport`),
  - Torn API errors (`Api` with path/status/code/message),
  - invalid JSON payloads (`InvalidJson`).

- `ClientError`
  - wraps environment (`EnvConfigError`), planner, executor, and typed deserialize failures.

## Wrapper Errors

- `SdkError::Client(ClientError)`
- `SdkError::Decode(serde_json::Error)`
- `SdkError::Validation(String)`

`Validation` is returned for client-side wrapper checks before network calls.

## Retry Guidance

Safe to retry:

- transient `ExecutorError::Transport`,
- rate-limit API failures (executor retries automatically when attempts remain).

Usually do not retry blindly:

- `PlannerError` and wrapper `Validation`,
- `ClientError::Deserialize` / `SdkError::Decode`,
- deterministic `ExecutorError::Api` (for example permission/parameter issues).

## Fallback Guidance

- v2 selection-combination failures can trigger split fallback automatically.
- v2 migration-style failures can trigger runtime v1 fallback automatically.
- Use `ExecutionReport.calls` to inspect actual route/strategy/fallback flags.

## Secret Safety

- API keys are redacted in debug formatting for transport requests and executed call traces.
- transport error strings are sanitized to remove `key=` query values.
- never log raw `.env` credentials.
