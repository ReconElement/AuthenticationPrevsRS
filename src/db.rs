
pub mod db {
    use std::option::Option;
    use std::time::SystemTime;
    use chrono::{NaiveDate, NaiveDateTime, Utc};
    use sea_orm::ActiveValue::Set;
    use sqlx::{Executor, Pool, Postgres};
    use sqlx::Error;
    use sqlx::postgres::{PgQueryResult};
    use crate::entity::{user,post};
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
    
    pub async fn seed_data(){
        let seed_user_name = Some(String::from("Omkar Panda"));
        let user = user::ActiveModel{
            name: Set(Some(String::from("John Smith"))),
            email: Set(String::from("johnsmith@gmail.com")),
            password: Set(Some(String::from("1870Er"))),
            is_deleted: Set(Some(true)),
            id: Set(1),
            posts: Set(1)
        };
        let naive_date = Utc::now().date_naive();
        let naive_time = Utc::now().time();
        let post = post::ActiveModel{
            id: Set(1),
            created_at: Set(NaiveDateTime::new(naive_date, naive_time)),
            //the creation is also an updation technically so it can be same for this seed instance
            updated_at: Set(Some(NaiveDateTime::new(naive_date, naive_time))),
            title: Set(String::from("I enjoy thinking and writing code")),
            author_id: Set(1),
            user: Set(1)
        };
    }
}
