use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::{Database, Transaction};
use uuid::Uuid;

/// # InterstellarTransaction
/// This struct holds the transaction to be stored and a unique identifier & date that it will give to the other services.
/// 
/// ## How to use this?
/// Create a normal transaction struct with sqlx, then when you're done using it (after all queries have been done with it) DON'T COMMIT IT.
/// Call the `interstellar!(tx)` macro right after the last time you use the Transaction.
/// 
#[derive(Debug)]
pub struct InterstellarTransaction<'a, DB>
where
    DB: Database,
{
    identifier: Uuid,
    transaction: Transaction<'a, DB>,
    #[allow(unused)]
    time_created: DateTime<Utc>,
}
#[async_trait]
pub trait Interstellar<'a, DB>  where DB: Database {
    fn new(transaction: Transaction<'a, DB>) -> Self;
    fn get_identifier<'b>(&'b self) -> &'b Uuid;
    async fn complete(self) -> Result<(), sqlx::Error>;
    async fn abort(self) -> Result<(), sqlx::Error>;
}
#[async_trait]
impl<'a, DB> Interstellar<'a, DB> for InterstellarTransaction<'a, DB> where DB: Database {
    /// Creates a new InterstellarTransaction from a transaction.
    fn new(transaction: Transaction<'a, DB>) -> Self {
        Self {
            identifier: Uuid::new_v4(),
            transaction,
            time_created: Utc::now(),
        }
    }
    /// Returns the Uuid for this transaction. This function clones the Uuid.
    fn get_identifier<'b>(&'b self) -> &'b Uuid {
        &self.identifier
    }
    /// Commits the inner transaction. You should never call this method directly
    async fn complete(self) -> Result<(), sqlx::Error> {
        self.transaction.commit().await
    }
    async fn abort(self) -> Result<(), sqlx::Error> {
        self.transaction.rollback().await
    }
}