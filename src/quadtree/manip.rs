use std::collections::HashMap;

use crate::{quadtree::Quadtree, utils::update_dict};

impl Quadtree {
    pub fn join(
        tl: Quadtree,
        tr: Quadtree,
        bl: Quadtree,
        br: Quadtree,
        dict: &mut HashMap<u64, Quadtree>,
    ) -> Quadtree {
        assert!(tl.height == tr.height && tr.height == bl.height && bl.height == br.height);
        update_dict(tl, dict);
        update_dict(tr, dict);
        update_dict(bl, dict);
        update_dict(br, dict);
        Quadtree {
            tl: tl.calc_hash(),
            tr: tr.calc_hash(),
            bl: bl.calc_hash(),
            br: br.calc_hash(),
            height: tl.height + 1,
            count: tl.count + tr.count + bl.count + br.count,
        }
    }
    pub fn join_with_u64(
        tl: u64,
        tr: u64,
        bl: u64,
        br: u64,
        dict: &mut HashMap<u64, Quadtree>,
    ) -> Quadtree {
        Quadtree::join(dict[&tl], dict[&tr], dict[&bl], dict[&br], dict)
    }
    pub fn add_border(t: Quadtree, dict: &mut HashMap<u64, Quadtree>) -> Quadtree {
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
    pub fn get_centre(t: Quadtree, dict: &mut HashMap<u64, Quadtree>) -> Quadtree {
        Quadtree::join_with_u64(
            dict[&t.tl].br,
            dict[&t.tr].bl,
            dict[&t.bl].tr,
            dict[&t.br].tl,
            dict,
        )
    }
}
