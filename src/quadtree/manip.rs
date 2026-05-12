use ahash::AHashMap;

use crate::{quadtree::Quadtree, utils::update_dict};

impl Quadtree {
    pub fn join(
        tl: Quadtree,
        tr: Quadtree,
        bl: Quadtree,
        br: Quadtree,
        dict: &mut AHashMap<u64, Quadtree>,
    ) -> Quadtree {
        assert!(tl.height == tr.height && tr.height == bl.height && bl.height == br.height);
        let ret = Quadtree {
            tl: tl.calc_hash(),
            tr: tr.calc_hash(),
            bl: bl.calc_hash(),
            br: br.calc_hash(),
            height: tl.height + 1,
            count: tl.count + tr.count + bl.count + br.count,
        };
        update_dict(ret, dict);
        ret
    }
    pub fn join_with_u64(
        tl_hash: u64,
        tr_hash: u64,
        bl_hash: u64,
        br_hash: u64,
        dict: &mut AHashMap<u64, Quadtree>,
    ) -> Quadtree {
        let tl = dict[&tl_hash];
        let tr = dict[&tr_hash];
        let bl = dict[&bl_hash];
        let br = dict[&br_hash];
        assert!(tl.height == tr.height && tr.height == bl.height && bl.height == br.height);
        let ret = Quadtree {
            tl: tl_hash,
            tr: tr_hash,
            bl: bl_hash,
            br: br_hash,
            height: tl.height + 1,
            count: tl.count + tr.count + bl.count + br.count,
        };
        update_dict(ret, dict);
        ret
    }
    pub fn add_border(t: Quadtree, dict: &mut AHashMap<u64, Quadtree>) -> Quadtree {
        let zero = Quadtree::zeros(t.height - 1, dict);
        Quadtree {
            tl: update_dict(Quadtree::join(zero, zero, zero, dict[&t.tl], dict), dict),
            tr: update_dict(Quadtree::join(zero, zero, dict[&t.tr], zero, dict), dict),
            bl: update_dict(Quadtree::join(zero, dict[&t.bl], zero, zero, dict), dict),
            br: update_dict(Quadtree::join(dict[&t.br], zero, zero, zero, dict), dict),
            height: t.height + 1,
            count: t.count,
        }
    }
    pub fn get_centre(t: Quadtree, dict: &mut AHashMap<u64, Quadtree>) -> Quadtree {
        Quadtree::join_with_u64(
            dict[&t.tl].br,
            dict[&t.tr].bl,
            dict[&t.bl].tr,
            dict[&t.br].tl,
            dict,
        )
    }
}
