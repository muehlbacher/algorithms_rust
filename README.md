## Algorithms and Data Structures in Rust


### Data Structures and Operation Complexity:

| **Data Structure**     | **Search**        | **Insert**        | **Delete**        | **Access**       |
|------------------------|-------------------|-------------------|-------------------|------------------|
| **Array**              | O(n)              | O(n)              | O(n)              | O(1)             |
| **Linked List**        | O(n)              | O(1)              | O(1)              | O(n)             |
| **Stack**              | O(n)              | O(1)              | O(1)              | O(1)             |
| **Queue**              | O(n)              | O(1)              | O(1)              | O(1)             |
| **Hash Table**         | O(1) (avg)        | O(1) (avg)        | O(1) (avg)        | O(1) (avg)       |
| **Binary Search Tree** | O(log n) (avg)    | O(log n) (avg)    | O(log n) (avg)    | O(log n) (avg)   |
| **AVL Tree**           | O(log n)          | O(log n)          | O(log n)          | O(log n)         |
| **Heap**               | O(n)              | O(log n)          | O(log n)          | O(1)             |
| **Graph**              | O(V + E)          | O(1)              | O(V + E)          | O(1) (adj list)  |
| **Trie**               | O(k)              | O(k)              | O(k)   

### Sorting Algorithms

| **Sorting Algorithm**      | **Best Case**     | **Average Case**   | **Worst Case**    | **Space Complexity** |
|----------------------------|-------------------|--------------------|-------------------|----------------------|
| **Bubble Sort**            | O(n)              | O(n^2)             | O(n^2)            | O(1)                 |
| **Selection Sort**         | O(n^2)            | O(n^2)             | O(n^2)            | O(1)                 |
| **Insertion Sort**         | O(n)              | O(n^2)             | O(n^2)            | O(1)                 |
| **Merge Sort**             | O(n log n)        | O(n log n)         | O(n log n)        | O(n)                 |
| **Quick Sort**             | O(n log n)        | O(n log n)         | O(n^2)            | O(log n)             |
| **Heap Sort**              | O(n log n)        | O(n log n)         | O(n log n)        | O(1)                 |
| **Radix Sort**             | O(nk)             | O(nk)              | O(nk)             | O(n + k)             |
| **Counting Sort**          | O(n + k)          | O(n + k)           | O(n + k)          | O(k)                 |
| **Bucket Sort**            | O(n + k)          | O(n^2)             | O(n^2)            | O(n)                 |
| **Shell Sort**             | O(n log n)        | O(n^3/2)           | O(n^2)            | O(1)                 |
| **Tim Sort**               | O(n)              | O(n log n)         | O(n log n)        | O(n)                 |


### Heaps
            20
          /    \
        15      10
       /  \    /   \
      8    5  7     3
     /  \
    4    2

[20, 15, 10, 8, 5, 7, 3, 4, 2]


Heaps are a simple and elegant data structure for efficiently supporting the 
priority queue operations insert and extract-min. Heaps work by maintaining a
partial order on the set of elements that is weaker than the sorted order (so it
can be efficient to maintain) yet stronger than random order (so the minimum
element can be quickly identified).

**Use case**: Heaps are commonly used in implementing priority queues, scheduling algorithms, and in algorithms like heap sort.

In this spirit, a heap-labeled tree is defined to be a binary tree such that
the key of **each node dominates the keys of its children**. In a min-heap, a node
dominates its children by having a smaller key than they do, while in a maxheap
parent nodes dominate by being bigger.
The heap is a slick data structure that enables
us to represent binary trees without using any pointers. 
We store the data as an array of keys, and use the position of the keys to implicity play the role of the pointers.

- Heapsort:
`O(n * log(n))`
It is basically an implementation of the selection sort using the right data structure.  



