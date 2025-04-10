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
            Some(current) => {
                let mut current = current.as_mut();
                while current.next.is_some() {
                    current = current.next.as_mut().unwrap();
                }
                current.next = Some(new_node);
            }
        }
    }
    /// Gets the value at index (0-based).
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
    // Deletes the first occurrence of the given data.
    pub fn delete_element(&mut self, data: T) -> bool {
        let mut current = self.head.as_mut();
        let mut prev: *mut Box<Node<T>> = std::ptr::null_mut();

        while let Some(node) = current {
            if node.data == data {
                if prev.is_null() {
                    self.head = node.next.take();
                } else {
                    unsafe {
                        (*prev).next = node.next.take();
                    }
                }
                return true;
            }
            prev = node as *mut _;
            current = node.next.as_mut();
        }

        false
    }
    // Inserts data at the given index (0-based).
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
    // Deletes the node at the given index (0-based).
    pub fn delete_at_index(&mut self, index: usize) -> bool {
        if index == 0 {
            if self.head.is_some() {
                self.head = self.head.take().unwrap().next;
                return true;
            }
            return false;
        }

        let mut current = self.head.as_mut();
        let mut i = 0;

        while let Some(node) = current {
            if i == index - 1 {
                if let Some(next_node) = node.next.take() {
                    node.next = next_node.next;
                    return true;
                }
                return false;
            }
            i += 1;
            current = node.next.as_mut();
        }

        false
    }
    // Updates the node at the given index (0-based).
    pub fn update_at_index(&mut self, index: usize, data: T) -> bool {
        let mut current = self.head.as_mut();
        let mut i = 0;

        while let Some(node) = current {
            if i == index {
                node.data = data;
                return true;
            }
            i += 1;
            current = node.next.as_mut();
        }

        false
    }
    // Finds the first occurrence of the given data.
    pub fn find(&self, data: T) -> bool {
        let mut current = self.head.as_ref();

        while let Some(node) = current {
            if node.data == data {
                return true;
            }
            current = node.next.as_ref();
        }

        false
    }
    
}
