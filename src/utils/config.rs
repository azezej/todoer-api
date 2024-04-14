use dotenv;

pub fn get_connection_string() -> String {
    return dotenv::var("CONNECTION_STRING").unwrap();
}
