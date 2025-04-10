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

    pub fn delete_element(&mut self, data: T) -> bool {
        let mut current = self.head.as_mut();
        let mut prev: Option<&mut Box<Node<T>>> = None;

        while let Some(node) = current {
            if node.data == data {
                match prev {
                    Some(prev_node) => {
                        prev_node.next = node.next.take();
                    }
                    None => {
                        self.head = node.next.take();
                    }
                }
                return true;
            }
            prev = Some(current);
            current = node.next.as_mut();
        }

        false
    }

    pub fn insert_at_index(&mut self, index: usize, data: T) -> bool {
        if index == 0 {
            let new_node = Box::new(Node { data, next: self.head.take() });
            self.head = Some(new_node);
            return true;
        }

        let mut current = self.head.as_mut();
        let mut i = 0;

        while let Some(node) = current {
            if i == index - 1 {
                let new_node = Box::new(Node { data, next: node.next.take() });
                node.next = Some(new_node);
                return true;
            }
            i += 1;
            current = node.next.as_mut();
        }

        false
    }
    
}
