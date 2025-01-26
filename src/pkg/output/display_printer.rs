
use ansi_term::{Style,Color};
use crate::pkg::{
    general::{
        characters::GameCharacterData,
        lines::TextLine
    },
    output::{
        dialogue_parser::DialogueParser,
        image_management,
        color_management
}}; 



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
    panel_width : u16,
    title_line : u16,
}
impl MenuSize {
    pub fn new()->MenuSize {
        MenuSize {
            panel_width : 0,
            title_line : 0
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
        self.termsize = size; // 0 = # of rows, 1 = # of cols 
        if size.1 < 100 || size.0 < 20 { 
            // small screen, no images
            self.img_enabled = false;
            self.menusize.panel_width = size.1;
        } else {
            self.img_enabled = true;
            self.menusize.panel_width = (size.1 as f64 / 3f64) as u16;
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

    pub fn print_menu_img(&mut self) {
        if self.img_enabled {
            // make image layout
            let ascii_result = image_management::get_ascii_art(
                &self.image_link,self.termsize.1 as u32, self.termsize.0 as u32
            );
            match ascii_result {
                Ok(img_s) => { 
                    print!("\x1B[H{}",img_s);
                }
                Err(e) => {
                    println!("{}", e);
                    return 
                }
            }
        }
    }

    pub fn print_app_screen(&self, selected : u8) {
        // make gray cover
        print!("\x1B[H");
        for i in 0..self.termsize.0{

            let c = if i == (6 + selected as u16 * 2) 
            { "\x1B[2D> \x1B[1m" } else { "\x1B[2m" }; // blinking else dim

            match i {
                3 => {
                    print!("{}",self::DialoguePrinter::menu_line(self.menusize.panel_width, "App!","\x1B[1m"))
                }
                4 => {
                    print!("{}", self::DialoguePrinter::horizontal_line(self.menusize.panel_width, self.menusize.panel_width/3))
                }

                6 => {
                    print!("{}",self::DialoguePrinter::menu_line(self.menusize.panel_width, "Start New",c))
                }

                8 => {
                    print!("{}",self::DialoguePrinter::menu_line(self.menusize.panel_width, "Load",c))
                }

                10 => {
                    print!("{}",self::DialoguePrinter::menu_line(self.menusize.panel_width, "Credits",c))
                }
                
                _ => {
                    print!("{}",self::DialoguePrinter::gray_line(self.menusize.panel_width));
                }
            }

            print!("\x1B[1B\r");
            }
        print!("\x1B[H\r\n");
        self::DialoguePrinter::cursor_default_pos();
    }

    pub fn print_menu_screen(&self,title : &str, selected : u8, options : &Vec<String>) {
        // make gray cover
        print!("\x1B[H");
        for i in 0..self.termsize.0{
            match i {
                3 => {
                    print!("{}",self::DialoguePrinter::menu_line(self.menusize.panel_width, title,"\x1B[1m"))
                }
                4 => {
                    print!("{}", self::DialoguePrinter::horizontal_line(self.menusize.panel_width, self.menusize.panel_width/3))
                }

                _ => {
                    print!("{}",self::DialoguePrinter::gray_line(self.menusize.panel_width));
                }
            }
            print!("\x1B[0m\x1B[1B\r");
        }
        for (i,n) in options.iter().enumerate() {
            // if selected -> highlight
            let c = if i as u8 == selected
            { "\x1B[2D> \x1B[1m" } else { "\x1B[2m" }; // blinking else dim
            let ln = 6 + i as u16 * 2;
            print!("\x1B[{};0H",ln);
            print!("{}",self::DialoguePrinter::menu_line(self.menusize.panel_width, n.as_str(),c))

        }
        
        print!("\x1B[H\r\n");
        self::DialoguePrinter::cursor_default_pos();
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

    pub fn print_main_title(){
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
        println!("\x1B[0m\x1B[2J\x1B[1;1H");
    }



/*  PRIVATE FUNCTIONS */

    fn gray_line(width : u16) -> String{
        let mut s = format!("\x1B[48;2;{};{};{}m",40,40,40);
        for i in 0..width {
            s.push(' ');
        }
        s
    }

    fn menu_line(width : u16, text : &str, style : &str) -> String {

        let mut s = format!("\x1B[48;2;{};{};{}m",40,40,40);
        for i in 0..width {
            s.push(' ');
        }
                    // toSpawn tab setBold 
        s.push_str("\r\t"); 
        s.push_str(style);
        s.push_str(text);
        s.push_str("\x1B[0m"); 
        s
    }

    fn horizontal_line(width : u16, line_width : u16) -> String {

        let mut s = format!("\x1B[48;2;{};{};{}m",40,40,40);
        for i in 0..width {
            s.push(' ');
        }
        s.push_str("\r\t");
        for i in 0..line_width {
            s.push('_');
        }
        s.push_str("\x1B[0m");
        s
    }

    fn cursor_default_pos(){
        print!("\x1B[5000;5000H");
    }

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


