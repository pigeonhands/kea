mod utils;
mod repo;

extern crate serde;
use utils::conf::Config;

use clap::{Arg, App, ArgMatches};

use std::error::Error;
use crate::utils::terminal;
use crate::repo::package::PackageInfoList;
use crate::repo::search;

type Result<T> = std::result::Result<T,Box<Error>>;

fn try_main() -> Result<()>{
    let matches = App::new("kia")
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
    
    .arg(Arg::with_name("package")
        .index(1)
        .help("package to search aur for")
        .required(false))
        .get_matches();

    let cfg = get_config(&matches)?;
    let alpm = alpm_rs::initialize(&cfg.alpm.root_dir, &cfg.alpm.db_path)?;

    for db in &cfg.packages.sync_dbs {
        alpm.register_syncdb(&db, 0);
    }


    match matches.value_of("package"){
        Some(package) => {
            let selected = handle_package_selection(&alpm, &matches, &cfg, package)?;
            for p in selected.pkgs{
                println!("{}", p.name);
            }
        },
        None => {

        },
    }

    Ok(())
}

fn get_config(matches: &ArgMatches) -> Result<Config> {
    let mut cfg : Config;

    if matches.is_present("gen-conf"){
        cfg = Config::default();
        cfg.save()?;
    }else{
        match Config::load() {
            Err(e) => {
                eprintln!("Failed to load config. use --gen-conf to make a new one.");
                return Err(e);
            },
            Ok(c) => cfg = c,
        }
    }

    Ok(cfg)
}

fn handle_package_selection(alpm: &alpm_rs::Handle, matches: &ArgMatches, cfg: &Config, query: &str) -> Result<PackageInfoList> {

    let mut pkgs = PackageInfoList::default();
    let mut aur_error: Option<Box<Error>> = None;

    if !matches.is_present("aur"){
        pkgs.merge(search::search_pkg_cashe(alpm, query));
    }
    
    if pkgs.len() <= cfg.packages.search_aur_local_under && !matches.is_present("local"){
        match search::search_aur(query){
            Ok(mut aur_pkgs) => {
                 pkgs = {
                    aur_pkgs.merge(pkgs);
                    aur_pkgs //Make aur packages show last
                }
            },
            Err(e) => aur_error = Some(e),
        }
    }

    let mut selected_packages = PackageInfoList::default();
    let input_indexes = terminal::package_selection(&pkgs, aur_error);
    for p in input_indexes {
        let pind = p as usize;
        if p >= 0 && pind < pkgs.len(){
            selected_packages.push(pkgs.get(pkgs.len() - pind));
        }
    }
    Ok(selected_packages)
}




fn main() {
    match try_main(){
        Ok(()) => {},
        Err(e) => {
            eprintln!("Error! {}", e);
        }
    }
}
