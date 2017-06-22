use abstree::abs_expr::{AbsExpr,AbsExprName};
use abstree::AbsTree;
use abstree::abs_position::AbsPosition;
use abstree::positioner::Positioner;
use abstree::visitor::Visitor;
use lexanal::position::Position;
use lexanal::symbol::Symbol;

pub trait AbsStmt : AbsExpr {}

pub struct AbsAssignStmt 
{
    abs_position : AbsPosition,
    // left side of expression
    pub left_sub_expr : Box<AbsExpr>,
    //right side of expression
    pub right_sub_expr : Box<AbsExpr>,
}

impl AbsAssignStmt 
{
    pub fn new(left_sub_expr : Box<AbsExpr>, right_sub_expr : Box<AbsExpr>) -> AbsAssignStmt
    {
        AbsAssignStmt{left_sub_expr,right_sub_expr,abs_position : AbsPosition::new()}
    }
}

impl Positioner for AbsAssignStmt 
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

impl AbsTree for AbsAssignStmt 
{
    fn accept(&self, visitor : &mut Visitor)
    {
        visitor.visit_abs_assign_stmt(self);
    }
}
impl AbsExpr for AbsAssignStmt {}
impl AbsStmt for AbsAssignStmt {}

//if statement tree
pub struct AbsIfStmt 
{
    abs_position : AbsPosition,
    //condition expression
    pub cond_expr : Box<AbsExpr>,
    //true subexpression
    pub then_expr : Box<AbsExpr>,
    //false subexpression
    pub else_expr : Option<Box<AbsExpr>>,
}

impl AbsIfStmt {
    pub fn new(cond_expr : Box<AbsExpr>, then_expr : Box<AbsExpr>, else_expr : Option<Box<AbsExpr>>) -> AbsIfStmt
    {
        AbsIfStmt{cond_expr,then_expr,else_expr, abs_position : AbsPosition::new()}
    }
}
impl Positioner for AbsIfStmt 
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

impl AbsTree for AbsIfStmt 
{
    fn accept(&self, visitor : &mut Visitor)
    {
        visitor.visit_abs_if_stmt(self);
    }
}
impl AbsExpr for AbsIfStmt {}
impl AbsStmt for AbsIfStmt {}

// for statement tree
pub struct AbsForStmt
{
    abs_position : AbsPosition,
    //loop variable name
    pub var_name : AbsExprName,
    //lower bound
    pub lower_bound : Box<AbsExpr>,
    //higher bound
     pub higher_bound : Box<AbsExpr>,
     //loop expression
     pub loop_expr : Box<AbsExpr>,
}

impl AbsForStmt
{
    pub fn new(var_name : AbsExprName, lower_bound : Box<AbsExpr>, higher_bound : Box<AbsExpr>, loop_expr : Box<AbsExpr>)-> AbsForStmt 
    {
        AbsForStmt{var_name,lower_bound,higher_bound,loop_expr,abs_position : AbsPosition::new()}
    }
}

impl Positioner for AbsForStmt 
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

impl AbsTree for AbsForStmt 
{
    fn accept(&self, visitor : &mut Visitor)
    {
        visitor.visit_abs_for_stmt(self);
    }
}
impl AbsExpr for AbsForStmt {}
impl AbsStmt for AbsForStmt {}

pub struct AbsWhileStmt 
{
    abs_position : AbsPosition,
    //condition 
    pub cond_expr : Box<AbsExpr>,
    //loop body
    pub loop_expr : Box<AbsExpr>,
}

impl AbsWhileStmt 
{
    pub fn new(cond_expr : Box<AbsExpr>, loop_expr : Box<AbsExpr> ) -> AbsWhileStmt
    {
        AbsWhileStmt{cond_expr,loop_expr, abs_position : AbsPosition::new()}
    }
}

impl Positioner for AbsWhileStmt 
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

impl AbsTree for AbsWhileStmt 
{
    fn accept(&self, visitor : &mut Visitor)
    {
        visitor.visit_abs_while_stmt(self);
    }
}
impl AbsExpr for AbsWhileStmt {}
impl AbsStmt for AbsWhileStmt {}