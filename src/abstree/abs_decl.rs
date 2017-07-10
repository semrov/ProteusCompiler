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
    pub fn add_decl(&mut self, decl : Box<AbsDecl>)
    {
        self.decls.push(decl);
        self.calculate_abs_position();
    }
    pub fn calculate_abs_position(&mut self)
    {
        self.abs_position.set_min(self.decls[0].get_position_ref().unwrap());
        self.abs_position.set_max(self.decls[self.decls.len() - 1].get_position_ref().unwrap());
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
impl Positioner for AbsDecls 
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
       let mut abs_var_decl = AbsVarDecl{var_name,var_type, abs_position : AbsPosition::new()};
       abs_var_decl.calculate_abs_position();
       abs_var_decl
    }
    pub fn calculate_abs_position(&mut self)
    {
        self.abs_position.set_min(self.var_name.get_position_ref().unwrap());
        self.abs_position.set_max(self.var_type.get_position_ref().unwrap());
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
impl Positioner for AbsVarDecl 
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
    pub exprs : Box<AbsExpr>,
}

impl AbsFunDecl
{
    pub fn new(name : AbsExprName, params : AbsDecls, return_type : Box<AbsType>, exprs : Box<AbsExpr> ) -> AbsFunDecl
    {
        let mut fun_decl = AbsFunDecl{name,params,return_type,exprs, abs_position : AbsPosition::new()};
        fun_decl.calculate_abs_position();
        fun_decl
    }
    pub fn calculate_abs_position(&mut self)
    {
        self.abs_position.set_min(self.name.get_position_ref().unwrap());
        self.abs_position.set_max(self.exprs.get_position_ref().unwrap());
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
impl Positioner for AbsFunDecl 
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
       let mut abs_type_decl = AbsTypeDecl{type_name,source_type,abs_position : AbsPosition::new()};
       abs_type_decl.calculate_abs_position();
       abs_type_decl
    }
     pub fn calculate_abs_position(&mut self)
    {
        self.abs_position.set_min(self.type_name.get_position_ref().unwrap());
        self.abs_position.set_max(self.source_type.get_position_ref().unwrap());
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
impl Positioner for AbsTypeDecl 
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
