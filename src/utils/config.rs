use dotenv;

pub fn get_connection_string() -> String {
    let _ = dotenv::from_path("../.env");
    dotenv::dotenv().ok();

    return format!(
        "postgresql://{}:{}@{}:{}/{}",
        dotenv::var("POSTGRES_USER").unwrap(),
        dotenv::var("POSTGRES_PASSWORD").unwrap(),
        "localhost",
        dotenv::var("POSTGRES_PORT").unwrap(),
        dotenv::var("POSTGRES_DATABASE").unwrap()
    );
}
