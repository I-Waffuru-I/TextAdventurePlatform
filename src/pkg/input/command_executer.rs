use console::Term;
use ansi_term::Style;
use super::command::CommandData;


pub fn execute(command_data : CommandData, term : &Term) -> Result<u64,String> {
    let cmd = command_data.command_params.clone();
    match command_data.command_name.as_str() {
        "ch" => Ok(choose_2(term,cmd)?),
        "ch3" => Ok(choose_3(term,cmd)?),
        "mv" => Ok(move_to_line(cmd)?),
        other => Err(format!(
            "Command {} not recognized!",
            other,
        )),
    }
}


fn try_str_to_int(int : &mut u64, txt : &str) -> Result<(),String>{
    match txt.parse::<u64>() {
        Ok(i) => *int = i,
        Err(_e) => return Err(format!("Parameter is not correct : '{}'",txt)),
    }
    Ok(())
}

/*
    Requires two text values followed by two ints; the displayed text values (choices) and the line indexes where they need to continue from in each scenario.
    F: ch(txt1, txt2, line1, line2)
*/
fn choose_2(term : &Term, param_s : String) -> Result<u64,String>{
    // TODO
    // print text and await answer. when answered, respond with the correct line
    // trim the params from whitespaces
    let mut quest = "";
    let mut opt1 = "";
    let mut opt2 = "";
    let mut ln1 :u64 = 0;
    let mut ln2: u64 = 0;

    for (index, p) in param_s.split(',').enumerate() {
        match index {
            0 => quest = p,
            1 => opt1 = p,
            2 => opt2 = p,
            3 => try_str_to_int(&mut ln1, p)?,
            4 => try_str_to_int(&mut ln2, p)?,
            _ => return Err(String::from("Error: Too many parameters!")),
        }
    }

    println!();
    println!("{}", Style::new().italic().paint(quest));
    println!("\t1 : {}",opt1);
    println!("\t2 : {}",opt2);
    

    loop {
        if let Ok(char) = term.read_char() {
            match char {

                '1' | '&' => return Ok(ln1),
                '2' | 'é' => return Ok(ln2),
                _ => {
                        println!(
                        "{}",
                        Style::new().italic().paint("Press 1 or 2!")
                    )
                }
            }
        }
    }
}


/*
    Requires three text values followed by three integers; the displayed text values (choices) and the line indices where they need to continue from in each scenario.
    F: ch3(txt1, txt2, txt3 , line1, line2, line3)
*/
fn choose_3(term : &Term, param_s : String) -> Result<u64, String>{
    let mut quest = "";
    let mut opt1 = "";
    let mut opt2 = "";
    let mut opt3= "";
    let mut ln1 :u64 = 0;
    let mut ln2: u64 = 0;
    let mut ln3: u64 = 0;

    for (index, p) in param_s.split(',').enumerate() {
        match index {
            0 => quest = p,
            1 => opt1 = p,
            2 => opt2 = p,
            3 => opt3 = p,
            4 => try_str_to_int(&mut ln1, p)?,
            5 => try_str_to_int(&mut ln2, p)?,
            6 => try_str_to_int(&mut ln3, p)?,
            _ => return Err(String::from("Too many parameters!")),
        }
    }

    println!();
    println!("{}", Style::new().italic().paint(quest));
    println!("\t1 : {}",opt1);
    println!("\t2 : {}",opt2);
    println!("\t3 : {}",opt3);
    

    loop {
        if let Ok(char) = term.read_char() {
            match char {

                '1' | '&' => return Ok(ln1),
                '2' | 'é' => return Ok(ln2),
                '3' | '"' => return Ok(ln3),
                _ => {
                        println!(
                        "{}",
                        Style::new().italic().paint("Press 1, 2 or 3!")
                    )
                }
            }
        }
    }
}


fn move_to_line(param_s : String) -> Result<u64,String> {
    let mut i : u64 = 0;
    try_str_to_int(&mut i, &param_s)?;

    Ok(i)
}
