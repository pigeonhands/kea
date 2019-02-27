use crate::utils::terminal;
use alpm_rs::question::*;
use termion::{color, color::Fg, style};

pub fn register_callbacks(alpm: &alpm_rs::Handle) {
    alpm_rs::callbacks::set_log_callback(&alpm, |level, message| {
        if level > alpm_rs::enums::ALPM_LOG_WARNING {
            return;
        }
        let level_text = match level {
            alpm_rs::enums::ALPM_LOG_ERROR => format!("{}error{}", Fg(color::Red), style::Reset),
            alpm_rs::enums::ALPM_LOG_WARNING => {
                format!("{}warn{}", Fg(color::Yellow), style::Reset)
            }
            alpm_rs::enums::ALPM_LOG_DEBUG => "debug".to_string(),
            alpm_rs::enums::ALPM_LOG_FUNCTION => "func".to_string(),
            _ => "?".to_string(),
        };
        print!("alpm] ({}) {}", level_text, message);
    });

    alpm_rs::callbacks::set_download_callback(&alpm, |file, xfered, total| {
        println!(
            "Downloading {} (%{} - {} bytes remaining)",
            file,
            (xfered / total) * 100,
            total - xfered
        );
    });

    alpm_rs::callbacks::set_question_callback(&alpm, question_callback);
}

fn question_callback(q: QuestionArgs) {
    match &q.question {
        Question::InstallIgnorePkg(_) => {
            q.set_answer(1);
        }
        Question::Conflict(c) => {
            println!(
                "Conflicting packages {} and {}. ({}) Remove conflict?",
                c.conflict.package1, c.conflict.package2, c.conflict.reason.name
            );
            print!("[Y]es/[n]o: ");
            if terminal::handle_yes_no(true) {
                q.set_answer(1);
            }
        }
        Question::SelectProvider(p) => {
            let pkgs: Vec<alpm_rs::package::Package> = p.providers.iter().collect();
            loop {
                let num_package = pkgs.len();
                for (i, pkg) in pkgs.iter().enumerate() {
                    println!("{}] {}", (num_package - i), pkg.name());
                    println!("\t {}", pkg.url());
                }
                let selection = terminal::handle_input(false);
                if selection.len() != 1 {
                    continue;
                }
                let provider_index = selection[0];
                if provider_index >= 0 && (provider_index as usize) < pkgs.len() {
                    q.set_answer(provider_index);
                    return;
                }
            }
        }
        _ => {
            //other questions
        }
    }
}
