use clap::{Arg, Command};

#[tokio::main]
async fn main() {
    let matches = Command::new("rust-advanced-learning")
        // .version(env!("CARGO_PKG_VERSION"))
        .author("Agung Yuliyanto <agung.96tm@gmail.com>")
        .about("CLI for the rust-advanced-learning project")
        .arg_required_else_help(true)
        .subcommand(
            Command::new("users")
                .about("User management")
                .arg_required_else_help(true)
                .subcommand(
                    Command::new("create")
                        .about("Create a new user")
                        .arg_required_else_help(true)
                        .arg(Arg::new("username").required(true))
                        .arg(Arg::new("password").required(true))
                        .arg(Arg::new("roles").required(true).num_args(1..).value_delimiter(','))
                )
                .subcommand(
                    Command::new("list")
                        .about("List all users")
                        .arg_required_else_help(true)
                )
                .subcommand(
                    Command::new("delete")
                        .about("Delete a user")
                        .arg_required_else_help(true)
                        .arg(Arg::new("id").required(true))
                )
        )
        .get_matches();
    

    match matches.subcommand() {
        Some(("users", sub_matches)) => {
            match sub_matches.subcommand() {
                Some(("create", sub_matches)) => {
                    rust_advanced_learning::commands::create_user(
                        sub_matches.get_one::<String>("username").unwrap().to_owned(),
                        sub_matches.get_one::<String>("password").unwrap().to_owned(),
                        sub_matches.get_many::<String>("roles").unwrap().map(|s| s.to_owned()).collect(),
                    ).await;
                }
                Some(("list", _)) => {
                    rust_advanced_learning::commands::list_users().await;
                }
                Some(("delete", sub_matches)) => {
                    rust_advanced_learning::commands::delete_user(
                        sub_matches.get_one::<i32>("id").unwrap().to_owned(),
                    ).await;
                }
                _ => {
                    println!("Invalid subcommand");
                }
            }
        }
        _ => {
            println!("Invalid subcommand");
        }
    }

}