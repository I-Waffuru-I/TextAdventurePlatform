#[derive(Clone)]
pub enum Line {
    Text(TextLine),
    Cmd(CmdLine)
}


#[derive(Clone)]
pub struct TextLine {
    pub name : String,
    pub text : String,
}


impl TextLine {
    pub fn new(name : &str, text : &str) -> TextLine {
        let mut input : (&str,&str) = ("","");

        if !name.is_empty() {
            input.0 = name;
        }
        if !text.is_empty(){
            input.1 = text;
        }


        TextLine {
            name: String::from(input.0),
            text : String::from(input.1),
        }
    }
}


#[derive(Clone)]
pub struct CmdLine {
    pub command : String,
}

impl CmdLine {
    pub fn new(command : &str) -> CmdLine {
        CmdLine {
            command : command.to_string()
        }
    }
}