# Database & Transactions

## Overview

Every database operation in the API is wrapped in an explicit transaction. This guarantees atomicity across all operations — if any query in a handler fails, the entire operation rolls back and the database is left in its prior state.

This is especially critical for multi-step operations like expense creation (INSERT expense + INSERT payments), tricount imports (project + users + N × (expense + payments)), and user creation (INSERT users + INSERT user_projects).

---

## Why Always Transactions

The choice is uniformity over selectivity: every handler begins a transaction, even read-only ones. This eliminates the cognitive overhead of deciding per-operation whether a transaction is needed, prevents partial writes on failure, and adds negligible overhead (PostgreSQL wraps every bare statement in an implicit transaction anyway).

---

## Executor Pattern

Repository functions accept `&mut PgConnection` instead of fetching a pool internally. This allows any caller to pass either:

- A raw connection (`pool.acquire().await?`)
- A transaction (`&mut *tx` via `DerefMut`)

**Repository function signature:**

```rust
pub async fn add_expense(
    executor: &mut PgConnection,
    expense: CreatableExpense,
) -> Result<i32, ServerFnError> {
    sqlx::query_scalar!("INSERT INTO expenses ... RETURNING id", ...)
        .fetch_one(&mut *executor)
        .await?
}
```

The `&mut *executor` reborrow is required for multi-query functions so that the mutable reference remains usable after each call.

---

## Handler Pattern

Every controller handler follows this structure:

```rust
#[post("/api/v1/some/route")]
pub async fn handler(...) -> Result<..., ServerFnError> {
    let pool = get_db().await;
    let mut tx = pool.begin().await.map_err(|e| ServerFnError::new(e.to_string()))?;

    // all repository calls use &mut *tx
    let result = some_repository::some_fn(&mut *tx, ...).await?;

    tx.commit().await.map_err(|e| ServerFnError::new(e.to_string()))?;
    Ok(result)
}
```

**Auto-rollback:** if any `?` propagates an error before `tx.commit()`, the `Transaction` is dropped and sqlx automatically rolls back. No explicit `tx.rollback()` is needed.

---

## Exception: Session Validation

`get_current_account_id()` in [`utils.rs`](../packages/api/src/utils.rs) performs its own session lookup using a dedicated connection — not the handler's transaction. This is intentional: session validation is a prerequisite check independent of the business operation.

```rust
let mut conn = pool.acquire().await.ok()?;
auth_repository::get_session_account_id(&mut *conn, session_id).await.ok()?
```

---

## Key Files

| File | Role |
|---|---|
| [`packages/api/src/db.rs`](../packages/api/src/db.rs) | `get_db()` — creates a PgPool from `DATABASE_URL` |
| [`packages/api/src/expenses/expenses_repository.rs`](../packages/api/src/expenses/expenses_repository.rs) | Expense queries |
| [`packages/api/src/payments/payments_repository.rs`](../packages/api/src/payments/payments_repository.rs) | Payment queries |
| [`packages/api/src/users/users_repository.rs`](../packages/api/src/users/users_repository.rs) | User + user_projects queries |
| [`packages/api/src/projects/projects_repository.rs`](../packages/api/src/projects/projects_repository.rs) | Project queries |
| [`packages/api/src/auth/auth_repository.rs`](../packages/api/src/auth/auth_repository.rs) | Account + session queries |

---

## Adding New Repository Functions

1. Accept `executor: &mut PgConnection` as the first parameter
2. Use `&mut *executor` on every `.fetch_*()` / `.execute()` call
3. Never call `get_db()` inside a repository function

## Adding New Controller Handlers

1. Call `get_db().await` once at the top
2. Call `pool.begin().await` to open a transaction
3. Pass `&mut *tx` to all repository calls
4. Call `tx.commit().await` at the end before returning `Ok(...)`
