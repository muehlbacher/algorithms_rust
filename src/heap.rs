pub const PQ_SIZE: usize = 10; // Example size for the queue

pub struct PriorityQueue<T> {
    pub q: [T; PQ_SIZE + 1], // Array to hold queue elements
    pub n: usize,            // Number of queue elements
}

impl<T: Default + Copy> PriorityQueue<T> {
    // Create a new PriorityQueue
    pub fn new() -> Self {
        Self {
            q: [T::default(); PQ_SIZE + 1], // Initialize with default values
            n: 0,                           // Start with zero elements
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
