use chrono::NaiveDateTime;
use crate::payments::domain::payment_model::Payment;
use crate::projects::application::project_application::forge_balance_from_payments;
use crate::users::domain::user_model::User;

#[test]
fn test_forge_balance_from_payments_with_correct_user_balances() {
    // Prepare test data
    let payments = vec![
        Payment { id: 1, expense_id: 1, user_id: 1, is_debt: false, amount: 50.0, created_at: NaiveDateTime::from_timestamp(0, 0) },
        Payment { id: 2, expense_id: 2, user_id: 2, is_debt: true, amount: 25.0, created_at: NaiveDateTime::from_timestamp(0, 0) },
        Payment { id: 3, expense_id: 3, user_id: 1, is_debt: true, amount: 25.0, created_at: NaiveDateTime::from_timestamp(0, 0) },
    ];

    let users_from_payments = vec![
        User { id: 1, name: "Alice".to_string(), balance: Some(100.0), created_at: Some(NaiveDateTime::from_timestamp(0, 0)) },
        User { id: 2, name: "Bob".to_string(), balance: Some(50.0), created_at: Some(NaiveDateTime::from_timestamp(0, 0)) },
    ];

    // Call the function under test
    let result = forge_balance_from_payments(payments, users_from_payments);

    // Assertions
    assert_eq!(result.currency, "€");
    assert_eq!(result.total_expenses, 50.0);
    assert_eq!(result.balances.len(), 2);
    assert_eq!(result.balances[0].user_id, 1);
    assert_eq!(result.balances[0].amount, 25.0);
    assert_eq!(result.balances[0].user_name, "Alice");
    assert_eq!(result.balances[1].user_id, 2);
    assert_eq!(result.balances[1].amount, -25.0);
    assert_eq!(result.balances[1].user_name, "Bob");
}

#[test]
fn test_forge_balance_from_payments_with_correct_user_reimbursement_suggestions() {
    // Prepare test data
    let payments = vec![
        Payment { id: 1, expense_id: 1, user_id: 1, is_debt: false, amount: 50.0, created_at: NaiveDateTime::from_timestamp(0, 0) },
        Payment { id: 2, expense_id: 2, user_id: 2, is_debt: true, amount: 25.0, created_at: NaiveDateTime::from_timestamp(0, 0) },
        Payment { id: 3, expense_id: 3, user_id: 1, is_debt: true, amount: 25.0, created_at: NaiveDateTime::from_timestamp(0, 0) },
    ];

    let users_from_payments = vec![
        User { id: 1, name: "Alice".to_string(), balance: Some(100.0), created_at: Some(NaiveDateTime::from_timestamp(0, 0)) },
        User { id: 2, name: "Bob".to_string(), balance: Some(50.0), created_at: Some(NaiveDateTime::from_timestamp(0, 0)) },
    ];

    // Call the function under test
    let result = forge_balance_from_payments(payments, users_from_payments);

    // Assertions
    assert_eq!(result.reimbursement_suggestions[0].amount, 25.0);
    assert_eq!(result.reimbursement_suggestions[0].user_id_payer, 1);
    assert_eq!(result.reimbursement_suggestions[0].user_id_debtor, 2);
}