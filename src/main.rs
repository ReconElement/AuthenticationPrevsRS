use std::arch::x86_64::_MM_MANTISSA_SIGN_ENUM;
use std::env;
use std::str::FromStr;
use axum::http::{HeaderName, HeaderValue, header, HeaderMap};
use axum::response::{Html, IntoResponse, Response};
use cookie::time::Duration;
use cookie::{Cookie};
use axum::routing::{connect, head};
use axum::{body, debug_handler};
// use axum_cookie::{CookieMiddleware, prelude::*};
use bcrypt::{DEFAULT_COST, bcrypt, hash, verify};
use dotenv::dotenv;
use axum::{
    routing::get,
    Router,
    routing::post,
    response::Json,
    http::StatusCode,
    http::header::SET_COOKIE,
    response::{AppendHeaders},
    body::Body,
};
use futures::future::lazy;
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
use jsonwebtoken::{encode, Algorithm, Header, EncodingKey};
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
    let app = Router::new().route("/",get(|| async {"Hello World"})).route("/signup",post(signup)).route("/signin",post(signin2));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn signup(Json(user): Json<User>)->StatusCode{
    println!("{:?}",user);
    dotenv().ok();
    let db_url: &str = &env::var("DATABASE_URL").unwrap();
    //write to db
    let db_conn = connect_to_db(db_url).await;
    // let _ = seed_data(db_conn).await;
    let password = user.password.clone();
    let hashed_password = hash(user.password, DEFAULT_COST).unwrap();
    let valid = verify(password, &hashed_password).unwrap();
    println!("{}",valid);
    let insert = sign_up_query(db_conn, &user.email,&user.full_name, &hashed_password, false).await;
    StatusCode::ACCEPTED
}

#[derive(Serialize, Deserialize, Debug)]
struct IResponse{
    status_code: i32,
    data: String
}

#[axum::debug_handler]
async fn signin2(Json(body_value):Json<sign_in_user>)->impl IntoResponse{
    dotenv().ok();
    let db_url: &str = &env::var("DATABASE_URL").unwrap();
    let mut auth_cookie: Cookie<'_>;
    let connection = connect_to_db(db_url).await;
    let user = sign_in_query(connection, &body_value.full_name, &body_value.password).await;
    match user{
        Ok(user)=>{
            let valid = verify(body_value.password, &user.password).unwrap();
            if valid == true{
                println!("{:#?}",user);
            }else{
                println!("No value found");
            }
        },
        Err(e)=>println!("{e}")
    }
    let mut auth_cookie = Cookie::new("auth_token","er3323erewe");
    auth_cookie.set_http_only(false);
    auth_cookie.set_secure(true);
    auth_cookie.set_max_age(Duration::seconds(3600));
    println!("{:#?}", auth_cookie.expires_datetime());
    let headers = AppendHeaders([
        (SET_COOKIE, format!("jwtToken={}",auth_cookie))
    ]);
    let content = Html("<h1>T<h1>");
    headers
}
//postgres password 2020
async fn connect_to_db(db_url: &str)->Pool<Postgres>{
    let pool = PgPoolOptions::new().max_connections(5).connect(db_url).await.expect("Connection with the db could not be established");
    pool
}