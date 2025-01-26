use console::Term;


#[derive(PartialEq,Clone,Copy)]
pub enum KeyInteraction {
    Up,
    Down,
    Select,
    Close,
    Next
}

pub fn await_key_press(term : &Term) -> KeyInteraction{
    let mut k = KeyInteraction::Close;
    'key_await : loop {
        if let Ok(char) = term.read_char(){
            match char {
                'k' | 'K' => {
                    k = KeyInteraction::Up; // go up
                    break  'key_await;
                }
                'j' | 'J' => {
                    k = KeyInteraction::Down; // go down
                    break  'key_await;
                }
                'y' | 'Y' => {
                    k = KeyInteraction::Select; // select option
                    break  'key_await;
                }
                'x' | 'X' => {
                    k = KeyInteraction::Close; // close / exit
                    break  'key_await;
                },
                ' ' | '\n' => {
                    k = KeyInteraction::Next; // next line
                    break  'key_await;
                },
                
                _ => {}
            }
        }
    }

    k
}
