use rand::prelude::*;
use rand::distributions;

/// performs a out-of-place insertion sort
fn insertion_sort<T: PartialOrd + Copy>(list: Vec<T>) -> Vec<T> {

	/// inserts a single value into the list
	fn insert<T: PartialOrd + Copy>(list: &mut Vec<T>, value: T) {
		let mut index = list.len(); // the current index of the value
		list.push(value); // adds the value to the end of the list
		while index > 0 && list[index - 1] > value {
			list.swap(index, index - 1);
			index -= 1;
		}
	}

	let mut sorted_list = Vec::<T>::with_capacity(list.len());

	for item in list {insert(&mut sorted_list, item);}

	sorted_list
}

/// performs an out-of-place merge sort
fn merge_sort<T: PartialOrd + Copy + Default>(list: Vec<T>) -> Vec<T> {

	fn sort<T: PartialOrd + Copy>(from: &[T], mut dest: &mut Vec<T>, start: usize, end: usize) {
		let range = end - start;
		let mid = (range / 2) + start;
		if range > 1 {
			sort(from, dest, start, mid);
			sort(from, dest, mid, end);
			merge(&mut dest, start, mid, end);
		}
	}

	fn merge<T: PartialOrd + Copy>(dest: &mut Vec<T>, start: usize, mid: usize, end: usize) {

		let mut i = start; // index in first list
		let mut j = mid; // index in second list
		let mut index = start; // index into new list

		let from = dest.clone();

		while i < mid && j < end {
			if from[i] < from[j] {
				dest[index] = from[i];
				i += 1;
			} else {
				dest[index] = from[j];
				j += 1;
			}

			index += 1;
		} if i < mid {
			for item in from.iter().take(mid).skip(i) {
				dest[index] = *item;
				index += 1;
			}
		} else if j < end {
			for item in from.iter().take(end).skip(j) {
				dest[index] = *item;
				index += 1;
			}
		}
	}

	let mut sorted_list = list.clone();
	sort(&list, &mut sorted_list, 0, list.len());

	sorted_list
}

/// Peforms a quick sort with the given pivot function
fn quick_sort<T: PartialOrd + Copy>(list: Vec<T>, pivot_fn: &dyn Fn(&[T]) -> T) -> Vec<T> {

	fn sort<T: PartialOrd + Copy>(list: Vec<T>, pivot_fn: &dyn Fn(&[T]) -> T) -> Vec<T> {

		let pivot = pivot_fn(&list); // select a pivot

		// intialize lists
		let mut less_list = Vec::with_capacity(list.len() / 3 + 1);
		let mut more_list = Vec::with_capacity(list.len() / 3 + 1);
		let mut eq_list = Vec::with_capacity(list.len() / 3);

		for item in list {
			if item < pivot {less_list.push(item)}
			else if item > pivot {more_list.push(item)}
			else {eq_list.push(item)}
		}

		// sort both lists
		if less_list.len() > 1 {less_list = sort(less_list, pivot_fn)}
		if more_list.len() > 1 {more_list = sort(more_list, pivot_fn)}

		// combine lists
		less_list.append(&mut eq_list);
		less_list.append(&mut more_list);

		less_list
	}

	sort(list.clone(), pivot_fn)
}

fn counting_sort(list: Vec<u8>) -> Vec<u8> {

	use std::collections::HashMap;

	let mut items = HashMap::<u8, usize>::with_capacity(list.len());

	for item in &list {
		if items.contains_key(item) {
			let value = items.get(item).unwrap();
			items.insert(*item, value + 1);
		} else {
			items.insert(*item, 1);
		}
	}

	let mut sorted_list = Vec::with_capacity(list.len());
	let mut current = 0;

	while current < std::u8::MAX {
		if let Some(n) = items.get(&current) {
			for _i in 0..*n {
				sorted_list.push(current);
			}
		}

		println!("{}", current);
		current += 1;
	}

	sorted_list
}


fn first_element_quicksort<T: PartialOrd + Copy>(list: Vec<T>) -> Vec<T> {
	fn first_element<T: Copy>(list: &[T]) -> T {
		list[0]
	}

	quick_sort(list, &first_element)
}

fn last_element_quicksort<T: PartialOrd + Copy>(list: Vec<T>) -> Vec<T> {
	fn last_element<T: Copy>(list: &[T]) -> T {
		list[list.len() - 1]
	}

	quick_sort(list, &last_element)
}

fn middle_element_quicksort<T: PartialOrd + Copy>(list: Vec<T>) -> Vec<T> {
	fn middle_element<T: Copy>(list: &[T]) -> T {
		list[list.len() / 2]
	}

	quick_sort(list, &middle_element)
}

fn random_quicksort<T: PartialOrd + Copy>(list: Vec<T>) -> Vec<T>
		where distributions::Standard: distributions::Distribution<T> {
	fn random_element<T: Copy>(list: &[T]) -> T {
		list[rand::random::<usize>() % list.len()]
	}

	quick_sort(list, &random_element)
}

fn mid_of_three_quicksort<T: Ord + Copy>(list: Vec<T>) -> Vec<T> {
	fn best_of_three<T: Ord + Copy>(list: &[T]) -> T {

		fn mid_of_three<T: Ord + Copy>(a: T, b: T, c: T) -> T {
			use std::cmp::{min, max};
			max(min(a, b), min(max(a, b), c))
		}

		let first_element = list[0];
		let middle_element = list[list.len() / 2];
		let last_element = list[list.len() - 1];

		mid_of_three(first_element, middle_element, last_element)
	}

	quick_sort(list, &best_of_three)
}

/// Randomly creates a list of a specifed type and length
fn generate_list<T>(length: usize) -> Vec<T>
		where distributions::Standard: distributions::Distribution<T> {

	// creates a list with the correct capacity
	let mut list = Vec::<T>::with_capacity(length);
	let mut rng = rand::thread_rng();

	// fills the list with randomly generated elements
	for _i in 0..length {list.push(rng.gen());}

	list
}

fn main() {
	
}

mod test {
	use super::*;

	#[allow(dead_code)]
	fn check_sort_fn(sort_fn: &dyn Fn(Vec<u8>) -> Vec<u8>) {

		fn check_sorted(list: Vec<u8>) {
			if list.len() > 1 {
				for index in 1..list.len() {
					if list[index] < list[index - 1] {
						panic!("Error at {}\n{:?}", index, list);
					}
				}
			}
		}

		fn long_list() -> Vec<u8> {
			generate_list::<u8>(std::u8::MAX as usize)
		}

		check_sorted(sort_fn(long_list()))
	}

	fn check_sorted(list: Vec<usize>) {
		if list.len() > 1 {
			for index in 1..list.len() {
				if list[index] < list[index - 1] {
					panic!("Error at {}\n{:?}", index, list);
				}
			}
		}
	}

	#[test]
	fn test_insertion_sort() {
		check_sort_fn(&insertion_sort)
	}

	#[test]
	fn test_merge_sort() {
		check_sort_fn(&merge_sort)
	}

	#[test]
	fn test_quick_sort() {
		check_sort_fn(&first_element_quicksort);
		check_sort_fn(&middle_element_quicksort);
		check_sort_fn(&last_element_quicksort);
		check_sort_fn(&random_quicksort);
		check_sort_fn(&mid_of_three_quicksort);
	}

	#[test]
	fn test_counting_sort() {
		check_sort_fn(&counting_sort)
	}
}