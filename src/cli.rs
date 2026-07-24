use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs};
use clap::{Parser, Subcommand};


#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub username: String,
    pub coordinates: [f64; 2],
    #[serde(default)]
    pub links: HashMap<String, String>,
}

#[derive(Parser)]
#[command(name = "map-builder")]
#[command(about = "Validates user JSON submissions and builds the final map file", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}
#[derive(Subcommand)]
pub enum Commands {
    Validate,
    Build,
}



pub fn parse_all_users() -> Result<Vec<User>, String> {

    let mut valid_users = Vec::new();
    let mut has_errors = false;

    
    let dir = match fs::read_dir("user") {
        Ok(d) => d,
        Err(_) => return Err("Failed to read the 'user' directory".into()),
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
                    match validate_user(&user, &file_stem) {
                        Ok(_) => {}
                        Err(e) => {
                            println!("❌ Error in {}: {}", file_name, e);
                            has_errors = true;
                        }
                    }
                    valid_users.push(user);
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


pub fn validate_user(user: &User, file_stem: &str) -> Result<(), String> {

    if !user.username.eq_ignore_ascii_case(&file_stem) {
        return Err(format!(
            "Username '{}' does not match the filename '{}.",
            user.username, file_stem
        ));
    }
    

    let lat = user.coordinates[0];
    let lng = user.coordinates[1];
    if lat < -90.0 || lat > 90.0 || lng < -180.0 || lng > 180.0 {
        return Err(format!(
            "Coordinates out of bounds: [{}, {}]",
            lat, lng
        ));
    }
    Ok(())
}




#[cfg(test)]
mod tests{

    use super::*;

    // # helper: build a User quickly
    fn make_user(username: &str, lat: f64, lng: f64) -> User {
        User {
            username: username.to_string(),
            coordinates: [lat, lng],
            links: HashMap::new(),
        }
    }

    #[test]
    fn valid_user_passes(){
        let u = make_user("dtolnay", 40.0, -74.0);
        assert!(validate_user(&u, "dtolnay").is_ok());
    }

    #[test]
    fn username_mismatch_fails(){
        let u = make_user("alice", 0.0, 0.0);
        assert!(validate_user(&u, "bob").is_err());
    }

    #[test]
    fn username_case_insensitive_ok(){
        let u = make_user("DTolnay", 0.0, 0.0);
        assert!(validate_user(&u, "dtolnay").is_ok());
    }

    #[test]
    fn latitude_out_of_range_fails(){
        let u = make_user("x", 95.0, 0.0);
        assert!(validate_user(&u, "x").is_err());
    }

    #[test]
    fn longitude_out_of_range_fails(){
        let u = make_user("x", 0.0, -200.0);
        assert!(validate_user(&u, "x").is_err());
    }

    #[test]
    fn bad_json_fails_to_parse(){
        let result = serde_json::from_str::<User>("{ not valid json }");
        assert!(result.is_err());
    }

    #[test]
    fn good_json_parses(){
        let json = r#"{ "username": "xq", "coordinates": [1.0, 2.0] }"#;
        let user = serde_json::from_str::<User>(json).unwrap();
        assert_eq!(user.username, "xq");
    }
}