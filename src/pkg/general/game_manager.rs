use crate::pkg::{
    input::{command_executer, command_parser, input_file_reader::InputFileReader}, 
    output::display_printer::DialoguePrinter as dp,
};
use super::{
    input_manager::{ self as cm, KeyInteraction as ki }, 
    lines::Line,
    states::{
        CurrentGameState as CGS,
        CurrentMenuState as CMS,
        MenuCursorState as MCS
    },
     store_file
};
use console::Term;

pub struct GameManager {
    file_reader : InputFileReader,
    current_state : CGS,
    dialogue_printer : dp,
    term : Term,
    selected_game_title : String,
    is_size_setup : bool,
    menu_state : CMS,
    cursor_state : MCS,
    menu_ops : Vec<String>

}


impl GameManager {

    pub fn new() -> GameManager{
        let file_reader = InputFileReader::new();
        let current_state  = CGS::MenuOpen;
        let dialogue_printer = dp::new();
        let term =  Term::buffered_stdout();
        let selected_game_title = String::from("");
        let is_size_setup = false;
        let menu_state = CMS::App(false);
        let cursor_state = MCS{ selected : 0, total : 0};
        let menu_ops : Vec<String> = vec![];
        
        GameManager {
            file_reader, 
            current_state, 
            dialogue_printer,
            term,
            selected_game_title,
            is_size_setup,
            menu_state,
            cursor_state,
            menu_ops
        }
    }

    // Main event loop
    pub fn run(&mut self) {
        'event_loop : loop {
            match self.current_state {
                CGS::AppIsStopping => {
                    self.stop_program();
                    break 'event_loop;
                }
                CGS::GameStarting => {
                    self.start_game();
                }
                CGS::GameRunning => {
                    self.continue_playing_game();
                }
                CGS::MenuOpen => {
                    self.open_menu();
                }
                CGS::StoryIsDone => {
                    self.close_game_to_menu();
                }
            }
        }
    }

    fn open_menu(&mut self){

        if !self.is_size_setup {
            dp::clear_screen();
            self.dialogue_printer.implement_size(self.term.size());
            self.dialogue_printer.print_menu_img();
            self.is_size_setup = true;
        }

        match self.menu_state {
            CMS::App(t) => { 
                if !t {
                    self.menu_state = CMS::App(true);
                    self.menu_ops = vec![String::from("New Game"), String::from("LoadGame"),String::from("Credits")];
                    self.cursor_state.selected = 0;
                    self.cursor_state.total = 2;
                }

                self.dialogue_printer.print_menu_screen("Load Game",self.cursor_state.selected, &self.menu_ops);
                if self.get_menu_input() {
                    match self.cursor_state.selected {
                        0 => self.menu_state = CMS::New(false),
                        1 => self.menu_state = CMS::Load(false),
                        _ => {
                            // credits or whatever
                        }
                    }
                }
            }
            CMS::Load(t) => {
                if !t {
                    self.menu_ops.clear();
                    self.menu_ops.push("\x1B[3mBack\x1B[23m".to_string());
                    let gl = store_file::present_game_titles().unwrap_or_else(|o| vec![]);
                    for game in gl {
                        self.menu_ops.push(game);
                    }
                    self.cursor_state.selected = 0; 
                    self.cursor_state.total = self.menu_ops.len() as u8 -1;
                    self.menu_state = CMS::Load(true);
                }
                self.dialogue_printer.print_menu_screen("Load Game",self.cursor_state.selected, &self.menu_ops);
                if self.get_menu_input() {
                    match self.cursor_state.selected {
                        0 => {
                            // back
                            self.menu_state = CMS::App(false);
                        }
                        x => {
                            // open game at x-th index in opt (save to game manager?)
                        }
                    }
                }
            }
            CMS::New(t) => {
                if !t {
                    self.menu_ops.clear();
                    self.menu_ops.push("\x1B[3mBack\x1B[23m".to_string());
                    let gl = store_file::present_game_titles().unwrap_or_else(|o| vec![]);
                    for game in gl {
                        self.menu_ops.push(game);
                    }
                    self.cursor_state.selected = 0; 
                    self.cursor_state.total = self.menu_ops.len() as u8 -1;
                    self.menu_state = CMS::New(true);
                }
                self.dialogue_printer.print_menu_screen("New Game",self.cursor_state.selected, &self.menu_ops);
                if self.get_menu_input() {
                    match self.cursor_state.selected {
                        0 => {
                            // back
                            self.menu_state = CMS::App(false);
                        }
                        x => {
                            // open game at x-th index in opt (save to game manager?)
                        }
                    }
                }
            }
        }
    }
    fn get_menu_input(&mut self) -> bool{
        match cm::await_key_press(&self.term) {
            ki::Up => {
                if self.cursor_state.selected == 0 {
                    self.cursor_state.selected = self.cursor_state.total;
                } else {
                    self.cursor_state.selected -= 1;
                }
                return false
            }
            ki::Down => {
                if self.cursor_state.selected == self.cursor_state.total {
                    self.cursor_state.selected = 0;
                } else {
                    self.cursor_state.selected += 1;
                }
                return false
            }
            ki::Next => {
                return false
            } 
            ki::Select => {
                return true
            }
            ki::Close => { 
                if self.menu_state == CMS::App(true) || self.menu_state == CMS::App(false)
                    {self.current_state = CGS::AppIsStopping }
                else 
                    { self.menu_state = CMS::App(false) }
                return false
            }
        }
    }


    fn start_game(&mut self) {
        dp::clear_screen();
        match store_file::start_game(&self.selected_game_title) {
            Ok(game) => {

                match self.file_reader.setup(&game.main_file_path){
                   Ok(_) => {},
                   Err(e) => {
                        dp::print_error(e);
                   },
                }
                self.dialogue_printer.implement_chars(game.characters);
                self.current_state = CGS::GameRunning;
            }
            Err(error) => {
                dp::print_error(format!("{:?}",error));
                self.current_state = CGS::AppIsStopping;
            }
        }
    }

    fn continue_playing_game(&mut self){
        let line  = self.file_reader.read_next_line(); 
        match line {
            Ok(l) => {
                self.handle_line(l);                 
            },
            Err(e) => {
                dp::print_info(e);
                self.current_state = CGS::StoryIsDone;
            }
        }
    }

    fn handle_line(&mut self, line : Line) {
        match line {
            Line::Text(t) => {
                self.dialogue_printer.print_dialogue_line(&t);
            }

            Line::Cmd(cmd) => {

                match command_parser::try_parse_to_cmd(&cmd.command) {
                    Ok(c) => {
                        match command_executer::execute(c,&self.term) {
                            Ok(i) => {
                               self.file_reader.set_current_line_to(i); 
                               self.dialogue_printer.cmd_executed();
                            } 
                            Err(e) => {
                                dp::print_error(e);
                            }
                        }
                    }
                    Err(s) => {
                        println!("{}",s);
                        self.current_state = CGS::AppIsStopping;
                    }
                }
            }
        };

    }

    fn close_game_to_menu(&mut self) {

        'input : loop {
            match cm::await_key_press(&self.term) {
                ki::Up => {}
                ki::Down => {}
                ki::Next => { break 'input }
                ki::Select => { break 'input }
                ki::Close => { break 'input }
            }
        }
        self.current_state = CGS::MenuOpen;      
        self.menu_state = CMS::App(false)
    }

    fn stop_program(&mut self) {
        dp::clear_screen();
        dp::print_info(format!("Stopping game now!")); 
        dp::print_info(format!("Thanks for playing! <3")); 
    }


}

