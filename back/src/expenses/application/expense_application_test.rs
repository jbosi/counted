// use std::fmt::Error;
// use diesel::pg::PgConnection;
// use diesel::r2d2::ConnectionManager;
// use mockall::*;
// use r2d2::ManageConnection;
// use r2d2::Pool;
//
// use crate::expenses::domain::expense_model::Expense;
// use crate::payments::domain::payment_model::{ExpenseDto, Payment};
// use crate::payments::domain::payment_query_params::PaymentQueryParams;
// use crate::query_strings::expense_query_string::ExpenseQueryParams;
//
// #[cfg(test)]
// mod expense_application_test {
//     #[actix_rt::test]
//     async fn should_return_expense_dto() {
//         // Arrange
//         let mut mock_pool = MockPool::new();
//         let mut mock_conn = MockConnection::new();
//         let params = ExpenseQueryParams {
//             project_id: Some(Uuid::new_v4()),
//             user_id: Some(1),
//         };
//         let expense = Expense {
//             id: 1,
//             author_id: 1,
//             project_id: Uuid::new_v4(),
//             date: NaiveDate::from_ymd(2022, 1, 1),
//             amount: 100.0,
//             description: Some("Expense description".to_string()),
//             name: "Expense name".to_string(),
//             expense_type: ExpenseType::Other,
//         };
//         let expenses = vec![expense.clone()];
//         let payment = Payment {
//             id: 1,
//             expense_id: 1,
//             user_id: 1,
//             is_debt: false,
//             amount: 50.0,
//             created_at: NaiveDateTime::from_timestamp(1640995200, 0),
//         };
//         let payments = vec![payment.clone()];
//         let expense_dto = ExpenseDto {
//             id: 1,
//             author_id: 1,
//             project_id: Uuid::new_v4(),
//             date: NaiveDate::from_ymd(2022, 1, 1),
//             amount: 100.0,
//             description: Some("Expense description".to_string()),
//             name: "Expense name".to_string(),
//             expense_type: ExpenseType::Other,
//             payments: vec![payment.clone()],
//         };
//
//         mock_pool.expect_get().returning(|| Ok(mock_conn));
//         mock_conn.expect_get_expenses().with(eq(params.clone())).returning(move |_| Ok(expenses.clone()));
//         mock_conn.expect_get_payments().with(eq(PaymentQueryParams { user_id: Some(1), expense_id: None })).returning(move |_| Ok(payments.clone()));
//         mock_conn.expect_to_expense_dto().with(eq(&expense), eq(&payments)).returning(move |_, _| expense_dto.clone());
//
//         let pool_data = web::Data::new(mock_pool);
//         let mut app = test::init_service(
//             App::new()
//                 .app_data(pool_data.clone())
//                 .route("/expenses", web::get().to(get_expenses_app))
//         )
//             .await;
//
//         // Act
//         let req = test::TestRequest::get()
//             .uri("/expenses?project_id=123&user_id=1")
//             .to_request();
//         let resp = test::call_service(&mut app, req).await;
//
//         // Assert
//         assert_eq!(resp.status(), StatusCode::OK);
//         let body = test::read_body(resp).await;
//         let expected_body = json!([expense_dto]);
//         assert_eq!(body, expected_body.to_string().as_bytes());
//     }
// }
//
// // Mocks
// #[automock]
// pub trait MockPool: Pool<ConnectionManager<PgConnection>> {}
// #[automock]
// pub trait MockConnection: diesel::Connection<Backend = diesel::pg::Pg> {
//     fn get_expenses(&self, params: ExpenseQueryParams) -> Result<Vec<Expense>, Error>;
//     fn get_payments(&self, params: PaymentQueryParams) -> Result<Vec<Payment>, Error>;
//     fn to_expense_dto(&self, expense: &Expense, payments: &Vec<Payment>) -> ExpenseDto;
// }
//
// impl<T> MockPool for T where T: Pool<ConnectionManager<PgConnection>> {}
// impl<T> MockConnection for T where T: diesel::Connection<Backend = diesel::pg::Pg> {
//     fn get_expenses(&self, params: ExpenseQueryParams) -> Result<Vec<Expense>, Error> {
//         unimplemented!()
//     }
//     fn get_payments(&self, params: PaymentQueryParams) -> Result<Vec<Payment>, Error> {
//         unimplemented!()
//     }
//     fn to_expense_dto(&self, expense: &Expense, payments: &Vec<Payment>) -> ExpenseDto {
//         unimplemented!()
//     }
// }