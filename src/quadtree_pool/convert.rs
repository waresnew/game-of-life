use crate::quadtree_pool::{Quadtree, QuadtreePool};

#[allow(dead_code)]
impl QuadtreePool {
    #[deprecated = "only use this for debugging"]
    pub fn to_string(&self, id: usize) -> String {
        let grid = self.to_array(id);
        grid.iter()
            .map(|row| {
                row.iter()
                    .map(|x| if *x == 1 { "*" } else { "." })
                    .collect()
            })
            .collect::<Vec<String>>()
            .join("\n")
    }
    fn to_array(&self, id: usize) -> Vec<Vec<u8>> {
        match &self[id] {
            Quadtree::Subtree(root) => {
                let tl = self.to_array(root.tl);
                let tr = self.to_array(root.tr);
                let bl = self.to_array(root.bl);
                let br = self.to_array(root.br);
                fn block_concat(
                    left: Vec<Vec<u8>>,
                    right: Vec<Vec<u8>>,
                ) -> impl Iterator<Item = Vec<u8>> {
                    left.into_iter().zip(right).map(|(x, y)| [x, y].concat())
                }
                let top = block_concat(tl, tr);
                let bottom = block_concat(bl, br);
                top.chain(bottom).collect()
            }
            &Quadtree::Cell(alive) => vec![vec![if alive { 1 } else { 0 }]],
        }
    }
}
