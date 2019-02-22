mod utils;
mod repo;

extern crate serde;
use std::cmp::Ordering;
use utils::conf::Config;

use clap::{Arg, App, SubCommand, ArgMatches};
use aur_client::aur;
use regex::Regex;

use std::error::Error;
use crate::utils::terminal;
use crate::repo::package::PackageInfoList;

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
        .short("l")
        .long("local")
        .help("searches local cashe only. Does not search aur"))

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
        ("remove", Some(remove))=>{
            
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

fn handle_search(alpm: &alpm_rs::Handle, matches: &ArgMatches, cfg: &Config, search: &str) -> Result<(), Box<Error>> {
    let dbs = alpm.sync_dbs();

    let re = Regex::new(search)?;
    let mut pkgs = PackageInfoList::default();
    let mut aur_error: Option<Box<Error>> = None;
    if !matches.is_present("local"){
        match aur::search(search){
            Err(e) => aur_error = Some(e),
            Ok(mut aur_pkgs) => {
                aur_pkgs.results
                    .sort_by(|a,b| 
                        a.Popularity
                        .partial_cmp(&b.Popularity).unwrap());
                pkgs.merge(aur_pkgs.results.into());
            }
        }
        
        for db in dbs.iter() {
            for p in db.pkgcache(){
                if  re.is_match(p.name()){
                    pkgs.push(p.into());
                }
            }
        }
        terminal::package_selection(pkgs, aur_error);
    }
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
