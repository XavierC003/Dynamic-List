#[derive(Copy, Clone)]
struct Node<T> {
    data: T,
    next: Option<usize>,
}

/// A static array-based singly linked list implementation.
/// # Type Parameters
/// `T`: The type of data stored in the list.
/// `N`: The maximum number of elements in the list.
pub struct StaticList<T, const N: usize> {
    nodes: [Option<Node<T>>; N],
    head: Option<usize>,
    free: Vec<usize>,
}

impl<T: Copy + PartialEq, const N: usize> StaticList<T, N> {
    /// Creates a new static list
    pub fn new() -> Self {
        let mut free = Vec::with_capacity(N);
        for i in (0..N).rev() {
            free.push(i);
        }

        StaticList {
            nodes: [None; N],
            head: None,
            free,
        }
    }

    /// Inserts a new element at the tail (end) of the list.
    /// # Parameters
    /// `data`: The data to insert.
    /// # Returns
    /// `true` if insertion is successful.
    /// `false` if the list is full.
    pub fn insert(&mut self, data: T) -> bool {
        if let Some(index) = self.free.pop() {
            let new_node = Node { data, next: None };

            if let Some(mut current_index) = self.head {
                while let Some(next_index) = self.nodes[current_index].as_ref().unwrap().next {
                    current_index = next_index;
                }
                self.nodes[current_index].as_mut().unwrap().next = Some(index);
            } else {
                self.head = Some(index);
            }

            self.nodes[index] = Some(new_node);
            true
        } else {
            false // no space
        }
    }

    /// Retrieves the value at a given index (0-based).
    /// # Parameters
    /// `i`: The index of the element to retrieve.
    /// # Returns
    /// `Some(value)` if the index is valid.
    /// `None` otherwise.
    pub fn get(&self, mut i: usize) -> Option<T> {
        let mut current = self.head?;
        while i > 0 {
            current = self.nodes[current]?.next?;
            i -= 1;
        }
        Some(self.nodes[current]?.data)
    }
    /// Insert data at index (0-based).
    /// # Parameters
    /// `index`: The index at which to insert the data.
    /// `data`: The data to insert.
    /// Returns true if successful, false otherwise.
    pub fn insert_at_index(&mut self, index: usize, data: T) -> bool {
        if index >= N {
            return false;
        }
        if let Some(free_index) = self.free.pop() {
            let new_node = Node { data, next: None };
            if index == 0 {
               let old_head = self.head.take();
                self.head = Some(free_index);
                self.nodes[free_index] = Some(new_node);
                self.nodes[free_index].as_mut().unwrap().next = old_head;
                return true
            }
            let mut current = self.head;
            let mut prev_index = None;

            while let Some(current_index) = current {
                if current_index == index {
                    break;
                }
                prev_index = current;
                current = self.nodes[current_index].as_ref().and_then(|n| n.next);
            }
            if let Some(prev_index) = prev_index {
                self.nodes[free_index] = Some(new_node);
                let node = self.nodes[prev_index].as_mut().unwrap();
                node.next = Some(free_index);
                return true;
            }
        } 
        false // failed to insert
    }

    /// Deletes the first occurrence of a specific value in the list.
    /// # Parameters
    /// `data`: The data to delete.
    /// # Returns
    /// `true` if the data is found and deleted.
    /// `false` if the data is not found.
    pub fn delete_element(&mut self, data: T) -> bool {
        let mut current_index: Option<usize> = self.head;
        let mut prev_index: Option<usize> = None;

        while let Some(index) = current_index {
            if self.nodes[index].as_ref().unwrap().data == data {
                if let Some(prev) = prev_index {
                    let next = self.nodes[index].as_ref().unwrap().next;
                    self.nodes[prev].as_mut().unwrap().next = next;
                } else {
                    self.head = self.nodes[index].as_ref().unwrap().next;
                }

                self.free.push(index);
                self.nodes[index] = None;
                return true;
            }
            prev_index = current_index;
            current_index = self.nodes[index].as_ref().unwrap().next;
        }
        false // data not found
    }
    /// Deletes the node at index (0-based).
    ///  # Parameters
    /// `index`: The index of the node to delete.
    /// Returns true if successful, false otherwise.
    pub fn delete_at_index(&mut self, index: usize) -> bool {
        if index >= N {
            return false;
        }
        let mut current_index: Option<usize> = self.head;
        let mut prev_index: Option<usize> = None;

        while let Some(current) = current_index {
            if current == index {
                if let Some(prev) = prev_index {
                    let next = self.nodes[current].as_ref().unwrap().next;
                    self.nodes[prev].as_mut().unwrap().next = next;
                } else {
                    self.head = self.nodes[current].as_ref().unwrap().next;
                }
                self.nodes[current] = None;
                self.free.push(current);
                return true;
            }
            prev_index = current_index;
            current_index = self.nodes[current].as_ref().unwrap().next;
        }
        false
    }
    
    /// Updates the first occurrence of the given data.
    /// # Parameters
    /// `old_data`: The value to search for.
    /// `new_data`: The new value to replace it with.
    /// Returns true if successful, false otherwise.
    pub fn update_element(&mut self, old_data: T, new_data: T) -> bool {
        let mut current_index = self.head;

        while let Some(index) = current_index {
            if self.nodes[index].as_ref().unwrap().data == old_data {
                self.nodes[index].as_mut().unwrap().data = new_data;
                return true;
            }
            current_index = self.nodes[index].as_ref().unwrap().next;
        }
        false // data not found
    }

    /// Updates the node at index (0-based).
    /// `index`: The index of the node to update.
    /// `data`: The new value to assign.
    /// Returns true if successful, false otherwise.
    pub fn update_at_index(&mut self, index: usize, data: T) -> bool {
        if index >= N {
            return false;
        }

        let mut current_index = self.head;
        let mut i = 0;
        
        while let Some(current) = current_index {
            if i == index {
                self.nodes[current].as_mut().unwrap().data = data;
                return true;
            }
            i += 1;
            current_index = self.nodes[current].as_ref().unwrap().next;
        }
        false
    }

    /// Checks if the list contains the given data.
    /// # Parameters
    /// `data`: The data to find.
    /// `true` if the data exists in the list.
    /// `false` otherwise.
    pub fn find(&self, data: T) -> bool {
        let mut current_index = self.head;

        while let Some(index) = current_index {
            if self.nodes[index].as_ref().unwrap().data == data {
                return true;
            }
            current_index = self.nodes[index].as_ref().unwrap().next;
        }
        false // data not found
    }


    

}
