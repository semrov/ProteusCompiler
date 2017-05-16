pub mod symbol;
pub mod position;
pub mod run;

use std::io;
use std::io::{Read,BufReader, Seek,SeekFrom};
use std::fs::File;
use std::collections::HashMap;
use lexanal::symbol::{Symbol,Token};
use self::position::Position;
use report::ExitCode;
use report;

#[derive(Copy,Clone,Debug)]
enum ParserState 
{
    InitialState,
    IdentifierState,
    IntConstState,
    RealConstState,
    RealExcpConstState,
    EqualsState,
    LessState,
    GraterState,
    CommentState,
    StringConstState,
    StringEscapeState,
}


pub struct LexicalAnalyzer 
{
    file_name : String,
    reader : BufReader<File>,
    line  : u64,
    column : u64,
    reserved_words : HashMap<&'static str, Token>
}


impl LexicalAnalyzer {
    fn get_reserved_keywords_hashmap() -> HashMap<&'static str, Token> 
    {
        let mut reserved_words : HashMap<&'static str, Token> = HashMap::new();
        reserved_words.insert("int",Token::INT);
        reserved_words.insert("real",Token::REAL);
        reserved_words.insert("bool",Token::BOOL);
        reserved_words.insert("string",Token::STRING);
        reserved_words.insert("arr",Token::ARR);
        reserved_words.insert("else",Token::ELSE);
        reserved_words.insert("for",Token::FOR);
        reserved_words.insert("fun",Token::FUN);
        reserved_words.insert("if",Token::IF);
        reserved_words.insert("rec",Token::REC);
        reserved_words.insert("then",Token::THEN);
        reserved_words.insert("typ",Token::TYP);
        reserved_words.insert("var",Token::VAR);
        reserved_words.insert("where",Token::WHERE);
        reserved_words.insert("while",Token::WHILE);
        reserved_words.insert("true",Token::BOOLCONST{lexeme : "true".to_string()});
        reserved_words.insert("false",Token::BOOLCONST{lexeme : "false".to_string()});

        reserved_words
    }

    fn get_symbol_type<'a>(&'a self, lexeme : String, begin_column : u64) -> Option<Symbol<'a>> 
    {
        let p = self.get_literal_position(begin_column, lexeme.len() as u64);
        match self.reserved_words.get(&*lexeme) 
        {
            Some(token) => Symbol::new(token.clone(),p),
            None => Symbol::new(Token::IDENTIFIER{lexeme}, p),
        }
    }
 

    /** Ustvari nov leksikalni analizator.  
    * @param programName ime datoteke z izvorno kodo programa.
    * @throws IOException Ce datoteke z izvorno kodo programa ni mogoce odpreti.
    */
    pub fn new(program_name : String) -> Result<LexicalAnalyzer, String> 
    {
        let file = File::open(&program_name);
        let file = match file
        {
                Ok(file) => file,
                Err(e) => return Err(format!("Error while opening file: {:?}!", e.kind())),
        };

        let reader = BufReader::new(file);
        //reader.seek(SeekFrom::Start(0));

        Ok (LexicalAnalyzer 
        {
            file_name : program_name,
            reader : reader,
            line : 1, 
            column : 1,
            reserved_words : Self::get_reserved_keywords_hashmap(),
        })
    }

    /** Vrne naslednji osnovni simbol.
	 * 
	 * @return Naslednji osnovni simbol ali <code>None</code> ob koncu datoteke.
	 * @returns io::Error Ce je prislo do napake pri branju vhodne datoteke.
	 */
     fn get_next_char(&mut self) -> Result<Option<char> , io::Error>
     {
        let mut byte = [0u8;1];
        match self.reader.read(&mut byte) 
        {
            Ok(n) => 
            {
                self.column += 1;
                if n > 0 {Ok(Some(byte[0] as char))} else {Ok(None)}
            },
            Err(e) => Err(e),
        }
     }

     /** Move file cursor 1 step back*/
     fn seek_back(&mut self) 
     {
         self.reader.seek(SeekFrom::Current(-1)).unwrap();
      }

     fn get_literal_position<'a>(&'a self, begin_column : u64, lenght : u64) -> Position
     {
        Position::new(&self.file_name, self.line,begin_column,self.line,begin_column + lenght -1)
     }

     pub fn get_next_symbol<'a>(&'a mut self) -> Result<Option<Symbol<'a>>, io::Error> 
     {
         let mut state : ParserState = ParserState::InitialState;
         let mut literal = String::new();
         let mut literal_begin = self.column;

         loop 
         {
             let chr = match self.get_next_char() 
             {
                 Ok(Some(c)) =>
                 {
                      //literal.push(c);
                      Some(c)
                 },
                 Ok(None) => None, //return Ok(Symbol::new(Token::EOF, self.get_literal_position(literal_begin,0))),
                 Err(e) => return Err(e),
             };

             match state 
             {
                 ParserState::InitialState => 
                 {
                     let chr = match chr 
                    {
                        Some(c) => c,
                        None => return Ok(Symbol::new(Token::EOF, self.get_literal_position(literal_begin,0))),
                    };
                    match chr 
                    {
                        //self.get_literal_position(literal_begin,literal.len() as u64)
                        ' '  => literal_begin += 1,
                        '\t' => literal_begin += 4,
                        '\r' => { self.column += 1; literal_begin +=1;}
                        '\n' =>{self.line +=1; self.column = 1; literal_begin =1;},
                        '+' => return Ok(Symbol::new(Token::ADD, self.get_literal_position(literal_begin,1))), 
                        '-' => return Ok(Symbol::new(Token::SUB, self.get_literal_position(literal_begin,1))),
                        '*' => return Ok(Symbol::new(Token::MUL, self.get_literal_position(literal_begin,1))),
                        '/' => return Ok(Symbol::new(Token::DIV, self.get_literal_position(literal_begin,1))),
                        '%' => return Ok(Symbol::new(Token::MOD, self.get_literal_position(literal_begin,1))),
                        '!' => return Ok(Symbol::new(Token::NOT, self.get_literal_position(literal_begin,1))),
                        '&' => return Ok(Symbol::new(Token::AND, self.get_literal_position(literal_begin,1))),
                        '|' => return Ok(Symbol::new(Token::OR, self.get_literal_position(literal_begin,1))),
                        '=' => state = ParserState::EqualsState,
                        '<' => state = ParserState::LessState,
                        '>' => state = ParserState::GraterState,
                        '#' => state = ParserState::CommentState,
                        '\"' => state = ParserState::StringConstState,
                        '(' =>  return Ok(Symbol::new(Token::LPARENT, self.get_literal_position(literal_begin,1))),
                        ')' =>  return Ok(Symbol::new(Token::RPARENT, self.get_literal_position(literal_begin,1))),
                        '[' =>  return Ok(Symbol::new(Token::LBRACKET, self.get_literal_position(literal_begin,1))),
                        ']' =>  return Ok(Symbol::new(Token::RBRACKET, self.get_literal_position(literal_begin,1))),
                        '{' =>  return Ok(Symbol::new(Token::LBRACE, self.get_literal_position(literal_begin,1))),
                        '}' =>  return Ok(Symbol::new(Token::RBRACE, self.get_literal_position(literal_begin,1))),
                        '.' =>  return Ok(Symbol::new(Token::DOT, self.get_literal_position(literal_begin,1))),
                        ',' =>  return Ok(Symbol::new(Token::COMMA, self.get_literal_position(literal_begin,1))),
                        ':' =>  return Ok(Symbol::new(Token::COLON, self.get_literal_position(literal_begin,1))),
                        ';' =>  return Ok(Symbol::new(Token::SEMIC, self.get_literal_position(literal_begin,1))),
                        c => 
                        {
                            if c.is_alphabetic() || c == '_' 
                            {
                                state = ParserState::IdentifierState;
                                literal.push(c);
                            }
                            else if c.is_numeric() 
                            {
                                state = ParserState::IntConstState;
                                literal.push(c);
                            }
                            else 
                            {
                                report::error_at_position(&format!("Lexical anayzer: Invalid character {} (ascii: {})",c,c as u8),
                                                                        &self.get_literal_position(literal_begin,1),
                                                                        ExitCode::LexicalAnalyzerIlegallChar);
                            }
                        }
                    }
                 },
                 ParserState::EqualsState => 
                 {
                     match chr 
                    {
                        Some('=') => return Ok(Symbol::new(Token::EQU,self.get_literal_position(literal_begin,2))),
                        _ =>
                        { 
                            self.seek_back();  
                            return Ok(Symbol::new(Token::ASSIGN, self.get_literal_position(literal_begin,1)));
                        },
                    }
                 },
                 ParserState::LessState =>
                 {
                     match chr 
                     {
                         Some('>') => return Ok(Symbol::new(Token::NEQ,self.get_literal_position(literal_begin,2))),
                         Some('=') => return Ok(Symbol::new(Token::LEQ, self.get_literal_position(literal_begin,2))),
                        _ => 
                        {
                            self.seek_back();
                            return Ok(Symbol::new(Token::LTH,self.get_literal_position(literal_begin,1)));
                        },
                     }
                 },
                 ParserState::GraterState => 
                 {
                     match chr 
                     {
                         Some('=') => return Ok(Symbol::new(Token::GEQ,self.get_literal_position(literal_begin,2))),
                         _ => 
                         {
                             self.seek_back();
                             return Ok(Symbol::new(Token::GTH, self.get_literal_position(literal_begin,1)));
                         },
                     }
                 },
                 ParserState::CommentState => 
                 {
                     match chr 
                     {
                         Some('\r') => {},
                         Some('\n') => state = ParserState::InitialState,
                         Some(c) if (c as u8) < 32 || (c as u8) > 126 => report::error_at_position(&format!("Invalid character '{}' (ascii: {}) in comment\n",c, c as u8),
                                                                                                                                            &self.get_literal_position(literal_begin,1),
                                                                                                                                            ExitCode::LexicalAnalyzerIlegallChar),
                        None => return Ok(Symbol::new(Token::EOF,self.get_literal_position(literal_begin,0))),
                         _ => {},
                     }
                 }
                 ParserState::IdentifierState =>
                 {
                     match chr 
                     {
                         Some(c) if c.is_alphabetic() || c.is_digit(10) || c == '_' => literal.push(c),
                         _ => 
                         {
                            self.seek_back();
                            return Ok(self.get_symbol_type(literal,literal_begin));
                         },
                     }
                 },
                 ParserState::IntConstState => 
                 {
                     match chr 
                     {
                         Some(c) if c.is_digit(10) => literal.push(c),
                         Some('.') => 
                         {
                             // REAL CONST STATE
                             state = ParserState::RealConstState;
                             literal.push('.');
                             match try!(self.get_next_char())
                             {
                                 Some(c) if c.is_digit(10) => literal.push(c),
                                 _ => report::error_at_position("Error while parsing REALCONST. Dot (.) must be followed by at least one digit!",
                                                                     &self.get_literal_position(literal_begin, literal.len() as u64),
                                                                     ExitCode::LexicalAnalyzerIlegallChar),
                             }
                             /*
                             match try!(self.get_next_char()) 
                             {
                                 Some(c) if c == '+' || c == '-' =>
                                 {
                                      literal.push(c);
                                      match try!(self.get_next_char()) 
                                      {
                                          Some(c) if c.is_digit(10) => literal.push(c),
                                          _ =>  report::error_at_position("Error while parsing REALCONST. Illegal character!",
                                                                     &self.get_literal_position(literal_begin, literal.len() as u64),
                                                                     ExitCode::LexicalAnalyzerIlegallChar),
                                      }
                                 },
                                 Some(c) if c.is_digit(10) => literal.push(c),
                                 _ => report::error_at_position("Error while parsing REALCONST. Illegal character!",
                                                                     &self.get_literal_position(literal_begin, literal.len() as u64),
                                                                     ExitCode::LexicalAnalyzerIlegallChar),
                             }
                             */
                             // END REAL CONST STATE
                         },
                         Some(c) if c.is_alphabetic() => 
                         {
                             report::error_at_position(&format!("Error while parsing INTCONST: illegal character {}!",c),
                                                                     &self.get_literal_position(literal_begin,literal.len() as u64),
                                                                     ExitCode::LexicalAnalyzerIlegallChar);
                         },
                         None => 
                         {
                             let len = literal.len() as u64;
                             return Ok(Symbol::new(Token::INTCONST{lexeme : literal}, self.get_literal_position(literal_begin, len)));
                         },
                         _ =>  
                         {
                             let len = literal.len() as u64;
                             self.seek_back();
                             return Ok(Symbol::new(Token::INTCONST{lexeme : literal}, self.get_literal_position(literal_begin, len)));
                         },
                     }
                 },
                 ParserState::RealConstState => 
                 {
                    match chr 
                    {
                        Some(c) if c.is_digit(10) => literal.push(c),
                        Some(c) if c == 'e' || c =='E' => 
                        {
                            state = ParserState::RealExcpConstState;
                            literal.push(c);
                            match try!(self.get_next_char()) 
                            {
                                 Some(c) if c == '+' || c == '-' =>
                                 {
                                      literal.push(c);
                                      match try!(self.get_next_char()) 
                                      {
                                          Some(c) if c.is_digit(10) => literal.push(c),
                                          _ =>  report::error_at_position("Error while parsing REALCONST. Illegal character!",
                                                                     &self.get_literal_position(literal_begin, literal.len() as u64),
                                                                     ExitCode::LexicalAnalyzerIlegallChar),
                                      }
                                 },
                                 Some(c) if c.is_digit(10) => literal.push(c),
                                 _=> report::error_at_position("Error while parsing REALCONST. Illegal character!",
                                                                     &self.get_literal_position(literal_begin, literal.len() as u64),
                                                                     ExitCode::LexicalAnalyzerIlegallChar),
                            }
                        }
                        Some(c) if c.is_alphabetic() => 
                        {
                             report::error_at_position(&format!("Error while parsing REALCONST: illegal character {}!",c),
                                                                     &self.get_literal_position(literal_begin,literal.len() as u64),
                                                                     ExitCode::LexicalAnalyzerIlegallChar);
                        },
                        None => 
                        {
                            let len = literal.len() as u64;
                             return Ok(Symbol::new(Token::REALCONST{lexeme : literal}, self.get_literal_position(literal_begin, len)));
                        },
                        _ => 
                        {
                             let len = literal.len() as u64;
                             self.seek_back();
                             return Ok(Symbol::new(Token::REALCONST{lexeme : literal}, self.get_literal_position(literal_begin, len)));
                        },
                    }
                 },
                 ParserState::RealExcpConstState => 
                 {
                    match chr {
                        Some(c) if c.is_digit(10) => literal.push(c),
                        Some(c) if c.is_alphabetic() => 
                        {
                            report::error_at_position(&format!("Error while parsing REALEXCPCONST: illegal character {}!",c),
                                                                     &self.get_literal_position(literal_begin,literal.len() as u64),
                                                                     ExitCode::LexicalAnalyzerIlegallChar);
                        }
                        None => 
                        {
                             let len = literal.len() as u64;
                             return Ok(Symbol::new(Token::REALCONST{lexeme : literal}, self.get_literal_position(literal_begin, len)));
                        },
                        _ => 
                        {
                             let len = literal.len() as u64;
                             self.seek_back();
                             return Ok(Symbol::new(Token::REALCONST{lexeme : literal}, self.get_literal_position(literal_begin, len)));
                        },
                    }
                 },
                 ParserState::StringConstState => 
                 {  
                     match chr 
                     {
                         Some(c) if c.is_alphabetic() || c.is_digit(10) || c.is_whitespace() => literal.push(c),
                         Some('\\') => {state = ParserState::StringEscapeState; },
                         Some('\"') => 
                         {
                             let len = literal.len() as u64;
                             return Ok(Symbol::new(Token::STRINGCONST{lexeme : literal}, self.get_literal_position(literal_begin, len)));
                         },
                         None =>  report::error_at_position(&format!("Error: End of file occured, but string not closed!"),
                                                                     &self.get_literal_position(literal_begin,literal.len() as u64),
                                                                     ExitCode::LexicalAnalyzerIlegallChar),
                         _ =>  {},
                     }
                 },
                 ParserState::StringEscapeState => 
                 {
                     match chr
                     {
                         Some('\\') => { literal.push('\\'); state = ParserState::StringConstState; },
                         Some('\"') => { literal.push('\"'); state = ParserState::StringConstState; },
                         Some('\'') => { literal.push('\''); state = ParserState::StringConstState; },
                         _ => report::error_at_position("Error: String escape!",
                                                                        &self.get_literal_position(literal_begin,literal.len() as u64),
                                                                        ExitCode::LexicalAnalyzerIlegallChar),
                     }
                 },
             }

         }
     }

}


