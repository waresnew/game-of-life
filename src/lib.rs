use crate::quadtree::{MaybeNode, Node};

mod quadtree;

fn next_step(cur: MaybeNode) -> Box<Node> {
    let cur = cur.unwrap();
    if cur.height == 2 {
        return solve_4x4(cur);
    }
}
fn solve_4x4(cur: MaybeNode) -> Box<Node> {}
