use crate::pkg::output::display_printer::DialoguePrinter as dp;
use super::{
    input_manager::{ self as cm, KeyInteraction as ki }, 
    lines::Line,
    states::{
        CurrentGameState as CGS,
        CurrentMenuState as CMS,
        MenuCursorState as MCS
    },
     store_file, story_manager::StoryManager
};
use console::Term;

pub struct GameManager {
    current_state : CGS,
    story_man : StoryManager,
    dialogue_printer : dp,
    term : Term,
    menu_state : CMS,
    cursor_state : MCS,
    menu_ops : Vec<String>,
    is_in_menu : bool

}


impl GameManager {

    pub fn new() -> GameManager{
        let current_state  = CGS::MenuOpen;
        let dialogue_printer = dp::new();
        let term =  Term::buffered_stdout();
        let menu_state = CMS::App(false);
        let cursor_state = MCS{ selected : 0, total : 0};
        let menu_ops : Vec<String> = vec![];
        let is_in_menu = false;
        let story_man = StoryManager::new();
        
        GameManager {
            current_state, 
            dialogue_printer,
            term,
            menu_state,
            cursor_state,
            menu_ops,
            is_in_menu,
            story_man
        }
    }

    // Main event loop
    pub fn run(&mut self) {
        'event_loop : loop {
            match &self.current_state {
                CGS::AppIsStopping => {
                    self.stop_program();
                    break 'event_loop;
                }
                CGS::GameStarting(ref t) => {
                    self.start_game(t.clone());
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
        match self.menu_state {
            CMS::App(t) => { 
                if !t {
                    self.setup_menu();
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
                    self.setup_menu();
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
                        0 => self.menu_state = CMS::App(false), // going back to main menu
                        x => {
                            // open game at x-th index in opt (save to game manager?)
                            // self.is_in_menu = false;
                        }
                    }
                }
            }
            CMS::New(t) => {
                if !t {
                    self.setup_menu();
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
                            let title_opt = self.menu_ops.iter().nth(x as usize);
                            match title_opt {
                                Some(title) => self.current_state = CGS::GameStarting(title.clone()),
                                None => {}
                            }
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
    fn setup_menu(&mut self){
        if !self.is_in_menu{
            dp::clear_screen();
            self.dialogue_printer.implement_size(self.term.size());
            self.dialogue_printer.print_menu_img();
            self.is_in_menu = true;
        }
    }

    fn start_game(&mut self, game_title : String) {
        match self.story_man.start_game(game_title) {
            Ok(_) => {
                self.current_state = CGS::GameRunning;
                dp::clear_screen();
                self.is_in_menu = false;
                //self.dialogue_printer.print_game_screen();
            }
            Err(e) => {
                dp::print_error(e);
                self.current_state = CGS::MenuOpen;
                self.menu_state = CMS::App(false);
            }
        }
    
    
    
    }


    fn continue_playing_game(&mut self){
        self.story_man.continue_game();
        // let line  = self.file_reader.read_next_line(); 
        // match line {
        //     Ok(l) => {
        //         self.handle_line(l);                 
        //     },
        //     Err(e) => {
        //         dp::print_info(e);
        //         self.current_state = CGS::StoryIsDone;
        //     }
        // }
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

