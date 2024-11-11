
use ansi_term::{Style,Color};

use crate::pkg::{general::store_file::GameCharacterData, output::color_management};

use super::super::general::lines::TextLine;

pub struct DialoguePrinter { 
    characters : Vec<GameCharacterData>,
    prev_char : String,
}
impl DialoguePrinter {
    pub fn new() -> DialoguePrinter {
        DialoguePrinter {
            characters : vec![],
            prev_char : String::new(),
        }
    }

    pub fn implement(&mut self, characters : Vec<GameCharacterData>) {
        self.characters = characters;
        self.prev_char = "".to_string();
    }

    pub fn print_dialogue_line(&mut self, line : &TextLine) { 
        if &line.name == "n" {
            if &line.name != &self.prev_char {
                println!();
            }

            println!( "{}",
                Style::new().italic().paint(&line.text)
            );
            self.prev_char = "n".to_string();
        }else {
            for char in &self.characters {
                if &char.short_name == &line.name || &char.full_name == &line.name {
                    if &char.short_name != &self.prev_char {
                        if &self.prev_char == "n" {
                            println!()
                        } 
                        // god this looks like garbage

                        println!(
                            "{} :",
                            color_management::get_color_by_rgb(char.c_r, char.c_g,char.c_b).paint(&char.full_name),
                        );
                        self.prev_char = char.short_name.clone();
                    }

                    println!(
                        "{}",
                        &line.text
                    );
                    return
                }
            }
            DialoguePrinter::print_error(
                format!("Character {} not found",&line.name)
            );
            println!(
                "{}",
                &line.text
            )
        }
    }
    
    pub fn cmd_executed(&mut self) {
        self.prev_char = "".to_string();
    }

    pub fn print_error(err : String) {
        println!(
            "{}: {}",
            Color::Red.bold().paint("ERROR"),
            Style::new().italic().paint(err),
        )
    }
    
    pub fn print_info(info : String) {
        println!(
            "{} : {}",
            Color::Blue.bold().paint("INFO"),
            Style::new().italic().paint(info),
        )
    }

    pub fn print_main_title()
    {
        println!("\t -----------------");
        println!("\t|  {}  |",Style::new().bold().paint("GAME SELECTOR"));
        println!("\t -----------------");
        println!("");
    }

    pub fn replace_line(line : &str) {
        print!("\u{001B}[A");
        print!("\x1b[K");
        println!("\r{}", line);
    }
    
    pub fn clear_screen(){
        println!("\u{001B}[2J\u{001B}[1;1H");
    }
}


