use dotenv;

pub fn init() {
    let _ = dotenv::from_path("../.env");
    dotenv::dotenv().ok();
    if dotenv::var("RELEASE").unwrap() == "false" {
        std::env::set_var("RUST_LOG", "debug");
        env_logger::init();
    }
}

pub fn get_connection_string() -> String {
    return dotenv::var("DATABASE_URL").unwrap();
}
