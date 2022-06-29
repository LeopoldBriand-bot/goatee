use mongodb::{options::ClientOptions, Client, Database};
use once_cell::sync::OnceCell;

pub static DB: OnceCell<Database> = OnceCell::new(); // Call database with: DB.get().unwrap()

pub async fn init() {
    // Get env variables.
    let mongo_address: &str =
        &std::env::var("MONGO_URL").expect("MONGO_URL env var not foud in .env file");
    let database_name: &str =
        &std::env::var("DATABASE_NAME").expect("DATABASE_NAME env var not foud in .env file");
    // Parse a connection string into an options struct.
    let client_options = ClientOptions::parse(mongo_address).await.unwrap();

    // Get connection up and running.
    let client = Client::with_options(client_options).unwrap();
    let database = client.database(database_name);
    DB.set(database).unwrap();
}
