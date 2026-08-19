use std::env;

#[derive(Clone)]
pub struct Config {
    pub database_url: String,
    pub jwt_secret: String,
    pub port: u16,
}

impl Config {
    pub fn from_env() -> Self {
        dotenvy::dotenv().ok();

        let database_url = env::var("DATABASE_URL")
            .expect("DATABASE_URL não definida no .env");
        let jwt_secret = env::var("JWT_SECRET")
            .expect("JWT_SECRET não definida no .env");
        let port = env::var("APP_PORT")
            .unwrap_or_else(|_| "3000".to_string())
            .parse()
            .unwrap_or(3000);

        Self {
            database_url,
            jwt_secret,
            port,
        }
    }
}
