use termion::{color, style, color::Fg, cursor};
use termion::event::{Key};
use termion::input::{TermRead};
use termion::raw::IntoRawMode;
use std::io::{Write, stdout, stdin};


use crate::repo::package::PackageInfoList;


pub fn package_selection(packages: &PackageInfoList, load_error: Option<Box<std::error::Error>>) -> Vec<i32>  {

    let num_package = packages.len();
    for i in 0..num_package {
        let p = &packages[i];
        println!(
            " {select_index} {source}/{packagename} {version}",
            select_index=(num_package - i),
            source=style_for_source(&p.source),
            packagename=style(&p.name, Fg(color::LightCyan)),
            version=style(&p.version, Fg(color::Yellow)),
        );
        println!("\t{}", p.description);
    }
    if let Some(err) = load_error{
        eprintln!("Error: {}", err);
    }
    handle_input()
}

fn style<T: std::fmt::Display>(input: &str, sty: T) -> String {
    format!(
        "{}{}{}",
        sty,
        input,
        style::Reset
    )
}

fn style_for_source(source: &str) -> String {
    match source{
        "aur" => style(source, Fg(color::LightGreen)),
        "core" => style(source, Fg(color::Magenta)),
        "extra" => style(source, Fg(color::Blue)),
        "community" => style(source, Fg(color::Cyan)),
        "local" => style(source, Fg(color::LightBlack)),
        _ => "".to_string(),
    }
}

fn handle_input() -> Vec<i32> {
    let poiner = "=> ";


    println!("{}{}Select package with arrow keys, or enter items seperated by space (e.g. 1 2 3){}",poiner, Fg(color::Red), style::Reset);
    println!("{}{}[Q] to quit, [Enter] to confirm.{}",poiner, Fg(color::Red), style::Reset);


    let buffer_lines = 4; //bottom of terminal offset
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();    
    let (_,height) = termion::terminal_size().expect("Failed to get size");

    let x = 0;
    let mut y = height - buffer_lines;
    let mut num_buf = String::new();
    let mut cursor_index = 1;
    let mut last_char = ' ';

    write!(stdout,"{}{}",cursor::Goto(0,height), poiner).unwrap();

    cursor::Goto(x,y);
    write!(stdout,"{}{}→",cursor::Hide, cursor::Goto(x,y)).unwrap();
    stdout.flush().unwrap();

    for c in stdin.keys() {
        match c.unwrap() {
            
            Key::Char('q') => {
                write!(stdout,"{}\n",cursor::Goto(0,height+buffer_lines)).unwrap();
                stdout.flush().unwrap();
                return Vec::new();
            },
            Key::Up => {
                write!(stdout,"{} ",cursor::Goto(x,y)).unwrap();
                if y > 2{
                    y -= 2;
                    cursor_index += 1;
                }
            },
            Key::Down => {
                write!(stdout,"{} ",cursor::Goto(x,y)).unwrap();
                if y < height - buffer_lines{
                    y += 2;
                    cursor_index-=1;
                }
            },
            Key::Backspace => {
                match  num_buf.pop(){
                    Some(c) => {
                        last_char = c;
                        write!(stdout,"{}{}{}{}",cursor::Goto(0,height), termion::clear::CurrentLine,poiner, &num_buf).unwrap();
                    },
                    None => {},
                }
            },
            Key::Char(c) =>{
                if c.is_numeric() {
                    num_buf.push(c);
                    write!(stdout,"{} ",cursor::Goto(x,y)).unwrap();
                    write!(stdout,"{}{}{}",cursor::Goto(0,height), poiner, &num_buf).unwrap();
                    last_char = c;
                }else if c == ' ' &&  num_buf.len() > 0 && last_char != ' '{
                    num_buf.push(' ');
                    last_char = ' ';
                } else if c == '\n'{
                    break;
                }
            },
            _ => {
                
            }
        }
        if num_buf.len() == 0{
            write!(stdout,"{}{}→",cursor::Hide, cursor::Goto(x,y)).unwrap();
        }else{
            write!(stdout,"{}{}",cursor::Show, cursor::Goto((poiner.len() + num_buf.len()+1) as u16, height+buffer_lines-1)).unwrap();
        }
        
        stdout.flush().unwrap();
    }


    write!(stdout,"{}\n",cursor::Goto(0,height+buffer_lines+1)).unwrap();
    stdout.flush().unwrap();
    
    if num_buf.len() > 0 {
        num_buf.split(' ').map(|s| s.parse().unwrap_or(-1)).collect() 
    }else{
        vec![cursor_index]
    }
}