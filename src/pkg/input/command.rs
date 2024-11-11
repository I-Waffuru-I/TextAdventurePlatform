use std::fmt::Display;

// TRAIT which all commands have to use. It's what allows to run as a method.
pub trait TextCommand {
    fn invoke(&self, ln : u64, params : String) -> u64;
}

// Wrapper for TextCommand closure
pub struct ClosureCommand<F>
where 
    F: Fn(u64,String) -> u64,
{
    closure : F,
}

impl<F> TextCommand for ClosureCommand<F>
where 
    F: Fn(u64,String) -> u64,
{
    fn invoke(&self, ln : u64, params : String) -> u64 {
       (self.closure)(ln,params) 
    }
}

// Struct from which to read a pointer to the function. CommPtr.func.invoke()
// A list of CommandPtr is made in the converter and these are returned, but not directly used in external code.
pub struct CommandPtr{
    pub func: Box<dyn TextCommand>,
} 
impl CommandPtr {

    pub fn new<F>(func: F ) -> Self 
    where 
        F: Fn(u64,String) -> u64 + 'static,
    {
        CommandPtr {
            func : Box::new(ClosureCommand {closure : func})
        }
        
    }
    
}
impl Display for CommandPtr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
} 

// The final struct to use everywhere, basically. You request a CommandData struct and can use the command_name to call the function, passing the params in as well.
pub struct CommandData{
    pub command_name : String,
    pub command_params: String,
}
impl CommandData {
    pub fn new(name : String, params : String) -> CommandData{
        CommandData {
            command_name : name,
            command_params : params,
        }
    }
}