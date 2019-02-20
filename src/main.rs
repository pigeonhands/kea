use clap::{Arg, App, SubCommand};
use aur_client::aur;

use std::error::Error;

fn try_main() -> Result<(), Box<Error>> {
    let matches = make_app().get_matches();

    
    match matches.subcommand(){
        ("remove", Some(remove))=>{
            
        },
       _ => { //search aur
            if let Some(package) = matches.value_of("package"){
                let pkgs = aur::search(package)?;
                if let Some(err) = pkgs.error {
                    eprintln!("Error: {}", err);
                }
                for p in pkgs.results {
                    println!("{}", p.Name);
                    println!("{}", p.URLPath);
                    println!("{}", p.URL.unwrap_or("".to_string()));
                    println!("");
                }
            }else{

            }
        }
    }

    Ok(())
}

fn make_app() -> App<'static,'static> {
    App::new("kia")
        .about("Rust aur client for arch linux")
        .author("Sam M.")
        .version(env!("CARGO_PKG_VERSION"))

    .arg(Arg::with_name("package")
        .index(1)
        .help("package to search aur for")
        .required(false))
}

fn main() {
    match try_main(){
        Ok(()) => {},
        Err(e) => {
            eprintln!("Error! {}", e);
        }
    }
}
