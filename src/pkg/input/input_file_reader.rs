use std::{
    fs::read_to_string,
    io::{BufReader, Read},
    fs::File
};
use crate::pkg::general::lines::{TextLine,Line,CmdLine};
use regex::Regex;


pub fn read_line_from_file(file : &str, line : usize) -> Result<Line,String> {
    match File::open(file) {

    }


}


fn setup_old(path : &str)  -> Result<(),String>{

    self.current_line_index = 0;
    self.current_raw_str = "".to_string();
    self.lines = vec![];

    let raw = match read_to_string(path) 
    {
        Ok(f) => f,
        Err(error) => return Err(format!("Couldn't read file: {error:?}"))
    }; 

    let re = Regex::new(r"^\d+;(?<n>.+);(?<t>.+)$|^\d+;(?<c>.+\(.*\))").unwrap();
    //let re = Regex::new(r"^(?<n>.+)\|(?<t>.+)\|(?<c>.+)?$").unwrap();

    // Eventually change this for memory purposes.
    // Instead of reading the entire file, open stream, read two lines, print first and put second in buffer. Every new line, push new into buffer and drop the eldest.
    // reading file stream basically
    for l in raw.lines() {
        if l.is_empty() {
            continue
        }

        let rslt: Vec<(&str, &str, &str)> = re.captures_iter(l).map(|line_data| {
            let mut n = "";
            let mut t = "";
            let mut c = "";

            match line_data.name("n") {
                Some(m) => {
                    n = m.as_str()
                }
                None => {}
            }
            
            match line_data.name("t") {
                Some(m) => {
                    t = m.as_str()
                }
                None => {}
            }
            match line_data.name("c") {
                Some(m) => {
                    c = m.as_str()
                }
                None => {}
            }
            (n,t,c)
        }).collect();

        // if n and t are empty, make Text, if c isset make command
        for (n, t, c) in rslt {
            if n.is_empty() && t.is_empty() {
                self.lines.push(Line::Cmd(CmdLine::new(c)))
            } else {
                self.lines.push(Line::Text(TextLine::new(n,t))); 
            }
        }
    }
    self.total_line_count = self.lines.len();
    Ok(())
}

// pub fn read_next_line() -> Result<Line,String>{
    
//     if self.current_line_index == self.total_line_count.try_into().unwrap() {
//         return Err(String::from("Last line in file reached!"));
//     }

//     self.current_line_index += 1;
//     let i = self.current_line_index -1;
//     Ok(self.lines[i as usize].clone())
// }

// pub fn set_current_line_to(line : u64) {
//     self.current_line_index = line
// }