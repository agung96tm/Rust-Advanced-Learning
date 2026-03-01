use diesel::result::Error as DieselError;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket_db_pools::Connection;

use crate::rocket_routes::{not_found_error, server_error};
use super::Db;
use crate::models::{NewRustacean, Rustacean, UpdateRustacean};
use crate::repositories::RustaceanRepository;

/// Kumpulan route untuk rustaceans. Tambah handler baru di sini.
pub fn routes() -> Vec<rocket::Route> {
    rocket::routes![get_rustaceans, get_rustacean, create_rustacean, update_rustacean, delete_rustacean]
}

#[rocket::get("/rustaceans")]
pub async fn get_rustaceans(mut db: Connection<Db>) -> Result<Json<Vec<Rustacean>>, (Status, String)> {
    let results = RustaceanRepository::find_all(&mut db, 100, 0)
        .await
        .map_err(|e| (Status::InternalServerError, e.to_string()))?;
    Ok(Json(results))
}

#[rocket::get("/rustaceans/<id>")]
pub async fn get_rustacean(mut db: Connection<Db>, id: i32) -> Result<Json<Rustacean>, (Status, String)> {
    let result = RustaceanRepository::find(&mut db, id).await.map_err(|e| {
        if matches!(e, DieselError::NotFound) {
            not_found_error("Rustacean not found")
        } else {
            server_error(e)
        }
    })?;
    Ok(Json(result))
}

#[rocket::post("/rustaceans", format="json", data = "<rustacean>")]
pub async fn create_rustacean(mut db: Connection<Db>, rustacean: Json<NewRustacean>) -> Result<(Status, Json<Rustacean>), (Status, String)> {
    let result = RustaceanRepository::create(&mut db, rustacean.into_inner())
        .await
        .map_err(|e| (Status::InternalServerError, e.to_string()))?;
    Ok((Status::Created, Json(result)))
}

#[rocket::put("/rustaceans/<id>", format = "json", data = "<body>")]
pub async fn update_rustacean(
    mut db: Connection<Db>,
    id: i32,
    body: Json<UpdateRustacean>,
) -> Result<Json<Rustacean>, (Status, String)> {
    let existing = RustaceanRepository::find(&mut db, id)
        .await
        .map_err(|e| (Status::InternalServerError, e.to_string()))?;
    let UpdateRustacean { name, email } = body.into_inner();
    let to_update = Rustacean {
        id,
        name,
        email,
        created_at: existing.created_at,
    };
    let result = RustaceanRepository::update(&mut db, to_update)
        .await
        .map_err(|e| (Status::InternalServerError, e.to_string()))?;
    Ok(Json(result))
}

#[rocket::delete("/rustaceans/<id>")]
pub async fn delete_rustacean(mut db: Connection<Db>, id: i32) -> Result<Status, (Status, String)> {
    RustaceanRepository::delete(&mut db, id)
        .await
        .map_err(|e| (Status::InternalServerError, e.to_string()))?;
    Ok(Status::NoContent)
}