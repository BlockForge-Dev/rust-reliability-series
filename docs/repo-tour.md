# Repo Tour

## Why this document exists

This file is for students who have just cloned the repository and want to understand the codebase before they start changing it.

Read this after `README.md` if you want a guided explanation of:

- what already exists in Version 0
- what each folder is responsible for
- what each source file does
- how a request moves through the system
- what is intentionally missing for later episodes

## Mental model

This repository currently contains one small Rust service.

Its job is:

1. Accept an execution request over HTTP.
2. Call a fake provider directly inside the request path.
3. Store the latest known result in SQLite.
4. Return an execution record to the client.

This is intentionally a weak system. It is supposed to look reasonable while still leaving room for future reliability improvements.

## What already exists today

Version 0 already includes:

- an Axum web server
- request handlers for creating and fetching executions
- a fake provider
- SQLite tables and repository methods
- structured logging
- demo scripts
- docs for the series and student workflow

Version 0 does not yet include:

- idempotency keys
- retries
- durable attempt history
- queues
- workers
- leases
- receipts
- reconciliation
- webhooks
- polling

Those missing pieces are not oversights. They are future teaching milestones.

## Request flow

When a client sends `POST /executions`, the flow is:

1. Axum routes the request to the execution handler.
2. The handler validates the JSON payload.
3. The handler creates or accepts a correlation ID.
4. The handler calls the fake provider through the provider adapter.
5. The provider returns a simple success or failure snapshot.
6. The repository writes the execution and provider response into SQLite.
7. The API returns the stored execution record as JSON.

When a client sends `GET /executions/:id`, the flow is:

1. Axum routes the request to the fetch handler.
2. The repository loads the execution from SQLite.
3. The API returns the latest stored snapshot.

## Top-level files and folders

### `Cargo.toml`

This is the Rust package manifest.

It defines:

- the crate name
- the Rust edition
- the dependencies used by the service

Important dependencies in this project:

- `axum` for the HTTP server
- `tokio` for async runtime
- `rusqlite` for SQLite access
- `serde` for JSON serialization
- `tracing` and `tracing-subscriber` for logs
- `uuid` for execution IDs and fallback correlation IDs

### `src/`

This is the application source code.

Think of it as the actual service implementation.

### `scripts/`

This contains runnable scripts that help demonstrate the current system.

Right now it only contains demo scripts for the happy path.

### `docs/`

This contains the course and repository documentation.

It is part of the teaching product, not just background notes.

### `.github/`

This contains GitHub automation.

Right now it includes a CI workflow that runs checks and tests.

### `.gitignore`

This tells Git which generated files should not be committed, such as `target/` and the local SQLite database under `data/`.

## Source code walkthrough

### `src/main.rs`

This is the application entry point.

Its responsibilities are:

- initialize logging
- read basic environment configuration
- initialize SQLite
- build the repository
- build the provider adapter
- build shared application state
- register API routes
- start the HTTP server

If you want to understand how the whole app gets assembled, start here.

### `src/app_state.rs`

This file defines `AppState`.

`AppState` is the shared object passed into request handlers. It currently contains:

- the execution repository
- the provider adapter

As the series grows, this is where more shared components may be added.

## API layer

The `api/` folder handles HTTP concerns.

### `src/api/mod.rs`

This just declares the API submodules.

In Rust, `mod.rs` is often the file that exposes the modules inside a folder.

### `src/api/routes.rs`

This file defines the Axum router.

It maps URLs to handlers:

- `GET /health`
- `POST /executions`
- `GET /executions/:id`

If a student wants to know which endpoints exist, this is one of the best files to open first.

### `src/api/handlers.rs`

This file contains the actual request handlers.

Responsibilities in this file:

- parse incoming request data
- validate execution input
- resolve the correlation ID
- call the provider adapter
- save execution results
- return JSON responses
- return not-found errors when an execution ID does not exist

This is the main orchestration layer for Version 0.

## Domain layer

The `domain/` folder holds application concepts and rules.

This is where we define the meaning of things instead of HTTP or database details.

### `src/domain/mod.rs`

This exposes the domain submodules.

### `src/domain/execution.rs`

This file defines the core execution-related data structures used by the app.

Examples:

- input payloads for creating an execution
- API response structures
- the execution record returned from storage
- the `HealthResponse`

It also contains basic validation such as:

- `action_type` must not be empty
- `target_id` must not be empty
- `amount` must be greater than zero

### `src/domain/status.rs`

This file defines `ExecutionStatus`.

Right now the statuses are simple:

- `accepted`
- `succeeded`
- `failed`

The file also includes logic for converting stored SQLite strings back into Rust enums.

### `src/domain/errors.rs`

This file defines `AppError`.

It converts internal errors into HTTP responses.

For example:

- validation errors become `400 Bad Request`
- missing records become `404 Not Found`
- storage and provider errors become `500 Internal Server Error`

This is the main error boundary for the current app.

## Storage layer

The `storage/` folder is responsible for persistence.

### `src/storage/mod.rs`

This exposes the storage submodules.

### `src/storage/sqlite.rs`

This file initializes the SQLite database.

Responsibilities:

- create the `data/` folder when needed
- open the SQLite connection
- create tables if they do not exist yet

This is where the database schema for Version 0 lives today.

### `src/storage/repository.rs`

This file contains `ExecutionRepository`.

The repository is the data-access layer for the app. It currently knows how to:

- insert a new execution
- insert the matching provider response
- fetch a stored execution by ID

This file is important because it shows what truth is actually persisted in Version 0.

That truth is still shallow:

- one execution row
- one latest provider response row
- no attempt history

## Provider layer

The `provider/` folder models the external system our service talks to.

For the series, this is intentionally a fake provider so we can control failure modes.

### `src/provider/mod.rs`

This exposes the provider submodules.

### `src/provider/adapter.rs`

This file defines the provider adapter used by the rest of the app.

Right now it is thin. It mostly forwards the request to the fake provider.

Later in the series, this layer is a natural place for:

- provider normalization
- timeout handling
- error classification
- semantics translation

### `src/provider/fake_provider.rs`

This file simulates the external provider.

It can:

- succeed
- fail
- delay its response
- simulate a transport error

That makes it useful for demos because you can trigger reliability scenarios without changing the HTTP API shape.

## Telemetry layer

The `telemetry/` folder holds logging-related code.

### `src/telemetry/mod.rs`

This exposes the telemetry submodules.

### `src/telemetry/logging.rs`

This file initializes structured logging with `tracing`.

It gives the service consistent logs that include useful request context such as correlation IDs.

Right now observability is intentionally basic. That will expand in later episodes.

## Scripts and docs

### `scripts/demo/v0_happy_path.sh`

This is the Bash demo script for Linux and macOS users.

It:

- creates an execution
- extracts the execution ID from the JSON response
- fetches the stored execution back

### `scripts/demo/v0_happy_path.ps1`

This is the PowerShell version of the same happy-path demo.

It is useful for Windows users, though some machines will require execution-policy bypass when running local scripts.

### `docs/series-roadmap.md`

This explains the big picture of the entire YouTube series:

- the teaching thesis
- the repeating episode loop
- the first-year release order
- the expected long-term direction of the repo

### `docs/version-0-weak-system.md`

This explains the current state of the app:

- what Version 0 includes
- what it intentionally lacks
- the current API contract
- the current data model
- why the design is weak on purpose

### `docs/github-student-workflow.md`

This explains how students should use the repo while following the series:

- clone
- check out the right tag
- run locally
- work on a personal branch
- compare with later tags

### `docs/episode_notes/`

This folder stores episode-specific notes.

It helps students connect a video episode to:

- the exact topic
- the exact tag
- the key change or invariant

### `.github/workflows/ci.yml`

This GitHub Action runs:

- `cargo check`
- `cargo test`

That gives students and contributors quick feedback that the repo still builds.

## What to read first

If you are new to the repo, this is a good order:

1. `README.md`
2. `docs/version-0-weak-system.md`
3. `docs/repo-tour.md`
4. `src/main.rs`
5. `src/api/routes.rs`
6. `src/api/handlers.rs`
7. `src/provider/fake_provider.rs`
8. `src/storage/repository.rs`

That order gives you a top-down understanding before you dive into implementation details.

## What students should understand before Episode 1

Before moving on, students should be comfortable answering these questions:

- Where does a request enter the system?
- Where is the provider called?
- Where is the result stored?
- What data do we persist?
- What important safety boundaries do we not have yet?

If they can answer those, they are in a good position for the next episode.
