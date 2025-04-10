#[derive(Debug)]
pub struct Node<T> {
    pub data: T,
    pub next: Option<Box<Node<T>>>,
}

#[derive(Debug)]
pub struct DynamicList<T> {
    head: Option<Box<Node<T>>>,
}

impl<T: PartialEq + Copy> DynamicList<T> {
    pub fn new() -> Self {
        DynamicList { head: None }
    }

    /// Inserts data at the tail of the list.
    pub fn insert(&mut self, data: T) {
        let new_node = Box::new(Node { data, next: None });
        match self.head.as_mut() {
            None => self.head = Some(new_node),
            Some(mut current) => {
                let mut current = current.as_mut();
                while current.next.is_some() {
                    current = current.next.as_mut().unwrap();
                }
                current.next = Some(new_node);
            }
        }
    }

    pub fn get(&self, index: usize) -> Option<T> {
        let mut current = self.head.as_ref();
        let mut i = 0;
    
        while let Some(node) = current {
            if i == index {
                return Some(node.data);
            }
            i += 1;
            current = node.next.as_ref();
        }
    
        None
    }
    
}
