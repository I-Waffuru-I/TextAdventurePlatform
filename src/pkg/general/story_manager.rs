
use crate::pkg::{
    general::{
        store_file,
        lines::Line
    },
    input::{
        command_executer,
        command_parser,
        input_file_reader::InputFileReader
    },
};

use super::characters::GameCharacterData;

pub struct StoryManager {
    chars : Vec<GameCharacterData>,
    current_line : u32
}


impl StoryManager {
    pub fn new() -> StoryManager {

        let chars = vec![];
        let current_line = 0;

        StoryManager {
            chars,
            current_line
        }
    }



    pub fn start_game(&mut self, game_title : String) -> Result<(), String> {

        match store_file::start_game(&game_title) {
            Ok(game) => {
                if let Err(e) = self.file_reader.setup(&game.main_file_path) 
                { return Err(e) };

                // self.dialogue_printer.implement_chars(game.characters);

            }
            Err(error) => {
                return Err(format!("{:?}",error))
            }
        }
    
        Ok(())
    }


    pub fn continue_game(&mut self) {
        let mut is_cmd = true;
        while is_cmd {
            self.current_line += 1;


        }

    }


    fn handle_line(&mut self, line : Line) {
        match line {
            Line::Text(t) => {
                // self.dialogue_printer.print_dialogue_line(&t);
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





}