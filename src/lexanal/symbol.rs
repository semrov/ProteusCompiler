use lexanal::position::Position;
use xml::XMLable;
use std::io::Write;

pub struct Symbol<'a> 
{
    token : Token,
    //lexeme : String,
    position : Position<'a>,
} 

impl<'a> Symbol<'a> 
{
    pub fn new(token : Token, position : Position<'a>)  -> Option<Symbol>
    {
        match token 
        {
            Token::EOF => None,
            _ => Some(Symbol { token,  position }),
        }
    }
}

/** Pripravi predstavitev osnovnega simbola za izpis v XML datoteki.
	 * 
	 * @param lexeme Znakovna predstavitev osnovnega simbola.
	 * @return Predstavitev osnovnega simbola za izpis v XML datoteki.
*/
fn str_to_xml(lexeme : &str) -> String 
{
    let mut lex = String::new(); 
    for c in lexeme.chars()
    {
        match c 
        {
            '\'' => lex.push_str("&#39;") ,
            '\"' => lex.push_str("&#34;"),
            '&' => lex.push_str("&#38;"),
            '<' => lex.push_str("&#60;"),
            '>' => lex.push_str("&#62;"),
             chr => lex.push(chr),
        }
    }

    lex
    
}


impl<'a> XMLable for Symbol<'a> 
{
    fn to_xml(&self, xml : &mut Write) 
    {
        match self.token 
        {
            Token::IDENTIFIER{ref lexeme} => 
            {
                writeln!(xml, "<symbol token=\"IDENTIFIER\"  lexeme=\"{}\">", str_to_xml(lexeme)).unwrap(); 
                self.position.to_xml(xml);
                writeln!(xml,"</symbol>").unwrap();
            },
            Token::INTCONST{ref lexeme} => 
            {
                writeln!(xml, "<symbol token=\"INTCONST\"  lexeme=\"{}\">", str_to_xml(lexeme)).unwrap(); 
                self.position.to_xml(xml);
                writeln!(xml,"</symbol>").unwrap();
            },
            Token::REALCONST{ref lexeme} => 
            {
                writeln!(xml, "<symbol token=\"REALCONST\"  lexeme=\"{}\">", str_to_xml(lexeme)).unwrap(); 
                self.position.to_xml(xml);
                writeln!(xml,"</symbol>").unwrap();
            },
            Token::BOOLCONST{ref lexeme} => 
            {
                writeln!(xml, "<symbol token=\"BOOLCONST\"  lexeme=\"{}\">", str_to_xml(lexeme)).unwrap(); 
                self.position.to_xml(xml);
                writeln!(xml,"</symbol>").unwrap();
            },
            Token::STRINGCONST{ref lexeme} => 
            {
                writeln!(xml, "<symbol token=\"STRINGCONST\"  lexeme=\"{}\">", str_to_xml(lexeme)).unwrap(); 
                self.position.to_xml(xml);
                writeln!(xml,"</symbol>").unwrap();
            },
            Token::INT => 
            {
                writeln!(xml, "<symbol token=\"INT\">").unwrap(); 
                self.position.to_xml(xml);
                writeln!(xml,"</symbol>").unwrap();
            },
            Token::REAL => 
            {
                writeln!(xml, "<symbol token=\"REAL\">").unwrap(); 
                self.position.to_xml(xml);
                writeln!(xml,"</symbol>").unwrap();
            },
            Token::BOOL => 
            {
                writeln!(xml, "<symbol token=\"BOOL\">").unwrap(); 
                self.position.to_xml(xml);
                writeln!(xml,"</symbol>").unwrap();
            },
            Token::STRING => 
            {
                writeln!(xml, "<symbol token=\"STRING\">").unwrap(); 
                self.position.to_xml(xml);
                writeln!(xml,"</symbol>").unwrap();
            },
            Token::ADD => 
            {
                writeln!(xml, "<symbol token=\"ADD\">").unwrap(); 
                self.position.to_xml(xml);
                writeln!(xml,"</symbol>").unwrap();
            },
            Token::SUB => 
            {
                writeln!(xml, "<symbol token=\"SUB\">").unwrap(); 
                self.position.to_xml(xml);
                writeln!(xml,"</symbol>").unwrap();
            },
            Token::MUL => 
            {
                writeln!(xml, "<symbol token=\"MUL\">").unwrap(); 
                self.position.to_xml(xml);
                writeln!(xml,"</symbol>").unwrap();
            },
            Token::DIV => 
            {
                writeln!(xml, "<symbol token=\"DIV\">").unwrap(); 
                self.position.to_xml(xml);
                writeln!(xml,"</symbol>").unwrap();
            },
            Token::MOD => 
            {
                writeln!(xml, "<symbol token=\"MOD\">").unwrap(); 
                self.position.to_xml(xml);
                writeln!(xml,"</symbol>").unwrap();
            },
            Token::NOT => 
            {
                writeln!(xml, "<symbol token=\"NOT\">").unwrap(); 
                self.position.to_xml(xml);
                writeln!(xml,"</symbol>").unwrap();
            },
            Token::AND => 
            {
                writeln!(xml, "<symbol token=\"AND\">").unwrap(); 
                self.position.to_xml(xml);
                writeln!(xml,"</symbol>").unwrap();
            },
            Token::OR => 
            {
                writeln!(xml, "<symbol token=\"OR\">").unwrap(); 
                self.position.to_xml(xml);
                writeln!(xml,"</symbol>").unwrap();
            },
            Token::EQU => 
            {
                writeln!(xml, "<symbol token=\"EQU\">").unwrap(); 
                self.position.to_xml(xml);
                writeln!(xml,"</symbol>").unwrap();
            },
            Token::NEQ => 
            {
                writeln!(xml, "<symbol token=\"NEQ\">").unwrap(); 
                self.position.to_xml(xml);
                writeln!(xml,"</symbol>").unwrap();
            },
            Token::LTH => 
            {
                writeln!(xml, "<symbol token=\"LTH\">").unwrap(); 
                self.position.to_xml(xml);
                writeln!(xml,"</symbol>").unwrap();
            },
            Token::GTH => 
            {
                writeln!(xml, "<symbol token=\"GTH\">").unwrap(); 
                self.position.to_xml(xml);
                writeln!(xml,"</symbol>").unwrap();
            },
            Token::LEQ => 
            {
                writeln!(xml, "<symbol token=\"LEQ\">").unwrap(); 
                self.position.to_xml(xml);
                writeln!(xml,"</symbol>").unwrap();
            },
            Token::GEQ => 
            {
                writeln!(xml, "<symbol token=\"GEQ\">").unwrap(); 
                self.position.to_xml(xml);
                writeln!(xml,"</symbol>").unwrap();
            },
            Token::ASSIGN => 
            {
                writeln!(xml, "<symbol token=\"ASSIGN\">").unwrap(); 
                self.position.to_xml(xml);
                writeln!(xml,"</symbol>").unwrap();
            },
            Token::LPARENT => 
            {
                writeln!(xml, "<symbol token=\"LPARENT\">").unwrap(); 
                self.position.to_xml(xml);
                writeln!(xml,"</symbol>").unwrap();
            },
            Token::RPARENT => 
            {
                writeln!(xml, "<symbol token=\"RPARENT\">").unwrap(); 
                self.position.to_xml(xml);
                writeln!(xml,"</symbol>").unwrap();
            },
            Token::LBRACKET => 
            {
                writeln!(xml, "<symbol token=\"LBRACKET\">").unwrap(); 
                self.position.to_xml(xml);
                writeln!(xml,"</symbol>").unwrap();
            },
            Token::RBRACKET => 
            {
                writeln!(xml, "<symbol token=\"RBRACKET\">").unwrap(); 
                self.position.to_xml(xml);
                writeln!(xml,"</symbol>").unwrap();
            },
            Token::LBRACE => 
            {
                writeln!(xml, "<symbol token=\"LBRACE\">").unwrap(); 
                self.position.to_xml(xml);
                writeln!(xml,"</symbol>").unwrap();
            },
            Token::RBRACE => 
            {
                writeln!(xml, "<symbol token=\"RBRACE\">").unwrap(); 
                self.position.to_xml(xml);
                writeln!(xml,"</symbol>").unwrap();
            },
            Token::DOT => 
            {
                writeln!(xml, "<symbol token=\"DOT\">").unwrap(); 
                self.position.to_xml(xml);
                writeln!(xml,"</symbol>").unwrap();
            },
            Token::COMMA => 
            {
                writeln!(xml, "<symbol token=\"COMMA\">").unwrap(); 
                self.position.to_xml(xml);
                writeln!(xml,"</symbol>").unwrap();
            },
            Token::COLON => 
            {
                writeln!(xml, "<symbol token=\"COLON\">").unwrap(); 
                self.position.to_xml(xml);
                writeln!(xml,"</symbol>").unwrap();
            },
            Token::SEMIC => 
            {
                writeln!(xml, "<symbol token=\"SEMIC\">").unwrap(); 
                self.position.to_xml(xml);
                writeln!(xml,"</symbol>").unwrap();
            },
            Token::ARR => 
            {
                writeln!(xml, "<symbol token=\"ARR\">").unwrap(); 
                self.position.to_xml(xml);
                writeln!(xml,"</symbol>").unwrap();
            },
            Token::ELSE => 
            {
                writeln!(xml, "<symbol token=\"ELSE\">").unwrap(); 
                self.position.to_xml(xml);
                writeln!(xml,"</symbol>").unwrap();
            },
            Token::FOR => 
            {
                writeln!(xml, "<symbol token=\"FOR\">").unwrap(); 
                self.position.to_xml(xml);
                writeln!(xml,"</symbol>").unwrap();
            },
             Token::FUN => 
            {
                writeln!(xml, "<symbol token=\"FUN\">").unwrap(); 
                self.position.to_xml(xml);
                writeln!(xml,"</symbol>").unwrap();
            },
            Token::IF => 
            {
                writeln!(xml, "<symbol token=\"IF\">").unwrap(); 
                self.position.to_xml(xml);
                writeln!(xml,"</symbol>").unwrap();
            },
            Token::REC => 
            {
                writeln!(xml, "<symbol token=\"REC\">").unwrap(); 
                self.position.to_xml(xml);
                writeln!(xml,"</symbol>").unwrap();
            },
            Token::THEN => 
            {
                writeln!(xml, "<symbol token=\"THEN\">").unwrap(); 
                self.position.to_xml(xml);
                writeln!(xml,"</symbol>").unwrap();
            },
            Token::TYP => 
            {
                writeln!(xml, "<symbol token=\"TYP\">").unwrap(); 
                self.position.to_xml(xml);
                writeln!(xml,"</symbol>").unwrap();
            },
            Token::VAR => 
            {
                writeln!(xml, "<symbol token=\"VAR\">").unwrap(); 
                self.position.to_xml(xml);
                writeln!(xml,"</symbol>").unwrap();
            },
            Token::WHERE => 
            {
                writeln!(xml, "<symbol token=\"WHERE\">").unwrap(); 
                self.position.to_xml(xml);
                writeln!(xml,"</symbol>").unwrap();
            },
            Token::WHILE => 
            {
                writeln!(xml, "<symbol token=\"WHILE\">").unwrap(); 
                self.position.to_xml(xml);
                writeln!(xml,"</symbol>").unwrap();
            },
            Token::EOF => {},
        }
    }
}


#[derive(Debug,Clone)]
pub enum Token 
{
    IDENTIFIER{lexeme : String},

    // constants
    INTCONST{lexeme : String},
    REALCONST{lexeme : String},
    BOOLCONST{lexeme : String},
    STRINGCONST{lexeme : String},

    //types
    INT,
    REAL,
    BOOL,
    STRING,

    //basic numeric operators
    ADD, 
    SUB,
    MUL,
    DIV,
    MOD,

    //logic operators
    NOT,
    AND, 
    OR,

    //comparators
    EQU, // ==
    NEQ, // !=
    LTH, // <
    GTH, // >
    LEQ, // <=
    GEQ, // >=

    //assign operator
    ASSIGN, // =

    LPARENT, // (
    RPARENT, // )
    LBRACKET, // [
    RBRACKET, // ]
    LBRACE, // {
    RBRACE, // }

    DOT, // .
    COMMA, // ,
    COLON, // :
    SEMIC, // ;

    // reserved words
    ARR,
    ELSE,
    FOR,
    FUN,
    IF,
    REC,
    THEN,
    TYP,
    VAR,
    WHERE, 
    WHILE,

    //End of file
    EOF,
}