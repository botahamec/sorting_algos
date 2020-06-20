use rand::prelude::*;
use rand::distributions;

/// performs a out-of-place insertion sort
fn insertion_sort<T: PartialOrd + Copy>(list: Vec<T>) -> Vec<T> {

	/// inserts a single value into the list
	fn insert<T: PartialOrd + Copy>(mut list: &mut Vec<T>, value: T) {

		/// swaps two values
		fn swap<T: Copy>(list: &mut Vec<T>, first: usize, second: usize) {
			let temp = list[first].clone();
			list[first] = list[second].clone();
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

	for index in 0..list.len() {insert(&mut sorted_list, list[index]);}

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
	println!("{:?}", insertion_sort(generate_list::<u8>(15)))
}

mod test {
	use super::*;

	fn check_sorted(list: Vec<usize>) {
		if list.len() > 1 {
			for index in 1..list.len() {
				if list[index] < list[index - 1] {
					panic!();
				}
			}
		}
	}

	fn long_list() -> Vec<usize> {
		generate_list::<usize>(u8::MAX as usize)
	}

	fn check_sort_fn(sort_fn: &dyn Fn(Vec<usize>) -> Vec<usize>) {
		check_sorted(sort_fn(long_list()))
	}

	#[test]
	fn check_insertion_sort() {
		check_sort_fn(&insertion_sort)
	}
}