mod heap; // Import the heap module

fn main() {
    // Test the parent function
    let mut result = heap::pq_parent(1);
    println!("Parent of 1: {}", result);
    result = heap::pq_parent(3);
    println!("Parent of 3: {}", result);

    // Test the young child function
    result = heap::pq_young_child(3);
    println!("Young child of 3: {}", result);

    // Example usage of the PriorityQueue
    let mut pq: heap::PriorityQueue<i32> = heap::PriorityQueue::new(); // PriorityQueue for integers
    println!("PriorityQueue initialized with size: {}", pq.q.len());
    println!("Number of elements: {}", pq.n);
}
