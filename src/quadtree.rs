pub struct Children {
    pub nw: Box<Node>,
    pub ne: Box<Node>,
    pub sw: Box<Node>,
    pub se: Box<Node>,
}
pub struct Node {
    pub children: Option<Children>,
    pub height: usize,
    pub count: usize,
}
impl Node {
    pub fn to_array(&self) -> Vec<u8> {
        let Some(children) = &self.children else {
            return vec![self.count as u8];
        };
        let nw = children.nw.to_array();
        let ne = children.ne.to_array();
        let sw = children.sw.to_array();
        let se = children.se.to_array();
        fn block_concat(left: &Vec<u8>, right: &Vec<u8>, width: usize) -> impl Iterator<Item = u8> {
            left.chunks_exact(width)
                .zip(right.chunks_exact(width))
                .map(|(x, y)| [x, y].concat())
                .flatten()
        }
        let width = 2_usize.pow((self.height - 1) as u32);
        let top = block_concat(&nw, &ne, width);
        let bottom = block_concat(&sw, &se, width);
        top.chain(bottom).collect()
    }
}
pub struct Quadtree {}
