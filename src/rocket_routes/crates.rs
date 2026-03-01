use diesel::result::Error as DieselError;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket_db_pools::Connection;

use super::Db;
use crate::models::{Crate, NewCrate, UpdateCrate};
use crate::repositories::CrateRepository;
use crate::rocket_routes::{not_found_error, server_error};

pub fn routes() -> Vec<rocket::Route> {
    rocket::routes![get_crates, get_crate, create_crate, update_crate, delete_crate]
}

#[rocket::get("/crates")]
pub async fn get_crates(mut db: Connection<Db>) -> Result<Json<Vec<Crate>>, (Status, String)> {
    let results = CrateRepository::find_all(&mut db, 100, 0)
        .await
        .map_err(|e| (Status::InternalServerError, e.to_string()))?;
    Ok(Json(results))
}


#[rocket::get("/crates/<id>")]
pub async fn get_crate(mut db: Connection<Db>, id: i32) -> Result<Json<Crate>, (Status, String)> {
    let result = CrateRepository::find(&mut db, id).await.map_err(|e| {
        if matches!(e, DieselError::NotFound) {
            not_found_error("Crate not found")
        } else {
            server_error(e)
        }
    })?;
    Ok(Json(result))
}

#[rocket::post("/crates", data = "<new_crate>")]
pub async fn create_crate(mut db: Connection<Db>, new_crate: Json<NewCrate>) -> Result<(Status, Json<Crate>), (Status, String)> {
    let created = CrateRepository::create(&mut db, new_crate.into_inner())
        .await
        .map_err(|e| (Status::InternalServerError, e.to_string()))?;
    Ok((Status::Created, Json(created)))
}

#[rocket::put("/crates/<id>", data = "<body>")]
pub async fn update_crate(mut db: Connection<Db>, id: i32, body: Json<UpdateCrate>) -> Result<Json<Crate>, (Status, String)> {
    let existing = CrateRepository::find(&mut db, id).await.map_err(|e| {
        if matches!(e, DieselError::NotFound) {
            not_found_error("Crate not found")
        } else {
            server_error(e)
        }
    })?;
    let up = body.into_inner();
    let to_update = Crate {
        id,
        rustacean_id: up.rustacean_id.unwrap_or(existing.rustacean_id),
        code: up.code.unwrap_or(existing.code),
        name: up.name.unwrap_or(existing.name),
        version: up.version.unwrap_or(existing.version),
        description: up.description.or(existing.description),
        created_at: existing.created_at,
    };
    let updated = CrateRepository::update(&mut db, to_update).await.map_err(|e| (Status::InternalServerError, e.to_string()))?;
    Ok(Json(updated))
}

#[rocket::delete("/crates/<id>")]
pub async fn delete_crate(mut db: Connection<Db>, id: i32) -> Result<Status, (Status, String)> {
    CrateRepository::delete(&mut db, id)
        .await
        .map_err(|e| (Status::InternalServerError, e.to_string()))?;
    Ok(Status::NoContent)
}
