# Counted — Technical Documentation

**Counted** is a collaborative expense-splitting app (à la Tricount) deployed at [counted.fr](https://counted.fr). Users create projects, add participants, log expenses, and get settlement suggestions to minimise the number of transfers needed to clear all debts.

---

## Table of Contents

1. [Repository Layout](#1-repository-layout)
2. [Tech Stack](#2-tech-stack)
3. [Backend — Dioxus Fullstack](#3-backend--dioxus-fullstack)
4. [Database](#4-database)
5. [Frontend — React](#5-frontend--react)
6. [Infrastructure](#6-infrastructure)
7. [Key Algorithms](#7-key-algorithms)
8. [Tricount Import](#8-tricount-import)
9. [Architectural Decisions](#9-architectural-decisions)

---

## 1. Repository Layout

```
counted/
├── packages/               # Rust workspace (backend + WASM)
│   ├── shared/             # DTOs and shared types (Rust + WASM)
│   ├── api/                # Server functions, controllers, repositories
│   ├── ui/                 # Dioxus components and routes (compiled to WASM)
│   ├── web/                # App entry point — wires api + ui
│   ├── desktop/            # Desktop target (future)
│   └── mobile/             # Mobile target (future)
├── frontend-react/counted/ # React SPA (legacy, being sunset)
├── migrations/             # sqlx migration files (shared DB schema)
├── Dockerfile              # Backend multi-stage build
└── docker-compose.yml      # Production orchestration
```

The Rust workspace uses resolver 2. `packages/web` is the actual build artifact — it produces both a WASM client bundle and a native server binary via `dx bundle`.

---

## 2. Tech Stack

| Layer               | Technology            | Version |
| ------------------- | --------------------- | ------- |
| Language (backend)  | Rust                  | stable  |
| Fullstack framework | Dioxus                | 0.7.2   |
| Web server          | axum                  | 0.7     |
| Async runtime       | tokio                 | 1.43    |
| Database            | PostgreSQL            | 15      |
| DB driver           | sqlx                  | 0.8.6   |
| Frontend framework  | React                 | 19      |
| Frontend routing    | React Router          | 7       |
| Server state        | TanStack React Query  | 5       |
| Forms               | React Hook Form + Zod | 7 / 4   |
| Styling             | TailwindCSS + DaisyUI | 4 / 5   |
| Build tool (React)  | Vite                  | 7       |
| Build tool (Rust)   | dx (Dioxus CLI)       | —       |
| Reverse proxy       | nginx                 | alpine  |
| Deployment          | Docker Compose        | —       |

---

## 3. Backend — Dioxus Fullstack

### 3.1 How Server Functions Work

Dioxus fullstack provides macros that compile differently depending on the build target:

```rust
#[get("/api/v1/projects")]
pub async fn get_projects() -> Result<Vec<ProjectDto>, ServerFnError> { ... }

#[post("/api/v1/expenses")]
pub async fn add_expense(Json(payload): Json<CreatableExpense>) -> Result<Expense, ServerFnError> { ... }
```

- **`--features server`** (native binary): the macro generates an axum route handler. The function body runs server-side.
- **`--features web`** (WASM): the macro generates a fetch call to the declared URL. The body is replaced entirely. Callers must wrap JSON payloads: `add_expense(Json(payload))`.

Available macros: `#[get]`, `#[post]`, `#[put]`, `#[delete]`, `#[server]` (generic).

Entry point: `packages/web/src/main.rs` calls `dioxus::launch(app)`, which handles both WASM hydration and server-side rendering/routing via the fullstack runtime.

### 3.2 Package Roles

| Package  | Role                                                                                                                                                              |
| -------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `shared` | Pure data: DTOs, enums, serialization contracts. No framework dependency. Compiled into both the server binary and the WASM bundle.                               |
| `api`    | All server-side logic: controllers, repositories, DB access. Feature-gated: server-only code is under `[features] server` (sqlx, axum, tokio, etc. are optional). |
| `ui`     | Dioxus components, pages, routes. Compiled to WASM for the browser.                                                                                               |
| `web`    | Thin entry point. Provides context, registers all server functions, produces the final artifact via `dx bundle --web --release --package web`.                    |

### 3.3 Module Structure (`packages/api/src/`)

```
api/src/
├── lib.rs                  # Module registry
├── db.rs                   # PgPool initialisation from DATABASE_URL
├── utils.rs                # round_currency helper
├── users/
│   ├── users_controller.rs # #[get]/[post]/[delete] endpoints
│   └── users_repository.rs # sqlx queries
├── expenses/
│   ├── expenses_controller.rs
│   └── expenses_repository.rs
├── projects/
│   ├── projects_controller.rs
│   └── projects_repository.rs
├── payments/
│   ├── payments_controller.rs
│   ├── payments_repository.rs
│   └── balances.rs         # Reimbursement algorithm
├── tricount/
│   ├── tricount_controller.rs
│   ├── tricount_client.rs  # RSA handshake + HTTP client
│   └── tricount_models.rs  # Tricount API response types
└── sse/
    └── sse.rs              # Server-Sent Events broadcaster
```

### 3.4 Shared Types (`packages/shared/src/lib.rs`)

All DTOs live here. They are serialised as `camelCase` JSON (`#[serde(rename_all = "camelCase")]`) to match the React frontend convention.

PostgreSQL enums are mapped via `sqlx::Type` and are conditionally derived:

```rust
#[cfg_attr(feature = "server", derive(sqlx::Type))]
#[cfg_attr(feature = "server", sqlx(type_name = "expense_type", rename_all = "lowercase"))]
pub enum ExpenseType { Expense, Transfer, Gain }
```

`FromRow` is similarly conditional:

```rust
#[cfg_attr(feature = "server", derive(sqlx::FromRow))]
pub struct ProjectDto { ... }
```

This keeps `shared` free of server-only dependencies when compiled to WASM.

### 3.5 Database Access

All queries use **non-macro sqlx** (`sqlx::query(...)`, `sqlx::query_as::<_, T>(...)`) instead of the `query!` / `query_as!` macros.

**Why**: The macro variants require a live database connection at compile time (for type checking). Checking the `api` package alone (`cargo check --package api --features server`) would fail unless PostgreSQL is running. Non-macro sqlx checks types at runtime instead.

**Correct compile target**: `cargo check --package web --features server` (or `--features web` for client).

Pool initialisation reads `DATABASE_URL` from the environment at startup.

### 3.6 Real-Time Updates (SSE)

It is not used and maybe never used :
`packages/api/src/sse/sse.rs` implements a broadcast channel using `tokio` + `async-stream`. The server publishes events (expense created/updated/deleted, project changes) and the client subscribes via a standard `EventSource` connection. The `EventSSE` enum in `shared` defines the event types.

---

## 4. Database

### 4.1 Migration Timeline

All migrations live in `/migrations/`. They are run by sqlx at startup via `runMigrationsAndBinary.sh`.

| Date       | Migration                      | Change                                                    |
| ---------- | ------------------------------ | --------------------------------------------------------- |
| 2025-06-15 | `users`                        | CREATE TABLE users                                        |
| 2025-06-15 | `create_expenses_and_projects` | CREATE TABLE projects, expenses; CREATE TYPE expense_type |
| 2025-06-15 | `create_payments`              | CREATE TABLE payments                                     |
| 2025-06-15 | `create_user_projects`         | CREATE TABLE user_projects                                |
| 2025-07-11 | `project.description`          | ALTER projects ADD description                            |
| 2026-02-08 | `project.deletion.cascade`     | Add CASCADE on expenses/payments FK                       |
| 2026-02-08 | `user.deletion.cascade`        | Add CASCADE on payments FK                                |
| 2026-02-13 | `expense.date`                 | ALTER expenses ADD date (DATE)                            |
| 2026-02-25 | `project.status`               | CREATE TYPE project_status; ALTER projects ADD status     |

### 4.2 Schema

#### `users`

| Column     | Type             | Notes                         |
| ---------- | ---------------- | ----------------------------- |
| id         | SERIAL PK        | —                             |
| name       | VARCHAR NOT NULL | Display name within a project |
| balance    | DOUBLE PRECISION | Nullable legacy field         |
| created_at | TIMESTAMP        | DEFAULT current_timestamp     |

Users are project-scoped — the same person creates a new user record per project.

#### `projects`

| Column      | Type               | Notes                                                  |
| ----------- | ------------------ | ------------------------------------------------------ |
| id          | UUID PK            | gen_random_uuid()                                      |
| name        | VARCHAR NOT NULL   | —                                                      |
| description | VARCHAR            | Nullable                                               |
| currency    | VARCHAR NOT NULL   | e.g. "EUR"                                             |
| created_at  | TIMESTAMP NOT NULL | —                                                      |
| status      | project_status     | `ongoing` \| `closed` \| `archived`, DEFAULT `ongoing` |

#### `expenses`

| Column       | Type                      | Notes                               |
| ------------ | ------------------------- | ----------------------------------- |
| id           | SERIAL PK                 | —                                   |
| author_id    | INTEGER FK → users        | Who entered the expense             |
| project_id   | UUID FK → projects        | CASCADE delete                      |
| created_at   | TIMESTAMP NOT NULL        | Record creation time                |
| date         | DATE                      | Actual expense date (user-supplied) |
| amount       | DOUBLE PRECISION NOT NULL | Total amount                        |
| name         | VARCHAR NOT NULL          | Label                               |
| description  | VARCHAR                   | Optional note                       |
| expense_type | expense_type              | `expense` \| `transfer` \| `gain`   |

#### `payments`

| Column     | Type                      | Notes                          |
| ---------- | ------------------------- | ------------------------------ |
| id         | SERIAL PK                 | —                              |
| expense_id | INTEGER FK → expenses     | —                              |
| user_id    | INTEGER FK → users        | CASCADE delete                 |
| is_debt    | BOOLEAN NOT NULL          | `false` = paid / `true` = owes |
| amount     | DOUBLE PRECISION NOT NULL | —                              |
| created_at | TIMESTAMP NOT NULL        | —                              |

Each expense produces multiple payment rows: one per payer (`is_debt = false`) and one per debtor (`is_debt = true`).

#### `user_projects`

Junction table linking users to projects (composite PK: `project_id`, `user_id`).

### 4.3 Design Notes

- **Hard deletes with CASCADE** throughout — no soft deletes.
- `expenses.date` (DATE) is separate from `expenses.created_at` (TIMESTAMP). Users can log past expenses with a historical date.
- PostgreSQL enums (`expense_type`, `project_status`) are used for DB-level constraint enforcement.

---

## 5. Frontend — React

The React app in `frontend-react/counted/` is a separate SPA that talks to the backend via REST. It predates the Dioxus rewrite and will eventually be replaced by the Dioxus UI (`packages/ui/`).

### 5.1 Routing

React Router v7 (browser history):

```
/                                      → Projects (dashboard)
/projects/:projectId                   → ProjectLayout
  (index)                              → ProjectDetails (expenses, balance, reimbursements)
  expenses/:expenseId                  → PaymentPage (single expense breakdown)
```

### 5.2 State Management

Three layers:

| Layer                  | Tool                         | What it manages                            |
| ---------------------- | ---------------------------- | ------------------------------------------ |
| Server state           | TanStack React Query         | API data, caching, mutations, invalidation |
| Persistent local state | React Context + localStorage | Project/user associations across sessions  |
| Form state             | react-hook-form + Zod        | Modal forms with validation                |

No Redux or Zustand — React Query covers all server state, Context covers the small amount of persistent client state.

**`CountedLocalStorageContext`**: Stores an array of `{ projectId, userId }` pairs. This is the only persistent state — it lets the user see their projects on the dashboard and remember which participant they are in each project. No sensitive data is stored.

### 5.3 API Layer

`src/services/` contains thin fetch wrappers (get, post, put, delete) with automatic JSON serialisation. All data fetching goes through React Query hooks in `src/hooks/`:

| Hook file              | Purpose                              |
| ---------------------- | ------------------------------------ |
| `useProjects.ts`       | CRUD for projects, status updates    |
| `useExpenses.ts`       | CRUD for expenses, summary/balance   |
| `useUsers.ts`          | Add/delete project participants      |
| `usePayments.ts`       | Fetch payments by expense or project |
| `useImportTricount.ts` | Trigger Tricount import              |
| `useLocalStorage.ts`   | Hydrate and update localStorage      |

### 5.4 Component Patterns

- Dialogs use native HTML `<dialog>` via `showModal()` / `close()`. The `openDialog` / `closeDialog` utils in `src/utils/open-dialog.ts` manage both the React state flag and the DOM call.
- The `Dropdown` component wraps a DaisyUI dropdown with a settings icon, used in page headers.
- Error boundaries wrap all pages; `ErrorFallback` displays a user-friendly error UI.

### 5.5 Styling

TailwindCSS v4 via the `@tailwindcss/vite` plugin, with DaisyUI v5 component classes. Theme: `cupcake` (set via `data-theme` attribute on the root element).

---

## 6. Infrastructure

### 6.1 Docker Compose

Three services on `counted-network` (bridge):

| Service    | Build                        | Ports   | Role                                                |
| ---------- | ---------------------------- | ------- | --------------------------------------------------- |
| `frontend` | Node 25 build → nginx alpine | 80, 443 | Serves React SPA; proxies `/api/*` to backend       |
| `backend`  | cargo-chef → minimal runtime | 8080    | Dioxus fullstack server; runs migrations on startup |
| `db`       | postgres:15 image            | 5432    | PostgreSQL; `pgdata` volume for persistence         |

`backend` depends on `db` with a `service_healthy` condition (pg_isready healthcheck).

### 6.2 Backend Build (Dockerfile)

Multi-stage using `cargo-chef` for dependency caching:

1. **Planner**: generates `recipe.json` from `Cargo.lock`
2. **Cook**: restores cached compiled dependencies
3. **Builder**: compiles the app (`dx bundle --web --release --package web`)
4. **Runtime**: minimal image, copies binary + `runMigrationsAndBinary.sh`

### 6.3 Nginx

The frontend container serves the React SPA and reverse-proxies all `/api/*` traffic to the backend.

**Security configuration:**

- TLS 1.2+ only, Let's Encrypt certificates
- HSTS `max-age=31536000` (1 year)
- Rate limiting: 10 req/s per IP, 10 concurrent connections per IP (returns 429)
- Security headers: `X-Frame-Options: DENY`, `X-Content-Type-Options: nosniff`, strict CSP, `Referrer-Policy: strict-origin-when-cross-origin`
- SPA fallback: `try_files $uri $uri/ /index.html`
- WebSocket upgrade headers forwarded to backend (for SSE / future WS)

### 6.4 Environment Variables

| Variable       | Consumer | Purpose                      |
| -------------- | -------- | ---------------------------- |
| `DATABASE_URL` | backend  | PostgreSQL connection string |

---

## 7. Key Algorithms

### 7.1 Reimbursement Suggestions

**Location**: `packages/api/src/payments/balances.rs`

Given a set of users and their net balances (positive = owed money, negative = owes money), the algorithm produces the minimal set of transfers to settle all debts.

**Two-phase greedy approach:**

1. **Exact-match phase**: Iterate over users with positive balances. For each, find a user with an exactly opposite negative balance. Pair them as a single transfer. This eliminates the easy cases without splitting.

2. **Greedy phase**: Sort remaining balances descending. Repeatedly pair the largest creditor with the largest debtor. If their amounts match, one transfer settles both. If not, the smaller balance is fully settled and the larger balance is reduced by the settled amount (cascade). Repeat until all balances are zero.

**Result**: A `Vec<ReimbursementSuggestion>` with `(debtor_id, creditor_id, amount)` tuples. The algorithm is covered by 15+ unit tests in the same file.

---

## 8. Tricount Import

**Location**: `packages/api/src/tricount/`

Allows users to migrate an existing Tricount project into Counted.

**Flow:**

1. User provides a Tricount share link (contains an opaque key).
2. Backend generates a 2048-bit RSA keypair per request (`rsa` crate).
3. Sends the public key PEM to the Tricount API (`api.tricount.bunq.com`) for session registration.
4. Receives a session token, then fetches the full project registry (expenses, members, allocations).
5. Maps the Tricount data model to Counted's schema:
   - Members → `users` + `user_projects`
   - Entries → `expenses` + `payments` (payers + debtors)
   - Amounts are split and rounded via `round_currency`
   - Dates are parsed from Tricount's format (first 10 chars → `YYYY-MM-DD`)
6. Everything is inserted in a single logical operation; the created project is returned to the caller.

**Dependencies**: `rsa = "0.9"`, `reqwest = "0.12"` (rustls-tls), `base64 = "0.22"`, `rand = "0.8"` — all optional under the `server` feature.

---

## 9. Architectural Decisions

### Why Dioxus fullstack instead of a separate Rust API?

Single Rust codebase for both server and client. Server function macros (`#[get]`, `#[post]`) provide type-safe RPC with no API schema maintenance. The same `CreatableExpense` type is used in the DB layer, the server function, and the WASM call site — the compiler enforces the contract.

### Why keep the React frontend?

The React SPA predates the Dioxus rewrite. It has a richer component ecosystem (React Hook Form, DaisyUI v5, React Query DevTools) and was quicker to iterate on for the initial product. It will be sunset once the Dioxus UI reaches feature parity.

### Why non-macro sqlx?

The `query!` / `query_as!` macros perform type checking against a live database at compile time. This forces every developer to have PostgreSQL running just to run `cargo check`. Non-macro sqlx trades compile-time type safety for developer ergonomics — a reasonable trade at this scale.

### Why no authentication yet?

The current access model is URL-based: anyone with a project UUID can access it. This works for the initial use case (share the link with friends). Authentication is planned as a strictly additive layer: owned projects will require a session, but existing URL-shared projects will remain accessible. Chosen design: email + password with argon2 hashing, stateful sessions (UUID cookie → `sessions` DB table), HttpOnly + SameSite=Lax cookies.

### Why stateful sessions over JWT?

Sessions are immediately revocable (just delete the row). JWTs require additional infrastructure (token blacklist or short expiry + refresh tokens) to achieve the same. Stateful sessions are simpler and fit the scale.

### Why HttpOnly cookies over Authorization header?

HttpOnly cookies are invisible to JavaScript — they cannot be stolen via XSS. The browser sends them automatically, so no token management is needed on the client. SameSite=Lax provides CSRF protection for state-mutating requests.

## 10. Documentation

More documentation on architectural / tech / feature choices could be find in ./docs folder.
