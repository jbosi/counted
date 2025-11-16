// use dioxus::logger::tracing::info;
// use serde::{Deserialize, Serialize};
// use shared::{Payment, User};
// use std::collections::HashMap;
// use std::cmp::Ordering;
// use std::ops::Sub;
//
// #[derive(Serialize, Deserialize, Debug, Clone)]
// pub struct Balance {
//     pub balances: Vec<UserBalance>,
//     pub currency: String,
//     pub total_expenses: f64,
//     pub reimbursement_suggestions: Vec<ReimbursementSuggestion>,
// }
//
// #[derive(Serialize, Deserialize, Debug, Clone)]
// pub struct UserBalance {
//     pub amount: f64,
//     pub user_id: i32,
//     pub user_name: String,
// }
//
// #[derive(Serialize, Deserialize, Debug, Clone)]
// pub struct UserBalanceComputation {
//     pub amount: f64,
//     pub remaining_amount: f64,
// }
//
// #[derive(Serialize, Deserialize, Debug, Clone)]
// pub struct ReimbursementSuggestion {
//     pub amount: f64,
//     pub user_id_debtor: i32,
//     // pub user_name_debtor: String,
//     pub user_id_payer: i32,
//     // pub user_name_payer: String,
// }
//
// pub fn forge_balance_from_payments(
//     payments: Vec<Payment>,
//     users_from_payments: Vec<User>,
// ) -> Balance {
//     let mut balance = get_initial_balance_values();
//
//     let (balances_by_user, total_expenses) = get_balances_by_user(payments);
//
//     for (user_id, amount) in balances_by_user {
//         balance.balances.push(UserBalance {
//             user_id: user_id,
//             amount: amount,
//             user_name: users_from_payments.iter().find(|u| u.id == user_id).cloned().unwrap().name,
//         });
//
//         balance.total_expenses = total_expenses;
//         balance.currency = "€".to_string()
//     }
//
//     balance.reimbursement_suggestions = get_reimbursement_suggestions(balance.clone());
//
//     return balance;
// }
//
// fn get_initial_balance_values() -> Balance {
//     return Balance {
//         balances: vec![],
//         currency: "".to_string(),
//         total_expenses: 0.0,
//         reimbursement_suggestions: vec![],
//     };
// }
//
// fn get_balances_by_user(payments: Vec<Payment>) -> (HashMap<i32, f64>, f64) {
//     let mut balances_by_user: HashMap<i32, f64> = Default::default();
//     let mut total_expenses: f64 = 0.0;
//
//     for payment in payments {
//         let mut default_insert: f64 = 0.0;
//
//         if payment.is_debt {
//             default_insert.sub_assign(payment.amount);
//         } else {
//             default_insert.add_assign(payment.amount);
//             total_expenses.add_assign(payment.amount)
//         }
//         balances_by_user
//             .entry(payment.user_id)
//             .and_modify(|p| {
//                 if payment.is_debt {
//                     return p.sub_assign(payment.amount);
//                 } else {
//                     return p.add_assign(payment.amount);
//                 }
//             })
//             .or_insert(default_insert);
//     }
//
//     return (balances_by_user, total_expenses);
// }
//
// fn get_reimbursement_suggestions(mut balance: Balance) -> Vec<ReimbursementSuggestion> {
//     let mut result: Vec<ReimbursementSuggestion> = Vec::new();
//
//     if balance.balances.is_empty() {
//         return result;
//     }
//
//     // desc order for positive and negative amounts
//     balance.balances.sort_by(|a, b| b.amount.abs().partial_cmp(&a.amount.abs()).unwrap());
//
//     let (mut unsolved_positive_balances_by_user, mut unsolved_negative_balances_by_user) =
//         get_unresolved_balances_by_user(&mut balance);
//
//     let equally_opposed_reimbursement_suggestions: Vec<ReimbursementSuggestion> =
//         resolve_equally_opposed_balances(
//             &mut unsolved_positive_balances_by_user,
//             &mut unsolved_negative_balances_by_user,
//         );
//     result = [result, equally_opposed_reimbursement_suggestions].concat();
//
//     let remaining_balances = resolve_remaining_balances(
//         &mut unsolved_positive_balances_by_user,
//         &mut unsolved_negative_balances_by_user,
//     );
//     result = [result, remaining_balances].concat();
//
//     // TODO handle small differences / computation errors
//
//     return result;
// }
//
// /////////////////////
// // Est-ce que ce ne serait pas mieux de faire la somme de tous les payments par userId (permet d'éliminer ceux qui sont à l'équilibre)
// // Ensuite on calcule qui doit combien a qui
// // Beaucoup plus simple
// // fn sum_
//
// // Examples
// // P 10, 30, 50 - 10, 30, 50
// // N 30, 60     - 30, 30, 30
//
// // Je pense que l'idéal serait de :
// // - trier les deux listes par ordre "extrème" :check:
// // - Pour chaque valeur, maximale (neg ou pos), retrancher n opposés
// // - Si il reste une valeur, regarder si il y a un équivalent négatif après chaque itération sinon retirer le reste
// fn resolve_remaining_balances(
//     unsolved_positive_balances_by_user: &mut HashMap<i32, UserBalanceComputation>,
//     unsolved_negative_balances_by_user: &mut HashMap<i32, UserBalanceComputation>,
// ) -> Vec<ReimbursementSuggestion> {
//     let mut result: Vec<ReimbursementSuggestion> = Vec::new();
//
//     let sorted_unsolved_positive_balances_by_user: HashMap<i32, UserBalanceComputation> =
//         unsolved_positive_balances_by_user
//             .iter()
//             .sorted_by(|(u1, b1), (u2, b2)| {
//                 f64::total_cmp(&b2.remaining_amount.abs(), &b1.remaining_amount.abs())
//             })
//             .map(|(a, b)| (a.clone(), b.clone())) // Needed for collect to build HashMap
//             .collect();
//
//     let sorted_unsolved_negative_balances_by_user: HashMap<i32, UserBalanceComputation> =
//         unsolved_negative_balances_by_user
//             .iter()
//             .sorted_by(|(u1, b1), (u2, b2)| {
//                 f64::total_cmp(&b2.remaining_amount.abs(), &b1.remaining_amount.abs())
//             })
//             .map(|(a, b)| (a.clone(), b.clone())) // Needed for collect to build HashMap
//             .collect();
//
//     while sorted_unsolved_positive_balances_by_user.len() > 0 {
//         let previous_lengths = sorted_unsolved_positive_balances_by_user.len()
//             + sorted_unsolved_negative_balances_by_user.len();
//
//         // Find max & min among balances
//         let MaxBalance { is_debt, max_balance, opposite_balances: min_balances } = get_max_balance(
//             sorted_unsolved_positive_balances_by_user.clone(),
//             sorted_unsolved_negative_balances_by_user.clone(),
//         );
//         let (opposite_balances_used, remainder) =
//             solve_max_balance(max_balance.clone(), min_balances);
//
//         opposite_balances_used.iter().for_each(|(user_id, _)| {
//             if (is_debt) {
//                 let previous_balance = unsolved_positive_balances_by_user.get(user_id).unwrap();
//                 result.push(ReimbursementSuggestion {
//                     amount: previous_balance.remaining_amount.abs(),
//                     user_id_debtor: max_balance.0,
//                     user_id_payer: *user_id,
//                 });
//
//                 unsolved_positive_balances_by_user.remove(user_id);
//             } else {
//                 let previous_balance = unsolved_negative_balances_by_user.get(user_id).unwrap();
//                 result.push(ReimbursementSuggestion {
//                     amount: previous_balance.remaining_amount.abs(),
//                     user_id_debtor: *user_id,
//                     user_id_payer: max_balance.0,
//                 });
//
//                 unsolved_negative_balances_by_user.remove(user_id);
//             }
//         });
//
//         let has_remainder = remainder.0 != 0;
//         if (has_remainder) {
//             if (is_debt) {
//                 let value = unsolved_positive_balances_by_user.get_mut(&remainder.0);
//                 value.unwrap().remaining_amount -= remainder.1.remaining_amount
//             } else {
//                 let value = unsolved_negative_balances_by_user.get_mut(&remainder.0);
//                 value.unwrap().remaining_amount -= remainder.1.remaining_amount
//             }
//         }
//
//         let current_lengths = sorted_unsolved_positive_balances_by_user.len()
//             + sorted_unsolved_negative_balances_by_user.len();
//
//         if (current_lengths == 0) {
//             break;
//         }
//
//         if (previous_lengths == current_lengths) {
//             info!("Unable to solve all balances");
//             break;
//         }
//     }
//
//     return result;
// }
//
// // returns the list of min_balances that are fully compensated and optionally the remainder
// fn solve_max_balance(
//     max_balance: (i32, UserBalanceComputation),
//     min_balances: HashMap<i32, UserBalanceComputation>,
// ) -> (HashMap<i32, UserBalanceComputation>, (i32, UserBalanceComputation)) {
//     let mut fully_compensated_balances = HashMap::new();
//     let mut remainder: (i32, UserBalanceComputation) =
//         (0, UserBalanceComputation { remaining_amount: 0.0, amount: 0.0 });
//
//     let mut max_balance_amount = max_balance.1.remaining_amount.abs();
//     for min_balance in min_balances {
//         if (max_balance_amount.total_cmp(&0.0) != Ordering::Greater) {
//             break;
//         }
//
//         let min_balance_amount = min_balance.1.remaining_amount;
//
//         if (max_balance_amount.total_cmp(&min_balance_amount) == Ordering::Greater) {
//             fully_compensated_balances.insert(min_balance.0, min_balance.1.clone());
//             max_balance_amount = max_balance_amount.sub(min_balance_amount);
//             continue;
//         }
//
//         if (max_balance_amount.total_cmp(&min_balance_amount) == Ordering::Equal) {
//             fully_compensated_balances.insert(min_balance.0, min_balance.1.clone());
//             continue;
//         }
//
//         if (max_balance_amount.total_cmp(&min_balance_amount) == Ordering::Less) {
//             remainder = (
//                 min_balance.0,
//                 UserBalanceComputation {
//                     remaining_amount: min_balance.1.remaining_amount.sub(max_balance_amount),
//                     amount: min_balance.1.amount,
//                 },
//             );
//             break;
//         }
//     }
//
//     return (fully_compensated_balances, remainder);
// }
//
// // Returns the max balance to solve only and the Hashmap of the min balances
// fn get_max_balance(
//     sorted_unsolved_positive_balances_by_user: HashMap<i32, UserBalanceComputation>,
//     sorted_unsolved_negative_balances_by_user: HashMap<i32, UserBalanceComputation>,
// ) -> MaxBalance {
//     let positive_max: Option<(i32, UserBalanceComputation)> =
//         sorted_unsolved_positive_balances_by_user
//             .iter()
//             .max_by(|(u1, b1), (u2, b2)| {
//                 f64::total_cmp(&b2.remaining_amount.abs(), &b1.remaining_amount.abs())
//             })
//             .map(|(a, b)| (a.clone(), b.clone()));
//
//     let negative_max: Option<(i32, UserBalanceComputation)> =
//         sorted_unsolved_negative_balances_by_user
//             .iter()
//             .max_by(|(u1, b1), (u2, b2)| {
//                 f64::total_cmp(&b2.remaining_amount.abs(), &b1.remaining_amount.abs())
//             })
//             .map(|(a, b)| (a.clone(), b.clone()));
//
//     if (positive_max.is_none() || negative_max.is_none()) {
//         // Log error here ?
//         return MaxBalance {
//             is_debt: false,
//             max_balance: (0, UserBalanceComputation { remaining_amount: 0.0, amount: 0.0 }),
//             opposite_balances: HashMap::new(),
//         };
//     }
//
//     let positive_max_value: (i32, UserBalanceComputation) = positive_max.unwrap();
//     let negative_max_value: (i32, UserBalanceComputation) = negative_max.unwrap();
//
//     if (positive_max_value.1.remaining_amount.abs() >= negative_max_value.1.remaining_amount.abs())
//     {
//         return MaxBalance {
//             is_debt: false,
//             max_balance: positive_max_value,
//             opposite_balances: sorted_unsolved_negative_balances_by_user,
//         };
//     } else {
//         return MaxBalance {
//             is_debt: true,
//             max_balance: negative_max_value,
//             opposite_balances: sorted_unsolved_positive_balances_by_user,
//         };
//     }
// }
//
// fn resolve_negative_balances(
//     unsolved_positive_balances_by_user: &mut HashMap<i32, UserBalanceComputation>,
//     unsolved_negative_balances_by_user: &mut HashMap<i32, UserBalanceComputation>,
// ) -> Vec<ReimbursementSuggestion> {
//     const RESULT: Vec<ReimbursementSuggestion> = Vec::new();
//
//     for (positive_user_id, positive_balance_amount) in &mut *unsolved_positive_balances_by_user {
//         let mut positive_balance_amount_mut = positive_balance_amount.remaining_amount;
//
//         for (negative_user_id, negative_balance_amount) in &mut *unsolved_negative_balances_by_user
//         {
//             if (positive_balance_amount_mut.eq(&0.0)) {
//                 break;
//             }
//
//             if (negative_balance_amount
//                 .remaining_amount
//                 .abs()
//                 .total_cmp(&positive_balance_amount_mut)
//                 == Ordering::Less
//                 || negative_balance_amount
//                     .remaining_amount
//                     .abs()
//                     .total_cmp(&positive_balance_amount_mut)
//                     == Ordering::Equal)
//             {
//                 RESULT.push(ReimbursementSuggestion {
//                     amount: negative_balance_amount.remaining_amount,
//                     user_id_debtor: *negative_user_id,
//                     user_id_payer: *positive_user_id,
//                 });
//
//                 positive_balance_amount_mut
//                     .sub_assign(negative_balance_amount.remaining_amount.abs());
//                 // unsolved_negative_balances_by_user.remove(negative_user_id);
//             }
//         }
//
//         //
//         if (positive_balance_amount.remaining_amount.total_cmp(&positive_balance_amount_mut)
//             != Ordering::Equal
//             && !positive_balance_amount_mut.eq(&0.0))
//         {
//             let (negative_user_id, negative_balance_amount) =
//                 unsolved_negative_balances_by_user.iter().collect_vec()[0];
//
//             RESULT.push(ReimbursementSuggestion {
//                 amount: positive_balance_amount_mut,
//                 user_id_debtor: *negative_user_id,
//                 user_id_payer: *positive_user_id,
//             });
//
//             // unsolved_positive_balances_by_user.remove(positive_user_id);
//             // unsolved_negative_balances_by_user.iter().filter(|(u, b)| u == negative_user_id).update(|(u, b)| (u, b.add_assign(positive_balance_amount_mut)));
//
//             // eprint!("amount is not completely resolved for user id {0} with a value of {1}", positive_user_id, positive_balance_amount_mut)
//         }
//     }
//
//     return RESULT;
// }
//
// fn resolve_equally_opposed_balances(
//     unsolved_positive_balances_by_user: &mut HashMap<i32, UserBalanceComputation>,
//     unsolved_negative_balances_by_user: &mut HashMap<i32, UserBalanceComputation>,
// ) -> Vec<ReimbursementSuggestion> {
//     let mut resolved_users: Vec<(i32, i32)> = Vec::new();
//     let mut result: Vec<ReimbursementSuggestion> = Vec::new();
//
//     for (positive_user_id, balance_amount) in &mut *unsolved_positive_balances_by_user {
//         let matching_equal_negative_balance = unsolved_negative_balances_by_user
//             .iter_mut()
//             .filter(|(debtor_id, _)| {
//                 !resolved_users
//                     .iter()
//                     .any(|(_, resolved_debtor_id)| resolved_debtor_id == *debtor_id)
//             })
//             .find(|(_, b_amount)| {
//                 b_amount.remaining_amount.abs().total_cmp(&balance_amount.remaining_amount)
//                     == Ordering::Equal
//             });
//
//         if let Some((resolved_user_id_debtor, negative_user_balance)) =
//             matching_equal_negative_balance
//         {
//             result.push(ReimbursementSuggestion {
//                 amount: balance_amount.remaining_amount,
//                 user_id_debtor: *resolved_user_id_debtor,
//                 user_id_payer: *positive_user_id,
//             });
//
//             balance_amount.remaining_amount = 0.0;
//             negative_user_balance.remaining_amount = 0.0;
//
//             resolved_users.push((*positive_user_id, *resolved_user_id_debtor));
//         }
//     }
//
//     resolved_users.iter().for_each(|(payer_id, debtor_id)| {
//         unsolved_positive_balances_by_user.remove(payer_id);
//         unsolved_negative_balances_by_user.remove(debtor_id);
//     });
//
//     return result;
// }
//
// fn get_unresolved_balances_by_user(
//     balance: &mut Balance,
// ) -> (HashMap<i32, UserBalanceComputation>, HashMap<i32, UserBalanceComputation>) {
//     let mut unsolved_positive_balances_by_user: HashMap<i32, UserBalanceComputation> =
//         Default::default();
//     let mut unsolved_negative_balances_by_user: HashMap<i32, UserBalanceComputation> =
//         Default::default();
//
//     for user_balance in balance.balances.iter() {
//         if user_balance.amount.is_sign_positive() {
//             unsolved_positive_balances_by_user.insert(
//                 user_balance.user_id,
//                 UserBalanceComputation {
//                     remaining_amount: user_balance.amount,
//                     amount: user_balance.amount,
//                 },
//             );
//         } else {
//             unsolved_negative_balances_by_user.insert(
//                 user_balance.user_id,
//                 UserBalanceComputation {
//                     remaining_amount: user_balance.amount,
//                     amount: user_balance.amount,
//                 },
//             );
//         }
//     }
//     (unsolved_positive_balances_by_user, unsolved_negative_balances_by_user)
// }
//
// struct MaxBalance {
//     is_debt: bool,
//     max_balance: (i32, UserBalanceComputation),
//     opposite_balances: HashMap<i32, UserBalanceComputation>,
// }
