use axum::{
    Extension,
    Json,
    Router,
    routing::{get, post, put, delete},
    response::{IntoResponse, Response}, http::StatusCode, extract::Path
};
use serde::Deserialize;

use prisma_client_rust::{Error, error_is_type, prisma_errors::query_engine::UniqueKeyViolation};
use crate::prisma::{PrismaClient, post};

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
type AppJsonResult<T> = AppResult<Json<T>>;

pub fn create_routes() -> Router {
    Router::new()
        .route("/post", post(handle_post_create))
        .route("/post", get(handle_posts_get))
        .route("/post/:post_id", get(handle_post_get))
        .route("/post/:post_id", put(handle_post_update))
        .route("/post/:post_id", delete(handle_post_delete))
}

#[derive(Deserialize)]
struct PostCreateInput {
    title: String,
    content: Option<String>,
}
async fn handle_post_create(
    db: Database,
    Json(input): Json<PostCreateInput>,
) -> AppJsonResult<post::Data> {
    let post = db
        .post()
        .create(
            post::title::set(input.title),
            vec![
                post::content::set(input.content),
            ]
        )
        .exec()
        .await?;

    Ok(Json::from(post))
}

async fn handle_posts_get(db: Database) -> AppJsonResult<Vec<post::Data>> {
    let posts = db
        .post()
        .find_many(vec![])
        .exec()
        .await?;

    Ok(Json::from(posts))
}

async fn handle_post_get(
    db: Database,
    Path(post_id): Path<String>,
) -> AppJsonResult<post::Data> {
    let post_result = db
        .post()
        .find_unique(post::id::equals(post_id))
        .exec()
        .await?;

    match post_result {
        Some(post) => Ok(Json::from(post)),
        _ => Err(AppError::NotFound)
    }
}

#[derive(Deserialize)]
struct PostUpdateInput {
    // TODO: found a convenient way to optional update a required field 
    // title: Option<String>,
    content: Option<String>,
}
async fn handle_post_update(
    db: Database,
    Path(post_id): Path<String>,
    Json(input): Json<PostUpdateInput>,
) -> AppJsonResult<post::Data> {
    let updated_post_result = db
        .post()
        .find_unique(post::id::equals(post_id))
        .update(vec![
            // post::title::set(input.title),
            post::content::set(input.content),
        ])
        .exec()
        .await?;

    match updated_post_result {
        Some(updated_post) => Ok(Json::from(updated_post)),
        None => Err(AppError::NotFound)
    }
}

async fn handle_post_delete(
    db: Database,
    Path(post_id): Path<String>,
) -> AppJsonResult<post::Data> {
    let deleted_post_result = db
        .post()
        .find_unique(post::id::equals(post_id))
        .delete()
        .exec()
        .await?;

    match deleted_post_result {
        Some(deleted_post) => Ok(Json::from(deleted_post)),
        None => Err(AppError::NotFound)
    }
}
