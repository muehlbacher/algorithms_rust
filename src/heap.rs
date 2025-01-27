pub const PQ_SIZE: usize = 10; // Example size for the queue

pub struct PriorityQueue<T, F>
where
    F: Fn(&T, &T) -> bool,
{
    pub q: [T; PQ_SIZE + 1], // Array to hold queue elements
    pub n: usize,            // Number of queue elements
    cmp: F,                  //C omparator function
}

impl<T: Default + Copy, F: Fn(&T, &T) -> bool> PriorityQueue<T, F> {
    // Create a new PriorityQueue
    pub fn new(cmp: F) -> Self {
        Self {
            q: [T::default(); PQ_SIZE + 1], // Initialize with default values
            n: 0,                           // Start with zero elements
            cmp,                            // Stores the comparator
        }
    }

    // Insert an element into the priority queue
    pub fn pq_insert(&mut self, x: T) {
        if self.n >= PQ_SIZE {
            panic!("Priority queue overflow");
        } else {
            self.n += 1;
            self.q[self.n] = x;
            self.bubble_up(self.n as i32);
        }
    }
    pub fn bubble_up(&mut self, pos: i32) {
        if pq_parent(pos) == -1 {
            return; //at root of heap
        }

        let parent_index = pq_parent(pos) as usize;
        let current_index = pos as usize;

        if (self.cmp)(&self.q[current_index], &self.q[parent_index]) {
            self.q.swap(current_index, pq_parent(pos as i32) as usize);
            self.bubble_up(pos);
        }
    }
}

// Get the parent index
pub fn pq_parent(n: i32) -> i32 {
    if n == 1 {
        return -1;
    }
    n / 2
}

// Get the left child index
pub fn pq_young_child(n: i32) -> i32 {
    n * 2
}
