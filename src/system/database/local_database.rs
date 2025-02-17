use sqlx::{Connection, SqliteConnection};

use super::local_database_transaction::LocalDatabaseTransaction;

#[derive(Debug)]
pub struct LocalDatabase {
    pub connection: SqliteConnection,
}

impl LocalDatabase {
    pub async fn new(local_database_uri: &str) -> Result<Self, sqlx::Error> {
        let connection = SqliteConnection::connect(local_database_uri).await?;

        Ok(Self { connection })
    }

    pub async fn make_transaction(
        &mut self,
    ) -> Result<LocalDatabaseTransaction<'_>, sqlx::Error> {
        let transaction =
            LocalDatabaseTransaction::new(&mut self.connection).await?;

        Ok(transaction)
    }
}
