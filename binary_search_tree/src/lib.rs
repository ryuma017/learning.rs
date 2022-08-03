#[derive(Debug)]
pub struct Node {
    data: isize,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    pub fn new(num: isize) -> Self {
        Self { data: num, left: None, right: None }
    }

    pub fn insert(&mut self, num: isize) {
        todo!()
    }

    pub fn search(&self, num: isize) -> bool {
        todo!()
    }

    pub fn delete(&mut self, num: isize) -> Option<isize> {
        todo!()
    }
}