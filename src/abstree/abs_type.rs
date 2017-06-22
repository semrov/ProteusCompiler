use abstree::AbsTree;
use abstree::abs_expr::{AbsExpr};
use abstree::abs_position::AbsPosition;
use abstree::abs_decl::AbsDecls;
use abstree::visitor::Visitor;
use abstree::positioner::Positioner;
use lexanal::position::Position;
use lexanal::symbol::Symbol;

pub trait AbsType : AbsTree {}

//array type
pub struct AbsArrType
{
    abs_position : AbsPosition,
    pub arr_type : Box<AbsType>,
    pub size : Box<AbsExpr>,
}

impl AbsArrType
{
    pub fn new(arr_type : Box<AbsType>, size : Box<AbsExpr>) -> AbsArrType
    {
        AbsArrType{arr_type,size, abs_position : AbsPosition::new()}
    }
}

impl Positioner for AbsArrType 
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

impl AbsTree for AbsArrType 
{
    fn accept(&self, visitor : &mut Visitor)
    {
        visitor.visit_abs_arr_type(self);
    }
}
impl AbsType for AbsArrType{}

pub enum AtomType
{
    INT,
    REAL,
    BOOL,
    STRING,
    VOID,
}

pub struct AbsAtomType 
{
    abs_position : AbsPosition,
    //atomic type
    pub atom_type : AtomType
}

impl AbsAtomType 
{
    pub fn new(atom_type : AtomType) -> AbsAtomType
    {
        AbsAtomType{atom_type, abs_position : AbsPosition::new()}
    }
}

impl Positioner for AbsAtomType 
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

impl AbsTree for AbsAtomType 
{
    fn accept(&self, visitor : &mut Visitor)
    {
        visitor.visit_abs_atom_type(self);
    }
}
impl AbsType for AbsAtomType {}


pub struct AbsPointerType
{
    abs_position : AbsPosition,
    // pointer type
    pub ptype : Box<AbsType>,
}

impl AbsPointerType
{
    pub fn new(ptype : Box<AbsType>) -> AbsPointerType
    {
        AbsPointerType{ptype, abs_position : AbsPosition::new()}
    }
}

impl Positioner for AbsPointerType 
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

impl AbsTree for AbsPointerType 
{
    fn accept(&self, visitor : &mut Visitor)
    {
        visitor.visit_abs_pointer_type(self);
    }
}
impl AbsType for AbsPointerType {}

pub struct AbsRecType
{
    abs_position : AbsPosition,
    //compoments of a record
    pub compoments : AbsDecls,
}

impl AbsRecType
{
    pub fn new(compoments : AbsDecls) -> AbsRecType
    {
        AbsRecType{compoments, abs_position : AbsPosition::new()}
    }
}

impl Positioner for AbsRecType 
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

impl AbsTree for AbsRecType 
{
    fn accept(&self, visitor : &mut Visitor)
    {
        visitor.visit_abs_rec_type(self);
    }
}
impl AbsType for AbsRecType {}


pub struct AbsTypeName {
    abs_position: AbsPosition,
    //name
    pub identifier : Symbol,
}

impl AbsTypeName
{
    pub fn new(identifier : Symbol)->AbsTypeName
    {
        AbsTypeName{identifier, abs_position : AbsPosition::new()}
    }
}

impl Positioner for AbsTypeName 
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

impl AbsTree for AbsTypeName 
{
    fn accept(&self, visitor : &mut Visitor)
    {
        visitor.visit_abs_type_name(self);
    }
}
impl AbsType for AbsTypeName {}