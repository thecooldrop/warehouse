#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate diesel_migrations;
extern crate dotenv;

mod schema;
pub mod product;
pub mod configuration;

// pub(crate) mod testing;
// pub(crate) mod mock;