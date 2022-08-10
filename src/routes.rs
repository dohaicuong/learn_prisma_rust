use axum::{
    Extension,
    Json,
    Router,
    routing::get,
    response::{IntoResponse, Response}, http::StatusCode
};

use crate::prisma::{PrismaClient, post};
use prisma_client_rust::{Error, error_is_type, prisma_errors::query_engine::UniqueKeyViolation};

type Database = Extension<std::sync::Arc<PrismaClient>>;

enum AppError {
    PrismaError(Error),
    NotFound,
}
impl From<Error> for AppError {
    fn from(inner: Error) -> Self {
        AppError::PrismaError(inner)
    }
}
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status = match self {
            AppError::PrismaError(Error::Execute(prisma_err)) => {
                if error_is_type::<UniqueKeyViolation>(&prisma_err) {
                    StatusCode::CONFLICT
                } else {
                    StatusCode::BAD_REQUEST
                }
            },
            AppError::NotFound => StatusCode::NOT_FOUND,
            _ => StatusCode::INTERNAL_SERVER_ERROR
        };

        status.into_response()
    }
}

type AppResult<T> = Result<T, AppError>;

pub fn create_routes() -> Router {
    Router::new()
        .route("/posts", get(handle_posts_get))
}

async fn handle_posts_get(db: Database) -> AppResult<Json<Vec<post::Data>>> {
    let posts = db
        .post()
        .find_many(vec![])
        .exec()
        .await?;

    Ok(Json::from(posts))
}