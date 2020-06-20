use rand::prelude::*;
use rand::distributions;

/// performs a out-of-place insertion sort
fn insertion_sort<T: PartialOrd + Copy>(list: Vec<T>) -> Vec<T> {

	/// inserts a single value into the list
	fn insert<T: PartialOrd + Copy>(mut list: &mut Vec<T>, value: T) {

		/// swaps two values
		fn swap<T: Copy>(list: &mut Vec<T>, first: usize, second: usize) {
			let temp = list[first];
			list[first] = list[second];
			list[second] = temp;
		}

		let mut index = list.len(); // the current index of the value
		list.push(value); // adds the value to the end of the list
		while index > 0 && list[index - 1] > value {
			swap(&mut list, index, index - 1);
			index -= 1;
		}
	}

	let mut sorted_list = Vec::<T>::with_capacity(list.len());

	for item in list {insert(&mut sorted_list, item);}

	sorted_list
}

/// performs an out-of-place merge sort
fn merge_sort<T: PartialOrd + Copy + Default + std::fmt::Debug>(list: Vec<T>) -> Vec<T> {

	fn sort<T: PartialOrd + Copy + std::fmt::Debug>(from: &Vec<T>, mut dest: &mut Vec<T>, start: usize, end: usize) {
		let range = end - start;
		let mid = (range / 2) + start;
		if range > 1 {
			sort(from, dest, start, mid);
			sort(from, dest, mid, end);
			merge(&mut dest, start, mid, end);
		}
	}

	fn merge<T: PartialOrd + Copy + std::fmt::Debug>(dest: &mut Vec<T>, start: usize, mid: usize, end: usize) {

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
			for k in i..mid {
				dest[index] = from[k];
				index += 1;
			}
		} else if j < end {
			for k in j..end {
				dest[index] = from[k];
				index += 1;
			}
		}
	}

	let mut sorted_list = list.clone();
	sort(&list, &mut sorted_list, 0, list.len());

	sorted_list
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
	fn check_sort_fn(sort_fn: &dyn Fn(Vec<usize>) -> Vec<usize>) {

		fn check_sorted(list: Vec<usize>) {
			if list.len() > 1 {
				for index in 1..list.len() {
					if list[index] < list[index - 1] {
						panic!("Error at {}\n{:?}", index, list);
					}
				}
			}
		}

		fn long_list() -> Vec<usize> {
			generate_list::<usize>(u8::MAX as usize)
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
}