# Series Roadmap

## Series thesis

This series is about one system, not a pile of disconnected tutorials.

The system is a critical-action execution engine in Rust. It accepts a request such as `issue_refund`, performs a provider call, stores what it knows, and eventually grows into a durable, observable, replay-safe, provider-aware platform.

The end-state is not "perfect software." The end-state is a system that handles ambiguity more honestly, narrows replay boundaries, exposes truth more clearly, and fails in more understandable ways.

## Teaching pattern

The core promise of the series is:

- Start with a design many engineers would call good enough.
- Stress it until a hidden production weakness becomes visible.
- Introduce one primitive at a time.
- Keep the same repo all year so students watch the system mature instead of restarting from scratch.

## Reusable episode loop

Every episode should reuse the same loop:

1. Baseline: run the current system without warning so it looks acceptable.
2. Exposure: trigger one failure mode and make the weakness visible.
3. Explanation: explain what happened at the boundary between local certainty and remote ambiguity.
4. Evolution: add one primitive, invariant, or architectural boundary.
5. Re-run: replay the same scenario and prove the new behavior is safer.

## Questions to answer on camera every time

- What failed: request, side effect, observation, or delivery?
- Where did the failure happen: client, worker, provider, callback, or storage boundary?
- What is known, unknown, and only inferred?
- What can be retried safely, and what would risk duplicate harm?
- What invariant did this episode add?
- What still remains unresolved after the change?

## First-year release order

The roadmap PDF already suggests a strong publishing order. This repo is set up to support that same path.

### Quarter 1

- Build the weak system.
- Expose timeouts.
- Show duplicate execution.
- Explain why duplicates happen.
- Add idempotency keys.

Why now: this is the first jump from a naive app to a safer logical-operation boundary.

### Quarter 2

- Add request fingerprinting.
- Reuse prior responses where safe.
- Move work out of the request path.
- Introduce pull workers.
- Add queue and lease basics.
- Show crashed workers and visibility timeout.
- Add heartbeats.

Why now: this is where execution ownership becomes durable instead of accidental.

### Quarter 3

- Classify transient vs terminal failures.
- Add backoff and jitter.
- Add max attempts.
- Add dead-letter handling.
- Show poison jobs.
- Add baseline observability.
- Start durable attempt histories.

Why now: this is where the queue becomes operationally real and recovery-oriented.

### Quarter 4

- Build receipts.
- Add reconciliation.
- Teach provider semantics and finality.
- Compare polling with push confirmation.
- Add operator review queues.
- Distinguish replay from repair.

Why now: this completes the truth-and-recovery story.

## How this maps to the repository

### Today

The repo only includes the Version 0 weak system:

- synchronous request-path provider call
- latest-state storage
- no idempotency
- no queue
- no attempt history
- no receipts
- no reconciliation

### Later

The repo will gradually grow into folders such as:

- `queue/`
- `worker/`
- `reconciliation/`
- `provider/webhook.rs`
- `provider/poller.rs`
- `domain/attempt.rs`
- `domain/receipt.rs`

The important constraint is that those folders appear only when the series needs them.

## Recommended GitHub structure for the series

This part is a recommendation for publishing, not a quote from the PDF.

- Keep `main` as the newest stable teaching state.
- Use tags for episode checkpoints.
- Keep a short episode note in the repo for each episode.
- Link the exact tag in each YouTube description.
- Never silently rewrite history between episodes; let students see the evolution.

Suggested tag pattern:

- `episode-00-v0-weak-system`
- `episode-01-timeouts`
- `episode-02-duplicate-execution`
- `episode-03-idempotency`

## What success looks like

- A student can watch the repo evolve from a happy-path API into a reliability-focused execution platform.
- Every episode demonstrates pain before introducing a fix.
- The explanation works at three levels: simple meaning, engineering mechanism, and staff-level architectural implication.
- By the end of the year, the audience has learned how to think about ambiguity, retries, queues, truth, and recovery in Rust systems.
