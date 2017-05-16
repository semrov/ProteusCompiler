use std::io::Write;
use std::io::stderr;
use lexanal::position::Position;
use std::fmt::Display;
use std::fmt::{Error,Formatter};


#[derive(Copy,Clone)]
pub enum ExitCode 
{
    LexicalAnalyzerIlegallChar,
}

impl Display for ExitCode {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error>
    {
        match self {
            &ExitCode::LexicalAnalyzerIlegallChar => writeln!(f, "LexicalAnalyzerIlegallChar"),
        }
    }
}


/** Izpise opozorilo.  */
pub fn warning(msg : &str) 
{
    writeln!(stderr(), ":-o {}",msg).unwrap();
}


/** Izpise opozorilo, ki je vezano na del vhodne datoteke.  */
pub fn warning_at_position(msg :&str, position : &Position) 
{
    writeln!(stderr(), ":-o {} {}", position, msg).unwrap()
}

/** Izpise obvestilo o napaki in konca izvajanje programa.  */
pub fn error(msg : &str, exit_code : ExitCode)	
{
    panic!( ":-( {}  ExitCode: {}", msg, exit_code);
}

/** Izpise obvestilo o napaki, ki je vezano na del vhodne datoteke, in konca izvajanje programa.  */
pub fn error_at_position(msg : &str, position : &Position, exit_code : ExitCode) 
{
    panic!(":-( {} {} {}", msg, exit_code, position);
}
