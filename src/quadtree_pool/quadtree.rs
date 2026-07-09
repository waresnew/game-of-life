use num_bigint::BigUint;

// disable accidental Copy
#[derive(Debug)]
pub enum Quadtree {
    Subtree(Subtree),
    Cell(bool),
}
#[derive(Debug)]
pub struct Subtree {
    pub tl: usize,
    pub tr: usize,
    pub bl: usize,
    pub br: usize,
    pub height: u32,
    pub count: BigUint,
}
impl Quadtree {
    pub fn as_subtree(&self) -> &Subtree {
        match self {
            Quadtree::Subtree(subtree) => subtree,
            _ => panic!("called assert_subtree on {:?} which is not a subtree", self),
        }
    }
    pub fn as_subtree_mut(&mut self) -> &mut Subtree {
        match self {
            Quadtree::Subtree(subtree) => subtree,
            _ => panic!("called assert_subtree on {:?} which is not a subtree", self),
        }
    }
}
