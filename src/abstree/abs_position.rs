use abstree::positioner::Positioner;
use lexanal::position::Position;
use lexanal::symbol::Symbol;
use abstree::AbsTree;

pub struct AbsPosition
{
    position : Option<Position>,
}

impl AbsPosition 
{
    pub fn new() -> AbsPosition { AbsPosition {position : None} }
    pub fn new_with_position(pos : &Position) -> AbsPosition{ AbsPosition {position : Some(pos.clone())} }
}


impl Positioner for AbsPosition 
{
    /*fn get_position(&self) -> Option<Position>
    {
        self.position.map(|pos| pos.clone())
    }*/
    fn get_position_ref(&self) -> Option<&Position> 
    {
        self.position.as_ref().map(|pos| pos)
    }
    fn get_position_ref_mut(&mut self) -> Option<&mut Position>
    {
        self.position.as_mut().map(|pos| pos)
    }

    fn set_min(&mut self, position : &Position) 
    {
        match self.position 
        {
            Some(ref mut pos) => 
            {
                pos.set_min(position);
                return;
            },
            None => {},
        }
        self.position = Some(position.clone());
    }

    fn set_max(&mut self, position : &Position) 
    {
        match self.position 
        {
            Some(ref mut pos) => 
            {
                pos.set_max(position);
                return;
            },
            None => {},
        }
        self.position = Some(position.clone());
    }
}

    /*
    fn set_min_by_position(&mut self, position : &Position) 
    {
        match self.position 
        {
            Some(ref mut pos) => 
            {
                pos.set_min(position);
                return;
            },
            None => {},
        }
        self.position = Some(position.clone());
    }
    fn set_min_by_symbol(&mut self, symbol : &Symbol) 
    {
        match self.position 
        {
            Some(ref mut pos) => 
            {
                pos.set_min(symbol.get_ref_position());
                return;
            }
            None => {},
        }
        self.position = Some(symbol.get_ref_position().clone());
    }
    fn set_min_by_abstree(&mut self, abstree : &AbsTree) 
    {
        match self.position
        {
            Some(ref mut pos) => 
            {
                match abstree.get_position_ref() 
                {
                    Some(tree_pos) => pos.set_min(tree_pos),
                    None => unimplemented!(),
                }
                return;
            },
            None => {},
        }
        match abstree.get_position_ref() 
        {
            Some(tree_pos) => self.position = Some(tree_pos.clone()),
            None => unimplemented!(),
        }
    }

        fn set_max_by_abstree(&mut self, abstree : &AbsTree)
    {
         match self.position
        {
            Some(ref mut pos) => 
            {
                match abstree.get_position_ref() 
                {
                    Some(tree_pos) => pos.set_max(tree_pos),
                    None => unimplemented!(),
                }
                return;
            },
            None => {},
        }
        match abstree.get_position_ref() 
        {
            Some(tree_pos) => self.position = Some(tree_pos.clone()),
            None => unimplemented!(),
        }
    }
  
    fn set_max_by_symbol(&mut self, symbol : &Symbol)  
    {
        match self.position 
        {
            Some(ref mut pos) => 
            {
                pos.set_max(symbol.get_ref_position());
                return;
            }
            None => {},
        }
        self.position = Some(symbol.get_ref_position().clone());
    }
    */

