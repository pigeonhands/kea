use std::iter::FromIterator;

use alpm_rs;
use aur_client::aur;

#[derive(Clone)]
pub struct PackageInfo {
    pub name: String,
    pub source: String,
    pub description: String,
    pub version: String,
    pub votes: i32,
    pub poplarity: f64,
}

impl From<aur::Package> for PackageInfo {
    fn from(p: aur::Package) -> PackageInfo {
        PackageInfo {
            name: p.Name,
            source: "aur".to_string(),
            description: p.Description.unwrap_or("-".to_string()),
            version: p.Version,
            votes: p.NumVotes,
            poplarity: p.Popularity,
        }
    }
}

impl From<alpm_rs::package::Package> for PackageInfo {
    fn from(p: alpm_rs::package::Package) -> PackageInfo {
        PackageInfo {
            name: p.name().to_string(),
            source: p.db().name().to_string(),
            description: p.description().to_string(),
            version: p.version().to_string(),
            votes: 0,
            poplarity: 0.0,
        }
    }
}

pub struct PackageInfoList {
    pub pkgs: Vec<PackageInfo>,
}

impl PackageInfoList {
    pub fn len(&self) -> usize {
        self.pkgs.len()
    }

    pub fn push(&mut self, p: PackageInfo) {
        self.pkgs.push(p);
    }

    pub fn merge(&mut self, lst: PackageInfoList) {
        for p in lst.pkgs {
            self.pkgs.push(p);
        }
    }

    #[allow(dead_code)]
    pub fn pick(&mut self, ammount: usize) -> PackageInfoList {
        PackageInfoList {
            pkgs: self.pkgs[0..ammount].to_vec(),
        }
    }

    pub fn get(&mut self, index: usize) -> PackageInfo {
        self.pkgs[index].clone()
    }
}

impl Default for PackageInfoList {
    fn default() -> PackageInfoList {
        PackageInfoList { pkgs: Vec::new() }
    }
}

impl std::ops::Index<usize> for PackageInfoList {
    type Output = PackageInfo;

    fn index(&self, i: usize) -> &Self::Output {
        &self.pkgs[i]
    }
}

impl From<Vec<aur::Package>> for PackageInfoList {
    fn from(pkgs: Vec<aur::Package>) -> PackageInfoList {
        PackageInfoList::from_iter(pkgs)
    }
}

impl FromIterator<aur::Package> for PackageInfoList {
    fn from_iter<I: IntoIterator<Item = aur::Package>>(iter: I) -> Self {
        let mut pl = PackageInfoList { pkgs: Vec::new() };
        for p in iter {
            pl.pkgs.push(p.into());
        }
        pl
    }
}

impl FromIterator<alpm_rs::package::Package> for PackageInfoList {
    fn from_iter<I: IntoIterator<Item = alpm_rs::package::Package>>(iter: I) -> Self {
        let mut pl = PackageInfoList { pkgs: Vec::new() };
        for p in iter {
            pl.pkgs.push(p.into());
        }
        pl
    }
}
