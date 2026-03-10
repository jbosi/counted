# Authentication & Authorization

## Overview

Authentication was added as a strictly additive layer on top of the existing URL-sharing model. Projects created before auth (or shared via UUID URL without an owner) remain accessible without a session. Authenticated users own their projects and are the only ones who can access or mutate them.

Design decisions are documented in [DOCUMENTATION.md §9](../DOCUMENTATION.md#9-architectural-decisions):

- **Email + password** with Argon2 hashing
- **Stateful sessions**: UUID stored in a DB table (immediately revocable)
- **HttpOnly + SameSite=Lax cookies** (XSS-safe, CSRF-protected)

---

## Database Schema

### `accounts`

| Column             | Type             | Notes                                              |
| ------------------ | ---------------- | -------------------------------------------------- |
| id                 | UUID PK          | `gen_random_uuid()`                                |
| email              | VARCHAR NOT NULL | UNIQUE constraint                                  |
| password_hash      | VARCHAR NOT NULL | Argon2id hash, never exposed                       |
| display_name       | VARCHAR NOT NULL | Shown in UI                                        |
| created_at         | TIMESTAMP        | DEFAULT current_timestamp                          |
| failed_login_count | INTEGER          | Incremented on each failed login, reset on success |
| locked_until       | TIMESTAMP        | Nullable — account locked until this time when set |

### `sessions`

| Column     | Type      | Notes                                       |
| ---------- | --------- | ------------------------------------------- |
| id         | UUID PK   | `gen_random_uuid()` — used as cookie value  |
| account_id | UUID FK   | References `accounts(id)` ON DELETE CASCADE |
| created_at | TIMESTAMP | DEFAULT current_timestamp                   |
| expires_at | TIMESTAMP | Set to `NOW() + 30 days` at creation        |

### `projects.owner_account_id`

Nullable `UUID` FK referencing `accounts(id)` ON DELETE SET NULL. When null, the project is URL-accessible to anyone (legacy mode). When set, only the owning account can access it.

---

## Registration Flow

**Endpoint**: `POST /api/v1/auth/register`
**File**: [packages/api/src/auth/auth_controller.rs](../packages/api/src/auth/auth_controller.rs)

1. Check email uniqueness — return error if already taken
2. Generate a random salt with `OsRng` and hash the password with `Argon2::default()` (Argon2id, RFC 9106 recommended params)
3. Insert row into `accounts`
4. Create a session (see [Session Management](#session-management))
5. Return the `Account` DTO (no password hash)

---

## Login Flow

**Endpoint**: `POST /api/v1/auth/login`
**File**: [packages/api/src/auth/auth_controller.rs](../packages/api/src/auth/auth_controller.rs)

1. Fetch account by email — return generic `"Invalid email or password"` if not found (prevents email enumeration)
2. Check `locked_until > NOW()` — return `"Account temporarily locked"` if true (see [Brute Force Protection](#brute-force-protection))
3. Parse stored hash and verify with `Argon2::default().verify_password()` (constant-time)
4. On failure: call `increment_failed_login()`, return `"Invalid email or password"`
5. On success: call `reset_failed_login()`, create a session, return the `Account` DTO

---

## Logout Flow

**Endpoint**: `POST /api/v1/auth/logout`

1. Extract `session_id` UUID from the `Cookie` request header
2. Delete the session row from DB (immediate revocation)
3. Set `session_id=; Max-Age=0` cookie in response to clear it client-side

---

## Session Management

**File**: [packages/api/src/auth/auth_repository.rs](../packages/api/src/auth/auth_repository.rs)

Sessions are created in `create_session_and_set_cookie()`:

- `expires_at` = `NOW() + 30 days`
- Session UUID comes from PostgreSQL `gen_random_uuid()` (CSPRNG)
- The UUID is set as a cookie (see [Cookie Security](#cookie-security))

Session validation runs on every authenticated request via `get_current_account_id()` in [packages/api/src/utils.rs](../packages/api/src/utils.rs):

1. Parse `session_id` UUID from `Cookie` header
2. Query: `SELECT account_id FROM sessions WHERE id = $1 AND expires_at > NOW()`
3. Returns `Option<Uuid>` — `None` on any failure (expired, invalid, missing)

---

## Brute Force Protection

Two complementary layers protect against credential attacks:

### 1. Nginx rate limiting (volumetric attacks)

**File**: [frontend-react/counted/nginx.conf](../frontend-react/counted/nginx.conf)

A dedicated `auth_limit` zone applies to `POST /api/v1/auth/login` and `POST /api/v1/auth/register`:

```nginx
limit_req_zone $binary_remote_addr zone=auth_limit:10m rate=5r/m;

location ~ ^/api/v1/auth/(login|register) {
    limit_req zone=auth_limit burst=2 nodelay;
    limit_req_status 429;
    ...
}
```

- **5 requests/minute per IP** (vs. the general 10 req/s zone — effectively no protection for auth)
- `burst=2` allows 2 rapid requests (e.g. a page load + immediate submit) before enforcing the limit
- Returns `429 Too Many Requests` — handled before the request reaches Rust
- Protects against flood attacks and credential stuffing at minimal cost (memory-only, no DB)

This location block is placed **before** the generic `/api` block so nginx matches it first (regex `~` takes priority over prefix `/api`).

### 2. Account lockout (targeted attacks)

**Files**: [packages/api/src/auth/auth_controller.rs](../packages/api/src/auth/auth_controller.rs), [packages/api/src/auth/auth_repository.rs](../packages/api/src/auth/auth_repository.rs)

Per-account lockout handles attackers who stay under the IP rate limit (e.g. rotating proxies):

- After **5 consecutive failed logins**, `locked_until` is set to `NOW() + 15 minutes` in the DB
- While locked, the login endpoint returns `"Account temporarily locked. Try again later."` — the password is not even checked
- On a **successful login**, `failed_login_count` and `locked_until` are reset to zero/null

The lockout check is implemented in `is_account_locked(locked_until, now) -> bool`, a pure function covered by 4 unit tests (`cargo test --package api`).

The DB update is atomic via a SQL CASE expression:

```sql
UPDATE accounts
SET
  failed_login_count = failed_login_count + 1,
  locked_until = CASE
    WHEN failed_login_count + 1 >= 5 THEN NOW() + INTERVAL '15 minutes'
    ELSE locked_until
  END
WHERE id = $1
```

> **DoS tradeoff**: Account lockout can theoretically be weaponised — an attacker who knows a victim's email can lock them out by failing 5 times. This is a known, accepted tradeoff for most apps at this scale. Mitigation: the 15-minute window is short and auto-expires; no manual intervention is needed.

---

## Cookie Security

The `session_id` cookie is set with:

```
session_id=<UUID>; HttpOnly; SameSite=Lax; Path=/; Max-Age=2592000[; Secure]
```

| Attribute  | Value       | Why                                                   |
| ---------- | ----------- | ----------------------------------------------------- |
| `HttpOnly` | always      | Cookie invisible to JavaScript — blocks XSS theft     |
| `SameSite` | `Lax`       | Sent on top-level navigations; blocks CSRF mutations  |
| `Path`     | `/`         | Available to all routes                               |
| `Max-Age`  | `2592000`   | 30 days (matches DB `expires_at`)                     |
| `Secure`   | default ON  | Omitted only when `COOKIE_SECURE=false` env var is set |

> **Local dev note**: Set `COOKIE_SECURE=false` in the devcontainer environment (already configured in `.devcontainer/docker-compose.yml`) to disable the `Secure` flag for HTTP-only local dev. Production gets `Secure` by default.

---

## Authorization Model

### Project endpoints

| Scenario                                                         | Behaviour                                         |
| ---------------------------------------------------------------- | ------------------------------------------------- |
| `owner_account_id IS NULL`                                       | Any request (authenticated or not) can read/write |
| `owner_account_id IS NOT NULL`, valid session matches owner      | Allowed                                           |
| `owner_account_id IS NOT NULL`, session missing or wrong account | `Forbidden`                                       |

This is enforced in [packages/api/src/projects/projects_controller.rs](../packages/api/src/projects/projects_controller.rs) on every GET, PUT, and DELETE handler.

`GET /api/v1/projects` returns:

- **Authenticated**: projects where `owner_account_id = current_account_id`
- **Unauthenticated**: projects where `owner_account_id IS NULL`

### Other endpoints (expenses, payments, users)

Expense, payment, and user endpoints **do not currently enforce authentication or project ownership**. See [Known Gaps](#known-gaps).

---

## Frontend Auth State

### Dioxus UI

**File**: [packages/web/src/main.rs](../packages/web/src/main.rs)

On app mount, `GET /api/v1/auth/me` is called. The result (`Option<Account>`) is stored in a Dioxus context signal and consumed by:

- **Navbar** ([packages/ui/src/common/navbar.rs](../packages/ui/src/common/navbar.rs)) — shows Login or Logout button
- **Projects page** ([packages/ui/src/projects/projects.rs](../packages/ui/src/projects/projects.rs)) — shows auth-gated UI (create project button, owned project list)

Routes `/login` and `/register` render [packages/ui/src/auth/login.rs](../packages/ui/src/auth/login.rs) and [packages/ui/src/auth/register.rs](../packages/ui/src/auth/register.rs).

### React Frontend

**Files**: [frontend-react/counted/src/](../frontend-react/counted/src/)

On app mount, `GET /api/v1/auth/me` is called via `useEffect` in `App.tsx`. The result is stored in `AuthContext`:

| Value | Meaning |
| --- | --- |
| `undefined` | Loading (request in flight) |
| `null` | Unauthenticated |
| `Account` | Authenticated |

**Key files:**

| File | Role |
| --- | --- |
| [frontend-react/counted/src/types/auth.model.ts](../frontend-react/counted/src/types/auth.model.ts) | `Account`, `RegisterPayload`, `LoginPayload` TS types |
| [frontend-react/counted/src/contexts/authContext.ts](../frontend-react/counted/src/contexts/authContext.ts) | `AuthContext` — holds `account` state |
| [frontend-react/counted/src/services/authService.ts](../frontend-react/counted/src/services/authService.ts) | `register`, `login`, `logout`, `me` fetch wrappers |
| [frontend-react/counted/src/hooks/useAuth.ts](../frontend-react/counted/src/hooks/useAuth.ts) | `useLogin`, `useRegister`, `useLogout` React Query mutations |
| [frontend-react/counted/src/pages/auth/LoginPage.tsx](../frontend-react/counted/src/pages/auth/LoginPage.tsx) | `/login` route — email + password form |
| [frontend-react/counted/src/pages/auth/RegisterPage.tsx](../frontend-react/counted/src/pages/auth/RegisterPage.tsx) | `/register` route — email + display name + password form |
| [frontend-react/counted/src/pages/auth/AccountPage.tsx](../frontend-react/counted/src/pages/auth/AccountPage.tsx) | `/account` route — account info + logout |

**Projects page header**: shows account `displayName` as a link to `/account` when authenticated, or a "Connexion" button to `/login` when not.

**Redirect guards**: `/login` and `/register` redirect to `/` if `account` is already set.

**Mutations behaviour:**
- Login / Register: set `account` in context, invalidate `['projects']` query
- Logout: set `account` to `null`, call `queryClient.clear()` to wipe all cached data

---

## Key Files

| File                                                                                                                      | Role                                                      |
| ------------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------- |
| [packages/api/src/auth/auth_controller.rs](../packages/api/src/auth/auth_controller.rs)                                   | register / login / logout / me endpoints, cookie creation |
| [packages/api/src/auth/auth_repository.rs](../packages/api/src/auth/auth_repository.rs)                                   | DB queries: create/get account, create/get/delete session |
| [packages/api/src/utils.rs](../packages/api/src/utils.rs)                                                                 | `get_current_account_id()` — reusable session validation  |
| [packages/api/src/projects/projects_controller.rs](../packages/api/src/projects/projects_controller.rs)                   | Project ownership enforcement                             |
| [packages/shared/src/lib.rs](../packages/shared/src/lib.rs)                                                               | `Account`, `RegisterPayload`, `LoginPayload` DTOs         |
| [migrations/20260220115825_create_accounts.up.sql](../migrations/20260220115825_create_accounts.up.sql)                   | accounts table                                            |
| [migrations/20260220115826_create_sessions.up.sql](../migrations/20260220115826_create_sessions.up.sql)                   | sessions table                                            |
| [migrations/20260220115827_project_owner_account_id.up.sql](../migrations/20260220115827_project_owner_account_id.up.sql) | owner_account_id column                                   |
| [migrations/20260308000000_account_lockout.up.sql](../migrations/20260308000000_account_lockout.up.sql)                   | failed_login_count + locked_until columns                 |
| [frontend-react/counted/nginx.conf](../frontend-react/counted/nginx.conf)                                                 | auth_limit rate zone + location block                     |

---

## Known Gaps

Items not yet implemented, ordered by severity:

| Gap                                             | Severity | Notes                                                                                        |
| ----------------------------------------------- | -------- | -------------------------------------------------------------------------------------------- |
| Expense / payment / user endpoints have no auth | Design   | Intentional — project UUID is the access token for anonymous projects (URL-sharing model)    |
| ~~No rate limiting on auth endpoints~~          | ~~High~~ | Implemented — nginx `auth_limit` zone (5 req/min/IP)                                         |
| ~~No account lockout after repeated failures~~  | ~~High~~ | Implemented — 5 failures → 15-min lockout in DB                                              |
| ~~Email enumeration via `/register`~~           | ~~High~~ | Fixed — generic `"Registration failed"` error regardless of email existence                  |
| ~~No input length validation~~                  | ~~High~~ | Fixed — email ≤ 254, password ≤ 128, display_name ≤ 100 enforced server-side                |
| ~~Argon2 blocking async thread~~                | ~~Critical~~ | Fixed — Argon2 runs in `tokio::task::spawn_blocking`                                     |
| ~~`Secure` cookie flag off by default~~         | ~~Medium~~ | Fixed — `Secure` is ON by default; opt-out via `COOKIE_SECURE=false` for local dev        |
| ~~No email normalization~~                      | ~~Medium~~ | Fixed — emails lowercased before storage and lookup                                        |
| No server-side password strength validation     | Medium   | Only frontend `minlength=8`, easily bypassed via API                                         |
| No email verification                           | Medium   | Accounts created with unverified email addresses                                             |
| No session cleanup job                          | Low      | Expired sessions accumulate; add `DELETE FROM sessions WHERE expires_at < NOW()` cron        |
| Manual cookie parsing                           | Low      | `logout` and `get_current_account_id` hand-parse the `Cookie` header; use the `cookie` crate |
| No frontend route guards                        | Low      | Routes are accessible in the browser regardless of auth state                                |
| No audit logging                                | Low      | No record of login / logout / failed attempts                                                |
