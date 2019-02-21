use crossterm::{Color, style};
use crate::repo::package::PackageInfoList;


pub fn package_selection(packages: PackageInfoList) -> i32 {

    let num_package = packages.len();
    for i in 0..num_package {
        let p = &packages[i];
        println!(
            "{select_index} {source}/{packagename} {version}",
            select_index=(num_package - i),
            source=p.source,
            packagename=p.name,
            version=p.version
        );
        println!("\t{}", p.description);
    }
    0

}