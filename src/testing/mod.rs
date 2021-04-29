use testcontainers::{Image, Docker, Container, RunArgs};
use std::collections::HashMap;
use testcontainers::core::Port;
use testcontainers::images::postgres::Postgres;
use testcontainers::clients::Cli;
use diesel::{PgConnection, Connection};
use rocket::{Config, Rocket};
use rocket::config::{Environment, ConfigBuilder, Value};

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

    fn with_database(self, database_name: &str) -> Self{
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


fn with_database_connection(local_port: u16, pg_credentials: PgTestCredentials ) -> (PgConnection, String) {
    let db_url = format!("postgres://{}:{}@localhost:{}/testing", &pg_credentials.username, &pg_credentials.password, local_port);
    (PgConnection::establish(&db_url).unwrap(), db_url)
}

fn with_migrations_applied(conn: &PgConnection) {
    embed_migrations!();
    embedded_migrations::run(conn);
}


fn free_local_port() -> Option<u16> {
    let socket = std::net::SocketAddrV4::new(std::net::Ipv4Addr::LOCALHOST, 0);
    std::net::TcpListener::bind(socket)
        .and_then(|listener| listener.local_addr())
        .map(|addr| addr.port())
        .ok()
}

pub fn with_migrated_database_connection(test: impl FnOnce(PgConnection) -> Result<(), String>) -> Result<(), String> {
    let cli = with_docker_cli();
    let postgres_container = with_postgres_db(&cli);
    let (database_connection, _) = with_database_connection(postgres_container.get_host_port(5432).unwrap(),
                                                                 PgTestCredentials::default());
    with_migrations_applied(&database_connection);
    test(database_connection)
}

pub fn with_migrated_database_information(test: impl FnOnce(PgConnection, String) -> Result<(), String>) -> Result<(), String> {
    let cli = with_docker_cli();
    let postgres_container = with_postgres_db(&cli);
    let (database_connection, db_url) = with_database_connection(postgres_container.get_host_port(5432).unwrap(),
                                                            PgTestCredentials::default());
    with_migrations_applied(&database_connection);
    test(database_connection, db_url)
}

pub fn rocket_test_config() -> ConfigBuilder {
    Config::build(Environment::Development)
        .address("localhost")
        .port(free_local_port().unwrap())
        .keep_alive(5)
        .read_timeout(5)
        .write_timeout(5)
}

pub fn rocket_test_db_config(configuration: ConfigBuilder, db_url: String) -> ConfigBuilder {
    let mut database_config = HashMap::new();
    database_config.insert("url", Value::from(db_url));

    let mut databases = HashMap::new();
    databases.insert("pg_db", Value::from(database_config));

    configuration.extra("databases", databases)
}

pub fn with_rocket_configured(db_url: String, test: impl FnOnce(Rocket) -> Result<(), String>) -> Result<(), String>{
    use crate::configuration;
    let rocket = rocket::custom(rocket_test_db_config(rocket_test_config(), db_url).finalize().unwrap());
    let rocket = configuration::configure_routes(rocket);
    let rocket = configuration::attach_fairings(rocket);
    test(rocket)
}

