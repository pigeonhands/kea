use crate::repo::package::PackageInfoList;
use alpm_rs::package::Package;
use alpm_rs::Handle;
use std::cmp;

use aur_client::aur;

pub fn get_outdated_pkgs(alpm: &Handle) -> (PackageInfoList, PackageInfoList) {
    let (alpm_pkgs, aur_pkgs) = get_pkgs(&alpm);
    let mut alpm_outdated = PackageInfoList::default();
    let mut aur_outdated = PackageInfoList::default();

    let dbs = alpm.sync_dbs();

    for p in alpm_pkgs {
        if let Some(_) = p.newer_version(&dbs) {
            alpm_outdated.push(p.into());
        }
    }

    let max_search_pkgs = 30;
    let mut current_pkg = 0;
    let aur_pkg_names: Vec<&str> = aur_pkgs.iter().map(|p| p.name()).collect();

    while current_pkg < aur_pkgs.len() {
        let pkgs_left = aur_pkgs.len() - current_pkg;
        let end_pkg = cmp::min(pkgs_left, max_search_pkgs) + current_pkg;

        let info_ret = aur::info(&aur_pkg_names[current_pkg..current_pkg + end_pkg]);
        current_pkg += max_search_pkgs;

        match info_ret {
            Err(e) => {
                println!("error: {}", e);
                break;
            }
            Ok(resp) => {
                if let Some(e) = resp.error {
                    println!("aur error: {}", e);
                    break;
                }

                for pkg in resp.results {
                    if let Some(local_pkg) = aur_pkgs.iter().find(|p| p.name() == &pkg.Name) {
                        if Package::vercmp(local_pkg.version(), &pkg.Version) < 0 {
                            aur_outdated.push(pkg.into());
                        }
                    } else {
                        println!("Error. Failed to re-find {}", &pkg.Name);
                    }
                }
            }
        }
    }
    (alpm_outdated, aur_outdated)
}

pub fn get_pkgs(alpm: &Handle) -> (Vec<Package>, Vec<Package>) {
    let mut alpm_pkgs = Vec::new();
    let mut aur_pkgs = Vec::new();

    let local_db = alpm.local_db();
    let alpm_dbs = alpm.sync_dbs();

    for p in local_db.pkgcache() {
        let mut found = false;
        for alpm_db in alpm_dbs.iter() {
            if let Some(got_pkg) = alpm_db.get_pkg(p.name()) {
                alpm_pkgs.push(got_pkg.into());
                found = true;
                break;
            }
        }
        if !found {
            aur_pkgs.push(p);
        }
    }

    (alpm_pkgs, aur_pkgs)
}
