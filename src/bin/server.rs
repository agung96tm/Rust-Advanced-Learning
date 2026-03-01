extern crate rust_advanced_learning;

use rocket_db_pools::Config;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    let figment = rocket::Config::figment()
        .merge((
            "databases.db",
            Config {
                url: database_url,
                min_connections: None,
                max_connections: 1024,
                connect_timeout: 5,
                idle_timeout: None,
                extensions: None,
            },
        ));

    rocket::custom(figment)
        .attach(rust_advanced_learning::rocket_routes::Db::init())
        .mount("/", rust_advanced_learning::rocket_routes::rustaceans::routes())
        .mount("/", rrust_advanced_learning::ocket_routes::crates::routes())
        .launch()
        .await?;

    Ok(())
}
