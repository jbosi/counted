use std::collections::HashMap;
use std::ops::{AddAssign, RemAssign, SubAssign};
use actix_web::web;
use actix_web::web::Query;
use itertools::Itertools;
use uuid::Uuid;

use crate::DbPool;
use crate::expenses::application::expense_application::get_expenses_app;
use crate::expenses::domain::expense_model::Expense;
use crate::models::user_project_model::UserProjects;
use crate::payments::application::payment_application::get_payments_app;
use crate::payments::domain::payment_model::{ExpenseDto, Payment};
use crate::payments::domain::payment_query_params::PaymentQueryParams;
use crate::payments::repository::payment_repository::get_payments;
use crate::projects::domain::balance_model::{Balance, UserBalance};
use crate::projects::domain::project_model::{CreatableProject, NewProject, Project, ProjectDto};
use crate::projects::repository::project_repository::{create_project, get_all_projects, get_project, get_projects_and_user_projects_for_user};
use crate::query_strings::expense_query_string::ExpenseQueryParams;
use crate::query_strings::project_query_string::ProjectQueryParams;

pub async fn get_projects_app(pool: web::Data<DbPool>, params: Query<ProjectQueryParams>) -> Vec<ProjectDto> {
    let all_projects: Vec<Project> = get_all_projects(pool.clone()).await;
    let mut projects_dto: Vec<ProjectDto> = Vec::new();

    let projects_and_user_projects_for_user: Vec<(UserProjects, Project)> = get_projects_and_user_projects_for_user(pool, params).await;

    let projects_group = projects_and_user_projects_for_user
        // .grouped_by(&users)
        .into_iter()
        .group_by(|(_up, p)| p.id);
    // .zip(projects)

    for (p_id, user_projects) in &projects_group {
        let current_project = all_projects.iter().find(|p| p.id == p_id).unwrap().clone();
        let users: Vec<i32> = user_projects
            .map(|(up, _p)| up.user_id)
            .collect();

        projects_dto.push(ProjectDto {
            id: current_project.id,
            created_at: current_project.created_at,
            currency: current_project.currency,
            name: current_project.name,
            users
        })
    }

    return projects_dto;
}

pub async fn get_project_app(pool: web::Data<DbPool>, project_id: Uuid) -> Project {
    return get_project(pool.clone(), project_id).await;
}
pub async fn create_project_app(pool: web::Data<DbPool>, creatable_project: web::Json<CreatableProject>) -> Project {
    let new_project = NewProject {
        name: creatable_project.name.to_string(),
        currency: "Euro".to_string(),
        // total_expenses: 0.0
    };

    return create_project(pool, creatable_project, new_project).await;
}

pub async fn get_balance_app(pool: web::Data<DbPool>, project_id: Uuid) -> Balance {
    let expense_params: ExpenseQueryParams = ExpenseQueryParams {
        project_id: Some(project_id),
        user_id: None,
    };

    let expenses: Vec<ExpenseDto> = get_expenses_app(pool.clone(), Query(expense_params)).await;
    let expense_ids: Vec<i32> = expenses.iter().map(|expense| expense.id).collect();

    let mut all_payments: Vec<Payment> = vec![];
    
    for expense_id in expense_ids {
        let payment_params: PaymentQueryParams = PaymentQueryParams {
            expense_id: Some(expense_id.clone()),
            user_id: None,
        };
        let mut payments = get_payments_app(pool.clone(), Query(payment_params)).await;
        all_payments.append(&mut payments);
    }

    return forgeBalanceFromPayments(all_payments);
}

fn forgeBalanceFromPayments(payments: Vec<Payment>) -> Balance {
    let mut balance: Balance = Balance {
        balances: vec![],
        currency: "".to_string(),
        total_expenses: 0.0,
    };

    let mut balances: HashMap<i32, f64> = Default::default();

    for payment in payments {
        // if let Some(matching_balance) = balances.get_mut(&payment.user_id) {
        //     matching_balance.add_assign(payment.amount);
        // } else {
        //     balances.insert(payment.user_id, payment.amount);
        // }
        let mut default_insert: f64 = 0.0;
        if (payment.is_debt) {
            default_insert.sub_assign(payment.amount);
        } else {
            default_insert.add_assign(payment.amount);
        }
        balances.entry(payment.user_id)
            .and_modify(|p| {
                if payment.is_debt {
                    return p.sub_assign(payment.amount);
                } else {
                    return p.add_assign(payment.amount);
                }
            })
            .or_insert(default_insert);
    }


    for (user_id, amount) in balances {
        balance.balances.push(UserBalance {
            user_id: user_id,
            amount: amount,
        });

        balance.total_expenses = 0.0;
        balance.currency = "€".to_string()
    }

    return balance;
}