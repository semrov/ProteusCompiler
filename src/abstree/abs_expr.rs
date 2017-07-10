use abstree::AbsTree;
use abstree::abs_position::AbsPosition;
use abstree::abs_decl::{AbsDecl,AbsDecls};
use abstree::positioner::Positioner;
use abstree::visitor::Visitor;
use lexanal::position::Position;
use lexanal::symbol::Symbol;
use lexanal::symbol::Token;
use report;


pub trait AbsExpr : AbsTree {}


pub struct AbsExprs
{
    pub exprs : Vec<Box<AbsExpr>>,
    abs_position : AbsPosition,
}

impl AbsExprs 
{
    pub fn new() -> Self
    {
        AbsExprs{exprs : Vec::new(), abs_position : AbsPosition::new()}
    }
    pub fn new_with_exprs(exprs : Vec<Box<AbsExpr>>) -> Self
    {
         AbsExprs{exprs : exprs, abs_position : AbsPosition::new()}
    }
    pub fn add_expression(&mut self, expr : Box<AbsExpr>)
    {
        self.exprs.push(expr);
        self.calculate_abs_position();
    }
    pub fn calculate_abs_position(&mut self)
    {
        self.abs_position.set_min(self.exprs[0].get_position_ref().unwrap());
        self.abs_position.set_max(self.exprs[self.exprs.len()-1].get_position_ref().unwrap());
    }
}

impl AbsTree for AbsExprs
{
    fn accept(&self, visitor : &mut Visitor)
    {
        visitor.visit_abs_exprs(self);
    }
}
impl AbsExpr  for AbsExprs {}
impl Positioner for AbsExprs 
{
    //fn get_position(&self) -> Option<Position>{}
    fn get_position_ref(&self) -> Option<&Position> { self.abs_position.get_position_ref() }
    fn get_position_ref_mut(&mut self) -> Option<&mut Position> { self.abs_position.get_position_ref_mut() }
    fn set_min(&mut self, position : &Position) 
    {
        self.abs_position.set_min(position);
    }
    fn set_max(&mut self, position : &Position) 
    {
        self.abs_position.set_max(position);
    }
}

// Atomic expression (constant of basic data types)
pub struct AbsAtomExpr
{
    pub expr : Option<Symbol>,
    abs_position : AbsPosition,
}

impl AbsAtomExpr
{
    fn is_const_type(token : Token) -> bool
    {
        token == Token::BOOLCONST || token == Token::INTCONST  || token == Token::REALCONST 
        || token == Token::STRINGCONST
    }
    pub fn new(expr : Symbol) -> AbsAtomExpr
    {
        if !Self::is_const_type(expr.get_token())
        {
            report::error("Internal error in AbsAtomExpr: expression is not constant!", report::ExitCode::AbstractSyntaxTreeInvalidExpression);
        }
        let mut abs_atom_expr = AbsAtomExpr{expr : Some(expr), abs_position : AbsPosition::new()};
        abs_atom_expr.calculate_abs_position();
        abs_atom_expr
    }

    pub fn calculate_abs_position(&mut self)
    {
        if let Some(ref symbol) = self.expr
        {
            self.abs_position.set_min(symbol.get_ref_position());
            self.abs_position.set_max(symbol.get_ref_position());
        }
    }

    pub fn new_with_option(expr : Option<Symbol>) -> AbsAtomExpr
    {
        match expr 
        {
            None => {},
            Some(ref e) => 
            {
                if !Self::is_const_type(e.get_token())
                {
                    report::error("Internal error in AbsAtomExpr: expression is not constant!", report::ExitCode::AbstractSyntaxTreeInvalidExpression);
                }
            }
        }
        let mut abs_atom_expr = AbsAtomExpr{expr, abs_position : AbsPosition::new()};
        abs_atom_expr.calculate_abs_position();
        abs_atom_expr
    }
    
}

impl AbsTree for AbsAtomExpr
{
    fn accept(&self, visitor : &mut Visitor)
    {
        visitor.visit_abs_atom_expr(self);
    }
}
impl AbsExpr for AbsAtomExpr{}
impl Positioner for AbsAtomExpr
{
    fn get_position_ref(&self) -> Option<&Position> { self.abs_position.get_position_ref() }
    fn get_position_ref_mut(&mut self) -> Option<&mut Position> { self.abs_position.get_position_ref_mut() }
    fn set_min(&mut self, position : &Position) 
    {
        self.abs_position.set_min(position);
    }
    fn set_max(&mut self, position : &Position) 
    {
        self.abs_position.set_max(position);
    }
}

//function's or variable's name
pub struct AbsExprName
{
    abs_position : AbsPosition,
    //name (identifier)
    pub identifier : Symbol,
}

impl AbsExprName
{
    pub fn new(identifier : Symbol) -> AbsExprName
    {
       let mut abs_expr_name = AbsExprName{identifier, abs_position : AbsPosition::new()};
       abs_expr_name.calculate_abs_position();
       abs_expr_name
    }
    pub fn calculate_abs_position(&mut self)
    {
        self.abs_position.set_min(self.identifier.get_ref_position());
        self.abs_position.set_max(self.identifier.get_ref_position());
    }
}


impl AbsTree for AbsExprName 
{
    fn accept(&self, visitor : &mut Visitor)
    {
        visitor.visit_abs_expr_name(self);
    }
}
impl AbsExpr for AbsExprName{}
impl Positioner for AbsExprName 
{
    fn get_position_ref(&self) -> Option<&Position> { self.abs_position.get_position_ref() }
    fn get_position_ref_mut(&mut self) -> Option<&mut Position> { self.abs_position.get_position_ref_mut() }
    fn set_min(&mut self, position : &Position) 
    {
        self.abs_position.set_min(position);
    }
    fn set_max(&mut self, position : &Position) 
    {
        self.abs_position.set_max(position);
    }
}


pub enum AbsBinOper
{
        OR,
        AND,
        EQU,
        NEQ,
        LTH,
        GTH,
        LEQ,
        GEQ,
        ADD,
        SUB,
        MUL,
        DIV,
        MOD,
        ARR,
        REC,
}

//expession with binary operator
pub struct AbsBinExpr 
{
    abs_position : AbsPosition,
    //binary operator
    pub operation : AbsBinOper,
    //left subexpression
    pub left_sub_expr : Box<AbsExpr>,
    //right subexpression
    pub right_sub_expr : Box<AbsExpr>,
}

impl AbsBinExpr 
{
    pub fn new(operation :AbsBinOper, left_sub_expr : Box<AbsExpr>, right_sub_expr : Box<AbsExpr>) -> AbsBinExpr
    {
        let mut abs_bin_expr = AbsBinExpr {operation,left_sub_expr,right_sub_expr,abs_position : AbsPosition::new()};
        abs_bin_expr.calculate_abs_position();
        abs_bin_expr
    }
    pub fn calculate_abs_position(&mut self)
    {
        self.abs_position.set_min(self.left_sub_expr.get_position_ref().unwrap());
        self.abs_position.set_max(self.right_sub_expr.get_position_ref().unwrap());
    }
}


impl AbsTree for AbsBinExpr 
{
    fn accept(&self, visitor : &mut Visitor)
    {
        visitor.visit_abs_bin_expr(self);
    }
}
impl AbsExpr for AbsBinExpr{}
impl Positioner for AbsBinExpr 
{
    fn get_position_ref(&self) -> Option<&Position> { self.abs_position.get_position_ref() }
    fn get_position_ref_mut(&mut self) -> Option<&mut Position> { self.abs_position.get_position_ref_mut() }
    fn set_min(&mut self, position : &Position) 
    {
        self.abs_position.set_min(position);
    }
    fn set_max(&mut self, position : &Position) 
    {
        self.abs_position.set_max(position);
    }
}

pub enum AbsUnOper
{
    ADD,
    SUB,
    MUL,
    AND,
    NOT,
}

pub struct AbsUnExpr 
{
    abs_position : AbsPosition,
    // Unar operator
    pub operation : AbsUnOper,
    //subexpression
    pub sub_expr : Box<AbsExpr>, 
}

impl AbsUnExpr
{
    pub fn new(operation : AbsUnOper, sub_expr : Box<AbsExpr>) -> AbsUnExpr
    {
       let mut abs_un_expr = AbsUnExpr{operation,sub_expr,abs_position : AbsPosition::new()};
       abs_un_expr.calculate_abs_position();
       abs_un_expr
    }
    pub fn calculate_abs_position(&mut self)
    {
        self.abs_position.set_min(self.sub_expr.get_position_ref().unwrap());
        self.abs_position.set_max(self.sub_expr.get_position_ref().unwrap());
    }
}

impl AbsTree for AbsUnExpr
{
    fn accept(&self, visitor : &mut Visitor)
    {
        visitor.visit_abs_abs_un_expr(self);
    }
}
impl AbsExpr for AbsUnExpr{}
impl Positioner for AbsUnExpr 
{
    fn get_position_ref(&self) -> Option<&Position> { self.abs_position.get_position_ref() }
    fn get_position_ref_mut(&mut self) -> Option<&mut Position> { self.abs_position.get_position_ref_mut() }
    fn set_min(&mut self, position : &Position) 
    {
        self.abs_position.set_min(position);
    }
    fn set_max(&mut self, position : &Position) 
    {
        self.abs_position.set_max(position);
    }
}


pub struct AbsFunCall 
{
    abs_position : AbsPosition,
    //function name
    pub name : AbsExprName,
    //function arguments
    pub args : AbsExprs,
}

impl AbsFunCall
{
    pub fn new(name : AbsExprName, args : AbsExprs) -> AbsFunCall
    {
        let mut abs_fun_call = AbsFunCall{name,args, abs_position : AbsPosition::new()};
        abs_fun_call.calculate_abs_position();
        abs_fun_call
    }
    pub fn calculate_abs_position(&mut self)
    {
        self.abs_position.set_min(self.name.get_position_ref().unwrap());
        self.abs_position.set_max(self.args.get_position_ref().unwrap());
    }
}


impl AbsTree for AbsFunCall 
{
    fn accept(&self, visitor : &mut Visitor)
    {
        visitor.visit_abs_fun_call(self);
    }
}
impl AbsExpr for AbsFunCall{}
impl Positioner for AbsFunCall 
{
    fn get_position_ref(&self) -> Option<&Position> { self.abs_position.get_position_ref() }
    fn get_position_ref_mut(&mut self) -> Option<&mut Position> { self.abs_position.get_position_ref_mut() }
    fn set_min(&mut self, position : &Position) 
    {
        self.abs_position.set_min(position);
    }
    fn set_max(&mut self, position : &Position) 
    {
        self.abs_position.set_max(position);
    }
}

pub struct AbsWhereExpr 
{
    abs_position : AbsPosition,
    // expression
    pub sub_expr : Box<AbsExpr>,
    // declarations
    pub decls : AbsDecls,
}

impl AbsWhereExpr
{
    pub fn new(sub_expr : Box<AbsExpr>, decls : AbsDecls) -> Self
    {
        let mut abs_where_expr =AbsWhereExpr{sub_expr, decls, abs_position : AbsPosition::new()};
        abs_where_expr.calculate_abs_position();
        abs_where_expr
    }
    pub fn calculate_abs_position(&mut self)
    {
        self.abs_position.set_min(self.sub_expr.get_position_ref().unwrap());
        self.abs_position.set_max(self.decls.get_position_ref().unwrap());
    }
}

impl AbsTree for AbsWhereExpr 
{
    fn accept(&self, visitor : &mut Visitor)
    {
        visitor.visit_abs_where_expr(self);
    }
}
impl AbsExpr for AbsWhereExpr {}
impl Positioner for AbsWhereExpr 
{
    fn get_position_ref(&self) -> Option<&Position> { self.abs_position.get_position_ref() }
    fn get_position_ref_mut(&mut self) -> Option<&mut Position> { self.abs_position.get_position_ref_mut() }
    fn set_min(&mut self, position : &Position) 
    {
        self.abs_position.set_min(position);
    }
    fn set_max(&mut self, position : &Position) 
    {
        self.abs_position.set_max(position);
    }
}