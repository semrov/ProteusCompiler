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


    pub fn parse(&mut self) -> Result<Option<Box<AbsTree>>, io::Error> 
    {
        self.symbol = match self.lexical_analyser.get_next_symbol()
        {
            Ok(Some(symbol)) => Some(symbol),
            Ok(None) => return Ok(None),
            Err(ioe) => return Err(ioe),
        }; 
        
        let result = self.parse_source();
        match result 
        {
            Ok(expr) => 
            {
                //at the end of parsing, self.symbol must be None
                 if self.symbol.is_some()
                {
                    report::error("Internal error: Syntax check finished, but symbol still available",report::ExitCode::SyntaxAnalyzerSyntaxError);
                }

                return Ok(Some(expr));
            },
            Err(ParseError::IoError(ioe)) => return Err(ioe),
            Err(ParseError::SyntaxError(syerr)) => 
            {
                match syerr.get_ref_symbol() 
                {
                    Some(symbol) => report::error_at_position(&format!("{}",syerr),symbol.get_ref_position(),report::ExitCode::SyntaxAnalyzerSyntaxError),
                    None => report::error(&format!("{}",syerr),report::ExitCode::SyntaxAnalyzerUnexpectedEndOfStream),
                }
                Ok(None)
            }
        }
    }


    fn parse_source(&mut self) -> Result<Box<AbsTree>, ParseError> 
    {
        self.debug("parse_source");
        let abstree = try!(self.parse_expressions());
        self.debug_end();
        Ok(abstree)
    }

    fn parse_expressions(&mut self) -> Result<Box<AbsExprs>,ParseError> 
    {
        self.debug("parse_expressions");
        let mut abs_exprs = Box::new(AbsExprs::new());
        let abs_expr = self.parse_expression()?;
        abs_exprs.exprs.push(abs_expr);
        abs_exprs = self.parse_expressions_rest(abs_exprs)?;
        abs_exprs.calculate_abs_position();
        self.debug_end();
        Ok(abs_exprs)
    }


    fn parse_expression(&mut self) -> Result<Box<AbsExpr>,ParseError> 
    {
        self.debug("parse_expression");
        let abs_expr =self.parse_or_expression()?;
        self.debug_end();
        Ok(abs_expr)
    }

     fn parse_expressions_rest(&mut self, abs_exprs_arg : Box<AbsExprs>) -> Result<AbsExprs,ParseError> 
    {
        self.debug("parse_expressions_rest");
        match self.symbol.as_ref().map(|symbol| symbol.get_token()) 
        {
            Some(Token::COMMA)  => 
            {
                self.skip(Token::COMMA)?;
                let abs_expr = self.parse_expression()?;
                abs_exprs_arg.exprs.push(abs_expr);
                abs_exprs_arg = self.parse_expressions_rest(abs_exprs_arg)?;
            },
            _ => {},
        }
        self.debug_end();
        Ok(abs_exprs_arg)
    }

    fn parse_or_expression(&mut self) -> Result<Box<AbsExpr>, ParseError>
    {
        self.debug("parse_or_expression");
        let abs_expr = self.parse_and_expression()?;
        let expr = self.parse_or_expression_rest(abs_expr)?;
        self.debug_end();
        Ok(expr)
    }

    fn parse_or_expression_rest(&mut self, abs_expr_arg : Box<AbsExpr>) -> Result<Box<AbsExpr>,ParseError>
    {
        let mut abs_expr = abs_expr_arg;
        self.debug("parse_or_expression_rest");
        match self.symbol.as_ref().map(|symbol| symbol.get_token()) 
        {
            Some(Token::OR) =>
            {
                self.skip(Token::OR)?;
                let abs_right_expr = self.parse_and_expression()?;
                let mut abs_bin_expr = Box::new(AbsBinExpr::new(AbsBinOper::OR,abs_expr,abs_right_expr));
                abs_bin_expr.calculate_abs_position();
                abs_expr = self.parse_or_expression_rest(abs_bin_expr)?;
            },
            _ => {},
        }
        self.debug_end();
        Ok(abs_expr)
    }

    fn parse_and_expression(&mut self) -> Result<Box<AbsExpr>,ParseError>
    {
         self.debug("parse_and_expression");
         let rel_expr = self.parse_relational_expression()?;
         let expr = self.parse_and_expression_rest(rel_expr)?;
         self.debug_end();
         Ok(expr)
    }

     fn parse_and_expression_rest(&mut self, abs_expr_arg : Box<AbsExpr>) -> Result<Box<AbsExpr>,ParseError>
    {
         self.debug("parse_and_expression_rest");
         let mut abs_expr = abs_expr_arg;
         match self.symbol.as_ref().map(|symbol|  symbol.get_token())
         {
             Some(Token::AND)  => 
             {
                 self.skip(Token::AND)?;
                 let rel_expr = self.parse_relational_expression()?;
                 let mut abs_bin_expr = Box::new(AbsBinExpr::new(AbsBinOper::AND,abs_expr,rel_expr));
                 abs_bin_expr.calculate_abs_position();
                 abs_expr = self.parse_and_expression_rest(abs_bin_expr)?;
             },
             _ => {},
         }
         self.debug_end();
         Ok(abs_expr)
    }

    fn parse_relational_expression(&mut self) -> Result<Box<AbsExpr>,ParseError>
    {
         self.debug("parse_relational_expression");
         let mut expr = self.parse_additive_expression()?;        
         match self.symbol.as_ref().map(|symbol| symbol.get_token()) 
         {
             Some(Token::EQU)  => 
             {
                 self.skip(Token::EQU)?;
                 let right_expr = self.parse_additive_expression()?;
                 expr = Box::new(AbsBinExpr::new(AbsBinOper::EQU, expr, right_expr));
                 expr.calculate_abs_position();
             },
             Some(Token::NEQ)  => 
             {
                self.skip(Token::NEQ)?;
                let right_expr = self.parse_additive_expression()?;
                expr = Box::new(AbsBinExpr::new(AbsBinOper::NEQ, expr, right_expr));
                expr.calculate_abs_position();
             },
             Some(Token::LEQ) =>
             {
                self.skip(Token::LEQ)?;
                let right_expr = self.parse_additive_expression()?;
                expr = Box::new(AbsBinExpr::new(AbsBinOper::LEQ, expr, right_expr));
                expr.calculate_abs_position();
             },
             Some(Token::GEQ) =>
             {
                self.skip(Token::GEQ)?;
                let right_expr = self.parse_additive_expression()?;
                expr = Box::new(AbsBinExpr::new(AbsBinOper::GEQ, expr, right_expr));
                expr.calculate_abs_position();
             },
             Some(Token::LTH) =>
             {
                self.skip(Token::LTH)?;
                let right_expr = self.parse_additive_expression()?;
                expr = Box::new(AbsBinExpr::new(AbsBinOper::LTH, expr, right_expr));
                expr.calculate_abs_position();
             },
             Some(Token::GTH) =>
             {
                self.skip(Token::GTH)?;
                let right_expr = self.parse_additive_expression()?;
                expr = Box::new(AbsBinExpr::new(AbsBinOper::GTH, expr, right_expr));
                expr.calculate_abs_position();
             },
             _ => {},
         }
         self.debug_end();
         Ok(expr)
    }

    fn parse_additive_expression(&mut self) -> Result<Box<AbsExpr>,ParseError>
    {
        self.debug("parse_additive_expression");
        let mult_expr = self.parse_multiplicative_expression()?;
        let expr = self.parse_additive_expression_rest(mult_expr)?;
        self.debug_end();
        Ok(expr)
    }

    fn parse_additive_expression_rest(&mut self, abs_expr_arg : Box<AbsExpr>) -> Result<(),ParseError>
    {
        self.debug("parse_additive_expression_rest");
        let mut expr = abs_expr_arg;
        match self.symbol.as_ref().map(|symbol| symbol.get_token())
        {
            Some(Token::ADD) => 
            {
                self.skip(Token::ADD)?;
                let mult_expr = self.parse_multiplicative_expression()?;
                let mut abs_bin_expr = Box::new(AbsBinExpr::new(AbsBinOper::ADD,expr,mult_expr));
                abs_bin_expr.calculate_abs_position();
                expr = self.parse_additive_expression_rest(abs_bin_expr)?
            },
            Some(Token::SUB) =>
            {
                self.skip(Token::SUB)?;
                let mult_expr = self.parse_multiplicative_expression()?;
                let mut abs_bin_expr = Box::new(AbsBinExpr::new(AbsBinOper::SUB,expr,mult_expr));
                abs_bin_expr.calculate_abs_position();
                expr = self.parse_additive_expression_rest(abs_bin_expr)?;
            }
            _ => {},
        }
        self.debug_end();
        Ok(expr)
    }


    fn parse_multiplicative_expression(&mut self) -> Result<Box<AbsExpr>,ParseError>
    {
        self.debug("parse_multiplicative_expression");
        let pref_expr = self.parse_prefix_expression()?;
        let expr = self.parse_multiplicative_expression_rest(pref_expr)?;
        self.debug_end();
        Ok(expr)
    }

    fn parse_multiplicative_expression_rest(&mut self, abs_expr_arg : Box<AbsExpr>) -> Result<(),ParseError>
    {
        self.debug("parse_multiplicative_expression_rest");
        let mut expr = abs_expr_arg;
        match self.symbol.as_ref().map(|symbol| symbol.get_token()) 
        {
            Some(Token::MUL) =>
            {
                self.skip(Token::MUL)?;
                let pref_expr = self.parse_prefix_expression()?;
                let mut abs_bin_expr = Box::new(AbsBinExpr::new(AbsBinOper::MUL,expr,pref_expr));
                abs_bin_expr.calculate_abs_position();
                expr = self.parse_multiplicative_expression_rest(abs_bin_expr)?;
            },
            Some(Token::DIV) =>
            {
                self.skip(Token::DIV)?;
                let pref_expr = self.parse_prefix_expression()?;
                let mut abs_bin_expr = Box::new(AbsBinExpr::new(AbsBinOper::DIV,expr,pref_expr));
                abs_bin_expr.calculate_abs_position();
                expr = self.parse_multiplicative_expression_rest(abs_bin_expr)?;
            },
            Some(Token::MOD)  =>
            {
                self.skip(Token::MOD)?;
                let pref_expr = self.parse_prefix_expression()?;
                let mut abs_bin_expr = Box::new(AbsBinExpr::new(AbsBinOper::MOD,expr,pref_expr));
                expr = self.parse_multiplicative_expression_rest(abs_bin_expr)?;
            },
            _ => {},
        }
        self.debug_end();
        Ok(expr)
    }

    fn parse_prefix_expression(&mut self) -> Result<Box<AbsExpr>,ParseError>
    {
        self.debug("parse_prefix_expression");
        let expr = match self.symbol.as_ref().map(|symbol| symbol.get_token())
        {
            Some(Token::ADD)  => 
            {
                self.skip(Token::ADD)?;
                let expr = self.parse_prefix_expression()?;
                let mut abs_un_expr = Box::new(AbsUnExpr::new(AbsUnOper::ADD,expr));
                abs_un_expr.calculate_abs_position();
                abs_un_expr
            },
            Some(Token::SUB)  =>
            {
                self.skip(Token::SUB)?;
                let expr = self.parse_prefix_expression()?;
                let mut abs_un_expr = Box::new(AbsUnExpr::new(AbsUnOper::SUB,expr));
                abs_un_expr.calculate_abs_position();
                abs_un_expr
            },
            Some(Token::MUL) =>
            {
                self.skip(Token::MUL)?;
                let expr = self.parse_prefix_expression()?;
                let mut abs_un_expr = Box::new(AbsUnExpr::new(AbsUnOper::MUL,expr));
                abs_un_expr.calculate_abs_position();
                abs_un_expr
            },
            Some(Token::AND) =>
            {
                self.skip(Token::AND)?;
                let expr = self.parse_prefix_expression()?;
                let mut abs_un_expr = Box::new(AbsUnExpr::new(AbsUnOper::AND,expr));
                abs_un_expr.calculate_abs_position();
                abs_un_expr
            },
            Some(Token::NOT) =>
            {
                self.skip(Token::NOT)?;
                let expr = self.parse_prefix_expression()?;
                let mut abs_un_expr = Box::new(AbsUnExpr::new(AbsUnOper::NOT,expr));
                abs_un_expr.calculate_abs_position();
                abs_un_expr
            },
            _ => {self.parse_postfix_expression()?},
        }
        self.debug_end();
        Ok(expr)
    }

    fn parse_postfix_expression(&mut self) -> Result<Box<AbsExpr>, ParseError> 
    {
        self.debug("parse_postfix_expression"); 
        let expr = match self.symbol.as_ref().map(|symbol| symbol.get_token())
        {
            Some(Token::INTCONST) => 
            {
                let mut atom_expr = Box::new(AbsAtomExpr::new(self.skip(Token::INTCONST)?));
                atom_expr.calculate_abs_position();
                self.parse_postfix_expression_rest(atom_expr)?
            },
            Some(Token::REALCONST)  => 
            {
                let mut atom_expr = Box::new(AbsAtomExpr::new(self.skip(Token::REALCONST)?));
                atom_expr.calculate_abs_position();
                self.parse_postfix_expression_rest(atom_expr)?
            },
            Some(Token::BOOLCONST) => 
            {
                let mut atom_expr = self.skip(Token::BOOLCONST)?;
                atom_expr.calculate_abs_position();
                self.parse_postfix_expression_rest(atom_expr)?
            }, 
            Some(Token::STRINGCONST) => 
            {
                let mut atom_expr = self.skip(Token::STRINGCONST)?;
                atom_expr.calculate_abs_position();
                self.parse_postfix_expression_rest()?
            },
            Some(Token::IDENTIFIER) => 
            {
                let identifier = self.skip(Token::IDENTIFIER)?;
                let mut abs_expr_name = AbsExprName::new(identifier);
                abs_expr_name.calculate_abs_position();
                let iden_expr =  match self.symbol.as_ref().map(|symbol| symbol.get_token()) 
                {
                    Some(Token::LPARENT) => 
                    {
                        self.skip(Token::LPARENT)?;
                        let fun_call_params = self.parse_expressions()?;
                        //fun_call_params.calculate_abs_position();
                        self.skip(Token::RPARENT)?;
                        let mut abs_fun_call = Box::new(AbsFunCall::new(abs_expr_name,*fun_call_params));
                        abs_fun_call.calculate_abs_position();
                        abs_fun_call
                    }
                    _ => 
                    {
                        Box::new(abs_expr_name)
                    },
                }
                self.parse_postfix_expression_rest(iden_expr)?
            },
            Some(Token::LPARENT) =>
            {
                self.skip(Token::LPARENT)?;
                let exprs = self.parse_expressions()?;
                self.skip(Token::RPARENT)?;
                self.parse_postfix_expression_rest(exprs)?
            },
            Some(Token::LBRACE) => 
            {
                self.skip(Token::LBRACE)?;
                let brace_expr = self.parse_postfix_brace_expression()?;
                self.parse_postfix_expression_rest(brace_expr)?
            }, 
            _ => 
            {
                return  Err(ParseError::SyntaxError(SymbolError::new(self.symbol.take())));
            },
        }
        self.debug_end();
        Ok(expr)
    }

    fn parse_postfix_expression_rest(&mut self, abs_expr_arg : Box<AbsExpr>) -> Result<Box<AbsExpr>,ParseError>
    {
         self.debug("parse_postfix_expression_rest");
         let mut expr = abs_expr_arg;
         match self.symbol.as_ref().map(|symbol| symbol.get_token()) 
         {
             Some(Token::DOT) =>
             {
                 self.skip(Token::DOT)?;
                 let identifier = Box::new(AbsExprName::new(self.skip(Token::IDENTIFIER)?));
                 identifier.calculate_abs_position();
                 let mut abs_bin_expr = Box::new(AbsBinExpr::new(AbsBinOper::REC,expr,identifier));
                 abs_bin_expr.calculate_abs_position();
                 expr = self.parse_postfix_expression_rest(abs_bin_expr)?;
             },
             Some(Token::LBRACKET) => 
             {
                 self.skip(Token::LBRACKET)?;
                 let offset_expr = self.parse_expression()?;
                 self.skip(Token::RBRACKET)?;
                 let mut abs_bin_expr = Box::new(AbsBinExpr::new(AbsBinOper::ARR,expr,offset_expr));
                 abs_bin_expr.calculate_abs_position();
                 expr = self.parse_postfix_expression_rest(abs_bin_expr)?;
             },
             Some(Token::WHERE) => 
             {
                 self.skip(Token::WHERE)?;
                 let decls = self.parse_declarations()?;
                 let mut where_expr = Box::new(AbsWhereExpr::new(expr,decls));
                 where_expr.calculate_abs_position();
                 expr = self.parse_postfix_expression_rest(where_expr)?;
             },
             _ => {},
         }
         self.debug_end();
         Ok(expr)
    }

     fn parse_postfix_brace_expression(&mut self) -> Result<Box<AbsExpr>,ParseError>
    {
        self.debug("parse_postfix_brace_expression");
        let expr = match self.symbol.as_ref().map(|symbol| symbol.get_token()) 
        {
            Some(Token::RBRACE)  => 
            {
                self.skip(Token::RBRACE)?;
                let mut atom_expr = Box::new(AbsAtomExpr::new_with_option(None));
                atom_expr.calculate_abs_position();
                atom_expr
            },
            Some(Token::IDENTIFIER) =>
            {
                let expr_name =  Box::new(AbsExprName::new(self.skip(Token::IDENTIFIER)?));
                expr_name.calculate_abs_position();
                self.skip(Token::ASSIGN)?;
                let right_expr = self.parse_expression()?;
                let mut assign_stmt = Box::new(AbsAssignStmt::new(expr_name,right_expr));
                assign_stmt.calculate_abs_position();
                self.skip(Token::RBRACE)?;
                assign_stmt
            },
            Some(Token::IF) =>
            {
                self.skip(Token::IF)?;
                let cond_expr = self.parse_expression()?;
                self.skip(Token::THEN)?;
                let then_expr = self.parse_expressions()?;
                let else_expr = match self.symbol.as_ref().map(|symbol| symbol.get_token())
                {
                    Some(Token::ELSE)  => 
                    {
                        self.skip(Token::ELSE)?;
                        Some(self.parse_expressions()?)
                    },
                    _ => {None},
                }
                self.skip(Token::RBRACE)?;
                let mut if_stmt = Box::new(AbsIfStmt::new(cond_expr,then_expr,else_expr));
                if_stmt.calculate_abs_position();
                if_stmt
            },
            Some(Token::FOR)  =>
            {
                self.skip(Token::FOR)?;
                let mut var_name = AbsExprName::new(self.skip(Token::IDENTIFIER)?);
                var_name.calculate_abs_position();
                self.skip(Token::ASSIGN)?;
                let lower_bound = self.parse_expression()?;
                self.skip(Token::COMMA)?;
                let higher_bound = self.parse_expression()?;
                self.skip(Token::COLON)?;
                let loop_exprs = self.parse_expressions()?;
                self.skip(Token::RBRACE)?;
                let mut for_loop_expr = Box::new(AbsForStmt::new(var_name,lower_bound,higher_bound,loop_exprs));
                for_loop_expr.calculate_abs_position();
                for_loop_expr
            },
            Some(Token::WHILE) => 
            {
                self.skip(Token::WHILE)?;
                let cond_expr = self.parse_expression()?;
                self.skip(Token::COLON)?;
                let loop_expr = self.parse_expressions()?;
                self.skip(Token::RBRACE)?;
                let mut while_expr = Box::new(AbsWhileStmt::new(cond_expr,loop_expr));
                while_expr.calculate_abs_position();
                while_expr
            },
            _ => 
            {
                return Err(ParseError::SyntaxError(SymbolError::new(self.symbol.take())));
            },
        }
        self.debug_end();
        Ok(expr)
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