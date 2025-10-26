
use axum::{
    routing::get,
    Router,
    routing::post
};
use serde::{
    Serialize, Deserialize
};
use axum::extract::{Json};
use sqlx::postgres::{
    PgPoolOptions,
    
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

//connect to db 
//do auth sign up and login - store user credentials and profile 
async fn signup(Json(payload): Json<serde_json::Value>){
    println!("{:?}",payload);
    let connection = connect_to_db().await;
    let query = db::db::seed_data(connection).await;
}
//postgres password 2020
async fn connect_to_db()->Pool<Postgres>{
    let pool = PgPoolOptions::new().max_connections(5).connect("postgres://postgres:2002@localhost/postgres").await.expect("successfully connected to db");
    pool
}

