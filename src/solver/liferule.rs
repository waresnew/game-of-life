#[derive(Debug, Clone, Copy)]
pub struct LifeRule {
    born: [bool; 9],
    survive: [bool; 9],
}
impl LifeRule {
    pub fn is_born(&self, num: usize) -> bool {
        self.born[num]
    }
    pub fn survives(&self, num: usize) -> bool {
        self.survive[num]
    }
    pub fn from_dense(b: Vec<usize>, s: Vec<usize>) -> Self {
        let mut ret = Self {
            born: [false; 9],
            survive: [false; 9],
        };
        for x in b {
            ret.born[x] = true;
        }
        for x in s {
            ret.survive[x] = true;
        }
        ret
    }
}
pub const GOL_RULES: LifeRule = LifeRule {
    born: [false, false, false, true, false, false, false, false, false],
    survive: [false, false, true, true, false, false, false, false, false],
};
