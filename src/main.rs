use serde::{Deserialize, Serialize};
use serde_json;
use std::{collections::HashMap, fs};
use clap::{Parser, Subcommand};


#[derive(Serialize, Deserialize, Debug)]
struct User {
    nick: String,
    coordinates: [f64; 2],
    #[serde(default)]
    links: HashMap<String, String>,
}

#[derive(Parser)]
#[command(name = "map-builder")]
#[command(about = "Validates user JSON submissions and builds the final map file", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}
#[derive(Subcommand)]
enum Commands {
    Validate,
    Build,
}


fn parse_all_users() -> Result<Vec<User>, String> {

    let mut valid_users = Vec::new();
    let mut has_errors = false;

    
    let dir = match fs::read_dir("user") {
        Ok(d) => d,
        Err(_) => return Err("Failed to read the 'people' directory".into()),
    };

    println!("Validating user JSON submissions...");



    for entry in dir {
        let entry = match entry {
            Ok(e) => e,
            Err(_) => continue,
        };

        let path = entry.path();

        if path.extension().and_then(|e| e.to_str()) == Some("json") {
            let file_name = path.file_name().unwrap().to_string_lossy();
            let file_stem = path.file_stem().and_then(|s| s.to_str()).unwrap_or("");

            let file_content = match fs::read_to_string(&path) {
                Ok(content) => content,
                Err(e) => {
                    println!("Failed to read {}: {}",file_name,e);
                    has_errors=true;
                    continue;
                }
            };



            match serde_json::from_str::<User>(&file_content) {
                Ok(user ) => {
                    let mut file_has_error = false;
                    // Filename and Nickname check
                    if !user.nick.eq_ignore_ascii_case(&file_stem) {
                        println!(
                            "❌ Error in {}: Nickname '{}' does not match the filename (expected '{}.json').",
                            file_name, user.nick, user.nick
                        );
                        file_has_error = true;
                    }
                    if file_has_error {
                        has_errors = true;
                    }
                    // Check lat [-90, 90] and lng [-180, 180] bounds
                    if user.coordinates[0] < -90.0 || user.coordinates[0] > 90.0 ||
                       user.coordinates[1] < -180.0 || user.coordinates[1] > 180.0 {
                        println!("❌ Error in {}: Coordinates out of bounds.", file_name);
                        has_errors = true;
                    } else {
                        valid_users.push(user);
                    }
                }
                Err(e) => {
                    println!("Error in {}: {}",file_name,e);
                    has_errors=true;
                }
            }
        }
    }


    if has_errors {
        Err("Validation failed, Please fix the errors listed above.".to_string())
    }
    else{
        Ok(valid_users)
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    let cli = Cli::parse();
    match &cli.command {
        Commands::Validate => {
            println!("Running validation check....");

            match parse_all_users() {
                Ok(users) =>{
                    println!("All {} files are valid!", users.len());
                }
                Err(e) => {
                    eprintln!("{}",e);
                    std::process::exit(1);
                }
                
            }
        }
        Commands::Build => {
            println!("Building bundle...");

            match parse_all_users() {
                Ok (users ) => {
                    let all_users_json = serde_json::to_string_pretty(&users).unwrap();
                    fs::create_dir_all("website").unwrap();
                    let _ =std::fs::write("website/users.json", all_users_json);
                    println!("✅ Build Successful! {} users written to website/users.json", users.len());
                }
                Err(e) => {
                    eprintln!("❌ Build Failed: {}",e);
                    std::process::exit(1);
                }
                
            }
        }
    }

  



    Ok(())
}
