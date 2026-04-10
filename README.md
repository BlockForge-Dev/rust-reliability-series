# Rust Reliability Series

This repository is the foundation for a YouTube series about building one intentionally weak Rust execution platform and then hardening it episode by episode until it becomes a reliability-focused system.

The teaching goal is simple: do not jump between unrelated demos. Keep one repo, keep one domain, and let students watch the architecture evolve under pressure.

## What this repo is

Version 0 is a deliberately weak critical-action execution engine.

It accepts a request such as `issue_refund`, calls a fake provider directly inside the HTTP request path, stores the latest known result in SQLite, and returns an execution record. It works on the happy path, which is exactly why it is useful for teaching.

## What students will learn

- How a system that looks correct on day one can still be unsafe under retries, timeouts, duplication, and ambiguity.
- How to evolve a Rust service from direct request-path execution into a queue-backed, lease-based, replay-aware execution platform.
- How to reason about truth boundaries, idempotency, retries, receipts, reconciliation, and provider semantics like an infrastructure engineer.

## Student Start Here

If you are following the YouTube series, use this order:

1. Clone the repository.
2. Check out the Git tag that matches the episode you are watching.
3. Read this `README.md` once from top to bottom.
4. Open [docs/version-0-weak-system.md](docs/version-0-weak-system.md) to understand what this version includes and what it intentionally does not include.
5. Read [docs/repo-tour.md](docs/repo-tour.md) for a folder-by-folder and file-by-file walkthrough.
6. Start the app with the command for your platform.
7. Run the health check.
8. Run the happy-path demo.
9. Watch the episode and build along from the same tagged starting point.
10. Create your own branch if you want to experiment without losing the episode baseline.

If you are watching Episode 0, start from the tag `episode-00-v0-weak-system`.

## Current milestone

The repo currently ships `v0-weak-system`:

- Axum HTTP API
- `POST /executions`
- `GET /executions/:id`
- SQLite persistence
- A fake provider adapter called synchronously in the request path
- Structured logs with `x-correlation-id`
- Demo scripts for the happy path

This version intentionally does **not** have idempotency, queues, attempt history, retries, receipts, or reconciliation yet.

## Quick start

### Requirements

- Rust stable toolchain
- `curl` or PowerShell

### Run the app

#### Linux

```bash
cargo run
```

#### macOS

```bash
cargo run
```

#### Windows PowerShell

```powershell
cargo run
```

The service starts on `127.0.0.1:3000` and creates a local SQLite database at `data/rust_reliability_series.db`.

### Create an execution

```bash
curl -X POST http://127.0.0.1:3000/executions \
  -H "content-type: application/json" \
  -H "x-correlation-id: demo-v0-001" \
  -d '{"action_type":"issue_refund","target_id":"pay_123","amount":5000}'
```

### Fetch an execution

```bash
curl http://127.0.0.1:3000/executions/<id>
```

### Run the demo script

#### Linux

```bash
bash scripts/demo/v0_happy_path.sh
```

#### macOS

```bash
bash scripts/demo/v0_happy_path.sh
```

#### Windows PowerShell

PowerShell may block local `.ps1` files by default. Use either of these:

```powershell
powershell -ExecutionPolicy Bypass -File .\scripts\demo\v0_happy_path.ps1
```

or:

```powershell
Set-ExecutionPolicy -Scope Process -ExecutionPolicy Bypass
.\scripts\demo\v0_happy_path.ps1
```

#### iPhone / iPad users

iOS and iPadOS are not supported as native local development environments for this repo.

Students on iPhone or iPad can still:

- read the code and docs on GitHub
- follow along with the video
- use a remote Linux/macOS/Windows machine, Codespaces, or another cloud dev environment to run the project

If you intended to support Apple laptops and desktops, use the macOS commands above.

### Happy-path test by platform

#### Linux and macOS

```bash
curl http://127.0.0.1:3000/health
bash scripts/demo/v0_happy_path.sh
```

#### Windows PowerShell

```powershell
Invoke-RestMethod http://127.0.0.1:3000/health
powershell -ExecutionPolicy Bypass -File .\scripts\demo\v0_happy_path.ps1
```

## Fake provider toggles

The fake provider includes simple action-type switches so you can demonstrate failure modes later without changing the external API:

- `issue_refund` -> normal success
- `issue_refund_fail` -> provider returns a failure snapshot
- `issue_refund_slow` -> provider sleeps longer before responding
- `issue_refund_error` -> provider raises a simulated transport error

## Repo layout

```text
src/
  api/
  domain/
  provider/
  storage/
  telemetry/
scripts/
  demo/
docs/
  episode_notes/
```

## Documentation map

- [Series roadmap](docs/series-roadmap.md)
- [Version 0 weak system guide](docs/version-0-weak-system.md)
- [Repo tour](docs/repo-tour.md)
- [GitHub and student workflow guide](docs/github-student-workflow.md)
- [Episode notes index](docs/episode_notes/README.md)



