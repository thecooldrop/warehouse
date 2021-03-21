use diesel::prelude::*;
use std::fmt::{Debug};
use testcontainers::{Docker, Image, clients::Cli, images::postgres::Postgres, RunArgs, Container};
use crate::schema::product_category;
use crate::schema::product_category::all_columns;
use std::collections::HashMap;
use testcontainers::core::Port;

#[derive(Debug, PartialEq, Queryable, Identifiable)]
#[table_name="product_category"]
struct ProductCategory{
    id: i32,
    name: String,
    version: i32
}

#[derive(Debug, Insertable)]
#[table_name="product_category"]
struct NewProductCategory<'a> {
    name: &'a str
}

impl<'a> NewProductCategory<'a> {
    pub fn create(self, conn: &PgConnection) -> Result<ProductCategory, diesel::result::Error>
    {
        use crate::schema::product_category::dsl::*;
        conn.transaction(|| {
            diesel::insert_into(product_category)
                .values(self)
                .on_conflict_do_nothing()
                .returning(all_columns)
                .get_result(conn)
        })
    }
}


fn with_docker_cli() -> impl Docker {
    Cli::default()
}

fn with_postgres_db(cli: &impl Docker) -> Container<'_, impl Docker, Postgres> {
    ImageRunBuilder::<Postgres>::default()
        .with_random_local_port(5432)
        .with_env_var("POSTGRES_PASSWORD", PgTestCredentials::default().password.as_str())
        .with_database("testing")
        .run(cli)
}

struct PgTestCredentials {
    username: String,
    password: String
}

impl Default for PgTestCredentials {
    fn default() -> Self {
        Self {
            username: "postgres".to_string(),
            password: "lol".to_string()
        }
    }
}

fn with_database_connection(local_port: u16, pg_credentials: PgTestCredentials ) -> PgConnection {
    let PgTestCredentials {username, password} = pg_credentials;
    let db_url = format!("postgres://{}:{}@localhost:{}/testing", username, password, local_port);
    PgConnection::establish(&db_url).unwrap()
}

fn with_migrations_applied(conn: &PgConnection) {
    embed_migrations!();
    embedded_migrations::run(conn);
}

struct ImageRunBuilder<T: Image> {
    image: T,
    env_vars: Option<HashMap<String, String>>,
    port_mapping: Option<Port>
}

impl ImageRunBuilder<Postgres> {

    fn with_env_var(mut self, env_var: &str, val: &str) -> Self {
        if let None = self.env_vars {
            self.env_vars = Some(HashMap::default());
        }
        if let Some(ref mut map) = self.env_vars {
            map.insert(env_var.to_string(), val.to_string());
        }
        self
    }

    fn with_random_local_port(mut self, internal: u16) -> Self {
        self.port_mapping = Some(Port { local: free_local_port().unwrap(), internal });
        self
    }

    fn with_port(mut self, local: u16, internal: u16) -> Self {
        self.port_mapping = Some(Port {local, internal});
        self
    }

    fn with_database(mut self, database_name: &str) -> Self{
        self.with_env_var("POSTGRES_DB", database_name)
    }


    fn run(self, cli: &impl Docker) -> Container<'_, impl Docker, Postgres> {
        let ImageRunBuilder {
            image,
            env_vars,
            port_mapping } = self;

        let image = image.with_env_vars(env_vars.unwrap_or_default());
        let port_mapping = port_mapping.unwrap_or(Port {local: 5432, internal: 5432});
        let docker_args = RunArgs::default().with_mapped_port(port_mapping);
        cli.run_with_args(image, docker_args)
    }
}

impl Default for ImageRunBuilder<Postgres> {
    fn default() -> Self {
        ImageRunBuilder {
            image: Postgres::default().with_version(13),
            env_vars: Default::default(),
            port_mapping: None
        }
    }
}


fn free_local_port() -> Option<u16> {
    let socket = std::net::SocketAddrV4::new(std::net::Ipv4Addr::LOCALHOST, 0);
    std::net::TcpListener::bind(socket)
        .and_then(|listener| listener.local_addr())
        .map(|addr| addr.port())
        .ok()
}


#[cfg(test)]
mod test {
    use crate::product::category::models::{with_docker_cli, with_postgres_db, with_database_connection, PgTestCredentials, with_migrations_applied, NewProductCategory, ProductCategory};
    use diesel::{prelude, RunQueryDsl};
    use testcontainers::Image;

    #[test]
    fn test_can_create_new_product_category_in_db() {
        use crate::schema::product_category::dsl::*;

        let cli = with_docker_cli();
        let postgres_container = with_postgres_db(&cli);
        let database_connection = with_database_connection(postgres_container.get_host_port(5432).unwrap(), PgTestCredentials::default());
        with_migrations_applied(&database_connection);

        let test_category = NewProductCategory{ name: "testing" };
        let saved_product_category = test_category.create(&database_connection).unwrap();

        let mut product_categories: Vec<ProductCategory> = product_category.load(&database_connection).unwrap();
        assert_eq!(product_categories.len(), 1);
        assert_eq!(saved_product_category.name, String::from("testing"));
        assert_eq!(saved_product_category.version, 0);
    }
}
