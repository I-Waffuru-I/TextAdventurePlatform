
use ansi_term::{Style,Color};
use crate::pkg::{general::characters::GameCharacterData, output::color_management};
use super::{super::general::lines::TextLine, dialogue_parser::{self, DialogueParser}, image_management};



struct GameSize {
    text_line : u16,
    img_size : (u16, u16),
    img_pos : (u16, u16)
}
impl GameSize {
    pub fn new()->GameSize{
        GameSize{
            text_line : 0,
            img_size : (0,0),
            img_pos : (0,0)
        }
    }
}

struct MenuSize {
    panel_width : u16
}
impl MenuSize {
    pub fn new()->MenuSize {
        MenuSize {
            panel_width : 0
        }
    }
}




pub struct DialoguePrinter { 
    characters : Vec<GameCharacterData>,
    prev_char : String,
    parser : DialogueParser,
    termsize : (u16,u16),
    gamesize : GameSize,
    menusize : MenuSize,
    img_enabled : bool,
    image_link : &'static str
}
impl DialoguePrinter {
    pub fn new() -> DialoguePrinter {
        DialoguePrinter {
            characters : vec![],
            prev_char : String::new(),
            parser : DialogueParser::new(),
            termsize : (0,0),
            gamesize : GameSize::new(),
            menusize : MenuSize::new(),
            img_enabled : false,
            image_link : "res/title_img.png"
        }
    }

    pub fn implement_chars(&mut self, characters : Vec<GameCharacterData>) {
        self.characters = characters;
        self.prev_char = "".to_string();
    }

    pub fn implement_size(&mut self, size : (u16,u16)){
        self.termsize = size;
        if size.0 < 100 || size.1 < 20 {
            // small screen, no images
            self.img_enabled = false;
            self.menusize.panel_width = size.0;
        } else {
            self.img_enabled = true;
            self.menusize.panel_width = (size.0 as f64 / 3f64) as u16;
        }
    }

    pub fn print_dialogue_line(&mut self, line : &TextLine) { 
         match self.characters.iter().find(|c| c.short_name == line.name).cloned() {
            Some(c) => self.char_talk(&line.text, &c),
            None => {

                if line.name == 'n'.to_string() {
                    self.narrator_talk(&line);
                } else {
                    DialoguePrinter::print_error(
                        format!("Character {} not found",&line.name)
                    );
                    println!("{}",&line.text)
                }
            }
        }
    }

    pub fn cmd_executed(&mut self) {
        self.prev_char = "".to_string();
    }

    pub fn print_app_screen(&self) {
        if self.img_enabled {
            // make image layout
            let img_w = self.termsize.0 as u32 - self.menusize.panel_width as u32;
            let ascii_result = image_management::get_ascii_art(&self.image_link,img_w, self.termsize.1 as u32);
            for i in 0..self.termsize.1 {
                // gray + append img row

                print!("ESC[48;2;{};{};{}m",120,120,120);
                match i {
                    3 => {}
                    _ => {}
                }
            }
        }
    }



    /*  STATIC FUNCTIONS  */

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
        print!("\x1B[A");
        print!("\x1B[K");
        println!("\r{}", line);
    }
    
    pub fn clear_screen(){
        println!("\x1B[2J\x1B[1;1H");
    }



/*  PRIVATE FUNCTIONS */


    fn char_talk(&mut self, line : &str, ch : &GameCharacterData) {
        if self.prev_char != ch.short_name {
            println!(
                "{}:",
                color_management::get_color_by_rgb(ch.c_r,ch.c_g,ch.c_b).paint(&ch.full_name)
            );
            self.prev_char = ch.short_name.clone()
        }
        match self.parser.parse_line(line) {
            Ok(s) => println!("{}",s),
            Err(e) => DialoguePrinter::print_error(e)
        }
    }

    fn narrator_talk(&mut self, line : &TextLine) {
        if self.prev_char != line.name {
            println!();
            self.prev_char = "n".to_string();
        }

        println!(
            "{}",
            Style::new().italic().paint(&line.text)
        );


        /*match self.parser.parse_line(line.text.as_str()) {
            Ok(s) => println!("{}",s),
            Err(e) => DialoguePrinter::print_error(e)
        }*/
    }
}


