pub mod kea;
pub use crate::app::kea::{Kea,Result};
use termion::{color, color::Fg, style};
use alpm_rs::question::*;
use alpm_rs::List;
use crate::utils::terminal;


pub fn start(kea: &Kea) -> Result<()>{
    
    register_callbacks(&kea.alpm);

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

fn register_callbacks(alpm: &alpm_rs::Handle) {
    alpm_rs::callbacks::set_log_callback(&alpm, |level,message| {
        if level > alpm_rs::enums::ALPM_LOG_WARNING{
            return;
        }
        let level_text = match level {
            alpm_rs::enums::ALPM_LOG_ERROR => format!("{}error{}", Fg(color::Red), style::Reset),
            alpm_rs::enums::ALPM_LOG_WARNING => format!("{}warn{}", Fg(color::Yellow), style::Reset),
            alpm_rs::enums::ALPM_LOG_DEBUG => "debug".to_string(),
            alpm_rs::enums::ALPM_LOG_FUNCTION => "func".to_string(),
            _ => "?".to_string(),
        };
        print!("alpm] ({}) {}", level_text, message);
    });

    alpm_rs::callbacks::set_download_callback(&alpm, |file,xfered,total|{
        println!("Downloading {} (%{} - {} bytes remaining)", file, (xfered/total)*100, total-xfered);
    });

    alpm_rs::callbacks::set_question_callback(&alpm, |q|{

        if let Question::InstallIgnorePkg(_) = q.question{
            q.set_answer(1);
            return;
        }

        if let Question::SelectProvider(p) = &q.question {
            let pkgs : Vec<alpm_rs::package::Package> = p.providers.iter().collect();
            loop {
                let num_package = pkgs.len();
                for (i, pkg) in pkgs.iter().enumerate(){
                    println!("{}] {}", (num_package - i), pkg.name());
                    println!("\t {}", pkg.url());
                }
                let selection = terminal::handle_input(false);
                if selection.len() != 1{
                    continue;
                }
                let provider_index = selection[0];
                if provider_index >= 0 && (provider_index as usize) < pkgs.len(){
                    q.set_answer(provider_index);
                    return;
                }
            }
        }


    });
}