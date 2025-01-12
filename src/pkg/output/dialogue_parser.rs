
pub struct DialogueParser {
    line : String,
    final_line : String,
    index : usize,
}
impl DialogueParser {

    pub fn new()->DialogueParser {
        DialogueParser{
            line : String::new(),
            final_line : String::new(),
            index : 0,

        }
    }


    pub fn parse_line(&mut self, line : &str) -> Result<String,String> {

        self.index = 0;
        self.line = line.to_string();
        self.final_line = String::from("");

        while self.index < line.len() {
            match self.peek(0) {

                Some(c) => {
                    if c == '<' {
                        self.incr();
                        match self.handle() {
                            Ok(s) => self.final_line.push_str(&s),
                            Err(e) => return Err(e),
                        }
                    } else {
                        self.final_line.push(c);
                        self.incr();
                    }
                },
                None => {
                    // technically shouldn't ever be called
                    println!("End of line reached : {}",self.final_line);
                    return Ok(self.final_line.clone());
                } 
            }
        }

        self.final_line.push_str("\x1B[0m");
        Ok(self.final_line.clone())
    }

    fn handle(&mut self) -> Result<String,String>{

        let mut cmd = String::new();
        'find_cmd : loop {
            match self.peek(0) {
                Some(c) => {
                    if c == '>'{
                        self.incr();
                        break 'find_cmd;
                    } else {
                        cmd.push(c);
                        self.incr();
                    }
                },
                None => {
                    return Err(format!("End of line reached before end of command : {}", cmd));
                }
            } 
        }
        let x = DialogueParser::handle_inline_cmd(&cmd);
        
        match x {
            Ok(s) => {
                Ok(s.to_string())
            },
            Err(e) => {
                return Err(e)
            },
        }

    }


    fn handle_inline_cmd(cmd : &str) -> Result<String,String>{

        match cmd {
            "/" => Ok("\x1B[0m".to_string()),
            "b" => Ok("\x1B[1m".to_string()),
            "/b" => Ok("\x1B[22m".to_string()),
            "i" => Ok("\x1B[3m".to_string()),
            "/i" => Ok("\x1B[23m".to_string()),
            x => {
                if x.chars().nth(0) == Some('c') && x.len() == 10 {
                    println!("Parsing inline colour : {};{};{}",&x[1..4],&x[4..7],&x[7..10]);
                    Ok(format!("\x1B[38;2;{};{};{}m", &x[1..4], &x[4..7], &x[7..10]))
                }   else {
                    Err(format!("Inline command not recognized : {}", x))
                }
            }
        }
    }

    fn peek(&mut self, pos: usize ) -> Option<char> {
        self.line.chars().nth(self.index + pos)
    }

    fn incr(&mut self) {
        self.index += 1
    }



}
