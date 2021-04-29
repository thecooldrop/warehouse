use crate::product::category::models::{ProductCategory};
use crate::product::category::models::NewProductCategory;

use rocket_contrib::json::Json;
use rocket::http::Status;
use diesel::prelude::*;
use crate::configuration::PostgresConnection;
use diesel::pg::Pg;


#[post("/product-category", format="application/json", data="<category>")]
pub fn post(category: Json<NewProductCategory>, conn: PostgresConnection) -> Json<ProductCategory> {
    Json(category.into_inner().create(&*conn).unwrap())
}

// #[cfg(test)]
// mod tests {
//     use crate::testing::{with_migrated_database_information, with_rocket_configured};
//     use rocket::local::Client;
//     use crate::product::category::models::{NewProductCategory, ProductCategory};
//     use rocket::http::{ContentType, Status};
//     use diesel::{RunQueryDsl, PgConnection, Connection};
//
//     #[test]
//     fn test_can_create_product_category() -> Result<(), String> {
//         use crate::schema::product_category::dsl::*;
//         with_migrated_database_information(|conn, db_url| {
//             with_rocket_configured(db_url, |rocket| {
//
//                 impl NewProductCategory {
//                     pub fn create(self, conn: &PgConnection) -> Result<ProductCategory, diesel::result::Error>
//                     {
//                         Err(diesel::NotFound)
//                     }
//                 }
//
//                 let new_product_category = NewProductCategory::new("first_category");
//
//                 let client = Client::new(rocket).expect("Valid rocket instance");
//                 let post_request = client.post("/product-category")
//                     .body(serde_json::to_string(&new_product_category).unwrap())
//                     .header(ContentType::JSON);
//
//                 let mut post_response = post_request.dispatch();
//                 let saved_categories = product_category.load::<ProductCategory>(&conn).unwrap();
//                 let response_category = serde_json::from_str::<ProductCategory>(post_response.body_string().unwrap().as_str()).unwrap();
//                 assert_eq!(post_response.status(), Status::Created);
//                 assert_eq!(saved_categories.len(), 1);
//                 assert_eq!(response_category.equal(&new_product_category), true);
//                 Ok(())
//             })
//         })
//     }
// }