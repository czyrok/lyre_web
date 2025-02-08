use sqlx::{Connection, Error, SqliteConnection};

#[derive(Debug)]
pub struct LocalDatabase {
    pub conn: SqliteConnection,
}

impl LocalDatabase {
    pub async fn new(local_database_uri: &str) -> Result<Self, Error> {
        let conn = SqliteConnection::connect(local_database_uri).await?;

        Ok(Self { conn })
    }
}
