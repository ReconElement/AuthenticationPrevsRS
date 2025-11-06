use std::arch::x86_64::_MM_MANTISSA_SIGN_ENUM;
use std::env;
use axum::response::IntoResponse;
use axum::routing::connect;
use bcrypt::{DEFAULT_COST, hash, verify};
use dotenv::dotenv;
use axum::{
    routing::get,
    Router,
    routing::post,
    response::Json,
    http::StatusCode
};
use serde::{
    Serialize, Deserialize
};
use serde_json::json;
// use axum::extract::{Json};
use sqlx::postgres::{
    PgPoolOptions, PgQueryResult,
    
};
use sqlx::{Executor, Pool, Postgres};
mod db;
use db::db::{seed_data, sign_up_query, sign_in_query};
#[derive(Serialize, Deserialize, Debug)]
struct User{
    full_name: String,
    email: String,
    user_name: String,
    password: String
}
#[derive(Serialize, Deserialize, Debug)]

struct sign_in_user{
    full_name: String,
    password: String
}

#[tokio::main]
async fn main(){
    let app = Router::new().route("/",get(|| async {"Hello World"})).route("/signup",post(signup)).route("/signin",post(signin));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn signup(Json(user): Json<User>)->StatusCode{
    println!("{:?}",user);
    //write to db
    let db_conn = connect_to_db().await;
    // let _ = seed_data(db_conn).await;
    let password = user.password.clone();
    let hashed_password = hash(user.password, DEFAULT_COST).unwrap();
    let valid = verify(password, &hashed_password).unwrap();
    println!("{}",valid);
    let insert = sign_up_query(db_conn, &user.email,&user.full_name, &hashed_password, false).await;
    StatusCode::ACCEPTED
}

#[derive(Serialize, Deserialize, Debug)]
struct Response{
    status_code: i32,
    data: String
}

// async fn signin(Json(sign_in_user):Json<sign_in_user>)->impl IntoResponse{
//     let connection = connect_to_db().await;
//     let signin = sign_in_query(connection, &sign_in_user.full_name, &sign_in_user.password).await;
//     if signin == true {
//         (StatusCode::FOUND, format!("Data found"))
//     }else{
//         (StatusCode::NOT_FOUND, format!("Data not found"))
//     }
// }

async fn signin(Json(sign_in_user):Json<sign_in_user>)->Json<Response>{
    let connection = connect_to_db().await;
    let signin = sign_in_query(connection, &sign_in_user.full_name, &sign_in_user.password).await;
    if signin {
        let res = Response{
            status_code: 200,
            data: String::from("User found, signin successful")
        };
        Json(res)
    }else{
        let res = Response{
            status_code: 404,
            data: String::from("User not found, signin unsuccessful")
        };
        Json(res)
    }
}
//signin2 has to return cookies with authentication jwt stored in it 
async fn signin2(Json(sign_in_user):Json<sign_in_user>)->impl IntoResponse{
    let connection = connect_to_db().await;
    let users = sign_in_query(connection, &sign_in_user.full_name, &sign_in_user.password).await;
    if !users.is_empty(){
        for user in users{
            
        }
    }
}
//postgres password 2020
async fn connect_to_db()->Pool<Postgres>{
    let pool = PgPoolOptions::new().max_connections(5).connect("postgres://postgres:2020@localhost/postgres").await.expect("Connection with the db could not be established");
    pool
}