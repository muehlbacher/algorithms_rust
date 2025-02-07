## Algorithms and Data Structures in Rust


### Data Structures and Operation Complexity:

## Data Structure Operations

<table>
  <tr>
    <th><strong>Data Structure</strong></th>
    <th><strong>Search</strong></th>
    <th><strong>Insert</strong></th>
    <th><strong>Delete</strong></th>
    <th><strong>Access</strong></th>
  </tr>
  <tr>
    <td><strong>Array</strong></td>
    <td>O(n)</td>
    <td>O(n)</td>
    <td>O(n)</td>
    <td>O(1)</td>
  </tr>
  <tr>
    <td><strong>Linked List</strong></td>
    <td>O(n)</td>
    <td>O(1)</td>
    <td>O(1)</td>
    <td>O(n)</td>
  </tr>
  <tr>
    <td><strong>Stack</strong></td>
    <td>O(n)</td>
    <td>O(1)</td>
    <td>O(1)</td>
    <td>O(1)</td>
  </tr>
  <tr>
    <td><strong>Queue</strong></td>
    <td>O(n)</td>
    <td>O(1)</td>
    <td>O(1)</td>
    <td>O(1)</td>
  </tr>
  <tr>
    <td><strong>Hash Table</strong></td>
    <td>O(1) (avg)</td>
    <td>O(1) (avg)</td>
    <td>O(1) (avg)</td>
    <td>O(1) (avg)</td>
  </tr>
  <tr>
    <td><strong>Binary Search Tree</strong></td>
    <td>O(log n) (avg)</td>
    <td>O(log n) (avg)</td>
    <td>O(log n) (avg)</td>
    <td>O(log n) (avg)</td>
  </tr>
  <tr>
    <td><strong>AVL Tree</strong></td>
    <td>O(log n)</td>
    <td>O(log n)</td>
    <td>O(log n)</td>
    <td>O(log n)</td>
  </tr>
  <tr>
    <td><strong>Heap</strong></td>
    <td>O(n)</td>
    <td>O(log n)</td>
    <td>O(log n)</td>
    <td>O(1)</td>
  </tr>
  <tr>
    <td><strong>Graph</strong></td>
    <td>O(V + E)</td>
    <td>O(1)</td>
    <td>O(V + E)</td>
    <td>O(1) (adj list)</td>
  </tr>
  <tr>
    <td><strong>Trie</strong></td>
    <td>O(k)</td>
    <td>O(k)</td>
    <td>O(k)</td>
    <td>O(k)</td>
  </tr>
</table>


### Sorting Algorithms
## Sorting Algorithms Complexity Comparison

<table>
  <tr>
    <th><strong>Sorting Algorithm</strong></th>
    <th><strong>Best Case</strong></th>
    <th><strong>Average Case</strong></th>
    <th><strong>Worst Case</strong></th>
    <th><strong>Space Complexity</strong></th>
  </tr>
  <tr>
    <td><strong>Bubble Sort</strong></td>
    <td>O(n)</td>
    <td>O(n^2)</td>
    <td>O(n^2)</td>
    <td>O(1)</td>
  </tr>
  <tr>
    <td><strong>Selection Sort</strong></td>
    <td>O(n^2)</td>
    <td>O(n^2)</td>
    <td>O(n^2)</td>
    <td>O(1)</td>
  </tr>
  <tr>
    <td><strong>Insertion Sort</strong></td>
    <td>O(n)</td>
    <td>O(n^2)</td>
    <td>O(n^2)</td>
    <td>O(1)</td>
  </tr>
  <tr>
    <td><strong>Merge Sort</strong></td>
    <td>O(n log n)</td>
    <td>O(n log n)</td>
    <td>O(n log n)</td>
    <td>O(n)</td>
  </tr>
  <tr>
    <td><strong>Quick Sort</strong></td>
    <td>O(n log n)</td>
    <td>O(n log n)</td>
    <td>O(n^2)</td>
    <td>O(log n)</td>
  </tr>
  <tr>
    <td><strong>Heap Sort</strong></td>
    <td>O(n log n)</td>
    <td>O(n log n)</td>
    <td>O(n log n)</td>
    <td>O(1)</td>
  </tr>
  <tr>
    <td><strong>Radix Sort</strong></td>
    <td>O(nk)</td>
    <td>O(nk)</td>
    <td>O(nk)</td>
    <td>O(n + k)</td>
  </tr>
  <tr>
    <td><strong>Counting Sort</strong></td>
    <td>O(n + k)</td>
    <td>O(n + k)</td>
    <td>O(n + k)</td>
    <td>O(k)</td>
  </tr>
  <tr>
    <td><strong>Bucket Sort</strong></td>
    <td>O(n + k)</td>
    <td>O(n^2)</td>
    <td>O(n^2)</td>
    <td>O(n)</td>
  </tr>
  <tr>
    <td><strong>Shell Sort</strong></td>
    <td>O(n log n)</td>
    <td>O(n^3/2)</td>
    <td>O(n^2)</td>
    <td>O(1)</td>
  </tr>
  <tr>
    <td><strong>Tim Sort</strong></td>
    <td>O(n)</td>
    <td>O(n log n)</td>
    <td>O(n log n)</td>
    <td>O(n)</td>
  </tr>
</table>



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


## Stack
A stack is an abstract data type, while a linked list is a data structure. A stack can be implemented using a linked list, but they are not the same thing.
A stack is a LIFO (Last In, First Out) data structure, It supports two main operations:
- Push: Add an element to the top
- Pop: Remove and return the top element

A stack does not specify how it is implemented (array or linked list); it just defines behavior.

### Stack Comparison: Linked List vs. Array (Vec)

<table>
  <tr>
    <th>Feature</th>
    <th>Linked List Stack</th>
    <th>Array (Vec) Stack</th>
  </tr>
  <tr>
    <td><strong>Memory</strong></td>
    <td>Uses extra memory for pointers</td>
    <td>More memory-efficient</td>
  </tr>
  <tr>
    <td><strong>Performance</strong></td>
    <td>O(1) push/pop</td>
    <td>O(1) push/pop (unless resizing)</td>
  </tr>
  <tr>
    <td><strong>Resizing</strong></td>
    <td>Dynamically grows</td>
    <td>May require reallocation</td>
  </tr>
  <tr>
    <td><strong>Cache Efficiency</strong></td>
    <td>Poor (scattered in memory)</td>
    <td>Good (contiguous memory)</td>
  </tr>
</table>


## ADT:
An abstract data type defines a set of operations and behavior without specifying how they are implemented.
-> an ADT defines what a data structure does, not how it does it.

### Stack: Abstract Data Type vs. Data Structure

<table>
  <tr>
    <th>Aspect</th>
    <th>Stack (ADT)</th>
    <th>Stack (Data Structure)</th>
  </tr>
  <tr>
    <td><strong>Definition</strong></td>
    <td>A set of operations (push, pop, peek)</td>
    <td>A concrete implementation of a stack</td>
  </tr>
  <tr>
    <td><strong>Implementation</strong></td>
    <td>Not defined</td>
    <td>Can use arrays, linked lists, etc.</td>
  </tr>
  <tr>
    <td><strong>Flexibility</strong></td>
    <td>Abstract, can be implemented in many ways</td>
    <td>Tied to a specific memory structure</td>
  </tr>
</table>

