use diesel::prelude::*;
use std::fmt::{Debug};
use diesel::{update, delete};
use crate::schema::product_category;
use diesel::query_builder::AsChangeset;
use diesel::sql_types::{Integer, Text};
use diesel::expression::{operators::Eq as DieselEq, ops::Add as DieselAdd, bound::Bound};
use serde::{Serialize, Deserialize};
use diesel::pg::Pg;

#[derive(Debug, PartialEq, Queryable, Identifiable, Deserialize, Serialize)]
#[table_name="product_category"]
pub struct ProductCategory{
    id: i32,
    name: String,
    version: i32
}


impl AsChangeset for ProductCategory {
    type Target = product_category::table;
    type Changeset = <(DieselEq<product_category::name, Bound<Text, String>>,
                       DieselEq<product_category::version, DieselAdd<product_category::version, Bound<Integer, i32>>>) as AsChangeset>::Changeset;

    fn as_changeset(self) -> Self::Changeset {
        (
            product_category::name.eq(self.name),
            product_category::version.eq(product_category::version + 1)
        ).as_changeset()
    }
}

impl ProductCategory {

    pub fn update(self, conn: &impl Connection<Backend=Pg>) -> Result<Option<ProductCategory>, diesel::result::Error> {
        use crate::schema::product_category::dsl::*;
        conn.transaction(|| {
           let updated_row = update(product_category.filter(id.eq(self.id).and(version.eq(self.version))))
               .set(self)
               .get_result(conn);

            match updated_row {
                Ok(e) => return Ok(Some(e)),
                Err(diesel::result::Error::NotFound) => return Ok(None),
                Err(e) => Err(e)
            }
        })
    }

    pub fn delete(self, conn: &impl Connection<Backend=Pg>) -> Result<usize, diesel::result::Error> {
        use crate::schema::product_category::dsl::*;
        conn.transaction(|| {
            delete(product_category.filter(id.eq(self.id).and(version.eq(self.version)))).execute(conn)
        })
    }

    pub(in super) fn equal(&self, other: &NewProductCategory) -> bool {
        if self.name == other.name {
            true
        } else {
            false
        }
    }
}

#[derive(Debug, Insertable, Serialize, Deserialize)]
#[table_name="product_category"]
pub struct NewProductCategory{
    name: String
}

impl NewProductCategory {
    pub fn create(self, conn: &impl Connection<Backend=Pg>) -> Result<ProductCategory, diesel::result::Error>
    {
        use crate::schema::product_category::dsl::*;
        use crate::schema::product_category::all_columns;
        conn.transaction(|| {
            diesel::insert_into(product_category)
                .values(self)
                .on_conflict_do_nothing()
                .returning(all_columns)
                .get_result(conn)
        })
    }
}

impl NewProductCategory {
    pub fn new(name: &str) -> NewProductCategory {
        NewProductCategory {
            name: name.to_lowercase()
        }
    }
}



#[cfg(test)]
mod test {
    use diesel::{prelude::*, RunQueryDsl};
    use testcontainers::Image;
    use crate::testing::with_migrated_database_connection;
    use crate::product::category::models::{NewProductCategory, ProductCategory};
    use diesel::sql_types::HasSqlType;
    use diesel::query_builder::{QueryId, AsQuery, QueryFragment};
    use diesel::backend::Backend;
    use diesel::connection::{TransactionManager, AnsiTransactionManager, SimpleConnection};
    use diesel::deserialize::QueryableByName;
    use diesel::pg::Pg;
    use std::ffi::NulError;
    use diesel::result::Error;

    #[test]
    fn test_can_create_new_product_category_in_db() -> Result<(), String> {
        use crate::schema::product_category::dsl::*;
        with_migrated_database_connection(|conn| {
            let test_category = NewProductCategory{name: "testing".to_string() };
            let saved_product_category = test_category.create(&conn).unwrap();

            let product_categories: Vec<ProductCategory> = product_category.load(&conn).unwrap();
            assert_eq!(product_categories.len(), 1);
            assert_eq!(saved_product_category.name, String::from("testing"));
            assert_eq!(saved_product_category.version, 0);

            Ok(())
        })
    }

    #[test]
    fn optimistically_locked_product_category_gets_saved_correctly_when_no_conflict() {
        use crate::schema::product_category::dsl::*;
        with_migrated_database_connection(|conn| {
            let test_category = NewProductCategory{name:"testing".to_string()};
            let mut saved_product_category = test_category.create(&conn).unwrap();
            saved_product_category.name = "testing_updated".to_string();
            let updated_product_category = saved_product_category.update(&conn);

            if let Ok(Some(pc)) = updated_product_category {
                assert_eq!(pc.name, "testing_updated".to_string());
                assert_eq!(pc.version, 1);
                assert_eq!(product_category.load::<ProductCategory>(&conn).unwrap().len(), 1);
                Ok(())
            } else {
                Err(String::from("Apparently there was an error with the database"))
            }
        });
    }

    #[test]
    fn optimistically_locked_product_category_does_not_get_saved_on_conflict() -> Result<(), String>{
        use crate::schema::product_category::dsl::*;
        with_migrated_database_connection(|conn| {

            let first_cat = NewProductCategory{name:"first".to_string()};
            let mut first_cat = first_cat.create(&conn).unwrap();
            let mut first_cat_second = product_category.first::<ProductCategory>(&conn).unwrap();
            first_cat.name = "first_updated".to_string();
            first_cat_second.name = "first_updated".to_string();

            let first_cat_updated = first_cat.update(&conn).unwrap();
            let first_cat_second_updated = first_cat_second.update(&conn).unwrap();

            if first_cat_updated.is_some() && first_cat_second_updated.is_none() {
                Ok(())
            } else {
                Err(String::from("Something did not work with optimistic locking"))
            }
        })
    }

    #[test]
    fn new_product_category_saves_string_as_lowercase() {
        let new_product_category = NewProductCategory::new("FiRsTcAtEgOrY");
        assert_eq!(new_product_category.name, "firstcategory".to_string());
    }

    #[test]
    fn product_category_can_be_deleted_from_db_when_version_did_not_change() -> Result<(), String> {
        use crate::schema::product_category::dsl::*;
        with_migrated_database_connection(|conn| {
            let first_product_category = NewProductCategory::new("first")
                .create(&conn)
                .unwrap();

            let second_product_category = NewProductCategory::new("second")
                .create(&conn)
                .unwrap();

            let deletion = first_product_category.delete(&conn);

            assert_eq!(1, product_category.load::<ProductCategory>(&conn).unwrap().len());
            assert_eq!(Ok(1), deletion);

            Ok(())
        })
    }

    #[test]
    fn product_category_can_not_be_deleted_from_db_if_it_was_updated_since_read() -> Result<(), String> {
        use crate::schema::product_category::dsl::*;
        with_migrated_database_connection(|conn| {
            let mut first_cat = NewProductCategory::new("first").create(&conn).unwrap();
            let first_cat_second = product_category.first::<ProductCategory>(&conn).unwrap();

            first_cat.name = "first_updated".to_string();
            first_cat.update(&conn);

            let deletion = first_cat_second.delete(&conn);
            assert_eq!(Ok(0), deletion);
            Ok(())
        })
    }

    // #[test]
    // fn creating_new_product_category_fails_on_any_kind_of_error() {
    //
    //     struct ErrorConnection(MockTransactionManager);
    //     struct MockTransactionManager;
    //     impl TransactionManager<ErrorConnection> for MockTransactionManager {
    //         fn begin_transaction(&self, conn: &ErrorConnection) -> QueryResult<()> {
    //             Ok(())
    //         }
    //
    //         fn rollback_transaction(&self, conn: &ErrorConnection) -> QueryResult<()> {
    //             Ok(())
    //         }
    //
    //         fn commit_transaction(&self, conn: &ErrorConnection) -> QueryResult<()> {
    //             Ok(())
    //         }
    //
    //         fn get_transaction_depth(&self) -> u32 {
    //             1
    //         }
    //     }
    //
    //     impl SimpleConnection for ErrorConnection {
    //         fn batch_execute(&self, query: &str) -> QueryResult<()> {
    //             todo!()
    //         }
    //     }
    //
    //     impl Connection for ErrorConnection {
    //         type Backend = Pg;
    //         type TransactionManager = MockTransactionManager;
    //
    //         fn establish(database_url: &str) -> ConnectionResult<Self> {
    //             Ok(ErrorConnection(MockTransactionManager))
    //         }
    //
    //         fn execute(&self, query: &str) -> QueryResult<usize> {
    //             Ok(0)
    //         }
    //
    //         fn query_by_index<T, U>(&self, source: T) -> QueryResult<Vec<U>> where
    //             T: AsQuery,
    //             T::Query: QueryFragment<Self::Backend> + QueryId,
    //             Self::Backend: HasSqlType<T::SqlType>,
    //             U: Queryable<T::SqlType, Self::Backend> {
    //             Err(diesel::result::Error::NotFound)
    //         }
    //
    //         fn query_by_name<T, U>(&self, source: &T) -> QueryResult<Vec<U>> where
    //             T: QueryFragment<Self::Backend> + QueryId,
    //             U: QueryableByName<Self::Backend> {
    //             Err(diesel::result::Error::NotFound)
    //         }
    //
    //         fn execute_returning_count<T>(&self, source: &T) -> QueryResult<usize> where
    //             T: QueryFragment<Self::Backend> + QueryId {
    //             Err(diesel::result::Error::NotFound)
    //         }
    //
    //         fn transaction_manager(&self) -> &Self::TransactionManager {
    //             &self.0
    //         }
    //     }
    //
    //     let conn = ErrorConnection(MockTransactionManager);
    //     let new_product_category = NewProductCategory::new("first_category");
    //     let error = new_product_category.create(&conn);
    //     match error {
    //         Ok(_) => panic!("I did not expect an okay"),
    //         Err(_) => {}
    //     }
    // }
}
