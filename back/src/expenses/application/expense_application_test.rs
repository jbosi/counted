#[cfg(test)]
mod expense_application_test {
    use actix_web::web;
    use r2d2::Pool;
    use diesel::r2d2::ConnectionManager;
    use diesel::PgConnection;
    use chrono::{NaiveDate, NaiveDateTime};
    use uuid::Uuid;
    use crate::DbPool;
    use crate::expenses::application::expense_application::get_expenses_app;
    use crate::expenses::domain::expense_model::ExpenseType;
    use crate::query_strings::expense_query_string::ExpenseQueryParams;

    #[actix_rt::test]
    async fn should_return_expense_dto_when_given_valid_inputs() {
        // Arrange
        let pool = create_test_pool();
        let params = web::Query(ExpenseQueryParams {
            project_id: Some(Uuid::new_v4()),
            user_id: Some(1),
        });

        // Act
        let result = get_expenses_app(pool.clone(), params).await;

        // Assert
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].id, 1);
        assert_eq!(result[0].author_id, 1);
        assert_eq!(result[0].project_id, params.project_id.unwrap());
        assert_eq!(result[0].date, NaiveDate::from_ymd(2022, 1, 1));
        assert_eq!(result[0].amount, 100.0);
        assert_eq!(result[0].description, Some("Expense description".to_string()));
        assert_eq!(result[0].name, "Expense name");
        assert_eq!(result[0].expense_type, ExpenseType::Expense);
        assert_eq!(result[0].payments.len(), 1);
        assert_eq!(result[0].payments[0].id, 1);
        assert_eq!(result[0].payments[0].expense_id, 1);
        assert_eq!(result[0].payments[0].user_id, 1);
        assert_eq!(result[0].payments[0].is_debt, false);
        assert_eq!(result[0].payments[0].amount, 50.0);
        assert_eq!(result[0].payments[0].created_at, NaiveDateTime::from_timestamp(1640995200, 0));
    }

    fn create_test_pool() -> web::Data<DbPool> {
        let database_url = "postgres://username:password@localhost/test_db";
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        let pool = Pool::builder().build(manager).unwrap();
        web::Data::new(pool)
    }
}