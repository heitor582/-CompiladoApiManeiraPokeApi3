use dotenv;
use mongodb;
use mongodb::sync::Client;

pub fn establish_connection() -> Client {
  let database_url: &str = match dotenv::var("DB_URI") {
    Ok(value) => &value.to_owned()
  };

  let client = Client::with_uri_str(database_url).ok()
  .expect("Fail to initialize client");  
  
  client
} 