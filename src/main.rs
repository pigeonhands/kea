mod utils;
mod repo;

extern crate serde;
use utils::conf::Config;

use clap::{Arg, App, ArgMatches};

use std::error::Error;
use crate::utils::terminal;
use crate::repo::package::PackageInfoList;
use crate::repo::search;

fn make_app() -> App<'static,'static> {
    App::new("kia")
        .about("Rust aur client for arch linux")
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
}

fn try_main() -> Result<(), Box<Error>> {
    let matches = make_app().get_matches();

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

    let alpm = alpm_rs::initialize(&cfg.alpm.root_dir, &cfg.alpm.db_path)?;

    for db in &cfg.packages.sync_dbs {
        alpm.register_syncdb(&db, 0);
    }

   
    
    match matches.subcommand(){
        ("remove", Some(_remove))=>{
            
        },
       _ => { //search aur
            match matches.value_of("package"){
                None => {},
                Some(package) => {
                    handle_search(&alpm, &matches, &cfg, package)?;
                },
            }

        }
    }

    Ok(())
}

fn handle_search(alpm: &alpm_rs::Handle, matches: &ArgMatches, cfg: &Config, query: &str) -> Result<(), Box<Error>> {

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

    terminal::package_selection(pkgs, aur_error);
    Ok(())
}



fn main() {
    match try_main(){
        Ok(()) => {},
        Err(e) => {
            eprintln!("Error! {}", e);
        }
    }
}
