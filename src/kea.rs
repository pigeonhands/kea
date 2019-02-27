

extern crate serde;


use clap::{ArgMatches};

use std::error::Error;

use crate::utils::conf::Config;
use crate::utils::terminal;
use crate::repo::package::PackageInfoList;
use crate::repo::{search,upgrade};


pub struct Kea {
    pub matches: ArgMatches<'static>,
    pub alpm: alpm_rs::Handle,
    pub config: Config,
    pub help_string: String,
}
pub type Result<T> = std::result::Result<T, Box<Error>>;

impl Kea {
    pub fn update_packages(&self) {
        let (alpm_pkgs, aur_pkgs) = upgrade::get_outdated_pkgs(&self.alpm);

        if alpm_pkgs.len() > 0 {
            terminal::print_pkg_list("alpm", &alpm_pkgs);
        }

        if aur_pkgs.len() > 0{
            terminal::print_pkg_list("aur", &aur_pkgs);
        }

        if alpm_pkgs.len() == 0 && aur_pkgs.len() == 0 {
            println!("Everything is up to date.");
        } 
    }

    pub fn install(&self, query: &str) -> Result<()> {
        let selected = self.handle_package_selection(&query)?;
        for p in selected.pkgs{
            println!("{}", p.name);
        }
        Ok(())
    }


    fn handle_package_selection(&self, query: &str) -> Result<PackageInfoList> {

        let mut pkgs = PackageInfoList::default();
        let mut aur_error: Option<Box<Error>> = None;

        if !self.matches.is_present("aur"){
            pkgs.merge(search::search_pkg_cashe(&self.alpm, query));
        }
        
        if pkgs.len() <= self.config.packages.search_aur_local_under && !self.matches.is_present("local"){
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

        if pkgs.len() < 1{
            println!("No packages found.");
            return Ok(Default::default());
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

    pub fn update_dbs(&self) {
        let dbs = self.alpm.sync_dbs();
        println!("-> Syncing databases");
        for db in dbs.iter() {
            let res = db.update(false);
            if res < 0{
                println!("Failed update {}. {:?}",db.name(), self.alpm.error_no());
            }else if res == 1 {
                println!("{} up to date", db.name());
            }
        }
    }

    pub fn update_alpm_packages(&self) -> bool {
        if !self.alpm.trans_init(1){
            return false;
        }
        if !self.alpm.sys_upgrade(true){
            self.alpm.trans_release();
            return false;
        }
        
        false
        
    }
}