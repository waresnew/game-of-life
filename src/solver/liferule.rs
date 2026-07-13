use serde::{Serialize, ser::SerializeStruct};

#[derive(Debug, Clone, Copy)]
pub struct LifeRule {
    born: [bool; 9],
    survive: [bool; 9],
}
impl Serialize for LifeRule {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let ret_b: Vec<usize> = self
            .born
            .iter()
            .enumerate()
            .filter_map(|(i, &x)| if x { Some(i) } else { None })
            .collect();
        let ret_s: Vec<usize> = self
            .survive
            .iter()
            .enumerate()
            .filter_map(|(i, &x)| if x { Some(i) } else { None })
            .collect();
        let mut state = serializer.serialize_struct("LifeRule", 2)?;
        state.serialize_field("born", &ret_b)?;
        state.serialize_field("survive", &ret_s)?;
        state.end()
    }
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
