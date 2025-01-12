use crate::pkg::{
    input::{command_executer, command_parser, input_file_reader::InputFileReader}, 
    output::display_printer::DialoguePrinter as dp,
};
use std::{thread, time::Duration};
use super::{current_game_state::{CurrentGameState, CurrentMenuState}, lines::Line, store_file};
use ansi_term::Style;
use console::Term;

pub struct GameManager {
    file_reader : InputFileReader,
    current_state : CurrentGameState,
    dialogue_printer : dp,
    term : Term,
    selected_game_title : String,
}


impl GameManager {
    pub fn new() -> GameManager{
        
        let file_reader = InputFileReader::new();
        let current_state : CurrentGameState = CurrentGameState::MenuOpen(CurrentMenuState::App);
        let dialogue_printer = dp::new();
        let term =  Term::buffered_stdout();
        let selected_game_title = String::from("");
        
        GameManager {
            file_reader, 
            current_state, 
            dialogue_printer,
            term,
            selected_game_title,
        }
    }
    // Main event loop
    pub fn run(&mut self) {
        'event_loop : loop {
            match self.current_state {
                CurrentGameState::GameIsStopping => {
                    self.stop_program();
                    break 'event_loop;
                }
                CurrentGameState::GameStarting => {
                    self.start_game();
                }
                CurrentGameState::GameRunning => {
                    self.continue_playing_game();
                }
                CurrentGameState::AwaitingKeyPress => {
                    self.await_key_press();    
                }
                CurrentGameState::MenuOpen(s) => {
                    self.open_menu(s);
                }
                CurrentGameState::StoryIsDone => {
                    self.await_key_press();
                    self.open_menu(CurrentMenuState::App);
                }
            }
        }
    }

    fn open_menu(&mut self, current_menu_state : CurrentMenuState){

        self.dialogue_printer.implement_size(self.term.size());
        dp::clear_screen();

        match current_menu_state {
            CurrentMenuState::App => {},
            CurrentMenuState::Load => {},
            CurrentMenuState::New => {}
        }



        let mut g_count = 0;
        let mut g_current_selected = 0;

        dp::print_main_title();
        match store_file::present_game_titles() {
            Ok(f_list) => {
                for f in &f_list {
                    println!(
                        "\t{}",
                        f
                    );
                    g_count += 1;
                }
                g_count -= 1;
                println!();
                println!("{}", Style::new().italic().paint("Use K / J to navigate, Space / Enter to select, X to quit."));
                println!("Selected {}",&f_list[g_current_selected].clone());
                'await_arrow_press : loop {

                    if let Ok(char) = self.term.read_char() {
                        match char {
                            ' ' | '\n' => {
                                self.selected_game_title = format!("data/{}",f_list[g_current_selected].clone()); 
                                self.current_state = CurrentGameState::GameStarting;
                                break 'await_arrow_press
                            },
                            'x' => {
                                self.current_state = CurrentGameState::GameIsStopping;
                                break 'await_arrow_press;
                            },
                            'k' => {
                                if g_current_selected > 0 {
                                    g_current_selected-=1;
                                    dp::replace_line(format!("Selected {}",f_list[g_current_selected].clone()).as_str());
                                } 
                            },
                            'j' => {
                                if g_current_selected <= g_count -1 {
                                    g_current_selected+=1;
                                    dp::replace_line(format!("Selected {}",f_list[g_current_selected].clone()).as_str());
                                }
                            }
                            _ => {
                                dp::replace_line("Press a valid key.");
                            },
                        }
                    }
                }
            },
            Err(e) => {
                dp::print_error(e);
            },
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
                self.dialogue_printer.implement_size(self.term.size());
                self.dialogue_printer.implement_chars(game.characters);
                self.current_state = CurrentGameState::GameRunning;
            }
            Err(error) => {
                dp::print_error(format!("{:?}",error));
                self.current_state = CurrentGameState::GameIsStopping;
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
                self.current_state = CurrentGameState::StoryIsDone;
            }
        }

    }

    fn await_key_press(&mut self){
        'await_press : loop {
            if let Ok(char) = self.term.read_char(){
                match char {
                    'x' => {
                        thread::sleep(Duration::from_millis(100));
                        self.current_state = CurrentGameState::MenuOpen(CurrentMenuState::App);
                        break 'await_press;
                    },
                    ' ' | '\n' => {
                        thread::sleep(Duration::from_millis(100));
                        self.current_state = CurrentGameState::GameRunning;
                        break 'await_press;
                    }
                    _ => println!("{}", Style::new().italic().paint("Press Enter / Space bar to continue . . ."))
                }
            }
        }
    }

    fn handle_line(&mut self, line : Line) {

        match line {
            Line::Text(t) => {
                self.dialogue_printer.print_dialogue_line(&t);
                self.current_state = CurrentGameState::AwaitingKeyPress
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
                        self.current_state = CurrentGameState::GameIsStopping;
                    }
                }
            }
        };

    }

    fn stop_program(&mut self) {
        dp::clear_screen();
        dp::print_info(format!("Stopping game now!")); 
        dp::print_info(format!("Thanks for playing! <3")); 
    }


}

