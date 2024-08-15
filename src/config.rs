use mongodb::{bson::doc, options::ClientOptions, Client, Database};
use std::env;

pub async fn get_mongodb() -> Database {
    let uri = env::var("MONGO_DB_URI").expect("ENV: MONGO_DB_URI must be set");
    let client_options = ClientOptions::parse(uri)
        .await
        .expect("Failed to parse MONGO_DB_URI");
    let client = Client::with_options(client_options).expect("Failed to create MongoDB client");

    client
        .database("PlanIt")
        .run_command(doc! {"ping": 1})
        .await
        .expect("Failed to ping MongoDB server");

    client.database("PlanIt")
}
