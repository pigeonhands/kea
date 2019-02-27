extern crate toml;
use std::path::PathBuf;
use serde::{Serialize, Deserialize};
use std::error::Error;
use std::fs;
use pacman::pacman_conf;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config{
    pub display : DisplayConfig,
    pub build : BuildConfig,
    pub packages: PackagesConfig,
    pub alpm : AlpmConfig,
    pub sources: Sources,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BuildConfig{
    build_dir : String,
    makepkg_path: String,
    keep_source : bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DisplayConfig{
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PackagesConfig{
    pub search_aur_local_under: usize,
    pub sync_dbs: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Sources{
    pub pkg_sources: HashMap<String,Vec<String>>,
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
        let mut sources = HashMap::new();
        for repo in &pmcfg.repos{
            sources.insert(repo.name.clone(), repo.servers.clone());
        }
        Config{
            display: DisplayConfig{
            },
            build: BuildConfig{
                build_dir: "~/.cache/kea".into(),
                makepkg_path: "makepkg".into(),
                keep_source: false,
            },
            packages: PackagesConfig{
                search_aur_local_under: 80,
                sync_dbs: pmcfg.repos.iter().map(|r| r.name.clone()).collect(),
            },
            alpm: AlpmConfig{
                root_dir: pmcfg.root_dir,
                db_path: pmcfg.db_path,
                architecture: pmcfg.architecture,
                ignore_pkg: pmcfg.ignore_pkg,
                ignore_group: pmcfg.ignore_group,
            },
            sources: Sources{
                pkg_sources: sources,
            }
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
            cfg.save()?;
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


