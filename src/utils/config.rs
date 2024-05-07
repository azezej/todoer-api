use dotenv;

pub fn init() {
    let _ = dotenv::from_path("../.env");
    dotenv::dotenv().ok();
    env_logger::init();
    if dotenv::var("RELEASE").unwrap() == "false" {
        std::env::set_var("RUST_LOG", "debug");
        std::env::set_var("SERDE_DEBUG", "1");
    }
}

pub fn get_connection_string() -> String {
    return dotenv::var("DATABASE_URL").unwrap();
}
