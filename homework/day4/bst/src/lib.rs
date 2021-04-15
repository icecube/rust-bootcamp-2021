use std::error::Error;
use std::fmt;
use std::rc::Rc;
use std::cell::RefCell;
use num::Integer;

#[derive(Debug)]
pub struct KeyError {
    message: String
}

impl KeyError {
    fn new(msg: &str) -> KeyError {
        KeyError{message: msg.to_string()}
    }
}

impl fmt::Display for KeyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}",self.message)
    }
}

impl Error for KeyError {
    fn description(&self) -> &str {
        &self.message
    }
}


struct BSTNode<K,V> {
    key: K,
    value: V,
    left: BST<K,V>,
    right: BST<K,V>,
}

impl<K,V> BSTNode<K,V> {
    fn new(key: K, value: V) -> BSTNode<K,V> {

    }
}

pub struct BST<K,V> {
    node: Option<Rc<RefCell<BSTNode<K,V>>>>,
}

impl<K: Integer + Copy, V: Clone> BST<K,V> {
    pub fn new() -> BST<K,V> {

    }

    pub fn search(&self, key: K) -> Result<V, KeyError> {

    }

    pub fn insert(&mut self, key: K, value: V) {

    }

    pub fn delete(&mut self, key: K) -> Result<V, KeyError> {

    }

    fn find_min_key(&self) -> Result<K, KeyError> {

    }
}

impl<K: Integer + Copy, V: Clone>  IntoIterator for &BST<K,V> {
    type Item = (K, V);
    type IntoIter = std::vec::IntoIter<(K, V)>;
    fn into_iter(self) -> Self::IntoIter {
        fn append<K: Integer + Copy, V: Clone>(bst: &BST<K,V>, v: &mut Vec<(K, V)>) {
            if let Some(node) = &bst.node {
                if node.borrow().left.node.is_some() {
                    append(&node.borrow().left, v);
                }
                v.push((node.borrow().key, node.borrow().value.clone()));
                if node.borrow().right.node.is_some() {
                    append(&node.borrow().right, v);
                }
            }
        }

        let mut result = vec![];
        append(&self, &mut result);
        result.into_iter()
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {

        let mut b = BST::new();
        b.insert(5, "five");
        b.insert(3, "three");
        b.insert(7, "seven");
        b.insert(2, "two");
        b.insert(3, "three-three");
        b.insert(6, "six");

        assert_eq!(b.search(5).unwrap(), "five");
        assert_eq!(b.search(2).unwrap(), "two");
        assert_eq!(b.search(6).unwrap(), "six");

        assert_eq!(b.delete(5).unwrap(), "five");

        let correct = vec![(2, "two"), (3, "three-three"), (6, "six"), (7, "seven")];
        assert_eq!(b.into_iter().collect::<Vec<(i32, &str)>>(), correct);
    }
}
