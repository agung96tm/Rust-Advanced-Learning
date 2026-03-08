use diesel_async::{AsyncConnection, AsyncPgConnection};
use crate::models::NewUser;
use crate::repositories::{UserRepository, RoleRepository};

async fn load_db_connection() -> AsyncPgConnection {
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    AsyncPgConnection::establish(&db_url).await.expect("Failed to connect to the database")
}

pub async fn create_user(username: String, password: String, role_codes: Vec<String>) {
    let mut c = load_db_connection().await;
    let user = UserRepository::create(&mut c, NewUser {
        username,
        password,
    }, role_codes).await.expect("Failed to create user");
    println!("User created successfully: {:?}", user);

    let roles = RoleRepository::find_by_user(&mut c, &user).await.expect("Failed to find roles");
    println!("Roles: {:?}", roles);
}

pub async fn list_users() {
    let mut c = load_db_connection().await;
    let users = UserRepository::find_all(&mut c, 100, 0).await.expect("Failed to list users");
    println!("Users: {:?}", users);
}

pub async fn delete_user(id: i32) {
    let mut c = load_db_connection().await;
    let result = UserRepository::delete(&mut c, id).await.expect("Failed to delete user");
    println!("User deleted successfully: {:?}", result);
}