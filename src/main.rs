use serde::{Deserialize, Serialize};
use std::collections::HashMap;
// use clap::{Parser, Subcommand};


#[derive(Serialize, Deserialize, Debug)]
struct User {
    nick: String,
    coordinates: [f64; 2],
    links: HashMap<String, String>,
}

// #[derive(Parser)]
// #[command(name = "map-builder")]
// #[command(about = "Validates user JSON submissions and builds the final map file", long_about = None)]
// struct Cli {
//     #[command(subcommand)]
//     command: Commands,
// }

// enum Commands {
//     Validate,
//     Build,
// }


fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello, world!");

    // let cli = Cli::parse();
    let user_file = std::fs::read_dir("user")?;

    let mut all_users = Vec::new();

    for user in user_file {
        
        let user_data = std::fs::read_to_string(user?.path())?;
        println!("{:?}", &user_data);

        let parsed_user: User = serde_json::from_str(&user_data)?;
        println!("{:?}", &parsed_user);

        all_users.push(parsed_user);

    }

    let all_users_json = serde_json::to_string_pretty(&all_users);

    println!("{:?}", &all_users_json);

    std::fs::write("website/users.json", all_users_json?)?;

  

    // match &cli.command {
    //     Commands::Validate => {
    //         println!("Validating user JSON submissions...");
    //         // Add validation logic here
    //     }
    //     Commands::Build => {
    //         println!("Building the final map file...");
    //         // Add build logic here
    //     }
    // }


    Ok(())
}
