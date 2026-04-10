# Version 0 Weak System

## Purpose

Version 0 must be convincing on the happy path and weak under stress.

That is the point of the entire series. Students should look at this code and think, "this seems reasonable," before the later episodes prove why it is not enough.

## What Version 0 includes

- Axum HTTP API
- `POST /executions`
- `GET /executions/:id`
- SQLite persistence
- Direct fake-provider call inside the request path
- Structured logs with correlation IDs
- Demo scripts for the happy path

## What Version 0 intentionally lacks

- No duplicate protection
- No idempotency key
- No queue
- No retries
- No leases
- No attempt history
- No receipts
- No reconciliation
- No webhook handling discipline
- No explicit unknown state after ambiguous provider outcomes

## API contract

### `POST /executions`

Request:

```json
{
  "action_type": "issue_refund",
  "target_id": "pay_123",
  "amount": 5000
}
```

Response:

```json
{
  "correlation_id": "demo-v0-001",
  "execution": {
    "id": "generated-uuid",
    "action_type": "issue_refund",
    "target_id": "pay_123",
    "amount": 5000,
    "status": "succeeded",
    "provider_status": "approved",
    "provider_message": "fake provider approved ...",
    "correlation_id": "demo-v0-001",
    "created_at": "timestamp",
    "updated_at": "timestamp"
  }
}
```

### `GET /executions/:id`

Returns the latest stored snapshot for one execution.

## Data model

### `executions`

Purpose: stores one row per submitted execution request.

Fields:

- `id`
- `action_type`
- `target_id`
- `amount`
- `status`
- `correlation_id`
- `created_at`
- `updated_at`

Weakness: there is no logical-operation boundary and no attempt history.

### `provider_responses`

Purpose: stores the latest provider snapshot associated with the execution.

Fields:

- `execution_id`
- `provider_status`
- `raw_message`
- `created_at`

Weakness: this is only the latest provider snapshot, not normalized execution evidence.

## Fake provider behavior

The fake provider is intentionally simple, but it already includes toggles you can use on camera:

- `issue_refund` -> success
- `issue_refund_fail` -> returns a stored failed status
- `issue_refund_slow` -> sleeps longer before returning
- `issue_refund_error` -> simulates a transport error

This gives you a clean way to demonstrate timeouts, ambiguous outcomes, and error-handling gaps without changing the request shape.

## Folder structure today

```text
src/
  main.rs
  app_state.rs
  api/
    mod.rs
    routes.rs
    handlers.rs
  domain/
    mod.rs
    execution.rs
    status.rs
    errors.rs
  storage/
    mod.rs
    sqlite.rs
    repository.rs
  provider/
    mod.rs
    fake_provider.rs
    adapter.rs
  telemetry/
    mod.rs
    logging.rs
```

For a full student-friendly walkthrough of what each folder and file does, read [docs/repo-tour.md](repo-tour.md).

## Happy-path demo

1. Start the server with `cargo run`.
2. Create a new execution.
3. Fetch it back by ID.
4. Show the stored provider snapshot.
5. Point out that it looks fine.

That last step matters. The audience needs to believe the design is plausible before you break it.

## Where the weakness lives

This version is fragile because execution truth is still too shallow.

- The provider call happens inline with the request.
- There is no dedup boundary.
- There is no durable record of attempts.
- There is no safe retry story.
- There is no way to represent "the side effect may have happened but I cannot prove it yet."

Those weaknesses are not bugs in isolation. They are the teaching surface for the rest of the series.

## Suggested Episode 0 ending

End the first episode by saying something close to this:

"This system works on the happy path, but it has no protection against duplicate execution, no durable attempt history, and no way to reason about ambiguity when a provider call times out. In the next episode, we're going to prove that with a failure."
