use aur_client::aur;
use std::error::Error;
use alpm_rs::Handle;

use crate::repo::package::PackageInfoList;

pub fn search_aur(search: &str) -> Result<PackageInfoList, Box<Error>> {
    let mut aur_pkgs = aur::search(search)?;
    aur_pkgs.results
         .sort_by(|a,b| 
             a.Popularity
             .partial_cmp(&b.Popularity).unwrap());
    Ok(aur_pkgs.results.into())
}

pub fn search_pkg_cashe(alpm: &Handle, query: &str) -> PackageInfoList {
    let mut pkgs = PackageInfoList::default();

    for db in alpm.sync_dbs(){
        for p in db.search_one(query){
            pkgs.push(p.into());
        }
    }
    pkgs
}