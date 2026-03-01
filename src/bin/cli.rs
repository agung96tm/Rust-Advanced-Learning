use clap::{Arg, Command};

fn main() {
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
                Some(("create", _)) => {
                    println!("Create a new user");
                    // let username = sub_matches.get_one::<String>("username").unwrap();
                    // let password = sub_matches.get_one::<String>("password").unwrap();
                    // let roles = sub_matches.get_many::<String>("roles").unwrap().collect::<Vec<String>>();
                }
                Some(("list", _)) => {
                    println!("List all users");
                    // let users = rust_advanced_learning::repositories::user::list_users().unwrap();
                    // println!("{:?}", users);
                }
                Some(("delete", _)) => {
                    println!("Delete a user");
                    // let id = sub_matches.get_one::<i32>("id").unwrap();
                    // rust_advanced_learning::repositories::user::delete_user(*id).unwrap();
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