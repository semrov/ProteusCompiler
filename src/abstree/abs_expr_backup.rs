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
}

impl AbsTree for AbsExprs
{
    fn accept(&self, visitor : &mut Visitor)
    {
        visitor.visit_abs_exprs(self);
    }
    fn  calculate_abs_position(&mut self)
    {
        if self.exprs.len() > 0
        {
            self.abs_position.set_min_by_abstree(&*self.exprs[0]);
            self.abs_position.set_max_by_abstree(&*self.exprs[self.exprs.len() - 1]);
        }
    }
    fn get_position_ref(&self) -> Option<&Position> { self.abs_position.get_position_ref() }
}
impl AbsExpr  for AbsExprs {}


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

    pub fn new(expr : Option<Symbol>) -> AbsAtomExpr
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
        AbsAtomExpr{expr, abs_position : AbsPosition::new()}
    }
}

impl AbsTree for AbsAtomExpr
{
    fn accept(&self, visitor : &mut Visitor)
    {
        visitor.visit_abs_atom_expr(self);
    }
    fn calculate_abs_position(&mut self)
    {
        if let  Some(ref symbol) = self.expr
        {
            self.abs_position.set_min_by_symbol(symbol);
            self.abs_position.set_max_by_symbol(symbol);
        }
    }
    fn get_position_ref(&self) -> Option<&Position>
    {
        self.abs_position.get_position_ref()
    }
}
impl AbsExpr for AbsAtomExpr{}

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
        AbsExprName{identifier, abs_position : AbsPosition::new()}
    }
}

impl AbsTree for AbsExprName 
{
    fn accept(&self, visitor : &mut Visitor)
    {
        visitor.visit_abs_expr_name(self);
    }

    fn calculate_abs_position(&mut self)
    {
        self.abs_position.set_min_by_symbol(&self.identifier);
        self.abs_position.set_max_by_symbol(&self.identifier);
    }

     fn get_position_ref(&self) -> Option<&Position>
    {
        self.abs_position.get_position_ref()
    }
}
impl AbsExpr for AbsExprName{}



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
        AbsBinExpr {operation,left_sub_expr,right_sub_expr,abs_position : AbsPosition::new()}
    }
}


impl AbsTree for AbsBinExpr 
{
     fn accept(&self, visitor : &mut Visitor)
    {
        visitor.visit_abs_bin_expr(self);
    }

    fn calculate_abs_position(&mut self)
    {
        self.abs_position.set_min_by_abstree(&*self.left_sub_expr);
        self.abs_position.set_max_by_abstree(&*self.right_sub_expr);
    }

    fn get_position_ref(&self) -> Option<&Position>
    {
        self.abs_position.get_position_ref()
    }
}
impl AbsExpr for AbsBinExpr{}

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
        AbsUnExpr{operation,sub_expr,abs_position : AbsPosition::new()}
    }
}

impl AbsTree for AbsUnExpr
{
    fn accept(&self, visitor : &mut Visitor)
    {
        visitor.visit_abs_abs_un_expr(self);
    }
     
    fn calculate_abs_position(&mut self) 
    {
        self.abs_position.set_min_by_abstree(&*self.sub_expr);
        self.abs_position.set_max_by_abstree(&*self.sub_expr);
    }

    fn get_position_ref(&self) -> Option<&Position>
    {
        self.abs_position.get_position_ref()
    } 
}
impl AbsExpr for AbsUnExpr{}


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
        AbsFunCall{name,args, abs_position : AbsPosition::new()}
    }
}

impl AbsTree for AbsFunCall 
{
    fn accept(&self, visitor : &mut Visitor)
    {
        visitor.visit_abs_fun_call(self);
    }

    fn calculate_abs_position(&mut self)
    {
        self.abs_position.set_min_by_abstree(&self.name);
        self.abs_position.set_max_by_abstree(self.args.)
    }
}
impl AbsExpr for AbsFunCall{}

pub struct AbsWhereExpr 
{
    abs_position : AbsPosition,
    // expression
    pub sub_expr : Box<AbsExpr>,
    // declarations
    pub abs_decl : AbsDecls,
}

impl Positioner for AbsWhereExpr 
{
    fn get_position_ref(&self) -> Option<&Position> { self.abs_position.get_position_ref() }
    fn get_position_ref_mut(&mut self) -> Option<&mut Position> { self.abs_position.get_position_ref_mut() }
    fn set_min_by_position(&mut self, position : &Position) 
    {
        self.abs_position.set_min_by_position(position);
    }
    fn set_min_by_symbol(&mut self, symbol : &Symbol) 
    {
        self.abs_position.set_min_by_symbol(symbol);
    }
    fn set_min_by_abstree(&mut self, abstree : &AbsTree) 
    {
        self.abs_position.set_min_by_abstree(abstree);
    }
    fn set_max_by_position(&mut self, position : &Position) 
    {
        self.abs_position.set_max_by_position(position);
    }
    fn set_max_by_symbol(&mut self, symbol : &Symbol)  
    {
        self.abs_position.set_max_by_symbol(symbol);
    }
    fn set_max_by_abstree(&mut self, abstree : &AbsTree)
    {
        self.abs_position.set_max_by_abstree(abstree);
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
