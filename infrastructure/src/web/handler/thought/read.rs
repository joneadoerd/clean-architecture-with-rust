use crate::web::handler::{reply_error, Result};
use adapter::{
    controller::thought::find_by_id::Controller, model::app::thought::Id,
    presenter::http_json_api::Presenter,
};
use application::gateway::repository::thought::Repo;
use std::sync::Arc;
use warp::{reply, Reply};

pub type Request = String;

pub async fn handle<R>(req: Request, repo: Arc<R>) -> Result<impl Reply>
where
    R: Repo<Id = Id> + 'static,
{
    let presenter = Presenter::default();
    let controller = Controller::new(repo, presenter);
    let res = controller.find_thought(&req);
    match res {
        Ok(res) => Ok(reply::with_status(reply::json(&res.data), res.status)),
        Err(err) => Ok(reply_error(err)),
    }
}

#[cfg(test)]
mod tests {
    use super::handle;
    use crate::web::tests::{add_thought_to_db, blank_db, corrupt_db, response_json_body};
    use adapter::model::view::json::Error;
    use serde_json::Value;
    use warp::{http::StatusCode, Reply};

    #[tokio::test]
    async fn read() {
        let db = blank_db();
        add_thought_to_db(&db, "foo");
        add_thought_to_db(&db, "bar");

        let req = "2".to_string();
        let res = handle(req, db.clone()).await.unwrap().into_response();

        assert_eq!(res.status(), StatusCode::OK);

        let body: Value = response_json_body(res).await.unwrap();
        let thought = body.as_object().unwrap();
        let title = thought.get("title").unwrap().as_str().unwrap();

        assert_eq!(title, "bar");
    }

    #[tokio::test]
    async fn read_non_existent() {
        let db = blank_db();

        let req = "5".to_string();
        let res = handle(req, db.clone()).await.unwrap().into_response();

        assert_eq!(res.status(), StatusCode::NOT_FOUND);

        let err: Error = response_json_body(res).await.unwrap();

        assert_eq!(err.msg.unwrap(), "Could not find thought");
        assert_eq!(err.status, StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn read_invalid_id() {
        let db = blank_db();

        let req = "invalid-id".to_string();
        let res = handle(req, db.clone()).await.unwrap().into_response();

        assert_eq!(res.status(), StatusCode::BAD_REQUEST);

        let err: Error = response_json_body(res).await.unwrap();
        assert_eq!(err.msg.unwrap(), "Unable to parse thought ID");
        assert_eq!(err.status, StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn read_with_corrupt_db() {
        let db = corrupt_db();

        let req = "1".to_string();
        let res = handle(req, db.clone()).await.unwrap().into_response();

        assert_eq!(res.status(), StatusCode::INTERNAL_SERVER_ERROR);

        let err: Error = response_json_body(res).await.unwrap();

        assert_eq!(err.msg, None);
        assert_eq!(err.status, StatusCode::INTERNAL_SERVER_ERROR);
    }
}
