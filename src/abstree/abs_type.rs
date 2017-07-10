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
    pub fn new(arr_type : Box<AbsType>, size : Box<AbsExpr>, arr_symbol : &Symbol) -> AbsArrType
    {
        let mut abs_arr_type = AbsArrType{arr_type,size, abs_position : AbsPosition::new()};
        abs_arr_type.calculate_abs_position(arr_symbol);
        abs_arr_type
    }
    pub fn calculate_abs_position(&mut self, arr_symbol : &Symbol)
    {
        self.abs_position.set_min(arr_symbol.get_ref_position());
        self.abs_position.set_max(self.arr_type.get_position_ref().unwrap());
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
impl Positioner for AbsArrType 
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
    pub fn new(atom_type : AtomType, symbol : &Symbol) -> AbsAtomType
    {
        let mut abs_atom_type = AbsAtomType{atom_type, abs_position : AbsPosition::new()};
        abs_atom_type.calculate_abs_position(symbol);
        abs_atom_type
    }
    pub fn new_void_type() -> AbsAtomType
    {
        AbsAtomType{atom_type: AtomType::VOID, abs_position : AbsPosition::new()}
    }
    pub fn calculate_abs_position(&mut self, symbol : &Symbol )
    {
        self.abs_position.set_min(symbol.get_ref_position());
        self.abs_position.set_max(symbol.get_ref_position());
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
impl Positioner for AbsAtomType 
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


pub struct AbsPointerType
{
    abs_position : AbsPosition,
    // pointer type
    pub ptype : Box<AbsType>,
}

impl AbsPointerType
{
    pub fn new(ptype : Box<AbsType>,pointer_symbol : &Symbol) -> AbsPointerType
    {
        let mut abs_ptype = AbsPointerType{ptype, abs_position : AbsPosition::new()};
        abs_ptype.calculate_abs_position(pointer_symbol);
        abs_ptype
    }
    pub fn calculate_abs_position(&mut self, pointer_symbol : &Symbol)
    {
        self.abs_position.set_min(pointer_symbol.get_ref_position());
        self.abs_position.set_max(self.ptype.get_position_ref().unwrap());
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
impl Positioner for AbsPointerType 
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

pub struct AbsRecType
{
    abs_position : AbsPosition,
    //compoments of a record
    pub compoments : AbsDecls,
}

impl AbsRecType
{
    pub fn new(compoments : AbsDecls, rec_symbol : &Symbol) -> AbsRecType
    {
        let mut abs_rec_type = AbsRecType{compoments, abs_position : AbsPosition::new()};
        abs_rec_type.calculate_abs_position(rec_symbol);
        abs_rec_type
    }
    pub fn calculate_abs_position(&mut self, rec_symbol : &Symbol)
    {
        self.abs_position.set_min(rec_symbol.get_ref_position());
        self.abs_position.set_max(self.compoments.get_position_ref().unwrap());
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
impl Positioner for AbsRecType 
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


pub struct AbsTypeName {
    abs_position: AbsPosition,
    //name
    pub identifier : Symbol,
}

impl AbsTypeName
{
    pub fn new(identifier : Symbol)->AbsTypeName
    {
        let mut abs_type_name = AbsTypeName{identifier, abs_position : AbsPosition::new()};
        abs_type_name.calculate_abs_position();
        abs_type_name
    }
    pub fn calculate_abs_position(&mut self)
    {
        self.abs_position.set_min(self.identifier.get_ref_position());
        self.abs_position.set_max(self.identifier.get_ref_position());
    }
}

impl Positioner for AbsTypeName 
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

impl AbsTree for AbsTypeName 
{
    fn accept(&self, visitor : &mut Visitor)
    {
        visitor.visit_abs_type_name(self);
    }
}
impl AbsType for AbsTypeName {}