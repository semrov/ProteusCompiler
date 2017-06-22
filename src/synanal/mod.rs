pub mod parse_error;
pub mod run;


use std::io::Write;
use std::io;
use xml::{ProteusXmlCreator, XMLable};
use lexanal::LexicalAnalyzer;
use lexanal::symbol::{Symbol, Token};
use lexanal::position::Position;
use synanal::parse_error::{ParseError,SymbolError};
use abstree::AbsTree;
use abstree::abs_decl::*;
use abstree::abs_expr::*;
use abstree::abs_position::AbsPosition;
use abstree::abs_stmt::*;
use abstree::abs_type::*;
use abstree::positioner::Positioner;
use report;


pub struct SyntaxAnalyzer //<'a> 
{
    lexical_analyser : LexicalAnalyzer,
    xml_creator : Option<ProteusXmlCreator>,
    symbol : Option<Symbol>,
}

//impl<'a> SyntaxAnalyzer<'a> 
impl SyntaxAnalyzer
{

    pub fn new(lexical_analyser : LexicalAnalyzer) -> SyntaxAnalyzer //<'a>
    {
        SyntaxAnalyzer 
        {
            lexical_analyser : lexical_analyser,
            xml_creator : None,
            symbol : None
        }
    }

    pub fn new_with_xml_creator(lexical_analyser : LexicalAnalyzer, xml_creator : ProteusXmlCreator) -> SyntaxAnalyzer //<'a>
    {
        SyntaxAnalyzer 
        {
            lexical_analyser : lexical_analyser,
            xml_creator : Some(xml_creator),
            symbol : None
        }
    }

    
    pub fn parse(&mut self) -> Result<Box<AbsTree>, io::Error> 
    {
        self.symbol = match self.lexical_analyser.get_next_symbol()
        {
            Ok(Some(symbol)) => Some(symbol),
            Ok(None) => return Ok(()),
            Err(ioe) => return Err(ioe),
        }; 
        
        let result = self.parse_source();
        match result 
        {
            Ok(expr) => {}, // next phase, AST 
            Err(ParseError::IoError(ioe)) => return Err(ioe),
            Err(ParseError::SyntaxError(syerr)) => 
            {
                match syerr.get_ref_symbol() 
                {
                    Some(symbol) => report::error_at_position(&format!("{}",syerr),symbol.get_ref_position(),report::ExitCode::SyntaxAnalyzerSyntaxError),
                    None => report::error(&format!("{}",syerr),report::ExitCode::SyntaxAnalyzerUnexpectedEndOfStream),
                }
            }
        }

        //at the end of parsing, self.symbol must be None
        if self.symbol.is_some()
        {
            report::error("Internal error: Syntax check finished, but symbol still available",report::ExitCode::SyntaxAnalyzerSyntaxError);
        }
     
        Ok(())
    }


    fn parse_source(&mut self) -> Result<(), ParseError> 
    {
        self.debug("parse_source");
        try!(self.parse_expressions());
        self.debug_end();
        Ok(())
    }

    fn parse_expressions(&mut self) -> Result<(),ParseError> 
    {
        self.debug("parse_expressions");
        self.parse_expression()?;
        self.parse_expressions_rest()?;
        self.debug_end();
        Ok(())
    }

    fn parse_expression(&mut self) -> Result<(),ParseError> 
    {
        self.debug("parse_expression");
        self.parse_or_expression()?;
        self.debug_end();
        Ok(())
    }

     fn parse_expressions_rest(&mut self) -> Result<(),ParseError> 
    {
        self.debug("parse_expressions_rest");
        match self.symbol.as_ref().map(|symbol| symbol.get_token()) 
        {
            Some(Token::COMMA)  => 
            {
                self.skip(Token::COMMA)?;
                self.parse_expression()?;
                self.parse_expressions_rest()?;
            },
            _ => {},
        }
        self.debug_end();
        Ok(())
    }

    fn parse_or_expression(&mut self) -> Result<(), ParseError>
    {
        self.debug("parse_or_expression");
        self.parse_and_expression()?;
        self.parse_or_expression_rest()?;
        self.debug_end();
        Ok(())
    }

    fn parse_or_expression_rest(&mut self) -> Result<(),ParseError>
    {
        self.debug("parse_or_expression_rest");
        match self.symbol.as_ref().map(|symbol| symbol.get_token()) 
        {
            Some(Token::OR) =>
            {
                self.skip(Token::OR)?;
                self.parse_and_expression()?;
                self.parse_or_expression_rest()?;
            },
            _ => {},
        }
        self.debug_end();
        Ok(())
    }

    fn parse_and_expression(&mut self) -> Result<(),ParseError>
    {
         self.debug("parse_and_expression");
         self.parse_relational_expression()?;
         self.parse_and_expression_rest()?;
         self.debug_end();
         Ok(())
    }

     fn parse_and_expression_rest(&mut self) -> Result<(),ParseError>
    {
         self.debug("parse_and_expression_rest");
         match self.symbol.as_ref().map(|symbol|  symbol.get_token())
         {
             Some(Token::AND)  => 
             {
                 self.skip(Token::AND)?;
                 self.parse_relational_expression()?;
                 self.parse_and_expression_rest()?;
             },
             _ => {},
         }
         self.debug_end();
         Ok(())
    }

    fn parse_relational_expression(&mut self) -> Result<(),ParseError>
    {
         self.debug("parse_relational_expression");
         self.parse_additive_expression()?;        
         match self.symbol.as_ref().map(|symbol| symbol.get_token()) 
         {
             Some(Token::EQU)  => 
             {
                 self.skip(Token::EQU)?;
                 self.parse_additive_expression()?;
             },
             Some(Token::NEQ)  => 
             {
                self.skip(Token::NEQ)?;
                self.parse_additive_expression()?;
             },
             Some(Token::LEQ) =>
             {
                self.skip(Token::LEQ)?;
                self.parse_additive_expression()?;
             },
             Some(Token::GEQ) =>
             {
                self.skip(Token::GEQ)?;
                self.parse_additive_expression()?;
             },
             Some(Token::LTH) =>
             {
                self.skip(Token::LTH)?;
                self.parse_additive_expression()?;
             },
             Some(Token::GTH) =>
             {
                self.skip(Token::GTH)?;
                self.parse_additive_expression()?;
             },
             _ => {},
         }
         self.debug_end();
         Ok(())
    }

    fn parse_additive_expression(&mut self) -> Result<(),ParseError>
    {
        self.debug("parse_additive_expression");
        self.parse_multiplicative_expression()?;
        self.parse_additive_expression_rest()?;
        self.debug_end();
        Ok(())
    }

    fn parse_additive_expression_rest(&mut self) -> Result<(),ParseError>
    {
        self.debug("parse_additive_expression_rest");
        match self.symbol.as_ref().map(|symbol| symbol.get_token())
        {
            Some(Token::ADD) => 
            {
                self.skip(Token::ADD)?;
                self.parse_multiplicative_expression()?;
                self.parse_additive_expression_rest()?
            },
            Some(Token::SUB) =>
            {
                self.skip(Token::SUB)?;
                self.parse_multiplicative_expression()?;
                self.parse_additive_expression_rest()?;
            }
            _ => {},
        }
        self.debug_end();
        Ok(())
    }


    fn parse_multiplicative_expression(&mut self) -> Result<(),ParseError>
    {
        self.debug("parse_multiplicative_expression");
        self.parse_prefix_expression()?;
        self.parse_multiplicative_expression_rest()?;
        self.debug_end();
        Ok(())
    }

    fn parse_multiplicative_expression_rest(&mut self) -> Result<(),ParseError>
    {
        self.debug("parse_multiplicative_expression_rest");
        match self.symbol.as_ref().map(|symbol| symbol.get_token()) 
        {
            Some(Token::MUL) =>
            {
                self.skip(Token::MUL)?;
                self.parse_prefix_expression()?;
                self.parse_multiplicative_expression_rest()?;
            },
            Some(Token::DIV) =>
            {
                self.skip(Token::DIV)?;
                self.parse_prefix_expression()?;
                self.parse_multiplicative_expression_rest()?;
            },
            Some(Token::MOD)  =>
            {
                self.skip(Token::MOD)?;
                self.parse_prefix_expression()?;
                self.parse_multiplicative_expression_rest()?;
            },
            _ => {},
        }
        self.debug_end();
        Ok(())
    }

    fn parse_prefix_expression(&mut self) -> Result<(),ParseError>
    {
        self.debug("parse_prefix_expression");
        match self.symbol.as_ref().map(|symbol| symbol.get_token())
        {
            Some(Token::ADD)  => 
            {
                self.skip(Token::ADD)?;
                self.parse_prefix_expression()?;
            },
            Some(Token::SUB)  =>
            {
                self.skip(Token::SUB)?;
                self.parse_prefix_expression()?;
            },
            Some(Token::MUL) =>
            {
                self.skip(Token::MUL)?;
                self.parse_prefix_expression()?;
            },
            Some(Token::AND) =>
            {
                self.skip(Token::AND)?;
                self.parse_prefix_expression()?;
            },
            Some(Token::NOT) =>
            {
                self.skip(Token::NOT)?;
                self.parse_prefix_expression()?;
            },
            _ => {self.parse_postfix_expression()?;},
        }
        self.debug_end();
        Ok(())
    }

    fn parse_postfix_expression(&mut self) -> Result<(), ParseError> 
    {
        self.debug("parse_postfix_expression"); 
        match self.symbol.as_ref().map(|symbol| symbol.get_token())
        {
            Some(Token::INTCONST) => 
            {
                let int_const = self.skip(Token::INTCONST)?;
                self.parse_postfix_expression_rest()?;
            },
            Some(Token::REALCONST)  => 
            {
                let real_const = self.skip(Token::REALCONST)?;
                self.parse_postfix_expression_rest()?;
            },
            Some(Token::BOOLCONST) => 
            {
                let bool_const = self.skip(Token::BOOLCONST)?;
                self.parse_postfix_expression_rest()?;
            }, 
            Some(Token::STRINGCONST) => 
            {
                let string_const = self.skip(Token::STRINGCONST)?;
                self.parse_postfix_expression_rest()?;
            },
            Some(Token::IDENTIFIER) => 
            {
                let identifier = self.skip(Token::IDENTIFIER)?;
                match self.symbol.as_ref().map(|symbol| symbol.get_token()) 
                {
                    Some(Token::LPARENT) => 
                    {
                        self.skip(Token::LPARENT)?;
                        self.parse_expressions()?;
                        self.skip(Token::RPARENT)?;
                    }
                    _ => {},
                }
                self.parse_postfix_expression_rest()?;
            },
            Some(Token::LPARENT) =>
            {
                self.skip(Token::LPARENT)?;
                self.parse_expressions()?;
                self.skip(Token::RPARENT)?;
                self.parse_postfix_expression_rest()?;
            },
            Some(Token::LBRACE) => 
            {
                self.skip(Token::LBRACE)?;
                self.parse_postfix_brace_expression()?;
                self.parse_postfix_expression_rest()?;
            }, 
            _ => 
            {
                return  Err(ParseError::SyntaxError(SymbolError::new(self.symbol.take())));
            },
        }
        self.debug_end();
        Ok(())
    }

    fn parse_postfix_expression_rest(&mut self) -> Result<(),ParseError>
    {
         self.debug("parse_postfix_expression_rest");
         match self.symbol.as_ref().map(|symbol| symbol.get_token()) 
         {
             Some(Token::DOT) =>
             {
                 self.skip(Token::DOT)?;
                 let identifier = self.skip(Token::IDENTIFIER)?;
                 self.parse_postfix_expression_rest()?;
             },
             Some(Token::LBRACKET) => 
             {
                 self.skip(Token::LBRACKET)?;
                 self.parse_expression()?;
                 self.skip(Token::RBRACKET)?;
                 self.parse_postfix_expression_rest()?;
             },
             Some(Token::WHERE) => 
             {
                 self.skip(Token::WHERE)?;
                 self.parse_declarations()?;
                 self.parse_postfix_expression_rest()?;
             },
             _ => {},
         }
         self.debug_end();
         Ok(())
    }

     fn parse_postfix_brace_expression(&mut self) -> Result<(),ParseError>
    {
        self.debug("parse_postfix_brace_expression");
        match self.symbol.as_ref().map(|symbol| symbol.get_token()) 
        {
            Some(Token::RBRACE)  => 
            {
                self.skip(Token::RBRACE)?;
            },
            Some(Token::IDENTIFIER) =>
            {
                let identifier = self.skip(Token::IDENTIFIER)?;
                self.skip(Token::ASSIGN)?;
                self.parse_expression()?;
                self.skip(Token::RBRACE)?;
            },
            Some(Token::IF) =>
            {
                self.skip(Token::IF)?;
                self.parse_expression()?;
                self.skip(Token::THEN)?;
                self.parse_expressions()?;
                match self.symbol.as_ref().map(|symbol| symbol.get_token())
                {
                    Some(Token::ELSE)  => 
                    {
                        self.skip(Token::ELSE)?;
                        self.parse_expressions()?;
                    },
                    _ => {},
                }
                self.skip(Token::RBRACE)?;
            },
            Some(Token::FOR)  =>
            {
                self.skip(Token::FOR)?;
                let identifier = self.skip(Token::IDENTIFIER)?;
                self.skip(Token::ASSIGN)?;
                self.parse_expression()?;
                self.skip(Token::COMMA)?;
                self.parse_expression()?;
                self.skip(Token::COLON)?;
                self.parse_expressions()?;
                self.skip(Token::RBRACE)?;
            },
            Some(Token::WHILE) => 
            {
                self.skip(Token::WHILE)?;
                self.parse_expression()?;
                self.skip(Token::COLON)?;
                self.parse_expressions()?;
                self.skip(Token::RBRACE)?;
            },
            _ => 
            {
                return Err(ParseError::SyntaxError(SymbolError::new(self.symbol.take())));
            },
        }
        self.debug_end();
        Ok(())
    }
    
    fn parse_declarations(&mut self) -> Result<(),ParseError>
    {
        self.debug("parse_declarations");
        self.parse_declaration()?;
        self.parse_declarations_rest()?;
        self.debug_end();
        Ok(())
    }

    fn parse_declarations_rest(&mut self) -> Result<(),ParseError>
    {
        self.debug("parse_declarations_rest");
        match self.symbol.as_ref().map(|symbol| symbol.get_token()) 
        {
            Some(token) if token == Token::TYP || token == Token::FUN || token == Token::VAR =>
            {
                self.parse_declaration()?;
                self.parse_declarations_rest()?;
            },
            _ => {},
        }
        self.debug_end();
        Ok(())
    }

    fn parse_declaration(&mut self) -> Result<(),ParseError>
    {
        self.debug("parse_declaration");
        match self.symbol.as_ref().map(|symbol| symbol.get_token())
        {
            Some(Token::TYP) => 
            {
                self.parse_type_declaration()?;
            },
            Some(Token::FUN)  => 
            {
                self.parse_function_declaration()?;
            }
            Some(Token::VAR) =>
            {
                self.parse_variable_declaration()?;
            }
            _ => return Err(ParseError::SyntaxError(SymbolError::new(self.symbol.take()))), 
        }
        self.debug_end();
        Ok(())
    }

     fn parse_type_declaration(&mut self) -> Result<(),ParseError>
    {
        self.debug("parse_type_declaration");
        self.skip(Token::TYP)?;
        let identifier = self.skip(Token::IDENTIFIER)?;
        self.skip(Token::COLON)?;
        //let type =
        self.parse_type()?;
        self.skip(Token::SEMIC)?;
        self.debug_end();
        Ok(())
    }

    fn parse_type(&mut self) -> Result<(),ParseError>
    {
        self.debug("parse_type");
        match self.symbol.as_ref().map(|symbol| symbol.get_token()) 
        {
            Some(Token::INT) => 
            {
                self.skip(Token::INT)?;
            },
            Some(Token::REAL) => 
            {
                self.skip(Token::REAL)?;
            },
            Some(Token::BOOL) => 
            {
                self.skip(Token::BOOL)?;
            },
            Some(Token::STRING) =>
            {
                self.skip(Token::STRING)?;
            },
            Some(Token::LBRACE) => 
            {
                self.skip(Token::LBRACE)?;
                self.skip(Token::RBRACE)?;
                //return void type
            },
            Some(Token::IDENTIFIER) => 
            {
                let identifier = self.skip(Token::IDENTIFIER)?;
            },
            Some(Token::MUL)  => 
            {
                self.skip(Token::MUL)?;
                self.parse_type()?;
                //return pointer type
            },
            Some(Token::ARR) => 
            {
                self.skip(Token::ARR)?;
                self.skip(Token::LBRACKET)?;
                self.parse_expression()?;
                self.skip(Token::RBRACKET)?;
                self.parse_type()?;
            },
            Some(Token::REC) => 
            {
                self.skip(Token::REC)?;
                self.skip(Token::LPARENT)?;
                self.parse_record_compoments()?;
                self.skip(Token::RPARENT)?;
            },
            Some(Token::LPARENT) => 
            {
                self.skip(Token::LPARENT)?;
                self.parse_type()?;
                self.skip(Token::RPARENT)?;
            },
            _ => 
            {
                 return Err(ParseError::SyntaxError(SymbolError::new(self.symbol.take()))); 
            },
        }
        self.debug_end();
        Ok(())
    }
    
    fn parse_record_compoments(&mut self) -> Result<(),ParseError>
    {
        self.debug("parse_record_compoments");
        self.parse_record_compoment()?;
        self.parse_record_compoments_rest()?;
        self.debug_end();
        Ok(())
    }

    fn parse_record_compoments_rest(&mut self) -> Result<(),ParseError>
    {
        self.debug("parse_record_compoments_rest");
        match self.symbol.as_ref().map(|symbol|  symbol.get_token())
        {
            Some(Token::COMMA)  => 
            {
                self.skip(Token::COMMA)?;
                self.parse_record_compoment()?;
                self.parse_record_compoments_rest()?;
            },
            _ => {},
        }
        self.debug_end();
        Ok(())
    }

    fn parse_record_compoment(&mut self) -> Result<(),ParseError>
    {
        self.debug("parse_record_compoment");
        match self.symbol.as_ref().map(|symbol| symbol.get_token()) 
        {
            Some(Token::IDENTIFIER) => 
            {
                let identifier = self.skip(Token::IDENTIFIER)?;
                self.skip(Token::COLON)?;
                self.parse_type()?;
            },
            _ => 
            {
                return Err(ParseError::SyntaxError(SymbolError::new(self.symbol.take())));
            },
        }
        self.debug_end();
        Ok(())
    }

    fn parse_function_declaration(&mut self) -> Result<(),ParseError>
    {
        self.debug("parse_function_declaration");
        self.skip(Token::FUN)?;
        let identifier = self.skip(Token::IDENTIFIER)?;
        self.skip(Token::LPARENT)?;
        self.parse_function_parameters()?;
        self.skip(Token::RPARENT)?;
        self.skip(Token::COLON)?;
        self.parse_type()?;
        self.skip(Token::ASSIGN)?;
        self.parse_expressions()?;
        self.skip(Token::SEMIC)?;
        self.debug_end();
        Ok(())
    }

    fn parse_function_parameters(&mut self) -> Result<(),ParseError>
    {
        self.debug("parse_function_parameters");
        self.parse_function_parameter()?;
        self.parse_function_parameters_rest()?;
        self.debug_end();
        Ok(())
    }

    fn parse_function_parameter(&mut self) -> Result<(),ParseError>
    {
        self.debug("parse_function_parameter");
        let identifier = self.skip(Token::IDENTIFIER)?;
        self.skip(Token::COLON)?;
        self.parse_type()?;
        self.debug_end();
        Ok(())
    }

    fn parse_function_parameters_rest(&mut self) -> Result<(),ParseError>
    {
        self.debug("parse_function_parameters_rest");
        match self.symbol.as_ref().map(|symbol| symbol.get_token()) 
        {
            Some(Token::COMMA) => 
            {
                self.skip(Token::COMMA)?;
                self.parse_function_parameter()?;
                self.parse_function_parameters_rest()?;
            },
            _ =>  {}, //return Err(ParseError::SyntaxError(SymbolError::new(self.symbol.take()))),
        }
        self.debug_end();
        Ok(())
    }

    fn parse_variable_declaration(&mut self) -> Result<(),ParseError>
    {
        self.debug("parse_variable_declaration");
        self.skip(Token::VAR)?;
        let identifier = self.skip(Token::IDENTIFIER)?;
        self.skip(Token::COLON)?;
        self.parse_type()?;
        self.skip(Token::SEMIC)?;
        self.debug_end();
        Ok(())
    }


    /*
     fn parse__expression(&mut self) -> Result<(),ParseError>
    {
        self.debug("parse_and_expression");
        
        self.debug_end();
        Ok(())
    }
    */

    

    fn skip(&mut self, token : Token) -> Result<Symbol, ParseError> 
    {
        //let skipped_symbol = self.symbol.take();
        match self.symbol.take()
        {
            None => return Err(ParseError::SyntaxError(SymbolError::new(None))),
            Some(symbol) =>
            {
                 if !symbol.is_token(token) { return Err(ParseError::SyntaxError(SymbolError::new(Some(symbol)))); }
                 self.xml_creator.as_mut().map(|xml| symbol.to_xml(xml));   
                 self.symbol =  self.lexical_analyser.get_next_symbol()?;
                 Ok(symbol)
            }
        }
    }    

    fn debug(&mut self, nontermial : &str) 
    {
        match self.xml_creator 
        {
            Some(ref mut xml) => 
            {
                writeln!(xml, "<production>").unwrap();
                writeln!(xml, "<leftside nonterminal=\"{}\"/>", nontermial).unwrap();
                writeln!(xml, "<rightside>").unwrap();
            },
            None => {},
        }
    }
    
    fn debug_end(&mut self)
    {
        match self.xml_creator 
        {
            Some(ref mut xml) => 
            {
                writeln!(xml, "</rightside>").unwrap();
                writeln!(xml, "</production>").unwrap();
            },
            None => {},
        }
    }
    

}