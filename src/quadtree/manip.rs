use ahash::AHashMap;

use crate::quadtree::Quadtree;

impl Quadtree {
    pub fn add_border(t: Quadtree, dict: &mut AHashMap<u64, Quadtree>) -> Quadtree {
        let zero = Quadtree::zeros(t.height - 1, dict).hash;
        Quadtree::join(
            Quadtree::join(zero, zero, zero, dict[&t.tl].hash, t.height, dict).hash,
            Quadtree::join(zero, zero, dict[&t.tr].hash, zero, t.height, dict).hash,
            Quadtree::join(zero, dict[&t.bl].hash, zero, zero, t.height, dict).hash,
            Quadtree::join(dict[&t.br].hash, zero, zero, zero, t.height, dict).hash,
            t.height + 1,
            dict,
        )
    }
    pub fn get_centre(t: Quadtree, dict: &mut AHashMap<u64, Quadtree>) -> Quadtree {
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
