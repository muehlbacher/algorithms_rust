mod heap;

fn main() {
    // Comparator for max-heap
    let max_cmp = |a: &i32, b: &i32| a > b;

    let mut max_pq = heap::Heap::new(max_cmp);

    // Insert elements into the max-heap
    max_pq.pq_insert(10);
    max_pq.pq_insert(20);
    max_pq.pq_insert(5);
    max_pq.pq_insert(15);
    max_pq.pq_insert(21);
    max_pq.pq_insert(6);

    // Print the priority queue elements
    println!("Max-Heap elements: {:?}", &max_pq.q[1..=max_pq.n]);

    // Comparator for min-heap
    let min_cmp = |a: &i32, b: &i32| a < b;

    let mut min_pq = heap::Heap::new(min_cmp);

    // Insert elements into the min-heap
    min_pq.pq_insert(10);
    min_pq.pq_insert(20);
    min_pq.pq_insert(5);
    min_pq.pq_insert(15);

    // Print the priority queue elements
    println!("Min-Heap elements: {:?}", &min_pq.q[1..=min_pq.n]);
}
