## Algorithms in Rust


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


Heap:
insert(), delete(): `O(log(n))`
delete(): `O(log(n))`
getMin(), getMax(): `O(1)`


# Heapsort:
`O(n * log(n))`
It is basically an implementation of the selection sort using the right data structure.


