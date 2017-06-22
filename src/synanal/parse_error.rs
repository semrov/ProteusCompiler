use lexanal::symbol::Symbol;
use std::io;
use std::error;
use std::fmt::Display;
use std::fmt;

#[derive(Debug)]
pub struct SymbolError
{
    symbol : Option<Symbol>,
    description : String,
}

impl SymbolError
{
    pub fn new(symbol : Option<Symbol>) -> SymbolError
    {
         SymbolError
        {
            description : match symbol 
            {
                Some(ref s) => format!("Invalid symbol {} in {}.", s, s.get_position()),
                None => format!("End of stream error!"),
            },
            symbol : symbol,
        }
     } 

     pub fn get_ref_symbol(&self)  -> Option<&Symbol>
     {
         self.symbol.as_ref()
     }
}


impl error::Error for SymbolError
{
    fn description(&self) -> &str 
    {
        &self.description
    }    

}

impl Display for SymbolError 
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result 
    {
        match self.symbol 
        {
            Some(ref symbol) => write!(f, "Symbol error:  {:?}", symbol.get_token()),
            None => write!(f, "Unexpected end of file."),
        }      
    }
}




#[derive(Debug)]
pub enum ParseError
{
    SyntaxError(SymbolError),
    IoError(io::Error),
}

impl Display for ParseError
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result 
    {
        match *self 
        {
            ParseError::SyntaxError(ref synerr) =>  write!(f, "Syntax error: {}", synerr),
            ParseError::IoError(ref ioe) => write!(f,"IO error: {}", ioe),
        }
    }
}


impl error::Error for ParseError
{
    fn description(&self) -> &str 
    {
        match *self 
        {
            ParseError::SyntaxError(ref synerr) => synerr.description(),
            ParseError::IoError(ref ioe) => ioe.description(),
        }
    }

    fn cause(&self) -> Option<&error::Error> 
    {
          match *self
        {
            ParseError::SyntaxError(ref synerr) => synerr.cause(),
            ParseError::IoError(ref ioe) => ioe.cause(),
        }
    }

}

impl From<io::Error> for ParseError {
    fn from(err : io::Error) -> ParseError
    {
        ParseError::IoError(err)
    }
}

impl From<SymbolError> for ParseError 
{
    fn from(err : SymbolError) -> ParseError
    {
        ParseError::SyntaxError(err)
    }
}

