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
        BSTNode {
            key: key,
            value: value,
            left: BST{node: None},
            right: BST{node: None},
        }
    }

    fn has_right(&self) -> bool {
        self.right.node.is_some()
    }

    fn has_left(&self) -> bool {
        self.left.node.is_some()
    }

    fn move_right(&mut self) -> Option<Rc<RefCell<BSTNode<K,V>>>> {
        if self.has_right() {
            let ret = Some(self.right.node.as_ref().unwrap().clone());
            self.right.node = None;
            ret
        } else {
            None
        }
    }

    fn move_left(&mut self) -> Option<Rc<RefCell<BSTNode<K,V>>>> {
        if self.has_left() {
            let ret = Some(self.left.node.as_ref().unwrap().clone());
            self.left.node = None;
            ret
        } else {
            None
        }
    }
}

pub struct BST<K,V> {
    node: Option<Rc<RefCell<BSTNode<K,V>>>>,
}

impl<K: Integer + Copy, V: Clone> BST<K,V> {
    pub fn new() -> BST<K,V> {
        BST {
            node: None
        }
    }

    pub fn search(&self, key: K) -> Result<V, KeyError> {
        match &self.node {
            None => Err(KeyError::new("not found")),
            Some(node) => {
                let node = node.borrow();
                if key == node.key {
                    Ok(node.value.clone())
                } else if key < node.key {
                    node.left.search(key)
                } else {
                    node.right.search(key)
                }
            }
        }
    }

    pub fn insert(&mut self, key: K, value: V) {
        match &mut self.node {
            None => {
                self.node = Some(Rc::new(RefCell::new(BSTNode::new(key, value))));
            },
            Some(node) => {
                let mut node = node.borrow_mut();
                if key == node.key {
                    node.value = value;
                } else if key < node.key {
                    node.left.insert(key, value);
                } else {
                    node.right.insert(key, value);
                }
            }
        };
    }

    pub fn delete(&mut self, key: K) -> Result<V, KeyError> {
        if self.node.is_none() {
            return Err(KeyError::new("not found"));
        }

        let node = self.node.as_ref().unwrap().clone();
        if key < node.borrow().key {
            return node.borrow_mut().left.delete(key)
        } else if key > node.borrow().key {
            return node.borrow_mut().right.delete(key)
        } else {
            let ret = node.borrow().value.clone();
            let both = {
                let node = node.borrow();
                node.has_left() && node.has_right()
            };
            if both {
                let new_key = node.borrow().right.find_min_key()?;
                let new_val = node.borrow_mut().right.delete(new_key)?;
                let mut n = node.borrow_mut();
                self.node = Some(Rc::new(RefCell::new(BSTNode {
                    key: new_key,
                    value: new_val,
                    left: BST{node: n.move_left()},
                    right: BST{node: n.move_right()},
                })))
            } else if node.borrow().has_left() {
                node.swap(&node.borrow().left.node.as_ref().unwrap());
            } else if node.borrow().has_right() {
                node.swap(&node.borrow().right.node.as_ref().unwrap())
            } else {
                self.node = None;
            }
            Ok(ret)
        }
    }

    fn find_min_key(&self) -> Result<K, KeyError> {
        match &self.node {
            Some(node) => {
                let node = node.borrow();
                if node.left.node.is_some() {
                    node.left.find_min_key()
                } else {
                    Ok(node.key)
                }
            },
            None => Err(KeyError::new("not found")),
        }
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
