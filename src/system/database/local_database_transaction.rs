use sqlx::{Connection, Sqlite, SqliteConnection, Transaction};

pub struct LocalDatabaseTransaction<'a> {
    pub value: Transaction<'a, Sqlite>,
}

impl<'a> LocalDatabaseTransaction<'a> {
    pub async fn new(
        conn: &'a mut SqliteConnection,
    ) -> Result<LocalDatabaseTransaction<'a>, sqlx::Error> {
        let transaction: Transaction<'a, Sqlite> = conn.begin().await?;

        Ok(LocalDatabaseTransaction { value: transaction })
    }
}
