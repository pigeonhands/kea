pub mod kea;
pub use crate::app::kea::{Kea,Result};

pub fn start(kea: &Kea) -> Result<()>{
    
    let mut print_help = true;

    if kea.matches.is_present("upgrade") {
        print_help = false;
        kea.update();
    }
    
    match kea.matches.value_of("package"){
        Some(query) => kea.install(query)?,
        None => if print_help{ println!("{}", kea.help_string) },
    };

    Ok(())
}