pub mod positioner;
pub mod abs_position;
pub mod abs_expr;
pub mod abs_stmt;
pub mod abs_decl;
pub mod abs_type;
pub mod visitor;
pub mod print_xml;

//use lexanal::position::Position;
use abstree::visitor::Visitor;
use abstree::positioner::Positioner;


pub trait AbsTree  : Positioner
{
    fn accept(&self, visitor : &mut Visitor);
    //fn calculate_abs_position()
}