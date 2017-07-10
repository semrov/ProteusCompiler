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

     fn parse_expressions_rest(&mut self, abs_exprs_arg : Box<AbsExprs>) -> Result<Box<AbsExprs>,ParseError> 
    {
        self.debug("parse_expressions_rest");
        let mut abs_exprs = abs_exprs_arg;
        match self.symbol.as_ref().map(|symbol| symbol.get_token()) 
        {
            Some(Token::COMMA)  => 
            {
                self.skip(Token::COMMA)?;
                abs_exprs.exprs.push(self.parse_expression()?);
                abs_exprs = self.parse_expressions_rest(abs_exprs)?;
            },
            _ => {},
        }
        self.debug_end();
        Ok(abs_exprs)
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
                let abs_bin_expr = Box::new(AbsBinExpr::new(AbsBinOper::OR,abs_expr,abs_right_expr));
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
                 let abs_bin_expr = Box::new(AbsBinExpr::new(AbsBinOper::AND,abs_expr,rel_expr));
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
             },
             Some(Token::NEQ)  => 
             {
                self.skip(Token::NEQ)?;
                let right_expr = self.parse_additive_expression()?;
                expr = Box::new(AbsBinExpr::new(AbsBinOper::NEQ, expr, right_expr));
             },
             Some(Token::LEQ) =>
             {
                self.skip(Token::LEQ)?;
                let right_expr = self.parse_additive_expression()?;
                expr = Box::new(AbsBinExpr::new(AbsBinOper::LEQ, expr, right_expr));
             },
             Some(Token::GEQ) =>
             {
                self.skip(Token::GEQ)?;
                let right_expr = self.parse_additive_expression()?;
                expr = Box::new(AbsBinExpr::new(AbsBinOper::GEQ, expr, right_expr));
             },
             Some(Token::LTH) =>
             {
                self.skip(Token::LTH)?;
                let right_expr = self.parse_additive_expression()?;
                expr = Box::new(AbsBinExpr::new(AbsBinOper::LTH, expr, right_expr));
             },
             Some(Token::GTH) =>
             {
                self.skip(Token::GTH)?;
                let right_expr = self.parse_additive_expression()?;
                expr = Box::new(AbsBinExpr::new(AbsBinOper::GTH, expr, right_expr));
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

    fn parse_additive_expression_rest(&mut self, abs_expr_arg : Box<AbsExpr>) -> Result<Box<AbsExpr>,ParseError>
    {
        self.debug("parse_additive_expression_rest");
        let mut expr = abs_expr_arg;
        match self.symbol.as_ref().map(|symbol| symbol.get_token())
        {
            Some(Token::ADD) => 
            {
                self.skip(Token::ADD)?;
                let mult_expr = self.parse_multiplicative_expression()?;
                let abs_bin_expr = Box::new(AbsBinExpr::new(AbsBinOper::ADD,expr,mult_expr));
                expr = self.parse_additive_expression_rest(abs_bin_expr)?
            },
            Some(Token::SUB) =>
            {
                self.skip(Token::SUB)?;
                let mult_expr = self.parse_multiplicative_expression()?;
                let abs_bin_expr = Box::new(AbsBinExpr::new(AbsBinOper::SUB,expr,mult_expr));
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

    fn parse_multiplicative_expression_rest(&mut self, abs_expr_arg : Box<AbsExpr>) -> Result<Box<AbsExpr>,ParseError>
    {
        self.debug("parse_multiplicative_expression_rest");
        let mut expr = abs_expr_arg;
        match self.symbol.as_ref().map(|symbol| symbol.get_token()) 
        {
            Some(Token::MUL) =>
            {
                self.skip(Token::MUL)?;
                let pref_expr = self.parse_prefix_expression()?;
                let abs_bin_expr = Box::new(AbsBinExpr::new(AbsBinOper::MUL,expr,pref_expr));
                expr = self.parse_multiplicative_expression_rest(abs_bin_expr)?;
            },
            Some(Token::DIV) =>
            {
                self.skip(Token::DIV)?;
                let pref_expr = self.parse_prefix_expression()?;
                let abs_bin_expr = Box::new(AbsBinExpr::new(AbsBinOper::DIV,expr,pref_expr));
                expr = self.parse_multiplicative_expression_rest(abs_bin_expr)?;
            },
            Some(Token::MOD)  =>
            {
                self.skip(Token::MOD)?;
                let pref_expr = self.parse_prefix_expression()?;
                let abs_bin_expr = Box::new(AbsBinExpr::new(AbsBinOper::MOD,expr,pref_expr));
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
                let abs_un_expr = Box::new(AbsUnExpr::new(AbsUnOper::ADD,expr));
                abs_un_expr
            },
            Some(Token::SUB)  =>
            {
                self.skip(Token::SUB)?;
                let expr = self.parse_prefix_expression()?;
                let abs_un_expr = Box::new(AbsUnExpr::new(AbsUnOper::SUB,expr));
                abs_un_expr
            },
            Some(Token::MUL) =>
            {
                self.skip(Token::MUL)?;
                let expr = self.parse_prefix_expression()?;
                let abs_un_expr = Box::new(AbsUnExpr::new(AbsUnOper::MUL,expr));
                abs_un_expr
            },
            Some(Token::AND) =>
            {
                self.skip(Token::AND)?;
                let expr = self.parse_prefix_expression()?;
                let abs_un_expr = Box::new(AbsUnExpr::new(AbsUnOper::AND,expr));
                abs_un_expr
            },
            Some(Token::NOT) =>
            {
                self.skip(Token::NOT)?;
                let expr = self.parse_prefix_expression()?;
                let abs_un_expr = Box::new(AbsUnExpr::new(AbsUnOper::NOT,expr));
                abs_un_expr
            },
            _ => {self.parse_postfix_expression()?},
        };
        self.debug_end();
        Ok(expr)
    }

    fn parse_postfix_expression(&mut self) -> Result<Box<AbsExpr>, ParseError> 
    {
        self.debug("parse_postfix_expression"); 
        let expr : Box<AbsExpr> = match self.symbol.as_ref().map(|symbol| symbol.get_token())
        {
            Some(Token::INTCONST) => 
            {
                let atom_expr = Box::new(AbsAtomExpr::new(self.skip(Token::INTCONST)?));
                self.parse_postfix_expression_rest(atom_expr)?
            },
            Some(Token::REALCONST)  => 
            {
                let atom_expr = Box::new(AbsAtomExpr::new(self.skip(Token::REALCONST)?));
                self.parse_postfix_expression_rest(atom_expr)?
            },
            Some(Token::BOOLCONST) => 
            {
                let atom_expr = Box::new(AbsAtomExpr::new(self.skip(Token::BOOLCONST)?));
                self.parse_postfix_expression_rest(atom_expr)?
            }, 
            Some(Token::STRINGCONST) => 
            {
                let atom_expr = Box::new(AbsAtomExpr::new(self.skip(Token::STRINGCONST)?));
                self.parse_postfix_expression_rest(atom_expr)?
            },
            Some(Token::IDENTIFIER) => 
            {
                let identifier = self.skip(Token::IDENTIFIER)?;
                let abs_expr_name = AbsExprName::new(identifier);
                let iden_expr : Box<AbsExpr> =  match self.symbol.as_ref().map(|symbol| symbol.get_token()) 
                {
                    Some(Token::LPARENT) => 
                    {
                        self.skip(Token::LPARENT)?;
                        let fun_call_params = self.parse_expressions()?;
                        //fun_call_params.calculate_abs_position();
                        self.skip(Token::RPARENT)?;
                        let abs_fun_call = Box::new(AbsFunCall::new(abs_expr_name,*fun_call_params));
                        abs_fun_call
                    }
                    _ => 
                    {
                        Box::new(abs_expr_name)
                    },
                };
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
        };
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
                 let abs_bin_expr = Box::new(AbsBinExpr::new(AbsBinOper::REC,expr,identifier));
                 expr = self.parse_postfix_expression_rest(abs_bin_expr)?;
             },
             Some(Token::LBRACKET) => 
             {
                 self.skip(Token::LBRACKET)?;
                 let offset_expr = self.parse_expression()?;
                 self.skip(Token::RBRACKET)?;
                 let abs_bin_expr = Box::new(AbsBinExpr::new(AbsBinOper::ARR,expr,offset_expr));
                 expr = self.parse_postfix_expression_rest(abs_bin_expr)?;
             },
             Some(Token::WHERE) => 
             {
                 self.skip(Token::WHERE)?;
                 let decls = self.parse_declarations()?;
                 let where_expr = Box::new(AbsWhereExpr::new(expr,decls));
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
        let expr : Box<AbsExpr> = match self.symbol.as_ref().map(|symbol| symbol.get_token()) 
        {
            Some(Token::RBRACE)  => 
            {
                self.skip(Token::RBRACE)?;
                let atom_expr = Box::new(AbsAtomExpr::new_with_option(None));
                atom_expr
            },
            Some(Token::IDENTIFIER) =>
            {
                let expr_name =  Box::new(AbsExprName::new(self.skip(Token::IDENTIFIER)?));
                self.skip(Token::ASSIGN)?;
                let right_expr = self.parse_expression()?;
                let assign_stmt = Box::new(AbsAssignStmt::new(expr_name,right_expr));
                self.skip(Token::RBRACE)?;
                assign_stmt
            },
            Some(Token::IF) =>
            {
                self.skip(Token::IF)?;
                let cond_expr = self.parse_expression()?;
                self.skip(Token::THEN)?;
                let then_expr = self.parse_expressions()?;
                let else_expr : Option<Box<AbsExpr>> = match self.symbol.as_ref().map(|symbol| symbol.get_token())
                {
                    Some(Token::ELSE)  => 
                    {
                        self.skip(Token::ELSE)?;
                        Some(self.parse_expressions()?)
                    },
                    _ => {None},
                };
                self.skip(Token::RBRACE)?;
                let if_stmt = Box::new(AbsIfStmt::new(cond_expr,then_expr,else_expr));
                if_stmt
            },
            Some(Token::FOR)  =>
            {
                self.skip(Token::FOR)?;
                let var_name = AbsExprName::new(self.skip(Token::IDENTIFIER)?);
                self.skip(Token::ASSIGN)?;
                let lower_bound = self.parse_expression()?;
                self.skip(Token::COMMA)?;
                let higher_bound = self.parse_expression()?;
                self.skip(Token::COLON)?;
                let loop_exprs = self.parse_expressions()?;
                self.skip(Token::RBRACE)?;
                let for_loop_expr = Box::new(AbsForStmt::new(var_name,lower_bound,higher_bound,loop_exprs));
                for_loop_expr
            },
            Some(Token::WHILE) => 
            {
                self.skip(Token::WHILE)?;
                let cond_expr = self.parse_expression()?;
                self.skip(Token::COLON)?;
                let loop_expr = self.parse_expressions()?;
                self.skip(Token::RBRACE)?;
                let while_expr = Box::new(AbsWhileStmt::new(cond_expr,loop_expr));
                while_expr
            },
            _ => 
            {
                return Err(ParseError::SyntaxError(SymbolError::new(self.symbol.take())));
            },
        };
        self.debug_end();
        Ok(expr)
    }
    
    fn parse_declarations(&mut self) -> Result<AbsDecls,ParseError>
    {
        self.debug("parse_declarations");
        let mut abs_decls = AbsDecls::new();
        let decl = self.parse_declaration()?;
        abs_decls.decls.push(decl);
        self.parse_declarations_rest(&mut abs_decls)?;
        abs_decls.calculate_abs_position();
        self.debug_end();
        Ok(abs_decls)
    }

    fn parse_declarations_rest(&mut self, abs_decls : &mut AbsDecls) -> Result<(),ParseError>
    {
        self.debug("parse_declarations_rest");
        match self.symbol.as_ref().map(|symbol| symbol.get_token()) 
        {
            Some(token) if token == Token::TYP || token == Token::FUN || token == Token::VAR =>
            {
                let decl = self.parse_declaration()?;
                abs_decls.decls.push(decl);
                self.parse_declarations_rest(abs_decls)?;
            },
            _ => {}, 
        }
        self.debug_end();
        Ok(())
    }

    fn parse_declaration(&mut self) -> Result<Box<AbsDecl>,ParseError>
    {
        self.debug("parse_declaration");
        let decl : Box<AbsDecl> = match self.symbol.as_ref().map(|symbol| symbol.get_token())
        {
            Some(Token::TYP) => 
            {
                self.parse_type_declaration()?
            },
            Some(Token::FUN)  => 
            {
                self.parse_function_declaration()?
            }
            Some(Token::VAR) =>
            {
                self.parse_variable_declaration()?
            }
            _ => return Err(ParseError::SyntaxError(SymbolError::new(self.symbol.take()))), 
        };
        self.debug_end();
        Ok(decl)
    }

     fn parse_type_declaration(&mut self) -> Result<Box<AbsTypeDecl>,ParseError>
    {
        self.debug("parse_type_declaration");
        self.skip(Token::TYP)?;
        let type_name = AbsTypeName::new(self.skip(Token::IDENTIFIER)?);
        self.skip(Token::COLON)?;
        let source_type = self.parse_type()?;
        self.skip(Token::SEMIC)?;
        let abs_type_decl = Box::new(AbsTypeDecl::new(type_name,source_type));
        self.debug_end();
        Ok(abs_type_decl)
    }

    fn parse_type(&mut self) -> Result<Box<AbsType>,ParseError>
    {
        self.debug("parse_type");
        let abs_type : Box<AbsType> = match self.symbol.as_ref().map(|symbol| symbol.get_token()) 
        {
            Some(Token::INT) => 
            {
                let symbol = self.skip(Token::INT)?;
                let atom_type = Box::new(AbsAtomType::new(AtomType::INT,&symbol));
                atom_type
            },
            Some(Token::REAL) => 
            {
                let symbol = self.skip(Token::REAL)?;
                let atom_type = Box::new(AbsAtomType::new(AtomType::REAL,&symbol));
                atom_type
            },
            Some(Token::BOOL) => 
            {
                let symbol = self.skip(Token::BOOL)?;
                let mut atom_type = Box::new(AbsAtomType::new(AtomType::BOOL,&symbol));
                atom_type
            },
            Some(Token::STRING) =>
            {
                let symbol = self.skip(Token::STRING)?;
                let mut atom_type = Box::new(AbsAtomType::new(AtomType::STRING,&symbol));
                atom_type
            },
            Some(Token::LBRACE) => 
            {
                //returns void type
                self.skip(Token::LBRACE)?;
                self.skip(Token::RBRACE)?;
                Box::new(AbsAtomType::new_void_type())
            },
            Some(Token::IDENTIFIER) => 
            {
                let mut type_name = Box::new(AbsTypeName::new(self.skip(Token::IDENTIFIER)?));
                type_name
            },
            Some(Token::MUL)  => 
            {
                //returns pointer type
                let pointer_symbol = self.skip(Token::MUL)?;
                let abs_type = self.parse_type()?;
                let abs_pointer_type = Box::new(AbsPointerType::new(abs_type,&pointer_symbol));
                abs_pointer_type
            },
            Some(Token::ARR) => 
            {
                let arr_symbol = self.skip(Token::ARR)?;
                self.skip(Token::LBRACKET)?;
                let size_expr = self.parse_expression()?;
                self.skip(Token::RBRACKET)?;
                let arr_type =self.parse_type()?;
                let array = Box::new(AbsArrType::new(arr_type,size_expr,&arr_symbol));
                array
            },
            Some(Token::REC) => 
            {
                let rec_symbol = self.skip(Token::REC)?;
                self.skip(Token::LPARENT)?;
                let record_compoments = self.parse_record_compoments()?;
                self.skip(Token::RPARENT)?;
                let mut record = Box::new(AbsRecType::new(record_compoments,&rec_symbol));
                record
            },
            Some(Token::LPARENT) => 
            {
                self.skip(Token::LPARENT)?;
                let abs_type = self.parse_type()?;
                self.skip(Token::RPARENT)?;
                abs_type
            },
            _ => 
            {
                 return Err(ParseError::SyntaxError(SymbolError::new(self.symbol.take()))); 
            },
        };
        self.debug_end();
        Ok(abs_type)
    }
    
    fn parse_record_compoments(&mut self) -> Result<AbsDecls,ParseError>
    {
        self.debug("parse_record_compoments");
        let compoment = self.parse_record_compoment()?;
        let mut abs_decls = AbsDecls::new();
        abs_decls.decls.push(compoment);
        self.parse_record_compoments_rest(&mut abs_decls)?;
        abs_decls.calculate_abs_position();
        self.debug_end();
        Ok(abs_decls)
    }

    fn parse_record_compoments_rest(&mut self, abs_decls : &mut AbsDecls) -> Result<(),ParseError>
    {
        self.debug("parse_record_compoments_rest");
        match self.symbol.as_ref().map(|symbol|  symbol.get_token())
        {
            Some(Token::COMMA)  => 
            {
                self.skip(Token::COMMA)?;
                let compoment = self.parse_record_compoment()?;
                abs_decls.decls.push(compoment);
                self.parse_record_compoments_rest(abs_decls)?;
            },
            _ => {},
        }
        self.debug_end();
        Ok(())
    }

    fn parse_record_compoment(&mut self) -> Result<Box<AbsVarDecl>,ParseError>
    {
        self.debug("parse_record_compoment");
        let rec_comp = match self.symbol.as_ref().map(|symbol| symbol.get_token()) 
        {
            Some(Token::IDENTIFIER) => 
            {
                let identifier = AbsExprName::new(self.skip(Token::IDENTIFIER)?);
                self.skip(Token::COLON)?;
                let abs_type = self.parse_type()?;
                let abs_var_decl = Box::new(AbsVarDecl::new(identifier,abs_type));
                abs_var_decl
            },
            _ => 
            {
                return Err(ParseError::SyntaxError(SymbolError::new(self.symbol.take())));
            },
        };
        self.debug_end();
        Ok(rec_comp)
    }

    fn parse_function_declaration(&mut self) -> Result<Box<AbsFunDecl>,ParseError>
    {
        self.debug("parse_function_declaration");
        let fun_symbol = self.skip(Token::FUN)?;
        let fun_name = AbsExprName::new(self.skip(Token::IDENTIFIER)?);
        self.skip(Token::LPARENT)?;
        let params = self.parse_function_parameters()?;
        self.skip(Token::RPARENT)?;
        self.skip(Token::COLON)?;
        let return_type = self.parse_type()?;
        self.skip(Token::ASSIGN)?;
        let body_exprs = self.parse_expressions()?;
        self.skip(Token::SEMIC)?;
        let  abs_fun_decl = Box::new(AbsFunDecl::new(fun_name,params,return_type,body_exprs));
        self.debug_end();
        Ok(abs_fun_decl)
    }

    fn parse_function_parameters(&mut self) -> Result<AbsDecls,ParseError>
    {
        self.debug("parse_function_parameters");
        let mut params = AbsDecls::new();
        let arg = self.parse_function_parameter()?;
        params.decls.push(arg);
        self.parse_function_parameters_rest(&mut params)?;
        params.calculate_abs_position();
        self.debug_end();
        Ok(params)
    }

    fn parse_function_parameter(&mut self) -> Result<Box<AbsVarDecl>,ParseError>
    {
        self.debug("parse_function_parameter");
        let  arg_name = AbsExprName::new(self.skip(Token::IDENTIFIER)?);
        self.skip(Token::COLON)?;
        let arg_type = self.parse_type()?;
        let arg = Box::new(AbsVarDecl::new(arg_name,arg_type));
        self.debug_end();
        Ok(arg)
    }

    fn parse_function_parameters_rest(&mut self, params : &mut AbsDecls) -> Result<(),ParseError>
    {
        self.debug("parse_function_parameters_rest");
        match self.symbol.as_ref().map(|symbol| symbol.get_token()) 
        {
            Some(Token::COMMA) => 
            {
                self.skip(Token::COMMA)?;
                let arg = self.parse_function_parameter()?;
                params.decls.push(arg);
                self.parse_function_parameters_rest(params)?;
            },
            _ =>  {}, //return Err(ParseError::SyntaxError(SymbolError::new(self.symbol.take()))),
        }
        self.debug_end();
        Ok(())
    }

    fn parse_variable_declaration(&mut self) -> Result<Box<AbsVarDecl>,ParseError>
    {
        self.debug("parse_variable_declaration");
        let var_symbol = self.skip(Token::VAR)?;
        let var_name = AbsExprName::new(self.skip(Token::IDENTIFIER)?);
        self.skip(Token::COLON)?;
        let var_type = self.parse_type()?;
        self.skip(Token::SEMIC)?;
        let variable = Box::new(AbsVarDecl::new(var_name,var_type));
        self.debug_end();
        Ok(variable)
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