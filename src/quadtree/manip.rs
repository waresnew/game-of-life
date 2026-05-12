use ahash::AHashMap;

use crate::quadtree::Quadtree;

impl Quadtree {
    pub fn join(
        tl: Quadtree,
        tr: Quadtree,
        bl: Quadtree,
        br: Quadtree,
        dict: &mut AHashMap<u64, Quadtree>,
    ) -> Quadtree {
        assert!(tl.height == tr.height && tr.height == bl.height && bl.height == br.height);
        Quadtree::new(tl.hash, tr.hash, bl.hash, br.hash, tl.height + 1, dict)
    }
    pub fn join_with_u64(
        tl_hash: u64,
        tr_hash: u64,
        bl_hash: u64,
        br_hash: u64,
        new_height: u32,
        dict: &mut AHashMap<u64, Quadtree>,
    ) -> Quadtree {
        Quadtree::new(tl_hash, tr_hash, bl_hash, br_hash, new_height, dict)
    }
    pub fn add_border(t: Quadtree, dict: &mut AHashMap<u64, Quadtree>) -> Quadtree {
        let zero = Quadtree::zeros(t.height - 1, dict);
        Quadtree::new(
            Quadtree::join(zero, zero, zero, dict[&t.tl], dict).hash,
            Quadtree::join(zero, zero, dict[&t.tr], zero, dict).hash,
            Quadtree::join(zero, dict[&t.bl], zero, zero, dict).hash,
            Quadtree::join(dict[&t.br], zero, zero, zero, dict).hash,
            t.height + 1,
            dict,
        )
    }
    pub fn get_centre(t: Quadtree, dict: &mut AHashMap<u64, Quadtree>) -> Quadtree {
        Quadtree::join_with_u64(
            dict[&t.tl].br,
            dict[&t.tr].bl,
            dict[&t.bl].tr,
            dict[&t.br].tl,
            t.height - 1,
            dict,
        )
    }
}
