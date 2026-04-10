# Episode 00: V0 Weak System

## Failure mode focus

None yet. This episode is the baseline.

## What the audience sees

- The API accepts a request.
- The fake provider is called.
- SQLite stores the latest result.
- `GET /executions/:id` returns a clean snapshot.

## What is actually weak

- No duplicate protection
- No idempotency
- No queue
- No durable attempt history
- No timeout ambiguity model
- No receipts or reconciliation

## Invariant added

Baseline only. The main invariant is visibility: every request gets an execution record and a provider snapshot on the happy path.

## Files to highlight on camera

- `src/api/handlers.rs`
- `src/provider/fake_provider.rs`
- `src/storage/repository.rs`
- `docs/version-0-weak-system.md`

## Suggested Git tag

- `episode-00-v0-weak-system`

## Next weakness to expose

Timeout ambiguity.
