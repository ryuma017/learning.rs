use std::cmp::Ordering;

#[derive(Debug)]
pub struct Node {
    data: isize,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    pub fn new(num: isize) -> Self {
        Self {
            data: num,
            left: None,
            right: None,
        }
    }

    pub fn insert(&mut self, num: isize) {
        match &self.data.cmp(&num) {
            Ordering::Greater => {
                if let Some(left_node) = &mut self.left {
                    left_node.insert(num);
                } else {
                    let node = Self::new(num);
                    self.left = Some(Box::new(node));
                }
            }
            Ordering::Less => {
                if let Some(right_node) = &mut self.right {
                    right_node.insert(num);
                } else {
                    let node = Self::new(num);
                    self.right = Some(Box::new(node));
                }
            }
            Ordering::Equal => {}
        }
    }

    pub fn search(&self, num: isize) -> bool {
        match &self.data.cmp(&num) {
            Ordering::Greater => {
                if let Some(left_node) = &self.left {
                    left_node.search(num)
                } else {
                    false
                }
            }
            Ordering::Less => {
                if let Some(right_node) = &self.right {
                    right_node.search(num)
                } else {
                    false
                }
            }
            Ordering::Equal => true,
        }
    }

    pub fn delete(&mut self, num: isize) -> Option<isize> {
        match &self.data.cmp(&num) {
            Ordering::Greater => {
                if let Some(left_node) = &mut self.left {
                    if left_node.data.cmp(&num).is_eq() {
                        self.left = None;
                        return Some(num);
                    }
                    left_node.delete(num)
                } else {
                    None
                }
            }
            Ordering::Less => {
                if let Some(right_node) = &mut self.right {
                    if right_node.data.cmp(&num).is_eq() {
                        self.right = None;
                        return Some(num);
                    }
                    right_node.delete(num)
                } else {
                    None
                }
            }
            Ordering::Equal => {
                self.left = None;
                self.right = None;
                Some(num)
            }
        }
    }
}
