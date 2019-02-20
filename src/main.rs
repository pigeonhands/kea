use clap::{Arg, App, SubCommand};

use std::error::Error;

fn try_main() -> Result<(), Box<Error>> {
    let matches = App::new("kia")
                    .about("Rust aur client for arch linux")
                    .author("Sam M.")
                    .version(env!("CARGO_PKG_VERSION"))
        .arg(Arg::with_name("package")
            .index(1)
            .help("package to search aur for")
            .required(true))
            .get_matches();

    
    match matches.subcommand(){
        ("remove", Some(remove))=>{

        },
       _ => { //search aur
            let package = matches.value_of("package").unwrap();
            println!("{}", package)
        }
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
