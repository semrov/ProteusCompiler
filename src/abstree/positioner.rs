use lexanal::position::Position;
use lexanal::symbol::Symbol;
use abstree::AbsTree;

pub trait  Positioner {
    //fn get_position(&self) -> Option<Position>;
    fn get_position_ref(&self) -> Option<&Position>;
    fn get_position_ref_mut(&mut self) -> Option<&mut Position>;
    fn set_min(&mut self, position : &Position);
    fn set_max(&mut self, position : &Position);
    /*
    fn set_min_by_position(&mut self, position : &Position);
    fn set_min_by_symbol(&mut self, symbol : &Symbol);
    fn set_min_by_abstree(&mut self, abstree : &AbsTree);
    fn set_max_by_position(&mut self, position : &Position);
    fn set_max_by_symbol(&mut self, symbol : &Symbol);
    fn set_max_by_abstree(&mut self, abstree : &AbsTree);
    */
}
