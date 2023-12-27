#[cfg(test)]
pub mod user_projects_web_test {
    use actix_web::{App, test, web};
    use test::{call_service, init_service, TestRequest};
    use crate::user_project_web::get_user_projects;
    use super::*;
    #[actix_web::test]
    async fn test_index_get() {
        let app = init_service(App::new()
            .service(get_user_projects)
        ).await;

        let req = TestRequest::get()
            .uri("/user-projects")
            .to_request();

        let resp = call_service(&app, req).await;

        assert!(resp.status().is_client_error());
    }
}