#[derive(Copy, Clone)]
struct Node<T> {
    data: T,
    next: Option<usize>,
}

pub struct StaticList<T, const N: usize> {
    nodes: [Option<Node<T>>; N],
    head: Option<usize>,
    free: Vec<usize>,
}

impl<T: Copy + PartialEq, const N: usize> StaticList<T, N> {
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

    /// Inserts at the tail of the list.
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

    /// Gets the value at index (0-based).
    pub fn get(&self, mut i: usize) -> Option<T> {
        let mut current = self.head?;
        while i > 0 {
            current = self.nodes[current]?.next?;
            i -= 1;
        }
        Some(self.nodes[current]?.data)
    }
}
