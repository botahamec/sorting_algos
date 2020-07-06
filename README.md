# sorting_algos
A some sorting algorithms, benchmarked. Made for this article: https://www.section.io/engineering-education/sorting-algorithms/

## Implementation
I've implemented the following algorithms in both Python and Rust:

* Insertion Sort
* Merge Sort
* Quick Sort (The Python implementation uses the middle element as the pivot. The Rust version uses various pivot functions)
* Counting Sort

## Benchmarks
I created a Rust program to benchmark half a dozen sorting algorithms. Here are the results I got. Feel free to write your own tests. For reference, my computer uses an [AMD A4-6300 APU](https://www.newegg.com/amd-a4-series-a4-6300/p/N82E16819113349) (I know, it's bad)

```
Test: Normal (5,000 elements, 0-1,000)
Insertion sort                          ... 17.10322 ms
Merge sort                              ... 13.981659 ms
Quick Sort with first element as pivot  ... 1.065194 ms
Quick Sort with middle element as pivot ... 0.76963 ms
Quick Sort with last element as pivot   ... 0.733984 ms
Quick Sort with random element as pivot ... 0.769882 ms
Middle of three Quick Sort              ... 0.703624 ms
Counting Sort                           ... 0.526621 ms

Test: Short list (1,000 elements, 0-1,000)
Insertion sort                          ... 0.545932 ms
Merge sort                              ... 0.681013 ms
Quick Sort with first element as pivot  ... 0.430774 ms
Quick Sort with middle element as pivot ... 0.216657 ms
Quick Sort with last element as pivot   ... 0.210781 ms
Quick Sort with random element as pivot ... 0.262477 ms
Middle of three Quick Sort              ... 0.184869 ms
Counting Sort                           ... 3.335415 ms

Test: Long list (20,000 elements, 0-1,000)
Insertion sort                          ... 213.55365 ms
Merge sort                              ... 190.12463 ms
Quick Sort with first element as pivot  ... 2.7936058 ms
Quick Sort with middle element as pivot ... 2.120229 ms
Quick Sort with last element as pivot   ... 2.82791 ms
Quick Sort with random element as pivot ... 2.8478389 ms
Middle of three Quick Sort              ... 2.4293318 ms
Counting Sort                           ... 2.034296 ms

Test: Small values (2,000 elements, 0-200)
Insertion sort                          ... 2.0900328 ms
Merge sort                              ... 2.278201 ms
Quick Sort with first element as pivot  ... 0.22092499 ms
Quick Sort with middle element as pivot ... 0.20254199 ms
Quick Sort with last element as pivot   ... 0.19515899 ms
Quick Sort with random element as pivot ... 0.217024 ms
Middle of three Quick Sort              ... 0.18922201 ms
Counting Sort                           ... 0.166287 ms

Test: Large values (2,000 elements, 0-1,000,000)
Insertion sort                          ... 2.117894 ms
Merge sort                              ... 2.461026 ms
Quick Sort with first element as pivot  ... 0.624457 ms
Quick Sort with middle element as pivot ... 0.500679 ms
Quick Sort with last element as pivot   ... 0.523691 ms
Quick Sort with random element as pivot ... 0.60896903 ms
Middle of three Quick Sort              ... 0.417824 ms
Counting Sort                           ... 38.11983 ms

Test: Already Sorted List (5,000 elements, 0-1,000)
Insertion sort                          ... 0.017531 ms
Merge sort                              ... 12.588013 ms
Quick Sort with first element as pivot  ... 23.00558 ms
Quick Sort with middle element as pivot ... 0.651617 ms
Quick Sort with last element as pivot   ... 15.246175 ms
Quick Sort with random element as pivot ... 0.830946 ms
Middle of three Quick Sort              ... 0.659572 ms
Counting Sort                           ... 0.493265 ms
```
