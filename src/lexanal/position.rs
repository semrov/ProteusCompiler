use xml::XMLable;
use std::fmt::Display;
use std::io::Write;
use std::fmt::Formatter;
use std::fmt::Error;


#[derive(Debug)]
pub struct Position<'a> {
    filename: &'a str,
    begin_line : u64,
    begin_column : u64,
    end_line : u64,
    end_column : u64,
}

impl<'a> Position<'a> {
   pub fn new(filename : &'a str, begin_line : u64, begin_column : u64, end_line : u64, end_column : u64 ) -> Position {
      Position{ 
        filename,
        begin_line,
        begin_column,
        end_line,
        end_column,
        }
   }

   pub fn get_description(&self) -> String 
   {
       // "line " + begLine + " column: " + begColumn + ", filename: " + filename;
       format!("filename: {}, line {} column {}",self.filename,self.begin_line,self.begin_column)
   }
}

impl<'a> Display for Position<'a> 
{
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error>
    {
        // return "[" + filename + ":" + begLine + "." + begColumn + "-" + endLine + "." + endColumn + "]";
        write!(f, "[{}: {}.{}-{}.{}]", self.filename,self.begin_line,self.begin_column,self.end_line,self.end_column)
    }
}

impl<'a> XMLable for Position<'a>
{
    fn to_xml(&self, xml : &mut Write)
    {
        // xml.println("<position filename=\"" + filename + "\" begLine=\"" + begLine + "\" begColumn=\"" + begColumn + "\"
        // endLine=\"" + endLine + "\" endColumn=\"" + endColumn + "\"/>");
        writeln!(xml, "<position filename=\"{}\" begLine=\"{}\" begColumn=\"{}\" endLine=\"{}\" endColumn=\"{}\"/> ", self.filename, self.begin_line,self.begin_column,self.end_line,self.end_column).unwrap();
    }
}