use chrono::NaiveDateTime;
use crate::payments::domain::payment_model::Payment;
use crate::projects::application::project_application::forge_balance_from_payments;
use crate::projects::domain::balance_model::UserBalance;
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

    let user_id_1: &UserBalance = result.balances.iter().find(|b| b.user_id == 1).unwrap();
    let user_id_2: &UserBalance = result.balances.iter().find(|b| b.user_id == 2).unwrap();
    
    assert_eq!(result.currency, "€");
    assert_eq!(result.total_expenses, 50.0);
    assert_eq!(result.balances.len(), 2);
    assert_eq!(user_id_1.amount, 25.0);
    assert_eq!(user_id_1.user_name, "Alice");
    assert_eq!(user_id_2.amount, -25.0);
    assert_eq!(user_id_2.user_name, "Bob");

    assert_eq!(result.reimbursement_suggestions[0].amount, 25.0);
    assert_eq!(result.reimbursement_suggestions[0].user_id_payer, 1);
    assert_eq!(result.reimbursement_suggestions[0].user_id_debtor, 2);
}

#[test]
fn test_forge_balance_from_payments_with_correct_user_balances_advanced() {
    // Prepare test data
    let payments = vec![
        Payment { id: 1, expense_id: 1, user_id: 1, is_debt: false, amount: 100.0, created_at: NaiveDateTime::from_timestamp(0, 0) },
        Payment { id: 2, expense_id: 1, user_id: 2, is_debt: true, amount: 30.0, created_at: NaiveDateTime::from_timestamp(0, 0) },
        Payment { id: 3, expense_id: 1, user_id: 1, is_debt: true, amount: 30.0, created_at: NaiveDateTime::from_timestamp(0, 0) },
        Payment { id: 4, expense_id: 1, user_id: 3, is_debt: true, amount: 40.0, created_at: NaiveDateTime::from_timestamp(0, 0) },
    ];

    let users_from_payments = vec![
        User { id: 1, name: "Cece".to_string(), balance: Some(0.0), created_at: Some(NaiveDateTime::from_timestamp(0, 0)) },
        User { id: 2, name: "Bob".to_string(), balance: Some(0.0), created_at: Some(NaiveDateTime::from_timestamp(0, 0)) },
        User { id: 3, name: "Jo".to_string(), balance: Some(0.0), created_at: Some(NaiveDateTime::from_timestamp(0, 0)) },
    ];

    // Call the function under test
    let result = forge_balance_from_payments(payments, users_from_payments);

    // Assertions
    let reimbursement_suggestions_user_2 = result.reimbursement_suggestions.iter().find(|u| u.user_id_debtor == 2).unwrap();

    assert_eq!(reimbursement_suggestions_user_2.amount, 30.0);
    assert_eq!(reimbursement_suggestions_user_2.user_id_payer, 1);

    let reimbursement_suggestions_user_3 = result.reimbursement_suggestions.iter().find(|u| u.user_id_debtor == 3).unwrap();

    assert_eq!(reimbursement_suggestions_user_3.amount, 40.0);
    assert_eq!(reimbursement_suggestions_user_3.user_id_payer, 1);
}

#[test]
fn test_forge_balance_from_payments_with_correct_user_balances_complex() {
    // Prepare test data
    let payments = vec![
        Payment { id: 1, expense_id: 1, user_id: 1, is_debt: false, amount: 100.0, created_at: NaiveDateTime::from_timestamp(0, 0) },
        Payment { id: 2, expense_id: 1, user_id: 2, is_debt: true, amount: 30.0, created_at: NaiveDateTime::from_timestamp(0, 0) },
        Payment { id: 3, expense_id: 1, user_id: 3, is_debt: true, amount: 40.0, created_at: NaiveDateTime::from_timestamp(0, 0) },
        Payment { id: 4, expense_id: 1, user_id: 1, is_debt: true, amount: 30.0, created_at: NaiveDateTime::from_timestamp(0, 0) },

        Payment { id: 5, expense_id: 2, user_id: 2, is_debt: false, amount: 150.0, created_at: NaiveDateTime::from_timestamp(0, 0) },
        Payment { id: 6, expense_id: 2, user_id: 1, is_debt: true, amount: 50.0, created_at: NaiveDateTime::from_timestamp(0, 0) },
        Payment { id: 7, expense_id: 2, user_id: 3, is_debt: true, amount: 10.0, created_at: NaiveDateTime::from_timestamp(0, 0) },
        Payment { id: 8, expense_id: 2, user_id: 2, is_debt: true, amount: 90.0, created_at: NaiveDateTime::from_timestamp(0, 0) },
    ];

    let users_from_payments = vec![
        User { id: 1, name: "Cece".to_string(), balance: Some(100.0), created_at: Some(NaiveDateTime::from_timestamp(0, 0)) },
        User { id: 2, name: "Bob".to_string(), balance: Some(50.0), created_at: Some(NaiveDateTime::from_timestamp(0, 0)) },
        User { id: 3, name: "Jo".to_string(), balance: Some(50.0), created_at: Some(NaiveDateTime::from_timestamp(0, 0)) },
    ];

    // Call the function under test
    let result = forge_balance_from_payments(payments, users_from_payments);

    // Assertions
    let reimbursement_suggestions_user_2 = result.reimbursement_suggestions.iter().find(|u| u.user_id_payer == 2).unwrap();

    assert_eq!(reimbursement_suggestions_user_2.amount, 30.0);
    assert_eq!(reimbursement_suggestions_user_2.user_id_debtor, 3);

    let reimbursement_suggestions_user_3 = result.reimbursement_suggestions.iter().find(|u| u.user_id_payer == 1).unwrap();

    assert_eq!(reimbursement_suggestions_user_3.amount, 20.0);
    assert_eq!(reimbursement_suggestions_user_3.user_id_debtor, 3);
}