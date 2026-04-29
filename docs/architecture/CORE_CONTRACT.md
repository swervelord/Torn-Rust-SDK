# Core Contract

This project treats the planner/executor/client layer as a stable foundation for the upcoming typed wrapper SDK.

## Stable Public Surface

- `RequestPlanner`
- `RequestExecutor`
- `TornClient`
- `PlanRequest` / `RequestPlan`
- `ExecutionReport` / `ExecutedCall`
- `DataRequestOptions`
- `ExecutionOptions`
- `RuntimeEnvConfig`

## Contract Rules

1. Keep method names and signatures backward-compatible unless a major version bump is planned.
2. Add new behavior through additive methods or additive option fields.
3. Keep error variants additive where possible (`non_exhaustive` migration can be considered later).
4. Keep all existing async semantics stable: cancellation by future drop, timeout behavior, and retry defaults.

## Guardrail Tests

- `tests/core_contract.rs` validates that public exports and async entrypoints compile and execute through mock transport.
- These tests must stay green before merging wrapper-layer changes.
