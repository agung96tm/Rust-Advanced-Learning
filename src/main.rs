use rocket_db_pools::{Database, Config};

mod models;
mod schema;
mod repositories;
mod rocket_routes;

#[derive(Database)]
#[database("db")]
pub struct Db(rocket_db_pools::diesel::PgPool);

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
        .attach(Db::init())
        .mount("/", rocket_routes::rustaceans::routes())
        .mount("/", rocket_routes::crates::routes())
        .launch()
        .await?;

    Ok(())
}
