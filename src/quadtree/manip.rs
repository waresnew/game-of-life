use ahash::AHashMap;

use crate::quadtree::Quadtree;

impl<'a> Quadtree {
    pub fn add_border(t_hash: u64, dict: &mut AHashMap<u64, Quadtree>) -> u64 {
        let &Quadtree {
            tl,
            tr,
            bl,
            br,
            height,
            ..
        } = &dict[&t_hash];
        let zero = Quadtree::zeros(height - 1, dict);
        Quadtree::join(
            Quadtree::join(zero, zero, zero, tl, height, dict),
            Quadtree::join(zero, zero, tr, zero, height, dict),
            Quadtree::join(zero, bl, zero, zero, height, dict),
            Quadtree::join(br, zero, zero, zero, height, dict),
            height + 1,
            dict,
        )
    }
    pub fn get_centre(t_hash: u64, dict: &mut AHashMap<u64, Quadtree>) -> u64 {
        let t = &dict[&t_hash];
        Quadtree::join(
            dict[&t.tl].br,
            dict[&t.tr].bl,
            dict[&t.bl].tr,
            dict[&t.br].tl,
            t.height - 1,
            dict,
        )
    }
}
