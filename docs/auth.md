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

| Column        | Type             | Notes                        |
| ------------- | ---------------- | ---------------------------- |
| id            | UUID PK          | `gen_random_uuid()`          |
| email         | VARCHAR NOT NULL | UNIQUE constraint            |
| password_hash | VARCHAR NOT NULL | Argon2id hash, never exposed |
| display_name  | VARCHAR NOT NULL | Shown in UI                  |
| created_at    | TIMESTAMP        | DEFAULT current_timestamp    |

### `sessions`

| Column     | Type           | Notes                          |
| ---------- | -------------- | ------------------------------ |
| id         | UUID PK        | `gen_random_uuid()` — used as cookie value |
| account_id | UUID FK        | References `accounts(id)` ON DELETE CASCADE |
| created_at | TIMESTAMP      | DEFAULT current_timestamp      |
| expires_at | TIMESTAMP      | Set to `NOW() + 30 days` at creation |

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
2. Parse stored hash and verify with `Argon2::default().verify_password()` (constant-time)
3. Create a session
4. Return the `Account` DTO

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

## Cookie Security

The `session_id` cookie is set with:

```
session_id=<UUID>; HttpOnly; SameSite=Lax; Path=/; Max-Age=2592000[; Secure]
```

| Attribute     | Value       | Why                                                      |
| ------------- | ----------- | -------------------------------------------------------- |
| `HttpOnly`    | always      | Cookie invisible to JavaScript — blocks XSS theft       |
| `SameSite`    | `Lax`       | Sent on top-level navigations; blocks CSRF mutations     |
| `Path`        | `/`         | Available to all routes                                  |
| `Max-Age`     | `2592000`   | 30 days (matches DB `expires_at`)                        |
| `Secure`      | conditional | Only set when `COOKIE_SECURE=true` env var is present    |

> **Production note**: Set `COOKIE_SECURE=true` in docker-compose environment to enable the `Secure` flag. Without it the cookie is sent over HTTP — harmless behind TLS termination, but risky in non-HTTPS environments.

---

## Authorization Model

### Project endpoints

| Scenario | Behaviour |
|---|---|
| `owner_account_id IS NULL` | Any request (authenticated or not) can read/write |
| `owner_account_id IS NOT NULL`, valid session matches owner | Allowed |
| `owner_account_id IS NOT NULL`, session missing or wrong account | `Forbidden` |

This is enforced in [packages/api/src/projects/projects_controller.rs](../packages/api/src/projects/projects_controller.rs) on every GET, PUT, and DELETE handler.

`GET /api/v1/projects` returns:
- **Authenticated**: projects where `owner_account_id = current_account_id`
- **Unauthenticated**: projects where `owner_account_id IS NULL`

### Other endpoints (expenses, payments, users)

Expense, payment, and user endpoints **do not currently enforce authentication or project ownership**. See [Known Gaps](#known-gaps).

---

## Frontend Auth State

**File**: [packages/web/src/main.rs](../packages/web/src/main.rs)

On app mount, `GET /api/v1/auth/me` is called. The result (`Option<Account>`) is stored in a Dioxus context signal and consumed by:

- **Navbar** ([packages/ui/src/common/navbar.rs](../packages/ui/src/common/navbar.rs)) — shows Login or Logout button
- **Projects page** ([packages/ui/src/projects/projects.rs](../packages/ui/src/projects/projects.rs)) — shows auth-gated UI (create project button, owned project list)

Routes `/login` and `/register` render [packages/ui/src/auth/login.rs](../packages/ui/src/auth/login.rs) and [packages/ui/src/auth/register.rs](../packages/ui/src/auth/register.rs).

---

## Key Files

| File | Role |
|---|---|
| [packages/api/src/auth/auth_controller.rs](../packages/api/src/auth/auth_controller.rs) | register / login / logout / me endpoints, cookie creation |
| [packages/api/src/auth/auth_repository.rs](../packages/api/src/auth/auth_repository.rs) | DB queries: create/get account, create/get/delete session |
| [packages/api/src/utils.rs](../packages/api/src/utils.rs) | `get_current_account_id()` — reusable session validation |
| [packages/api/src/projects/projects_controller.rs](../packages/api/src/projects/projects_controller.rs) | Project ownership enforcement |
| [packages/shared/src/lib.rs](../packages/shared/src/lib.rs) | `Account`, `RegisterPayload`, `LoginPayload` DTOs |
| [migrations/20260220115825_create_accounts.up.sql](../migrations/20260220115825_create_accounts.up.sql) | accounts table |
| [migrations/20260220115826_create_sessions.up.sql](../migrations/20260220115826_create_sessions.up.sql) | sessions table |
| [migrations/20260220115827_project_owner_account_id.up.sql](../migrations/20260220115827_project_owner_account_id.up.sql) | owner_account_id column |

---

## Known Gaps

Items not yet implemented, ordered by severity:

| Gap | Severity | Notes |
|---|---|---|
| Expense / payment / user endpoints have no auth | Critical | Anyone with a project UUID can mutate its expenses |
| No rate limiting on auth endpoints | High | Login and register are open to brute force |
| No account lockout after repeated failures | High | No failed attempt tracking in DB |
| `Secure` cookie flag off by default | Medium | Requires `COOKIE_SECURE=true` env var to enable |
| No server-side password strength validation | Medium | Only frontend `minlength=8`, easily bypassed via API |
| No email verification | Medium | Accounts created with unverified email addresses |
| No session cleanup job | Low | Expired sessions accumulate; add `DELETE FROM sessions WHERE expires_at < NOW()` cron |
| Manual cookie parsing | Low | `logout` and `get_current_account_id` hand-parse the `Cookie` header; use the `cookie` crate |
| Dead `LoginPayload.status` field | Low | `LoginPayload` contains a `status: ProjectStatus` field that is never used |
| No frontend route guards | Low | Routes are accessible in the browser regardless of auth state |
| No audit logging | Low | No record of login / logout / failed attempts |
