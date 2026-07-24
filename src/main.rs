
mod cli;
use cli::{Cli, Commands, parse_all_users};

use serde_json;
use std::fs;
use clap::Parser;





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
