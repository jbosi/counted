use crate::utils::round_currency;
use dioxus::logger::tracing::info;
use itertools::Itertools;
use shared::{ReimbursementSuggestion, UserBalance, UserBalanceComputation};
use std::cmp::Ordering;
use std::collections::HashMap;
use std::ops::Sub;

pub fn get_reimbursement_suggestions(
    mut balances: Vec<UserBalance>,
) -> Vec<ReimbursementSuggestion> {
    let mut result: Vec<ReimbursementSuggestion> = Vec::new();

    if balances.is_empty() {
        return result;
    }

    // desc order for positive and negative amounts
    balances.sort_by(|a, b| b.amount.abs().partial_cmp(&a.amount.abs()).unwrap());

    let (mut unsolved_positive_balances_by_user, mut unsolved_negative_balances_by_user) =
        get_unresolved_balances_by_user(&mut balances);

    let equally_opposed_reimbursement_suggestions: Vec<ReimbursementSuggestion> =
        resolve_equally_opposed_balances(
            &mut unsolved_positive_balances_by_user,
            &mut unsolved_negative_balances_by_user,
        );
    result = [result, equally_opposed_reimbursement_suggestions].concat();

    let remaining_balances = resolve_remaining_balances(
        &mut unsolved_positive_balances_by_user,
        &mut unsolved_negative_balances_by_user,
    );
    result = [result, remaining_balances].concat();

    // TODO handle small differences / computation errors

    result
}

//
// // Est-ce que ce ne serait pas mieux de faire la somme de tous les payments par userId (permet d'éliminer ceux qui sont à l'équilibre)
// // Ensuite on calcule qui doit combien a qui
//
// // Examples
// // P 10, 30, 50 - 10, 30, 50
// // N 30, 60     - 30, 30, 30
//
// // Je pense que l'idéal serait de :
// // - trier les deux listes par ordre "extrême" :check:
// // - Pour chaque valeur, maximale (neg ou pos), retrancher n opposés
// // - Si il reste une valeur, regarder si il y a un équivalent négatif après chaque itération sinon retirer le reste
fn resolve_remaining_balances(
    unsolved_positive_balances_by_user: &mut HashMap<i32, UserBalanceComputation>,
    unsolved_negative_balances_by_user: &mut HashMap<i32, UserBalanceComputation>,
) -> Vec<ReimbursementSuggestion> {
    let mut result: Vec<ReimbursementSuggestion> = Vec::new();

    while !unsolved_positive_balances_by_user.is_empty()
        && !unsolved_negative_balances_by_user.is_empty()
    {
        let previous_lengths =
            unsolved_positive_balances_by_user.len() + unsolved_negative_balances_by_user.len();

        // Create sorted versions for processing
        let sorted_unsolved_positive_balances_by_user: HashMap<i32, UserBalanceComputation> =
            unsolved_positive_balances_by_user
                .iter()
                .sorted_by(|(_u1, b1), (_u2, b2)| {
                    f64::total_cmp(&b2.remaining_amount.abs(), &b1.remaining_amount.abs())
                })
                .map(|(a, b)| (*a, b.clone()))
                .collect();

        let sorted_unsolved_negative_balances_by_user: HashMap<i32, UserBalanceComputation> =
            unsolved_negative_balances_by_user
                .iter()
                .sorted_by(|(_u1, b1), (_u2, b2)| {
                    f64::total_cmp(&b2.remaining_amount.abs(), &b1.remaining_amount.abs())
                })
                .map(|(a, b)| (*a, b.clone()))
                .collect();

        // Find max & min among balances
        let MaxBalance { is_debt, max_balance, opposite_balances: min_balances } = get_max_balance(
            sorted_unsolved_positive_balances_by_user,
            sorted_unsolved_negative_balances_by_user,
        );
        let (opposite_balances_used, remainder) =
            solve_max_balance(max_balance.clone(), min_balances);

        opposite_balances_used.iter().for_each(|(user_id, balance)| {
            if is_debt {
                result.push(ReimbursementSuggestion {
                    amount: round_currency(balance.remaining_amount.abs()),
                    user_id_debtor: max_balance.0,
                    user_id_payer: *user_id,
                });

                unsolved_positive_balances_by_user.remove(user_id);
            } else {
                result.push(ReimbursementSuggestion {
                    amount: round_currency(balance.remaining_amount.abs()),
                    user_id_debtor: *user_id,
                    user_id_payer: max_balance.0,
                });

                unsolved_negative_balances_by_user.remove(user_id);
            }
        });

        let has_remainder = remainder.0 != 0;
        if has_remainder {
            // Calculate how much of the remainder balance was used
            let original_remainder_balance = if is_debt {
                unsolved_positive_balances_by_user.get(&remainder.0).unwrap().remaining_amount.abs()
            } else {
                unsolved_negative_balances_by_user.get(&remainder.0).unwrap().remaining_amount.abs()
            };
            let amount_used = original_remainder_balance - remainder.1.remaining_amount.abs();

            // Create suggestion for the partial payment
            if is_debt {
                result.push(ReimbursementSuggestion {
                    amount: round_currency(amount_used),
                    user_id_debtor: max_balance.0,
                    user_id_payer: remainder.0,
                });
            } else {
                result.push(ReimbursementSuggestion {
                    amount: round_currency(amount_used),
                    user_id_debtor: remainder.0,
                    user_id_payer: max_balance.0,
                });
            }

            // Update the remainder balance
            if is_debt {
                let value = unsolved_positive_balances_by_user.get_mut(&remainder.0);
                value.unwrap().remaining_amount = remainder.1.remaining_amount;
            } else {
                let value = unsolved_negative_balances_by_user.get_mut(&remainder.0);
                value.unwrap().remaining_amount = remainder.1.remaining_amount;
            }
            // Remove the max_balance user since it's fully compensated
            if is_debt {
                unsolved_negative_balances_by_user.remove(&max_balance.0);
            } else {
                unsolved_positive_balances_by_user.remove(&max_balance.0);
            }
        } else {
            // If no remainder, remove the max_balance user
            if is_debt {
                unsolved_negative_balances_by_user.remove(&max_balance.0);
            } else {
                unsolved_positive_balances_by_user.remove(&max_balance.0);
            }
        }

        let current_lengths =
            unsolved_positive_balances_by_user.len() + unsolved_negative_balances_by_user.len();

        if current_lengths == 0 {
            break;
        }

        if previous_lengths == current_lengths {
            info!("Unable to solve all balances");
            break;
        }
    }

    result
}

// returns the list of min_balances that are fully compensated and optionally the remainder
fn solve_max_balance(
    max_balance: (i32, UserBalanceComputation),
    min_balances: HashMap<i32, UserBalanceComputation>,
) -> (HashMap<i32, UserBalanceComputation>, (i32, UserBalanceComputation)) {
    let mut fully_compensated_balances = HashMap::new();
    let mut remainder: (i32, UserBalanceComputation) =
        (0, UserBalanceComputation { remaining_amount: 0.0, amount: 0.0 });

    let mut max_balance_amount = max_balance.1.remaining_amount.abs();
    for min_balance in min_balances {
        if max_balance_amount.total_cmp(&0.0) != Ordering::Greater {
            break;
        }

        let min_balance_amount = min_balance.1.remaining_amount.abs();

        if max_balance_amount.total_cmp(&min_balance_amount) == Ordering::Greater {
            fully_compensated_balances.insert(min_balance.0, min_balance.1.clone());
            max_balance_amount = max_balance_amount.sub(min_balance_amount);
            continue;
        }

        if max_balance_amount.total_cmp(&min_balance_amount) == Ordering::Equal {
            fully_compensated_balances.insert(min_balance.0, min_balance.1.clone());
            max_balance_amount = 0.0;
            continue;
        }

        if max_balance_amount.total_cmp(&min_balance_amount) == Ordering::Less {
            // The min_balance partially compensates the max_balance
            // remaining_amount keeps the same sign but reduces in absolute value
            let new_remaining_amount = if min_balance.1.remaining_amount < 0.0 {
                min_balance.1.remaining_amount + max_balance_amount
            } else {
                min_balance.1.remaining_amount - max_balance_amount
            };

            remainder = (
                min_balance.0,
                UserBalanceComputation {
                    remaining_amount: round_currency(new_remaining_amount),
                    amount: round_currency(min_balance.1.amount),
                },
            );
            max_balance_amount = 0.0;
            break;
        }
    }

    (fully_compensated_balances, remainder)
}

// Returns the max balance to solve only and the Hashmap of the min balances
fn get_max_balance(
    sorted_unsolved_positive_balances_by_user: HashMap<i32, UserBalanceComputation>,
    sorted_unsolved_negative_balances_by_user: HashMap<i32, UserBalanceComputation>,
) -> MaxBalance {
    let positive_max: Option<(i32, UserBalanceComputation)> =
        sorted_unsolved_positive_balances_by_user
            .iter()
            .max_by(|(_u1, b1), (_u2, b2)| {
                f64::total_cmp(&b2.remaining_amount.abs(), &b1.remaining_amount.abs())
            })
            .map(|(a, b)| (*a, b.clone()));

    let negative_max: Option<(i32, UserBalanceComputation)> =
        sorted_unsolved_negative_balances_by_user
            .iter()
            .max_by(|(_u1, b1), (_u2, b2)| {
                f64::total_cmp(&b2.remaining_amount.abs(), &b1.remaining_amount.abs())
            })
            .map(|(a, b)| (*a, b.clone()));

    if positive_max.is_none() || negative_max.is_none() {
        // Log error here ?
        return MaxBalance {
            is_debt: false,
            max_balance: (0, UserBalanceComputation { remaining_amount: 0.0, amount: 0.0 }),
            opposite_balances: HashMap::new(),
        };
    }

    let positive_max_value: (i32, UserBalanceComputation) = positive_max.unwrap();
    let negative_max_value: (i32, UserBalanceComputation) = negative_max.unwrap();

    if positive_max_value.1.remaining_amount.abs() >= negative_max_value.1.remaining_amount.abs() {
        MaxBalance {
            is_debt: false,
            max_balance: positive_max_value,
            opposite_balances: sorted_unsolved_negative_balances_by_user,
        }
    } else {
        MaxBalance {
            is_debt: true,
            max_balance: negative_max_value,
            opposite_balances: sorted_unsolved_positive_balances_by_user,
        }
    }
}

fn resolve_equally_opposed_balances(
    unsolved_positive_balances_by_user: &mut HashMap<i32, UserBalanceComputation>,
    unsolved_negative_balances_by_user: &mut HashMap<i32, UserBalanceComputation>,
) -> Vec<ReimbursementSuggestion> {
    let mut resolved_users: Vec<(i32, i32)> = Vec::new();
    let mut result: Vec<ReimbursementSuggestion> = Vec::new();

    for (positive_user_id, balance_amount) in &mut *unsolved_positive_balances_by_user {
        let matching_equal_negative_balance = unsolved_negative_balances_by_user
            .iter_mut()
            .filter(|(debtor_id, _)| {
                !resolved_users
                    .iter()
                    .any(|(_, resolved_debtor_id)| resolved_debtor_id == *debtor_id)
            })
            .find(|(_, b_amount)| {
                b_amount.remaining_amount.abs().total_cmp(&balance_amount.remaining_amount)
                    == Ordering::Equal
            });

        if let Some((resolved_user_id_debtor, negative_user_balance)) =
            matching_equal_negative_balance
        {
            result.push(ReimbursementSuggestion {
                amount: round_currency(balance_amount.remaining_amount),
                user_id_debtor: *resolved_user_id_debtor,
                user_id_payer: *positive_user_id,
            });

            balance_amount.remaining_amount = 0.0;
            negative_user_balance.remaining_amount = 0.0;

            resolved_users.push((*positive_user_id, *resolved_user_id_debtor));
        }
    }

    resolved_users.iter().for_each(|(payer_id, debtor_id)| {
        unsolved_positive_balances_by_user.remove(payer_id);
        unsolved_negative_balances_by_user.remove(debtor_id);
    });

    return result;
}

fn get_unresolved_balances_by_user(
    balances: &mut Vec<UserBalance>,
) -> (HashMap<i32, UserBalanceComputation>, HashMap<i32, UserBalanceComputation>) {
    let mut unsolved_positive_balances_by_user: HashMap<i32, UserBalanceComputation> =
        Default::default();
    let mut unsolved_negative_balances_by_user: HashMap<i32, UserBalanceComputation> =
        Default::default();

    for user_balance in balances.iter() {
        if user_balance.amount.is_sign_positive() {
            unsolved_positive_balances_by_user.insert(
                user_balance.user_id,
                UserBalanceComputation {
                    remaining_amount: round_currency(user_balance.amount),
                    amount: round_currency(user_balance.amount),
                },
            );
        } else {
            unsolved_negative_balances_by_user.insert(
                user_balance.user_id,
                UserBalanceComputation {
                    remaining_amount: round_currency(user_balance.amount),
                    amount: round_currency(user_balance.amount),
                },
            );
        }
    }
    (unsolved_positive_balances_by_user, unsolved_negative_balances_by_user)
}

struct MaxBalance {
    is_debt: bool,
    max_balance: (i32, UserBalanceComputation),
    opposite_balances: HashMap<i32, UserBalanceComputation>,
}

#[cfg(test)]
mod tests {
    use shared::UserBalance;

    use super::*;

    #[test]
    fn test_empty_balances() {
        let balances = vec![];
        let suggestions = get_reimbursement_suggestions(balances);
        assert!(suggestions.is_empty());
    }

    #[test]
    fn test_single_equally_opposed_balance() {
        let balances = vec![
            UserBalance { amount: 50.0, user_id: 1 },
            UserBalance { amount: -50.0, user_id: 2 },
        ];

        let suggestions = get_reimbursement_suggestions(balances);
        assert_eq!(suggestions.len(), 1);
        assert_eq!(suggestions[0].amount, 50.0);
        assert_eq!(suggestions[0].user_id_payer, 1);
        assert_eq!(suggestions[0].user_id_debtor, 2);
    }

    #[test]
    fn test_multiple_equally_opposed_balances() {
        let balances = vec![
            UserBalance { amount: 50.0, user_id: 1 },
            UserBalance { amount: -50.0, user_id: 2 },
            UserBalance { amount: 30.0, user_id: 3 },
            UserBalance { amount: -30.0, user_id: 4 },
        ];

        let suggestions = get_reimbursement_suggestions(balances);
        assert_eq!(suggestions.len(), 2);

        // Verify total reimbursement amounts
        let total_reimbursement: f64 = suggestions.iter().map(|s| s.amount).sum();
        assert_eq!(total_reimbursement, 80.0);
    }

    #[test]
    fn test_one_person_owes_multiple() {
        let balances = vec![
            UserBalance { amount: 30.0, user_id: 1 },
            UserBalance { amount: 20.0, user_id: 2 },
            UserBalance { amount: -50.0, user_id: 3 },
        ];

        let suggestions = get_reimbursement_suggestions(balances);
        assert_eq!(suggestions.len(), 2);

        // Charlie should pay both Alice and Bob
        let charlie_payments: Vec<_> =
            suggestions.iter().filter(|s| s.user_id_debtor == 3).collect();
        assert_eq!(charlie_payments.len(), 2);

        let total_charlie_pays: f64 = charlie_payments.iter().map(|s| s.amount).sum();
        assert_eq!(total_charlie_pays, 50.0);
    }

    #[test]
    fn test_multiple_people_owe_one() {
        let balances = vec![
            UserBalance { amount: 50.0, user_id: 1 },
            UserBalance { amount: -30.0, user_id: 2 },
            UserBalance { amount: -20.0, user_id: 3 },
        ];

        let suggestions = get_reimbursement_suggestions(balances);
        assert_eq!(suggestions.len(), 2);

        // Both Bob and Charlie should pay Alice
        let alice_receives: Vec<_> = suggestions.iter().filter(|s| s.user_id_payer == 1).collect();
        assert_eq!(alice_receives.len(), 2);

        let total_alice_receives: f64 = alice_receives.iter().map(|s| s.amount).sum();
        assert_eq!(total_alice_receives, 50.0);
    }

    #[test]
    fn test_complex_scenario() {
        let balances = vec![
            UserBalance { amount: 100.0, user_id: 1 },
            UserBalance { amount: 50.0, user_id: 2 },
            UserBalance { amount: -60.0, user_id: 3 },
            UserBalance { amount: -90.0, user_id: 4 },
        ];

        let suggestions = get_reimbursement_suggestions(balances);

        // Verify all debts are resolved by checking total amount
        let total_payments: f64 = suggestions.iter().map(|s| s.amount).sum();
        // Allow small floating point errors
        assert!((total_payments - 150.0).abs() < 0.01, "Expected 150.0 but got {}", total_payments);

        // Verify Alice and Bob only receive (are payers) and amounts are positive
        for suggestion in &suggestions {
            assert!(suggestion.amount > 0.0);
            if suggestion.user_id_payer == 1 || suggestion.user_id_payer == 2 {
                // Alice or Bob receiving
                assert!(suggestion.user_id_debtor == 3 || suggestion.user_id_debtor == 4);
            }
        }

        // Verify Charlie and David only pay (are debtors)
        for suggestion in &suggestions {
            if suggestion.user_id_debtor == 3 || suggestion.user_id_debtor == 4 {
                // Charlie or David paying
                assert!(suggestion.user_id_payer == 1 || suggestion.user_id_payer == 2);
            }
        }
    }

    #[test]
    fn test_partial_compensation() {
        let balances = vec![
            UserBalance { amount: 100.0, user_id: 1 },
            UserBalance { amount: -25.0, user_id: 2 },
            UserBalance { amount: -25.0, user_id: 3 },
            UserBalance { amount: -50.0, user_id: 4 },
        ];

        let suggestions = get_reimbursement_suggestions(balances);
        assert_eq!(suggestions.len(), 3);

        // All three should pay Alice
        let alice_receives: Vec<_> = suggestions.iter().filter(|s| s.user_id_payer == 1).collect();
        assert_eq!(alice_receives.len(), 3);

        let total_alice_receives: f64 = alice_receives.iter().map(|s| s.amount).sum();
        assert_eq!(total_alice_receives, 100.0);
    }

    #[test]
    fn test_balanced_group() {
        let balances =
            vec![UserBalance { amount: 0.0, user_id: 1 }, UserBalance { amount: 0.0, user_id: 2 }];

        let suggestions = get_reimbursement_suggestions(balances);
        // No suggestions should be made when everyone is balanced
        assert!(suggestions.is_empty() || suggestions.iter().all(|s| s.amount == 0.0));
    }

    #[test]
    fn test_three_way_split() {
        let balances = vec![
            UserBalance { amount: 60.0, user_id: 1 },
            UserBalance { amount: -30.0, user_id: 2 },
            UserBalance { amount: -30.0, user_id: 3 },
        ];

        let suggestions = get_reimbursement_suggestions(balances);

        // Verify both Bob and Charlie pay Alice
        let total_paid: f64 = suggestions.iter().map(|s| s.amount).sum();
        assert_eq!(total_paid, 60.0);

        // All payments should go to Alice
        assert!(suggestions.iter().all(|s| s.user_id_payer == 1));
    }

    #[test]
    fn test_small_amounts() {
        let balances = vec![
            UserBalance { amount: 0.50, user_id: 1 },
            UserBalance { amount: -0.50, user_id: 2 },
        ];

        let suggestions = get_reimbursement_suggestions(balances);
        assert_eq!(suggestions.len(), 1);
        assert_eq!(suggestions[0].amount, 0.50);
    }

    #[test]
    fn test_large_group() {
        let balances = vec![
            UserBalance { amount: 100.0, user_id: 1 },
            UserBalance { amount: 50.0, user_id: 2 },
            UserBalance { amount: 25.0, user_id: 3 },
            UserBalance { amount: -40.0, user_id: 4 },
            UserBalance { amount: -60.0, user_id: 5 },
            UserBalance { amount: -75.0, user_id: 6 },
        ];

        let suggestions = get_reimbursement_suggestions(balances);

        // Verify total balance equals sum of positive balances (allow small floating point errors)
        let total_payments: f64 = suggestions.iter().map(|s| s.amount).sum();
        assert!((total_payments - 175.0).abs() < 0.01, "Expected 175.0 but got {}", total_payments);

        // Each suggestion should have positive amount
        for suggestion in &suggestions {
            assert!(
                suggestion.amount > 0.0,
                "Amount should be positive but got {}",
                suggestion.amount
            );
        }

        // Verify creditors only receive (users 1, 2, 3)
        let creditors = [1, 2, 3];
        let debtors = [4, 5, 6];

        for suggestion in &suggestions {
            assert!(
                creditors.contains(&suggestion.user_id_payer),
                "Payer {} should be a creditor",
                suggestion.user_id_payer
            );
            assert!(
                debtors.contains(&suggestion.user_id_debtor),
                "Debtor {} should be a debtor",
                suggestion.user_id_debtor
            );
        }
    }
}
