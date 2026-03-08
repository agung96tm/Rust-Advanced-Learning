use diesel::prelude::*;
use diesel::BelongingToDsl;
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


pub struct UserRepository;

impl UserRepository {
    pub async fn create(c: &mut AsyncPgConnection, user: NewUser, role_codes: Vec<String>) -> QueryResult<User> {
        let user: User = diesel::insert_into(users::table).values(user).get_result(c).await?;

        for role_code in role_codes {
            let role_id = match RoleRepository::find_by_code(c, role_code.clone()).await {
                Ok(role) => role.id,
                Err(_) => {
                    let new_role = RoleRepository::create(c, NewRole {
                        code: role_code.clone(),
                        name: role_code,
                    }).await?;
                    new_role.id
                }
            };

            diesel::insert_into(user_roles::table)
                .values(NewUserRole {
                    user_id: user.id,
                    role_id,
                })
                .execute(c)
                .await?;
        }

        Ok(user)
    }

    pub async fn find_all(c: &mut AsyncPgConnection, limit: i64, offset: i64) -> QueryResult<Vec<User>> {
        users::table.limit(limit).offset(offset).get_results(c).await
    }

    pub async fn update(c: &mut AsyncPgConnection, user: User) -> QueryResult<User> {
        diesel::update(users::table.find(user.id))
            .set((
                users::username.eq(user.username.clone()),
                users::password.eq(user.password.clone()),
            ))
            .get_result(c)
            .await
    }

    pub async fn delete(c: &mut AsyncPgConnection, id: i32) -> QueryResult<usize> {
        diesel::delete(users::table.find(id)).execute(c).await
    }
}

pub struct RoleRepository;

impl RoleRepository {
    pub async fn find_by_ids(c: &mut AsyncPgConnection, ids: Vec<i32>) -> QueryResult<Vec<Role>> {
        roles::table.filter(roles::id.eq_any(ids)).load(c).await
    }

    pub async fn find_by_code(c: &mut AsyncPgConnection, code: String) -> QueryResult<Role> {
        roles::table.filter(roles::code.eq(code)).first(c).await
    }

    pub async fn find_by_user(c: &mut AsyncPgConnection, user: &User) -> QueryResult<Vec<Role>> {
        let user_roles = UserRole::belonging_to(user).get_results::<UserRole>(c).await?;
        let role_ids = user_roles.iter().map(|ur: &UserRole| ur.role_id).collect::<Vec<i32>>();
        Self::find_by_ids(c, role_ids).await
    }

    pub async fn create(c: &mut AsyncPgConnection, role: NewRole) -> QueryResult<Role> {
        diesel::insert_into(roles::table).values(role).get_result(c).await
    }
}