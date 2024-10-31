use chrono::NaiveDateTime;
use crate::payments::domain::payment_model::Payment;
use crate::projects::application::project_application::forge_balance_from_payments;
use crate::users::domain::user_model::User;

// Generates a balance with correct user balances from given payments
