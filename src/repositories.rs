use diesel::prelude::*;
use rocket_db_pools::diesel::{AsyncPgConnection, RunQueryDsl};

use crate::schema::*;
use crate::models::*;

pub struct RustaceanRepository;

impl RustaceanRepository {
    pub async fn find(c: &mut AsyncPgConnection, id: i32) -> QueryResult<Rustacean> {
        rustaceans::table.find(id).get_result(c).await
    }

    pub async fn find_all(c: &mut AsyncPgConnection, limit: i64, offset: i64) -> QueryResult<Vec<Rustacean>> {
        rustaceans::table.limit(limit).offset(offset).get_results(c).await
    }

    pub async fn create(c: &mut AsyncPgConnection, rustacean: NewRustacean) -> QueryResult<Rustacean> {
        diesel::insert_into(rustaceans::table).values(rustacean).get_result(c).await
    }

    pub async fn update(c: &mut AsyncPgConnection, rustacean: Rustacean) -> QueryResult<Rustacean> {
        diesel::update(rustaceans::table.find(rustacean.id))
            .set((
                rustaceans::name.eq(rustacean.name),
                rustaceans::email.eq(rustacean.email),
            ))
            .get_result(c)
            .await
    }

    pub async fn delete(c: &mut AsyncPgConnection, id: i32) -> QueryResult<usize> {
        diesel::delete(rustaceans::table.find(id)).execute(c).await
    }
}

pub struct CrateRepository;

impl CrateRepository {
    pub async fn find(c: &mut AsyncPgConnection, id: i32) -> QueryResult<Crate> {
        crates::table.find(id).get_result(c).await
    }

    pub async fn find_all(c: &mut AsyncPgConnection, limit: i64, offset: i64) -> QueryResult<Vec<Crate>> {
        crates::table.limit(limit).offset(offset).get_results(c).await
    }

    pub async fn create(c: &mut AsyncPgConnection, new_crate: NewCrate) -> QueryResult<Crate> {
        diesel::insert_into(crates::table).values(new_crate).get_result(c).await
    }

    pub async fn update(c: &mut AsyncPgConnection, crate_data: Crate) -> QueryResult<Crate> {
        diesel::update(crates::table.find(crate_data.id))
            .set((
                crates::rustacean_id.eq(crate_data.rustacean_id),
                crates::code.eq(crate_data.code),
                crates::name.eq(crate_data.name),
                crates::version.eq(crate_data.version),
                crates::description.eq(crate_data.description),
            ))
            .get_result(c)
            .await
    }

    pub async fn delete(c: &mut AsyncPgConnection, id: i32) -> QueryResult<usize> {
        diesel::delete(crates::table.find(id)).execute(c).await
    }
}