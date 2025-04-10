pub struct Node<T> {
    pub data: T,
    pub next: Option<Box<Node<T>>>,
}

pub struct DynamicList<T> {
    head: Option<Box<Node<T>>>,
}

impl<T: PartialEq + Copy> DynamicList<T> {
    pub fn new() -> Self {
        Self { head: None }
    }

    pub fn insert(&mut self, data: T) {
        let new_node = Box::new(Node { data, next: None });
        match self.head.as_mut() {
            None => self.head = Some(new_node),
            Some(mut node) => {
                while let Some(ref mut next) = node.next {
                    node = next;
                }
                node.next = Some(new_node);
            }
        }
    }

    pub fn get(&self, index: usize) -> Option<T> {
        let mut current = &self.head;
        let mut count = 0;
        while let Some(node) = current {
            if count == index {
                return Some(node.data);
            }
            current = &node.next;
            count += 1;
        }
        None
    }
}
