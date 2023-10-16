use envconfig::Envconfig;
#[derive(Envconfig)]
pub struct Config {
    #[envconfig(from = "DB_HOST", default = "sqlite://sqlite.db")]
    pub db_host: String,

    #[envconfig(from = "DB_MIGRATION_PATH", default = "./migrations")]
    pub db_migration_path: String,

    #[envconfig(from = "HTTP_HOST", default = "0.0.0.0")]
    pub http_host: String,

    #[envconfig(from = "HTTP_PORT", default = "8080")]
    pub http_port: u16,
}