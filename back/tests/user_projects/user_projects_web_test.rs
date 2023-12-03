#[cfg(test)]
pub mod user_projects_web_test {
    use super::*;
    use diesel::r2d2::{self, ConnectionManager};
    use crate::get_user_projects;
    use actix_web::test::TestRequest;
    use actix_web::{http::header::ContentType, test, App};

    use super::*;

    #[actix_web::test]
    async fn test_index_get() {
        let mut app = test::init_service(App::new().service(get_user_projects)).await;
        let req = TestRequest::get()
            .uri("/user-projects")
            .to_request();
        let resp = test::call_service(&mut app, req).await;
        assert!(resp.status().is_success());
    }
}