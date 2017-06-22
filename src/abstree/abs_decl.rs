use abstree::AbsTree;
use abstree::abs_expr::{AbsExprName,AbsExpr};
use abstree::positioner::Positioner;
use abstree::abs_position::AbsPosition;
use abstree::abs_type::{AbsType,AbsTypeName};
use lexanal::position::Position;
use lexanal::symbol::Symbol;
use abstree::visitor::Visitor;

pub trait AbsDecl : AbsTree {}

pub struct AbsDecls
{
    abs_position : AbsPosition,
    //vector of delcarations
    pub decls : Vec<Box<AbsDecl>>,
}

impl AbsDecls {
    pub fn new() -> AbsDecls
    {
        AbsDecls{decls : Vec::new(), abs_position : AbsPosition::new()}
    }
}


impl Positioner for AbsDecls 
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

impl AbsTree for AbsDecls 
{
    fn accept(&self, visitor : &mut Visitor)
    {
        visitor.visit_abs_decls(self);
    }
}
impl AbsDecl for AbsDecls {}

pub struct AbsVarDecl
{
    abs_position: AbsPosition,
    //variable name
    pub var_name : AbsExprName,
    //variable type
    pub var_type : Box<AbsType>
}

impl AbsVarDecl
{
    pub fn new(var_name: AbsExprName, var_type : Box<AbsType>) -> AbsVarDecl
    {
        AbsVarDecl{var_name,var_type, abs_position : AbsPosition::new()}
    }
}

impl Positioner for AbsVarDecl 
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

impl AbsTree for AbsVarDecl 
{
     fn accept(&self, visitor : &mut Visitor)
    {
        visitor.visit_abs_var_decl(self);
    }
}
impl AbsDecl for AbsVarDecl {}

pub struct AbsFunDecl
{
    abs_position: AbsPosition,
    //function name
    pub name : AbsExprName,
    //function parameters
    pub params : AbsDecls,
    //return type of function
    pub return_type : Box<AbsType>,
    //function body
    pub expr : Box<AbsExpr>,
}

impl AbsFunDecl
{
    pub fn new(name : AbsExprName, params : AbsDecls, return_type : Box<AbsType>, expr : Box<AbsExpr> ) -> AbsFunDecl
    {
        AbsFunDecl{name,params,return_type,expr, abs_position : AbsPosition::new()}
    }
}

impl Positioner for AbsFunDecl 
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

impl AbsTree for AbsFunDecl 
{
     fn accept(&self, visitor : &mut Visitor)
    {
        visitor.visit_abs_fun_decl(self);
    }
}
impl AbsDecl for AbsFunDecl {}

pub struct AbsTypeDecl
{
    abs_position : AbsPosition,
    //new type name
    pub type_name : AbsTypeName,
    //source type
    pub source_type : Box<AbsType>,
}

impl AbsTypeDecl
{
    pub fn new(type_name : AbsTypeName, source_type : Box<AbsType>) -> AbsTypeDecl
    {
        AbsTypeDecl{type_name,source_type,abs_position : AbsPosition::new()}
    }
}

impl Positioner for AbsTypeDecl 
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

impl AbsTree for AbsTypeDecl 
{
    fn accept(&self, visitor : &mut Visitor)
    {
        visitor.visit_abs_type_decl(self);
    }
}
impl AbsDecl for AbsTypeDecl {}
