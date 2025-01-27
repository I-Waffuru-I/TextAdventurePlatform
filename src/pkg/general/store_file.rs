use serde::{Deserialize, Serialize};
use std::fs;
use super::characters::GameCharacterData;

#[derive(Debug,Deserialize,Serialize)]
pub struct MainGameFile {
    pub main_file_path : String,
    pub characters : Vec<GameCharacterData> 
}


pub fn present_game_titles() -> Result<Vec<String>,String>{
    let path = fs::read_dir("data/").unwrap();
    let mut v : Vec<String> = vec![];
    let mut index : usize = 0;
    for fil in path {
        match fil {
            Ok(f) => {
                if f.path().extension().and_then(|s| s.to_str()) == Some("toml") {
                    v.insert(index, f.file_name().into_string().unwrap());
                    index += 1;
                }
            },
            Err(e) => return Err(format!("Something went wrong getting the path: {e}")),
        }
    }

    Ok(v)
}


pub fn start_game(game_info_file: &str) -> Result<MainGameFile,Box<dyn std::error::Error>> {
    let s = format!("data/{}", game_info_file);
    let contents = fs::read_to_string(s).unwrap();
    Ok(toml::from_str(&contents)?)
/*
    println!("--------");
    println!("{:?}", game);
    println!("--------");
*/

}