mod aur;
mod kea;
mod repo;
mod utils;
use crate::aur::callbacks;
use crate::utils::sys;
use kea::Kea;

use std::error::Error;
extern crate serde;
use clap::{App, Arg};
use utils::conf::Config;

type Result<T> = std::result::Result<T, Box<Error>>;

fn main() {
    if let Some(e) = try_main().err() {
        eprintln!("Error! {}", e);
    }
}

fn try_main() -> Result<()> {
    let mut app = App::new("kia")
        .about("Package manager for arch linux")
        .author("Sam M.")
        .version(env!("CARGO_PKG_VERSION"))
        .arg(
            Arg::with_name("gen-conf")
                .long("gen-conf")
                .takes_value(false)
                .help("generates a new kea config file in ~/.kea"),
        )
        .arg(
            Arg::with_name("local")
                .long("local")
                .help("searches local cashes only."),
        )
        .arg(Arg::with_name("aur").long("aur").help("Searches aur only"))
        .arg(
            Arg::with_name("sync")
                .short("-s")
                .long("sync")
                .help("Sync databases"),
        )
        .arg(
            Arg::with_name("upgrade")
                .short("-u")
                .long("upgrade")
                .help("upgrade packages"),
        )
        .arg(
            Arg::with_name("package")
                .index(1)
                .help("package to search for")
                .required(false),
        );

    let help = {
        let mut v = Vec::new();
        app.write_long_help(&mut v)?;
        String::from_utf8(v)?
    };

    let matches = app.get_matches();

    let cfg = if matches.is_present("gen-conf") {
        gen_config()?
    } else {
        load_config()?
    };

    let kea = Kea {
        matches: matches,
        alpm: init_alpm(&cfg)?,
        config: cfg,
        help_string: help,
    };

    start_with_kea(&kea)
}

fn init_alpm(cfg: &Config) -> Result<alpm_rs::Handle> {
    let alpm = alpm_rs::initialize(&cfg.alpm.root_dir, &cfg.alpm.db_path)?;

    for dbname in &cfg.packages.sync_dbs {
        let db = alpm.register_syncdb(&dbname, 0);
        if let Some(servers) = cfg.sources.pkg_sources.get(dbname) {
            for s in servers {
                if !db.add_server(&s) {
                    eprintln!("{}] Failed to add server {}", dbname, &s);
                }
            }
        }
    }
    Ok(alpm)
}

fn gen_config() -> Result<Config> {
    let config = Config::default();
    config.save()?;
    Ok(config)
}
fn load_config() -> Result<Config> {
    Config::load().or_else(|e| {
        eprintln!("Failed to load config.");
        eprintln!("{}", e);
        eprintln!("use --gen-conf to make a new one.");
        Err(e)
    })
}

pub fn start_with_kea(kea: &Kea) -> Result<()> {
    callbacks::register_callbacks(&kea.alpm);

    let mut _print_help = true;

    if kea.matches.is_present("sync") {
        if !sys::is_root() {
            return Err("Sync (-s, --sync) requires root.".into());
        }
        _print_help = false;
        kea.update_dbs();
    }

    if kea.matches.is_present("upgrade") {
        kea.update_packages();
    }

    match kea.matches.value_of("package") {
        Some(query) => kea.install(query)?,
        None => {
            if _print_help {
                println!("Nothing to do.")
            }
        }
    };

    if !kea.alpm.release() {
        eprintln!("Failed to release apm handle. {:?}", kea.alpm.error_no());
    }
    Ok(())
}
