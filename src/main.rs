use std::env;
use dotenv::dotenv;
mod entity;
use entity::{post, user};
use axum::{
    routing::get,
    Router,
    routing::post,
    response::Json,
    http::StatusCode
};
use sea_orm::ActiveValue::Set;
use sea_orm::{Database, DatabaseConnection};
use serde::{
    Serialize, Deserialize
};
// use axum::extract::{Json};
use sqlx::postgres::{
    PgPoolOptions, PgQueryResult,
    
};
use sqlx::{Executor, Pool, Postgres};
mod db;
#[derive(Serialize, Deserialize, Debug)]
struct User{
    full_name: String,
    email: String,
    user_name: String,
    password: String
}

#[tokio::main]
async fn main(){
    let app = Router::new().route("/",get(|| async {"Hello World"})).route("/signup",post(signup));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn signup(Json(user): Json<User>)->StatusCode{
    println!("{:?}",user);
    //write to db 
    let db_conn = connect_to_db_sea().await;
    StatusCode::ACCEPTED
}
//postgres password 2020
async fn connect_to_db_sea()->DatabaseConnection{
    let connection_string = env::var("DATABASE_URL").unwrap();
    // let db: DatabaseConnection =  Database::connect("postgres://postgres:2020@localhost/postgres").await.expect("Successfully connected to db");
    let db: DatabaseConnection = Database::connect(env::var("DATABASE_URL").unwrap()).await.expect("Connection established");
    db
}
async fn connect_to_db()->Pool<Postgres>{
    let pool = PgPoolOptions::new().max_connections(5).connect("postgres://postgres:2020@localhost/postgres").await.expect("successfully connected to db");
    pool
}
