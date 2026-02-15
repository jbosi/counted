use anyhow::Context;
use dioxus::prelude::*;
use std::collections::HashMap;
use uuid::Uuid;

#[cfg(feature = "server")]
use crate::db::get_db;
use crate::utils::round_currency;
#[cfg(feature = "server")]
use crate::{
    expenses::expenses_repository::get_expenses_by_project_id,
    payments::balances::get_reimbursement_suggestions,
};
use shared::{Expense, ExpenseType, NewPayment, Payment};

#[cfg(feature = "server")]
use sqlx::{Pool, Postgres};

#[cfg(feature = "server")]
pub async fn add_payments(creatable_payments: Vec<NewPayment>) -> Result<(), ServerFnError> {
    let pool: Pool<Postgres> = get_db().await;

    let expense_ids: Vec<i32> = creatable_payments.iter().map(|p| p.expense_id).collect();
    let user_ids: Vec<i32> = creatable_payments.iter().map(|p| p.user_id).collect();
    let is_debts: Vec<bool> = creatable_payments.iter().map(|p| p.is_debt).collect();
    let amounts: Vec<f64> = creatable_payments.iter().map(|p| p.amount).collect();

    sqlx::query_scalar::<_, i32>(
        r"
        INSERT INTO payments
         (
            expense_id,
            user_id,
            is_debt,
            amount
        ) SELECT * FROM UNNEST(
            $1::INT4[],
            $2::INT4[],
            $3::BOOL[],
            $4::FLOAT8[]
        ) RETURNING id",
    )
    .bind(&expense_ids)
    .bind(&user_ids)
    .bind(&is_debts)
    .bind(&amounts)
    .fetch_all(&pool)
    .await
    .context("Failed add payments")
    .map_err(|e| ServerFnError::new(e.to_string()))?;

    Ok(())
}

#[cfg(feature = "server")]
pub async fn get_payments_by_expense_id(expense_id: i32) -> Result<Vec<Payment>, ServerFnError> {
    let pool: Pool<Postgres> = get_db().await;

    let payments: Vec<Payment> = sqlx::query_as!(
        Payment,
        "SELECT id, expense_id, user_id, is_debt, amount, created_at \
        FROM payments \
        WHERE expense_id = $1",
        expense_id
    )
    .fetch_all(&pool)
    .await
    .context("Failed get payments")
    .map_err(|e| ServerFnError::new(e.to_string()))?;

    Ok(payments)
}

#[cfg(feature = "server")]
pub async fn get_payments_by_user_id(user_id: i32) -> Result<Vec<Payment>, ServerFnError> {
    let pool: Pool<Postgres> = get_db().await;

    let payments: Vec<Payment> = sqlx::query_as!(
        Payment,
        "SELECT id, expense_id, user_id, is_debt, amount, created_at \
        FROM payments \
        WHERE user_id = $1",
        user_id
    )
    .fetch_all(&pool)
    .await
    .context("Failed get payments")
    .map_err(|e| ServerFnError::new(e.to_string()))?;

    Ok(payments)
}

#[cfg(feature = "server")]
pub async fn get_payments_by_expense_ids(
    expense_ids: Vec<i32>,
) -> Result<Vec<Payment>, ServerFnError> {
    let pool: Pool<Postgres> = get_db().await;

    let payments: Vec<Payment> = sqlx::query_as!(
        Payment,
        "SELECT id, expense_id, user_id, is_debt, amount, created_at \
        FROM payments \
        WHERE expense_id = ANY($1)",
        &expense_ids[..] // a bug of the parameter typechecking code requires all array parameters to be slices
    )
    .fetch_all(&pool)
    .await
    .context("Failed get payments")
    .map_err(|e| ServerFnError::new(e.to_string()))?;

    Ok(payments)
}

#[cfg(feature = "server")]
pub async fn delete_payments_by_expense_id(expense_id: i32) -> Result<(), ServerFnError> {
    let pool: Pool<Postgres> = get_db().await;

    sqlx::query!("DELETE FROM payments WHERE expense_id = $1", expense_id)
        .execute(&pool)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    Ok(())
}

#[cfg(feature = "server")]
pub async fn get_summary_by_project_id(
    project_id: Uuid,
) -> Result<shared::UserSummary, ServerFnError> {
    use shared::UserBalance;
    let pool: Pool<Postgres> = get_db().await;

    let expenses: Vec<Expense> = get_expenses_by_project_id(project_id)
        .await
        .context("Failed get expenses")
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    let expense_ids: Vec<i32> = expenses.iter().map(|expense| expense.id).collect();

    let payments: Vec<Payment> = sqlx::query_as!(
        Payment,
        "SELECT id, expense_id, user_id, is_debt, amount, created_at \
        FROM payments \
        WHERE expense_id = ANY($1)",
        &expense_ids[..] // a bug of the parameter typechecking code requires all array parameters to be slices
    )
    .fetch_all(&pool)
    .await
    .context("Failed get payments")
    .map_err(|e| ServerFnError::new(e.to_string()))?;

    let balances = calculate_balances(&expenses, &payments);

    let reimbursement_suggestions = get_reimbursement_suggestions(
        balances
            .iter()
            .map(|(user_id, amount)| UserBalance { amount: *amount, user_id: *user_id })
            .collect(),
    );

    let user_summary = shared::UserSummary { reimbursement_suggestions, summary: balances };

    Ok(user_summary)
}

/// Calculates user balances from expenses and payments.
///
/// For Expense and Transfer types:
/// - Payers (is_debt=false) have positive balance
/// - Debtors (is_debt=true) have negative balance
///
/// For Gain types:
/// - Receivers (is_debt=true) have positive balance (inverted)
/// - Contributors (is_debt=false) have negative balance (inverted)
pub fn calculate_balances(expenses: &[Expense], payments: &[Payment]) -> HashMap<i32, f64> {
    // Create a lookup map from expense_id to expense_type for balance calculation
    let expense_type_by_id: HashMap<i32, ExpenseType> =
        expenses.iter().map(|expense| (expense.id, expense.expense_type.clone())).collect();

    let mut balances: HashMap<i32, f64> = HashMap::new();

    payments.iter().for_each(|payment| {
        let expense_type = expense_type_by_id
            .get(&payment.expense_id)
            .expect("Payment references non-existent expense");

        // For Gain expenses, invert the balance logic
        let multiplier = if matches!(expense_type, ExpenseType::Gain) { -1.0 } else { 1.0 };
        let base_adjustment = if payment.is_debt { -payment.amount } else { payment.amount };
        let balance_adjustment = base_adjustment * multiplier;

        if let Some(existing_balance) = balances.get_mut(&payment.user_id) {
            *existing_balance = round_currency(*existing_balance + balance_adjustment);
        } else {
            balances.insert(payment.user_id, round_currency(balance_adjustment));
        }
    });

    balances
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Helper function to create a mock expense
    fn create_expense(id: i32, expense_type: ExpenseType) -> Expense {
        use chrono::DateTime;

        Expense {
            id,
            author_id: 1,
            project_id: Uuid::new_v4(),
            created_at: DateTime::from_timestamp(0, 0).unwrap().naive_utc(),
            date: chrono::NaiveDate::from_ymd_opt(1970, 1, 1).unwrap(),
            amount: 0.0, // Not used in balance calculation
            description: None,
            name: "Test Expense".to_string(),
            expense_type,
        }
    }

    /// Helper function to create a mock payment
    fn create_payment(
        id: i32,
        expense_id: i32,
        user_id: i32,
        is_debt: bool,
        amount: f64,
    ) -> Payment {
        use chrono::DateTime;

        Payment {
            id,
            expense_id,
            user_id,
            is_debt,
            amount,
            created_at: DateTime::from_timestamp(0, 0).unwrap().naive_utc(),
        }
    }

    #[test]
    fn test_simple_expense_one_payer_one_debtor() {
        let expenses = vec![create_expense(1, ExpenseType::Expense)];

        let payments = vec![
            create_payment(1, 1, 100, false, 50.0), // User 100 paid 50
            create_payment(2, 1, 101, true, 50.0),  // User 101 owes 50
        ];

        let balances = calculate_balances(&expenses, &payments);

        assert_eq!(balances.get(&100), Some(&50.0));
        assert_eq!(balances.get(&101), Some(&-50.0));

        // Balances should sum to zero
        let sum: f64 = balances.values().sum();
        assert!((sum - 0.0).abs() < 0.01);
    }

    #[test]
    fn test_simple_gain_one_receiver_one_contributor() {
        let expenses = vec![create_expense(1, ExpenseType::Gain)];

        let payments = vec![
            create_payment(1, 1, 100, true, 30.0), // User 100 receives 30 (marked as debt but inverted)
            create_payment(2, 1, 101, false, 30.0), // User 101 contributes 30 (marked as payer but inverted)
        ];

        let balances = calculate_balances(&expenses, &payments);

        assert_eq!(balances.get(&100), Some(&30.0)); // Receiver has positive balance
        assert_eq!(balances.get(&101), Some(&-30.0)); // Contributor has negative balance

        // Balances should sum to zero
        let sum: f64 = balances.values().sum();
        assert!((sum - 0.0).abs() < 0.01);
    }

    #[test]
    fn test_transfer_same_as_expense() {
        let expenses = vec![create_expense(1, ExpenseType::Transfer)];

        let payments = vec![
            create_payment(1, 1, 100, false, 25.0), // User 100 transfers 25
            create_payment(2, 1, 101, true, 25.0),  // User 101 receives 25
        ];

        let balances = calculate_balances(&expenses, &payments);

        assert_eq!(balances.get(&100), Some(&25.0));
        assert_eq!(balances.get(&101), Some(&-25.0));
    }

    #[test]
    fn test_multiple_payments_same_expense() {
        let expenses = vec![create_expense(1, ExpenseType::Expense)];

        let payments = vec![
            create_payment(1, 1, 100, false, 60.0), // User 100 paid 60
            create_payment(2, 1, 101, true, 20.0),  // User 101 owes 20
            create_payment(3, 1, 102, true, 20.0),  // User 102 owes 20
            create_payment(4, 1, 103, true, 20.0),  // User 103 owes 20
        ];

        let balances = calculate_balances(&expenses, &payments);

        assert_eq!(balances.get(&100), Some(&60.0));
        assert_eq!(balances.get(&101), Some(&-20.0));
        assert_eq!(balances.get(&102), Some(&-20.0));
        assert_eq!(balances.get(&103), Some(&-20.0));

        // Balances should sum to zero
        let sum: f64 = balances.values().sum();
        assert!((sum - 0.0).abs() < 0.01);
    }

    #[test]
    fn test_complex_mixed_expense_types() {
        // Complex scenario: Multiple users, multiple expenses of different types
        let expenses = vec![
            create_expense(1, ExpenseType::Expense), // Expense 1: Restaurant bill
            create_expense(2, ExpenseType::Gain),    // Expense 2: Lottery win
            create_expense(3, ExpenseType::Transfer), // Expense 3: Money transfer
            create_expense(4, ExpenseType::Expense), // Expense 4: Groceries
            create_expense(5, ExpenseType::Gain),    // Expense 5: Refund received
        ];

        let payments = vec![
            // Expense 1: User 1 paid $120 for dinner, Users 2, 3, 4 each owe $30
            create_payment(1, 1, 1, false, 120.0),
            create_payment(2, 1, 2, true, 30.0),
            create_payment(3, 1, 3, true, 30.0),
            create_payment(4, 1, 4, true, 30.0),
            create_payment(5, 1, 1, true, 30.0), // User 1 also owes their share
            // Expense 2: User 2 and User 3 won $100 lottery, User 1 and 4 contributed $50 each
            create_payment(6, 2, 2, true, 50.0), // User 2 receives 50 (Gain, so inverted)
            create_payment(7, 2, 3, true, 50.0), // User 3 receives 50 (Gain, so inverted)
            create_payment(8, 2, 1, false, 50.0), // User 1 contributed 50 (Gain, so inverted)
            create_payment(9, 2, 4, false, 50.0), // User 4 contributed 50 (Gain, so inverted)
            // Expense 3: User 3 transferred $40 to User 4
            create_payment(10, 3, 3, false, 40.0),
            create_payment(11, 3, 4, true, 40.0),
            // Expense 4: User 4 paid $80 for groceries, everyone owes $20
            create_payment(12, 4, 4, false, 80.0),
            create_payment(13, 4, 1, true, 20.0),
            create_payment(14, 4, 2, true, 20.0),
            create_payment(15, 4, 3, true, 20.0),
            create_payment(16, 4, 4, true, 20.0),
            // Expense 5: User 1 received $60 refund, User 2, 3, 4 contributed $20 each
            create_payment(17, 5, 1, true, 60.0), // User 1 receives 60 (Gain, so inverted)
            create_payment(18, 5, 2, false, 20.0), // User 2 contributed 20 (Gain, so inverted)
            create_payment(19, 5, 3, false, 20.0), // User 3 contributed 20 (Gain, so inverted)
            create_payment(20, 5, 4, false, 20.0), // User 4 contributed 20 (Gain, so inverted)
        ];

        let balances = calculate_balances(&expenses, &payments);

        // Calculate expected balances:
        // User 1: +120 (paid exp1) -30 (owes exp1) -50 (contributed gain2) -20 (owes exp4) +60 (received gain5)
        //       = 120 - 30 - 50 - 20 + 60 = 80
        let expected_user1 = 120.0 - 30.0 - 50.0 - 20.0 + 60.0;

        // User 2: -30 (owes exp1) +50 (received gain2) -20 (owes exp4) -20 (contributed gain5)
        //       = -30 + 50 - 20 - 20 = -20
        let expected_user2 = -30.0 + 50.0 - 20.0 - 20.0;

        // User 3: -30 (owes exp1) +50 (received gain2) +40 (transferred exp3) -20 (owes exp4) -20 (contributed gain5)
        //       = -30 + 50 + 40 - 20 - 20 = 20
        let expected_user3 = -30.0 + 50.0 + 40.0 - 20.0 - 20.0;

        // User 4: -30 (owes exp1) -50 (contributed gain2) -40 (received exp3) +80 (paid exp4) -20 (owes exp4) -20 (contributed gain5)
        //       = -30 - 50 - 40 + 80 - 20 - 20 = -80
        let expected_user4 = -30.0 - 50.0 - 40.0 + 80.0 - 20.0 - 20.0;

        assert_eq!(balances.get(&1), Some(&expected_user1));
        assert_eq!(balances.get(&2), Some(&expected_user2));
        assert_eq!(balances.get(&3), Some(&expected_user3));
        assert_eq!(balances.get(&4), Some(&expected_user4));

        // Balances should sum to zero
        let sum: f64 = balances.values().sum();
        assert!((sum - 0.0).abs() < 0.01, "Balances should sum to zero, got {}", sum);
    }

    #[test]
    fn test_edge_case_zero_amounts() {
        let expenses = vec![create_expense(1, ExpenseType::Expense)];

        let payments =
            vec![create_payment(1, 1, 100, false, 0.0), create_payment(2, 1, 101, true, 0.0)];

        let balances = calculate_balances(&expenses, &payments);

        assert_eq!(balances.get(&100), Some(&0.0));
        assert_eq!(balances.get(&101), Some(&0.0));
    }

    #[test]
    fn test_edge_case_single_user() {
        let expenses = vec![create_expense(1, ExpenseType::Expense)];

        let payments =
            vec![create_payment(1, 1, 100, false, 50.0), create_payment(2, 1, 100, true, 50.0)];

        let balances = calculate_balances(&expenses, &payments);

        // User paid and owes the same amount
        assert_eq!(balances.get(&100), Some(&0.0));
    }

    #[test]
    fn test_rounding_precision() {
        let expenses = vec![create_expense(1, ExpenseType::Expense)];

        let payments = vec![
            create_payment(1, 1, 100, false, 10.0),
            create_payment(2, 1, 101, true, 3.333333333),
            create_payment(3, 1, 102, true, 3.333333333),
            create_payment(4, 1, 103, true, 3.333333334),
        ];

        let balances = calculate_balances(&expenses, &payments);

        // Should be rounded to 2 decimal places
        assert_eq!(balances.get(&100), Some(&10.0));
        assert_eq!(balances.get(&101), Some(&-3.33));
        assert_eq!(balances.get(&102), Some(&-3.33));
        assert_eq!(balances.get(&103), Some(&-3.33));
    }

    #[test]
    fn test_user_reported_issue() {
        // Exact data from user's application
        let expenses = vec![
            create_expense(5, ExpenseType::Expense), // Expense 5: "Dépense 1"
            create_expense(6, ExpenseType::Transfer), // Expense 6: "Transfert 1"
            create_expense(7, ExpenseType::Gain),    // Expense 7: "Gain"
        ];

        let payments = vec![
            // Expense 5 payments
            create_payment(13, 5, 5, true, 50.0), // User 5 owes 50
            create_payment(14, 5, 6, true, 50.0), // User 6 owes 50
            create_payment(15, 5, 5, false, 100.0), // User 5 paid 100
            // Expense 6 payments
            create_payment(16, 6, 6, true, 20.0), // User 6 owes 20
            create_payment(17, 6, 5, false, 20.0), // User 5 paid 20
            // Expense 7 payments (Gain)
            create_payment(18, 7, 5, true, 15.0), // User 5 receives 15
            create_payment(19, 7, 6, true, 15.0), // User 6 receives 15
            create_payment(20, 7, 5, false, 30.0), // User 5 contributes 30
        ];

        let balances = calculate_balances(&expenses, &payments);

        // Expected calculation:
        // User 5:
        //   Expense 5: -50 (owes) + 100 (paid) = +50
        //   Expense 6: +20 (paid transfer)
        //   Expense 7 (Gain): +15 (receives, inverted) - 30 (contributes, inverted) = -15
        //   Total: 50 + 20 - 15 = +55
        //
        // User 6:
        //   Expense 5: -50 (owes)
        //   Expense 6: -20 (owes transfer)
        //   Expense 7 (Gain): +15 (receives, inverted)
        //   Total: -50 - 20 + 15 = -55

        println!("User 5 balance: {:?}", balances.get(&5));
        println!("User 6 balance: {:?}", balances.get(&6));

        assert_eq!(balances.get(&5), Some(&55.0), "User 5 should have balance of +55.0");
        assert_eq!(balances.get(&6), Some(&-55.0), "User 6 should have balance of -55.0");
    }
}
