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
    let query = seed_data(connection).await;
}
//postgres password 2020
async fn connect_to_db()->Pool<Postgres>{
    let pool = PgPoolOptions::new().max_connections(5).connect("postgres://postgres:2002@localhost/postgres").await.expect("successfully connected to db");
    pool
}

async fn seed_data(connection: Pool<Postgres>)->Result<PgQueryResult, Error>{
    let query: &str = "
        CREATE TABLE \"User\" (
            \"id\" SERIAL NOT NULL,
            \"email\" TEXT NOT NULL,
            \"name\" TEXT,
            \"password\" TEXT,
            \"isDeleted\" BOOLEAN,
            CONSTRAINT \"User_pkey\" PRIMARY KEY (\"id\")
        );
    ";
     
    let query2: &str = "
        CREATE TABLE \"Post\" (
            \"id\" SERIAL NOT NULL,
            \"createdAt\" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
            \"updatedAt\" TIMESTAMP(3)
            \"title\" TEXT NOT NULL,
            \"authorId\" INTEGER NOT NULL,
            CONSTRAINT \"Post_pkey\" PRIMARY KEY (\"id\")
        );
    ";

    let query3: &str = "
        CREATE UNIQUE INDEX \"User_email_key\" ON \"User\"(\"email\");
    ";

    let query4: &str = "
        ALTER TABLE \"Post\" ADD CONSTRAINT \"Post_authorId_fkey\" FOREIGN KEY (\"authorId\") REFERENCES \"User\"(\"id\") ON DELETE RESTRICT ON UPDATE CASCADE;
    ";
    let parts = [query, query2, query3, query4];
    let coll = parts.concat();
    let string_slice: &str = coll.as_str();
    let result = connection.execute(string_slice).await;
    result
}

