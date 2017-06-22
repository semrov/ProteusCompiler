use xml::XMLable;
use std::fmt::Display;
use std::io::Write;
use std::fmt::Formatter;
use std::fmt::Error;
use std::rc::Rc;
//use std::fmt::Error;


#[derive(Debug, Clone)]
pub struct Position {
    filename: Rc<String>,
    begin_line : u64,
    begin_column : u64,
    end_line : u64,
    end_column : u64,
}

impl Position {
   pub fn new(filename : &Rc<String>, begin_line : u64, begin_column : u64, end_line : u64, end_column : u64 ) -> Position {
      Position{ 
        filename : filename.clone(),
        begin_line,
        begin_column,
        end_line,
        end_column,
        }
   }

   pub fn get_description(&self) -> String 
   {
       // "line " + begLine + " column: " + begColumn + ", filename: " + filename;
       format!("filename: {}, line {} column {}",*self.filename,self.begin_line,self.begin_column)
   }

   pub fn set_min(&mut self, position : &Position) 
   {
       if (position.begin_line < self.begin_line) || ((position.begin_line == self.begin_line) && (position.begin_column < self.begin_column))
       {
           self.begin_line = position.begin_line;
           self.begin_column = position.begin_column;
       }
   }

   pub fn set_max(&mut self, position : &Position) 
   {
       if (position.end_line > self.end_line) || ((position.end_line == self.end_line) && (position.end_column > self.end_column))
       {
           self.end_line = position.end_line;
           self.end_column = position.end_column;
       }
   }
}

impl Display for Position 
{
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error>
    {
        // return "[" + filename + ":" + begLine + "." + begColumn + "-" + endLine + "." + endColumn + "]";
        write!(f, "[{}: {}.{}-{}.{}]", *self.filename,self.begin_line,self.begin_column,self.end_line,self.end_column)
    }
}

impl XMLable for Position
{
    fn to_xml(&self, xml : &mut Write)
    {
        // xml.println("<position filename=\"" + filename + "\" begLine=\"" + begLine + "\" begColumn=\"" + begColumn + "\"
        // endLine=\"" + endLine + "\" endColumn=\"" + endColumn + "\"/>");
        writeln!(xml, "<position filename=\"{}\" begLine=\"{}\" begColumn=\"{}\" endLine=\"{}\" endColumn=\"{}\"/> ", *self.filename, self.begin_line,self.begin_column,self.end_line,self.end_column).unwrap();
    }
}