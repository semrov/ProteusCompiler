use lexanal::position::Position;
use xml::XMLable;
use std::io::Write;
//use synanal::syntoken::SyntaxToken;
use std::fmt::Display;
use std::fmt;

#[derive(Debug)]
pub struct Symbol 
{
    token : Token,
    pub lexeme : String,
    position : Position,
} 

impl Symbol 
{
    pub fn new(token : Token, lexeme : String, position : Position)  -> Option<Symbol>
    {
        match token 
        {
            Token::EOF => None,
            _ => Some(Symbol { token, lexeme,  position }),
        }
    }

    pub fn get_token(&self) -> Token
    {
        self.token
    }
    
    pub fn get_ref_position(&self) -> &Position
    {
        &self.position
    }

    pub fn is_token(&self, token : Token) -> bool 
    {
        self.token == token
    }

/*
    pub fn check_token(&self, syn_token : SyntaxToken) -> bool 
    {
        match self.lex_token 
        {
            LexToken::IDENTIFIER(_) => syn_token == SyntaxToken::IDENTIFIER,
            LexToken::INTCONST(_) => syn_token == SyntaxToken::INTCONST,
            LexToken::REALCONST(_) => syn_token == SyntaxToken::REALCONST,
            LexToken::BOOLCONST(_) => syn_token == SyntaxToken::BOOLCONST,
            LexToken::STRINGCONST(_) => syn_token == SyntaxToken::STRINGCONST,
            LexToken::INT => syn_token == SyntaxToken::INT,
            LexToken::REAL => syn_token == SyntaxToken::REAL,
            LexToken::BOOL => syn_token == SyntaxToken::BOOL,
            LexToken::STRING => syn_token == SyntaxToken::STRING,
            LexToken::ADD => syn_token == SyntaxToken::ADD,
            LexToken::SUB => syn_token == SyntaxToken::SUB,
            LexToken::MUL => syn_token == SyntaxToken::MUL,
            LexToken::DIV => syn_token == SyntaxToken::DIV,
            LexToken::MOD => syn_token == SyntaxToken::MOD,
            LexToken::NOT => syn_token == SyntaxToken::NOT,
            LexToken::AND => syn_token == SyntaxToken::AND,
            LexToken::OR => syn_token == SyntaxToken::OR,
            LexToken::EQU => syn_token == SyntaxToken::EQU,
            LexToken::NEQ => syn_token == SyntaxToken::NEQ,
            LexToken::LTH => syn_token == SyntaxToken::LTH,
            LexToken::GTH => syn_token == SyntaxToken::GTH,
            LexToken::LEQ => syn_token == SyntaxToken::LEQ,
            LexToken::GEQ => syn_token == SyntaxToken::GEQ,
            LexToken::ASSIGN => syn_token == SyntaxToken::ASSIGN,
            LexToken::LPARENT => syn_token == SyntaxToken::LPARENT,
            LexToken::RPARENT => syn_token == SyntaxToken::RPARENT,
            LexToken::LBRACKET => syn_token == SyntaxToken::LBRACKET,
            LexToken::RBRACKET => syn_token == SyntaxToken::RBRACKET,
            LexToken::LBRACE => syn_token == SyntaxToken::LBRACE,
            LexToken::RBRACE => syn_token == SyntaxToken::RBRACE,
            LexToken::DOT => syn_token == SyntaxToken::DOT,
            LexToken::COMMA => syn_token == SyntaxToken::COMMA,
            LexToken::COLON => syn_token == SyntaxToken::COLON,
            LexToken::SEMIC => syn_token == SyntaxToken::SEMIC,
            LexToken::ARR => syn_token == SyntaxToken::ARR,
            LexToken::FOR => syn_token == SyntaxToken::FOR,
            LexToken::IF => syn_token == SyntaxToken::IF,
            LexToken::ELSE => syn_token == SyntaxToken::ELSE,
            LexToken::FUN => syn_token == SyntaxToken::FUN,
            LexToken::REC => syn_token == SyntaxToken::REC,
            LexToken::THEN => syn_token == SyntaxToken::THEN,
            LexToken::TYP => syn_token == SyntaxToken::TYP,
            LexToken::VAR => syn_token == SyntaxToken::VAR,
            LexToken::WHERE => syn_token == SyntaxToken::WHERE,
            LexToken::WHILE => syn_token == SyntaxToken::WHILE,
            LexToken::EOF => false,
        }
    }

    fn get_symbol_string(&self) -> &str
    {
        match self.lex_token 
        {
            LexToken::IDENTIFIER(ref identifier) => identifier,
            LexToken::INTCONST(ref constant) => constant,
            LexToken::REALCONST(ref constant) => constant,
            LexToken::BOOLCONST(ref constant) => constant,
            LexToken::STRINGCONST(ref constant) => constant,
            LexToken::INT => "int",
            LexToken::REAL => "real",
            LexToken::BOOL => "bool",
            LexToken::STRING => "string",
            LexToken::ADD => "+",
            LexToken::SUB => "-",
            LexToken::MUL => "*",
            LexToken::DIV => "/",
            LexToken::MOD => "%",
            LexToken::NOT => "!",
            LexToken::AND => "&",
            LexToken::OR => "|",
            LexToken::EQU => "==",
            LexToken::NEQ => "<>",
            LexToken::LTH => "<",
            LexToken::GTH => ">",
            LexToken::LEQ => "<=",
            LexToken::GEQ => ">=",
            LexToken::ASSIGN => "=",
            LexToken::LPARENT => "(",
            LexToken::RPARENT => "",
            LexToken::LBRACKET => "[",
            LexToken::RBRACKET => "]",
            LexToken::LBRACE => "{{",
            LexToken::RBRACE => "}}",
            LexToken::DOT => ".",
            LexToken::COMMA => ",",
            LexToken::COLON => ":",
            LexToken::SEMIC => ";",
            LexToken::ARR => "arr",
            LexToken::FOR => "for",
            LexToken::IF => "if",
            LexToken::ELSE => "else",
            LexToken::FUN => "fun",
            LexToken::REC => "rec",
            LexToken::THEN => "then",
            LexToken::TYP => "typ",
            LexToken::VAR => "var",
            LexToken::WHERE => "where",
            LexToken::WHILE => "while",
            LexToken::EOF => "EOF",
        }   
    }
    */

    pub fn get_ref_lexeme(&self) -> &str
    {
        self.lexeme.as_str()
    }

    pub fn get_position(&self) -> &Position 
    {
        &self.position
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


impl XMLable for Symbol
{
    fn to_xml(&self, xml : &mut Write) 
    {
        match self.token 
        {
            Token::IDENTIFIER => 
            {
                writeln!(xml, "<symbol token=\"IDENTIFIER\"  lexeme=\"{}\">", str_to_xml(&self.lexeme)).unwrap(); 
                self.position.to_xml(xml);
                writeln!(xml,"</symbol>").unwrap();
            },
            Token::INTCONST => 
            {
                writeln!(xml, "<symbol token=\"INTCONST\"  lexeme=\"{}\">", str_to_xml(&self.lexeme)).unwrap(); 
                self.position.to_xml(xml);
                writeln!(xml,"</symbol>").unwrap();
            },
            Token::REALCONST => 
            {
                writeln!(xml, "<symbol token=\"REALCONST\"  lexeme=\"{}\">", str_to_xml(&self.lexeme)).unwrap(); 
                self.position.to_xml(xml);
                writeln!(xml,"</symbol>").unwrap();
            },
            Token::BOOLCONST => 
            {
                writeln!(xml, "<symbol token=\"BOOLCONST\"  lexeme=\"{}\">", str_to_xml(&self.lexeme)).unwrap(); 
                self.position.to_xml(xml);
                writeln!(xml,"</symbol>").unwrap();
            },
            Token::STRINGCONST => 
            {
                writeln!(xml, "<symbol token=\"STRINGCONST\"  lexeme=\"{}\">", str_to_xml(&self.lexeme)).unwrap(); 
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



impl Display for Symbol 
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result 
    {
        write!(f, "{}", self.lexeme)
    }
}


#[derive(Debug,Copy,Clone,PartialEq)]
pub enum Token 
{
    IDENTIFIER,

    // constants
    INTCONST,
    REALCONST,
    BOOLCONST,
    STRINGCONST,

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