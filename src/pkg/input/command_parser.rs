use regex::Regex;

use super::command::CommandData;

pub fn try_parse_to_cmd(raw_cmd: &str) -> Result<CommandData, String> {
    let re = Regex::new(r"^(?<f>.+)\((?<p>.*)\)$").unwrap();

    let rslt = re.captures(raw_cmd).map(|line_data| {
        let mut s : (&str, &str) = ("","");
        match line_data.name("f") {
            Some(m) => {
                s.0 = m.as_str()
            }
            None => {
                panic!("ERROR: command is missing a name!")
            }
        }
        match line_data.name("p") {
            Some(m) => {
                s.1 = m.as_str()
            }
            None => {}
        }

        s
    });

    match rslt {
        Some(s) => {

            Ok(CommandData::new(s.0.to_string(),s.1.to_string()))
        }
        None => {
            Err(format!("Command '{}' not found!",raw_cmd))
        }
    }
}