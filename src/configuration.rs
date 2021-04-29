use rocket_contrib::databases::diesel;
use rocket::Rocket;


#[database("pg_db")]
pub struct PostgresConnection(diesel::PgConnection);

pub fn configure_routes(server: Rocket) -> Rocket {
    server.mount("/", routes![crate::product::category::routes::post])
}

pub fn attach_fairings(server: Rocket) -> Rocket {
    server.attach(PostgresConnection::fairing())
}
