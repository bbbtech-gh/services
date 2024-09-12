use axum::response::IntoResponse;
use axum::http::StatusCode;
use axum::extract::Path;

use axum::{Extension, Json};
use sqlx::PgPool;

use crate::errors::CustomError;
use crate::models::*;


pub async fn all_tokens(Extension(pool): Extension<PgPool>) -> impl IntoResponse {
    let sql = "SELECT * FROM tokens ".to_string();

    let token = sqlx::query_as::<_, Token>(&sql).fetch_all(&pool).await.unwrap();

    (StatusCode::OK, Json(token))
}

pub async fn get_token_by_id(
    Path(id):Path<i32>, 
    Extension(pool): Extension<PgPool>
) -> Result <Json<Token>, CustomError> {

    let sql = "SELECT * FROM tokens where id=$1".to_string();

    let task: Token = sqlx::query_as(&sql)
        .bind(id)
        .fetch_one(&pool)
        .await
        .map_err(|_| {
            CustomError::TokenNotFound
        })?;


    Ok(Json(task))  
}


pub async fn all_users(Extension(pool): Extension<PgPool>) -> impl IntoResponse {
    let sql = "SELECT * FROM users".to_string();

    let task = sqlx::query_as::<_, User>(&sql).fetch_all(&pool).await.unwrap();

    (StatusCode::OK, Json(task))
}

pub async fn get_user_by_id(
    Path(id):Path<i32>, 
    Extension(pool): Extension<PgPool>
) -> Result <Json<User>, CustomError> {

    let sql = "SELECT * FROM users where id=$1".to_string();

    let task: User = sqlx::query_as(&sql)
        .bind(id)
        .fetch_one(&pool)
        .await
        .map_err(|_| {
            CustomError::UserNotFound
        })?;


    Ok(Json(task))  
}


pub async fn all_clients(Extension(pool): Extension<PgPool>) -> impl IntoResponse {
    let sql = "SELECT * FROM clients".to_string();

    let task = sqlx::query_as::<_, Client>(&sql).fetch_all(&pool).await.unwrap();

    (StatusCode::OK, Json(task))
}

pub async fn get_client_by_id(
    Path(id):Path<i32>, 
    Extension(pool): Extension<PgPool>
) -> Result <Json<Client>, CustomError> {

    let sql = "SELECT * FROM clients where id=$1".to_string();

    let task: Client = sqlx::query_as(&sql)
        .bind(id)
        .fetch_one(&pool)
        .await
        .map_err(|_| {
            CustomError::ClientNotFound
        })?;


    Ok(Json(task))  
}


// POST
#[axum::debug_handler]
pub async fn new_token(
    Extension(pool): Extension<PgPool>,
    Json(token): Json<NewToken>, // JSON at end to avoid
    // error: `Json<_>` consumes the request body and thus must be the last argument to the handler function
) -> Result <(StatusCode, Json<NewToken>), CustomError> {

    println!("{:?} {:?}", token, chrono::offset::Local::now());
    
    let sql = "INSERT INTO tokens (client_id, scopes, email, created_on, updated_on) values ($1, $2, $3, $4, $5)";

    let _ = sqlx::query(&sql)
        .bind(&token.client_id)
        .bind(&token.scopes)
        .bind(&token.email)
        .bind(chrono::offset::Local::now())
        .bind(chrono::offset::Local::now())
        .execute(&pool)
        .await
        .map_err(|e| {
            println!("{:?}", e);
            CustomError::InternalServerError
        })?;

    Ok((StatusCode::CREATED, Json(token)))
}

#[axum::debug_handler]
pub async fn new_client(
    Extension(pool): Extension<PgPool>,
    Json(client): Json<NewClient>, // JSON at end to avoid
    // error: `Json<_>` consumes the request body and thus must be the last argument to the handler function
) -> Result <(StatusCode, Json<NewClient>), CustomError> {

    // println!("{:?} {:?}", client, chrono::offset::Local::now());
    
    // let sql = "INSERT INTO clients (email, domain, detail, created_on, updated_on) values ($1, $2, $3, $4, $5)";

    // let _ = sqlx::query(&sql)
    //     .bind(&client.email)
    //     .bind(&client.domain)
    //     .bind(&client.detail)
    //     .bind(chrono::offset::Local::now())
    //     .bind(chrono::offset::Local::now())
    //     .execute(&pool)
    //     .await
    //     .map_err(|e| {
    //         println!("{:?}", e);
    //         CustomError::InternalServerError
    //     })?;

    Ok((StatusCode::CREATED, Json(client)))
}