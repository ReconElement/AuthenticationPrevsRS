
pub mod db {
    use sqlx::{Executor, Pool, Postgres};
    use sqlx::Error;
    use sqlx::postgres::{PgQueryResult};
    pub async fn seed_data(connection: Pool<Postgres>) -> Result<PgQueryResult, Error> {
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
}
