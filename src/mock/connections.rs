use diesel::connection::{TransactionManager, SimpleConnection};
use diesel::backend::Backend;
use diesel::{QueryResult, Connection, Queryable, ConnectionResult};
use diesel::sql_types::HasSqlType;
use diesel::query_builder::{QueryId, AsQuery, QueryFragment};
use diesel::deserialize::QueryableByName;
use crate::mock::transactions::OkayTransactionManager;
use serde::ser::StdError;

pub struct AlreadyInTransactionConnection<T: TransactionManager<Self>>(T);

impl<T: TransactionManager<Self>> SimpleConnection for AlreadyInTransactionConnection<T> {
    fn batch_execute(&self, _query: &str) -> QueryResult<()> {
        Err(diesel::result::Error::AlreadyInTransaction)
    }
}

impl<U: Backend, T: TransactionManager<Self> + Default> Connection for AlreadyInTransactionConnection<T> {
    type Backend = U;
    type TransactionManager = T;

    fn establish(database_url: &str) -> ConnectionResult<Self> {
        Ok(AlreadyInTransactionConnection(T::default()))
    }

    fn execute(&self, query: &str) -> QueryResult<usize> {
        Err(diesel::result::Error::AlreadyInTransaction)
    }

    fn query_by_index<T, U>(&self, source: T) -> QueryResult<Vec<U>> where
        T: AsQuery,
        T::Query: QueryFragment<Self::Backend> + QueryId,
        Self::Backend: HasSqlType<T::SqlType>,
        U: Queryable<T::SqlType, Self::Backend> {
        Err(diesel::result::Error::AlreadyInTransaction)
    }

    fn query_by_name<T, U>(&self, source: &T) -> QueryResult<Vec<U>> where
        T: QueryFragment<Self::Backend> + QueryId,
        U: QueryableByName<Self::Backend> {
        Err(diesel::result::Error::AlreadyInTransaction)
    }

    fn execute_returning_count<T>(&self, source: &T) -> QueryResult<usize> where
        T: QueryFragment<Self::Backend> + QueryId {
        Err(diesel::result::Error::AlreadyInTransaction)
    }

    fn transaction_manager(&self) -> &Self::TransactionManager {
        &self.0
    }
}

pub struct RollBackTransactionConnection<T: TransactionManager<Self>>(T);

impl<T: TransactionManager<Self>> SimpleConnection for RollBackTransactionConnection<T> {
    fn batch_execute(&self, query: &str) -> QueryResult<()> {
        Err(diesel::result::Error::RollbackTransaction)
    }
}

impl<U: Backend, T: TransactionManager<Self> + Default> Connection for RollBackTransactionConnection<T> {
    type Backend = U;
    type TransactionManager = T;

    fn establish(database_url: &str) -> ConnectionResult<Self> {
        Okay(RollBackTransactionConnection(T::default()))
    }

    fn execute(&self, query: &str) -> QueryResult<usize> {
        Err(diesel::result::Error::RollbackTransaction)
    }

    fn query_by_index<T, U>(&self, source: T) -> QueryResult<Vec<U>> where
        T: AsQuery,
        T::Query: QueryFragment<Self::Backend> + QueryId,
        Self::Backend: HasSqlType<T::SqlType>,
        U: Queryable<T::SqlType, Self::Backend> {
        Err(diesel::result::Error::RollbackTransaction)
    }

    fn query_by_name<T, U>(&self, source: &T) -> QueryResult<Vec<U>> where
        T: QueryFragment<Self::Backend> + QueryId,
        U: QueryableByName<Self::Backend> {
        Err(diesel::result::Error::RollbackTransaction)
    }

    fn execute_returning_count<T>(&self, source: &T) -> QueryResult<usize> where
        T: QueryFragment<Self::Backend> + QueryId {
        Err(diesel::result::Error::RollbackTransaction)
    }

    fn transaction_manager(&self) -> &Self::TransactionManager {
        &self.0
    }
}

pub struct NotFoundErrorConnection<T: TransactionManager<Self>>(T);

impl<T: TransactionManager<Self>> SimpleConnection for NotFoundErrorConnection<T> {
    fn batch_execute(&self, query: &str) -> QueryResult<()> {
        Err(diesel::NotFound)
    }
}

impl<U: Backend, T: TransactionManager<Self> + Default> Connection for NotFoundErrorConnection<T> {
    type Backend = U;
    type TransactionManager = T;

    fn establish(database_url: &str) -> ConnectionResult<Self> {
        Ok(NotFoundErrorConnection(T::default()))
    }

    fn execute(&self, query: &str) -> QueryResult<usize> {
        Err(diesel::NotFound)
    }

    fn query_by_index<T, U>(&self, source: T) -> QueryResult<Vec<U>> where
        T: AsQuery,
        T::Query: QueryFragment<Self::Backend> + QueryId,
        Self::Backend: HasSqlType<T::SqlType>,
        U: Queryable<T::SqlType, Self::Backend> {
        Err(diesel::NotFound)
    }

    fn query_by_name<T, U>(&self, source: &T) -> QueryResult<Vec<U>> where
        T: QueryFragment<Self::Backend> + QueryId,
        U: QueryableByName<Self::Backend> {
        Err(diesel::NotFound)
    }

    fn execute_returning_count<T>(&self, source: &T) -> QueryResult<usize> where
        T: QueryFragment<Self::Backend> + QueryId {
        Err(diesel::NotFound)
    }

    fn transaction_manager(&self) -> &Self::TransactionManager {
        &self.0
    }
}

