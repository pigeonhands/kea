extern crate toml;
use std::path::PathBuf;
use serde::{Serialize, Deserialize};
use std::error::Error;
use std::fs;
use pacman::pacman_conf;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config{
    pub display : DisplayConfig,
    pub packages: PackagesConfig,
    pub alpm : AlpmConfig,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DisplayConfig{
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PackagesConfig{
    pub use_aur: bool,
    pub sync_dbs: Vec<String>,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct AlpmConfig{
    pub root_dir: String,
    pub db_path : String,
    pub architecture: String,
    pub ignore_pkg: Vec<String>,
    pub ignore_group: Vec<String>,
}


impl Default for Config{
    fn default() -> Self{
        let pmcfg = pacman_conf::get_config();
        Config{
            display: DisplayConfig{
            },
            packages: PackagesConfig{
                use_aur: true,
                sync_dbs: pmcfg.repos.iter().map(|r| r.name.clone()).collect(),
            },
            alpm: AlpmConfig{
                root_dir: pmcfg.root_dir,
                db_path: pmcfg.db_path,
                architecture: pmcfg.architecture,
                ignore_pkg: pmcfg.ignore_pkg,
                ignore_group: pmcfg.ignore_group,
            },
        }
    }
}

impl Config{
    fn config_path() -> PathBuf {
         dirs::home_dir()
                .expect("cant get home dir")
                .join(".kea")
    }
    pub fn load() -> Result<Self, Box<Error>>{

        let cfg_path = Config::config_path();

        
        if !cfg_path.exists(){
            let cfg = Config::default();
            cfg.save();
            return Ok(cfg);
        }

        let cfg_content = fs::read_to_string(cfg_path)?;

        let cfg : Config = toml::from_str(&cfg_content)?;
        Ok(cfg)
    }

    pub fn save(&self) -> Result<(), Box<Error>>{
        let cfg_str = toml::to_string(&self)?;
        fs::write(Config::config_path(), cfg_str)?;
        Ok(())
    }
}


