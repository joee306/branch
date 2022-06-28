use std::fs;
use directories::ProjectDirs;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub path : String,
    pub deepnis : u128,
    pub mode : u8,
    pub hidden : bool,
}

pub fn new() {
    if let Some(proj_dirs) = 
        ProjectDirs::from("dev", "Jbranch" , "branch")
    {
        let config_dir = proj_dirs.config_dir();
        
        let config_file = fs::read_to_string(
            config_dir.join("branch.toml"),
        );

        let config: Config = match config_file {
            Ok(file) => toml::from_str(&file).unwrap(),
            Err(_) => Config {
                path : "./".to_string(),
                deepnis : 2,
                mode : 0,
                hidden : false,
            },
        };
        dbg!(config);
    }
}