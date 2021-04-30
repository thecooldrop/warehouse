use diesel::connection::TransactionManager;
use diesel::{Connection, QueryResult};

/// A mock of transaction manager, which always returns an Ok() and transaction depth of 1
pub struct OkayTransactionManager;

impl<Conn: Connection> TransactionManager<Conn> for OkayTransactionManager {
    fn begin_transaction(&self, conn: &Conn) -> QueryResult<()> {
        Ok(())
    }

    fn rollback_transaction(&self, conn: &Conn) -> QueryResult<()> {
        Ok(())
    }

    fn commit_transaction(&self, conn: &Conn) -> QueryResult<()> {
        Ok(())
    }

    fn get_transaction_depth(&self) -> u32 {
        1
    }
}