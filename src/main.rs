mod utils;
mod repo;
mod app;

extern crate serde;
use utils::conf::Config;

use clap::{Arg, App};

use std::error::Error;

use crate::app::{Kea};

type Result<T> = std::result::Result<T,Box<Error>>;

fn main() {
    match try_main(){
        Ok(()) => {},
        Err(e) => {
            eprintln!("Error! {}", e);
        }
    }
}

fn try_main() -> Result<()>{
    let mut app =  App::new("kia")
        .about("Package manager for arch linux")
        .author("Sam M.")
        .version(env!("CARGO_PKG_VERSION"))
    .arg(Arg::with_name("gen-conf")
        .long("gen-conf")
        .takes_value(false)
        .help("generates a new kea config file in ~/.kea"))
    .arg(Arg::with_name("local")
        .long("local")
        .help("searches local cashes only."))
    .arg(Arg::with_name("aur")
        .long("aur")
        .help("Searches aur only"))
    .arg(Arg::with_name("upgrade")
        .short("-u")
        .long("upgrade")
        .help("upgrade packages"))
    .arg(Arg::with_name("package")
        .index(1)
        .help("package to search for")
        .required(false));

    let help = {
        let mut v = Vec::new();
        app.write_long_help(&mut v)?;
        String::from_utf8(v)?
    };

    let matches = app.get_matches();
    
    let cfg = if matches.is_present("gen-conf"){
        gen_config()?
    }else{
        load_config()?
    };

    let kea = Kea{
        matches: matches,
        alpm: init_alpm(&cfg.alpm.root_dir, &cfg.alpm.db_path, &cfg.packages.sync_dbs)?,
        config: cfg,
        help_string: help,
    };

    app::start(&kea)?;
    Ok(())
}

fn init_alpm(root: &str, db_path: &str, sync_dbs: &Vec<String>) -> Result<alpm_rs::Handle> {
    let alpm = alpm_rs::initialize(root, db_path)?;

    for db in sync_dbs {
        alpm.register_syncdb(&db, 0);
    }

    Ok(alpm)
}

fn gen_config() -> Result<Config>{
    let config = Config::default();
    config.save()?;
    Ok(config)
}
fn load_config() -> Result<Config>{
    match Config::load() {
        Err(e) => {
            eprintln!("Failed to load config. use --gen-conf to make a new one.");
            return Err(e);
        },
        Ok(c) => Ok(c),
    }
}





