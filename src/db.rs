
pub mod db {
    use std::ops::Deref;
    use std::option::Option;
    use std::task::Context;
    use sqlx::postgres::PgRow;
    use futures::{FutureExt, Stream, TryStreamExt};
    use futures::stream::{self, StreamExt};
    use futures::task::Poll;
    use futures::executor::block_on_stream;
    use std::time::SystemTime;
    use chrono::{NaiveDate, NaiveDateTime, Utc};
    use sqlx::FromRow;
    use sqlx::{Executor, Pool, Postgres, Row};
    use sqlx::Error;
    use sqlx::postgres::{PgQueryResult};
    use std::pin::Pin;
    #[derive(sqlx::FromRow, Debug)]
    struct User{
        id: i32, 
        email: String,
        name: String,
        password: String,
        is_denied: bool
    }
    pub async fn create_tables(connection: Pool<Postgres>) -> Result<PgQueryResult, Error> {
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
            \"updatedAt\" TIMESTAMP(3),
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
    pub async fn seed_data(connection: Pool<Postgres>){
        // let insert_user = "INSERT INTO \"User\" (id, email, name, password, \"isDeleted\") VALUES (1, 'omkarpanda895@gmail.com', 'Omkar Panda', 'er73er',FALSE)";
        let insert_post = "INSERT INTO \"Post\" (id, title, \"authorId\") VALUES (1, 'I like to speak in native code', 1)";
        // let query = sqlx::query(insert_user).execute(&connection).await.expect("Query didn't go as expected");
        let query1 = sqlx::query(insert_post).execute(&connection).await.expect("Query 2 didn't go as expected");
        println!("{:?}",query1);
    }
    pub async fn sign_up_query(connection: Pool<Postgres>, email: &str, name: &str, password: &str, is_deleted: bool){
        let mut struck = sqlx::query("SELECT * FROM \"User\";").map(|row: PgRow| {println!("{:?}",row)}).fetch(&connection);
        let mut insert_user = format!("INSERT INTO \"User\" (email, name, password, \"isDeleted\") values ('{}','{}','{}',{})", email, name, password, is_deleted);
        let insert_user: &str = &insert_user;
        let query = sqlx::query(insert_user).execute(&connection).await.expect("Something went wrong while inserting data");
    } 
    pub async fn sign_in_query(connection: Pool<Postgres>, name: &str, password: &str){
        let mut sign_in = format!("SELECT * FROM \"User\" where name='{}' and password='{}';",name, password);
        println!("{}",sign_in);
        let sign_in: &str = &sign_in;
        let mut stream_rows = sqlx::query_as::<_, User>(sign_in).fetch(&connection);
        while let Some(value) = stream_rows.next().await{
            match value {
                Ok(value)=>println!("{:?}",value),
                Err(e)=>println!("{e}")
            }
        }
        // let mut stream = block_on_stream(stream_rows);
        // while let Some(value) = stream.next(){
        //     match value {
        //         Ok(value)=>println!("{:?}",value),
        //         Err(e)=>println!("{e}")
        //     }
        // }
        // let stream_rows = sqlx::query_as::<_,User>(sign_in).fetch_one(&connection).await;
        // match stream_rows{
        //     Ok(stream_rows)=>println!("{:?}",stream_rows),
        //     Err(e)=>println!("{e}")
        // }
        // let stream_rows = sqlx::query_as::<_, User>(sign_in).fetch_all(&connection).await;
        // let stream = sqlx::query(sign_in).fetch_all(&connection).await;
        // while let Some(value) = stream.iter().next(){
        //     value.iter().for_each(|t|{
        //         println!("{:?}",t)
        //     });
        // }

        // let stream = sqlx::query(sign_in).fetch_one(&connection).await;
        // match stream {
        //     Ok(stream)=>println!("{:?}",stream),
        //     Err(e)=>println!("{:?}",e)
        // }
        let stream = sqlx::query_as::<_,User>(sign_in).fetch_one(&connection).await;
        match stream {
            Ok(stream)=>println!("{:#?}",stream.name),
            Err(e)=>println!("{e}")
        }
    }
}
