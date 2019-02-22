use crossterm::{Color, style, StyledObject};
use crate::repo::package::PackageInfoList;


fn style_for_source(source: &str) -> StyledObject<&str> {
    style(source).with(
    match source {
        "aur" => Color::DarkGreen,
        "core" => Color::Magenta,
        "extra" => Color::DarkBlue,
        "community" => Color::DarkYellow,

        "local" => Color::Grey,
        _ => Color::Green,
    })
}

pub fn package_selection(packages: PackageInfoList, load_error: Option<Box<std::error::Error>>) -> i32 {

    let num_package = packages.len();
    for i in 0..num_package {
        let p = &packages[i];
        println!(
            "{select_index} {source}/{packagename} {version}",
            select_index=(num_package - i),
            source=style_for_source(&p.source),
            packagename=style(&p.name).with(Color::DarkCyan),
            version=style(&p.version).with(Color::Yellow),
        );
        println!("\t{}", p.description);
    }
    if let Some(err) = load_error{
        eprintln!("Error: {}", err);
    }
    0

}