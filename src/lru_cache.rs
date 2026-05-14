use std::fmt::Debug;
use std::hash::Hash;

use ahash::AHashMap;

#[derive(Default)]
struct DLLNode<T> {
    prev: *mut DLLNode<T>,
    next: *mut DLLNode<T>,
    val: T,
}
#[derive(Default, Debug)]
pub struct LRUCache<K, V> {
    nodes: AHashMap<K, *mut DLLNode<(K, V)>>,
    head: *mut DLLNode<(K, V)>,
    tail: *mut DLLNode<(K, V)>,
    max_size: usize,
}
impl<K, V> LRUCache<K, V>
where
    K: Hash + Eq + Default + Copy + Debug,
    V: Copy + Default + Debug,
{
    pub fn new(max_size: usize) -> Self {
        if max_size == 0 {
            panic!("max_size must be postive");
        }
        let head = Box::into_raw(Box::new(DLLNode::default()));
        let tail = Box::into_raw(Box::new(DLLNode::default()));
        unsafe {
            (*head).next = tail;
            (*tail).prev = head;
        }
        Self {
            max_size,
            head,
            tail,
            nodes: AHashMap::new(),
        }
    }
    pub fn remove(&mut self, k: K) {
        let node = self.nodes[&k];
        unsafe {
            let prev = (*node).prev;
            let next = (*node).next;
            (*prev).next = next;
            (*next).prev = prev;
            self.nodes.remove(&(*node).val.0);
            drop(Box::from_raw(node));
        }
    }
    fn remove_lru(&mut self) {
        unsafe {
            let node = (*self.tail).prev;
            self.remove((*node).val.0);
        }
    }
    pub fn contains(&self, k: K) -> bool {
        self.nodes.contains_key(&k)
    }
    pub fn insert(&mut self, k: K, v: V) {
        if self.nodes.contains_key(&k) {
            self.remove(k);
        }
        if self.nodes.len() == self.max_size {
            self.remove_lru();
        }
        let node = Box::into_raw(Box::new(DLLNode::default()));
        unsafe {
            (*node).val = (k, v);
            (*node).prev = self.head;
            let head_next = (*self.head).next;
            (*node).next = head_next;
            (*head_next).prev = node;
            (*self.head).next = node;
        }
        self.nodes.insert(k, node);
    }
    pub fn len(&self) -> usize {
        self.nodes.len()
    }
    pub fn get(&mut self, k: K) -> Option<V> {
        let Some(&node) = self.nodes.get(&k) else {
            return None;
        };

        //Box disallows sharing the stack ptr
        unsafe {
            let prev = (*node).prev;
            let next = (*node).next;
            (*prev).next = next;
            (*next).prev = prev;

            let head_next = (*self.head).next;
            (*node).prev = self.head;
            (*node).next = head_next;

            (*head_next).prev = node;
            (*self.head).next = node;
            Some((*node).val.1)
        }
    }
    pub fn iter(&self) -> impl Iterator<Item = (K, V)> {
        self.nodes.values().map(|&node| unsafe { (*node).val })
    }
}
impl<K, V> Drop for LRUCache<K, V> {
    fn drop(&mut self) {
        unsafe {
            self.nodes.iter().for_each(|(_, &x)| drop(Box::from_raw(x)));
            drop(Box::from_raw(self.head));
            drop(Box::from_raw(self.tail));
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_lru1() {
        let mut lru: LRUCache<i32, i32> = LRUCache::new(3);
        lru.insert(1, 10);
        lru.insert(2, 20);
        assert_eq!(lru.get(1).unwrap(), 10);
        lru.insert(3, 30);
        lru.insert(4, 40);
        assert!(lru.len() == 3);
        assert!(!lru.contains(2));
        assert_eq!(lru.get(2), None);
        assert_eq!(lru.get(4).unwrap(), 40);
        assert_eq!(lru.get(3).unwrap(), 30);
        lru.insert(4, 50);
        assert_eq!(lru.get(4).unwrap(), 50);
        lru.remove(4);
        assert_eq!(lru.get(4), None);
    }
    #[test]
    #[should_panic]
    fn test_zero() {
        let _lru: LRUCache<i32, i32> = LRUCache::new(0);
    }
}
