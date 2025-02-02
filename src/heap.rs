pub const PQ_SIZE: usize = 10; // Example size for the queue

pub struct Heap<T, F>
where
    F: Fn(&T, &T) -> bool,
{
    pub q: [T; PQ_SIZE + 1], // Array to hold queue elements
    pub n: usize,            // Number of queue elements
    cmp: F,                  //C omparator function
}

impl<T: Default + Copy, F: Fn(&T, &T) -> bool> Heap<T, F> {
    // Create a new PriorityQueue
    pub fn new(cmp: F) -> Self {
        Self {
            q: [T::default(); PQ_SIZE + 1], // Initialize with default values
            n: 0,                           // Start with zero elements
            cmp,                            // Stores the comparator
        }
    }

    pub fn make_heap(&mut self, arr: &[T]) {
        for &item in arr {
            self.pq_insert(item);
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
            self.q.swap(current_index, parent_index);
            self.bubble_up(parent_index as i32);
        }
    }
    pub fn extract_min(&mut self) -> T {
        let min: T;
        if self.n <= 0 {
            println!("Warning: empty priority queue. \n");
            return self.q[0];
        } else {
            min = self.q[1];

            self.q[1] = self.q[self.n];
            self.n = self.n - 1;
            self.bubble_down(1);
        }
        return min;
    }
    pub fn bubble_down(&mut self, pos: i32) {
        let c: usize;
        let mut i: usize = 0;
        let mut min_index: usize;

        c = pq_young_child(pos as usize);
        min_index = pos as usize;

        while i <= 1 {
            if c + i <= self.n {
                if (self.cmp)(&self.q[c + i], &self.q[min_index]) {
                    min_index = c + i;
                }
            }
            i += 1;
        }

        if min_index != pos as usize {
            self.q.swap(pos as usize, min_index);
            self.bubble_down(min_index as i32);
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

//Get the left child index
pub fn pq_young_child(n: usize) -> usize {
    n * 2
}

pub fn heapsort_(arr: &mut [i32]) {
    let min_cmp = |a: &i32, b: &i32| a < b;

    let mut min_pq = Heap::new(min_cmp);

    min_pq.make_heap(arr);

    for elem in arr.iter_mut() {
        *elem = min_pq.extract_min()
    }
}
